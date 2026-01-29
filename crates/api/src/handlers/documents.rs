use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use shared::dto::documents::{
    CreateDocumentRequest, DocumentFilters, UpdateDocumentRequest, UpdateStatusRequest,
};
use shared::Document;

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::documents as service;
use crate::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(filters): Query<DocumentFilters>,
) -> Result<Json<Vec<Document>>, AppError> {
    let documents = service::list(&state.pool, &filters).await?;
    Ok(Json(documents))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Document>, AppError> {
    let document = service::get_by_id(&state.pool, id).await?;
    Ok(Json(document))
}

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Document>), AppError> {
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut metadata: Option<CreateDocumentRequest> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Invalid multipart data: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" => {
                file_name = field.file_name().map(|n| n.to_string());
                content_type = field.content_type().map(|ct| ct.to_string());
                file_bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| AppError::Validation(format!("Failed to read file: {e}")))?
                        .to_vec(),
                );
            }
            "metadata" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::Validation(format!("Failed to read metadata: {e}")))?;
                metadata = Some(serde_json::from_str(&text).map_err(|e| {
                    AppError::Validation(format!("Invalid metadata JSON: {e}"))
                })?);
            }
            _ => {}
        }
    }

    let file_bytes =
        file_bytes.ok_or_else(|| AppError::Validation("Missing file field".to_string()))?;
    let file_name =
        file_name.ok_or_else(|| AppError::Validation("Missing file name".to_string()))?;
    let content_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());
    let metadata =
        metadata.ok_or_else(|| AppError::Validation("Missing metadata field".to_string()))?;

    let document = service::create(
        &state.pool,
        &state.config,
        metadata,
        file_bytes,
        &file_name,
        &content_type,
        user.id,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(document)))
}

pub async fn update_metadata(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateDocumentRequest>,
) -> Result<Json<Document>, AppError> {
    let document = service::update_metadata(&state.pool, id, data).await?;
    Ok(Json(document))
}

pub async fn update_status(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateStatusRequest>,
) -> Result<Json<Document>, AppError> {
    let document = service::update_status(&state.pool, id, request).await?;
    Ok(Json(document))
}

pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete(&state.pool, &state.config, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
