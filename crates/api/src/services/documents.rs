use chrono::DateTime;
use shared::dto::documents::{
    CreateDocumentRequest, DocumentFilters, UpdateDocumentRequest, UpdateStatusRequest,
};
use shared::dto::pagination::PaginatedResponse;
use shared::Document;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::errors::AppError;
use crate::repositories::documents as repo;
use crate::repositories::pagination::{clamp_limit, decode_cursor, encode_cursor};
use crate::services::storage;

pub async fn list(
    pool: &PgPool,
    filters: &DocumentFilters,
) -> Result<PaginatedResponse<Document>, AppError> {
    let limit = clamp_limit(filters.limit);
    let (cursor_updated_at, cursor_id) = match &filters.cursor {
        Some(c) => {
            let (ts_str, id) = decode_cursor(c)?;
            let ts = DateTime::parse_from_rfc3339(&ts_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|_| AppError::Validation("Invalid cursor timestamp".to_string()))?;
            (Some(ts), Some(id))
        }
        None => (None, None),
    };

    let (mut rows, total) =
        repo::find_all(pool, filters, cursor_updated_at, cursor_id, limit + 1).await?;

    let has_more = rows.len() > limit as usize;
    if has_more {
        rows.pop();
    }

    let next_cursor = if has_more {
        rows.last()
            .map(|d| encode_cursor(&d.updated_at.to_rfc3339(), d.id))
    } else {
        None
    };

    Ok(PaginatedResponse {
        items: rows,
        next_cursor,
        total,
    })
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Document, AppError> {
    repo::find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Document {id} not found")))
}

pub async fn create(
    pool: &PgPool,
    config: &AppConfig,
    data: CreateDocumentRequest,
    file_bytes: Vec<u8>,
    file_name: &str,
    content_type: &str,
    uploader_id: Uuid,
) -> Result<Document, AppError> {
    if data.title.trim().is_empty() {
        return Err(AppError::Validation("Document title cannot be empty".to_string()));
    }

    // Auto-increment revision
    let current_rev =
        repo::get_latest_revision(pool, data.machine_id, data.document_type_id).await?;
    let new_revision = current_rev + 1;

    // Build storage path
    let machine_folder = data
        .machine_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "general".to_string());
    let path = format!(
        "{}/{}/rev_{}_{}", machine_folder, data.document_type_id, new_revision, file_name
    );

    // Upload file
    let storage_path =
        storage::upload_file(config, "documents", &path, file_bytes, content_type).await?;

    // Create DB record
    repo::create(pool, &data, &storage_path, new_revision, uploader_id)
        .await
        .map_err(AppError::from)
}

pub async fn update_metadata(
    pool: &PgPool,
    id: Uuid,
    data: UpdateDocumentRequest,
) -> Result<Document, AppError> {
    if let Some(ref title) = data.title {
        if title.trim().is_empty() {
            return Err(AppError::Validation("Document title cannot be empty".to_string()));
        }
    }

    repo::update_metadata(pool, id, &data)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Document {id} not found")))
}

pub async fn update_status(
    pool: &PgPool,
    id: Uuid,
    request: UpdateStatusRequest,
) -> Result<Document, AppError> {
    let document = get_by_id(pool, id).await?;

    if !document.status.can_transition_to(&request.status) {
        return Err(AppError::Validation(format!(
            "Cannot transition from {:?} to {:?}",
            document.status, request.status
        )));
    }

    repo::update_status(pool, id, &request.status)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Document {id} not found")))
}

pub async fn delete(pool: &PgPool, config: &AppConfig, id: Uuid) -> Result<(), AppError> {
    let document = repo::delete(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Document {id} not found")))?;

    // Clean up storage - extract path from "bucket/path" format
    if let Some(path) = document.storage_path.strip_prefix("documents/") {
        if let Err(e) = storage::delete_file(config, "documents", path).await {
            tracing::warn!(error = %e, "Failed to clean up storage for deleted document");
        }
    }

    Ok(())
}
