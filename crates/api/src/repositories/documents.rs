use chrono::{DateTime, Utc};
use shared::dto::documents::{CreateDocumentRequest, DocumentFilters, UpdateDocumentRequest};
use shared::{Document, DocumentStatus};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(
    pool: &PgPool,
    filters: &DocumentFilters,
    cursor_updated_at: Option<DateTime<Utc>>,
    cursor_id: Option<Uuid>,
    limit: i32,
) -> Result<(Vec<Document>, i64), sqlx::Error> {
    let rows = sqlx::query_as::<_, Document>(
        "SELECT * FROM documents
         WHERE ($1::uuid IS NULL OR machine_id = $1)
           AND ($2::text IS NULL OR status = $2)
           AND ($3::uuid IS NULL OR document_type_id = $3)
           AND (
               $4::timestamptz IS NULL
               OR (updated_at, id) < ($4, $5::uuid)
           )
         ORDER BY updated_at DESC, id DESC
         LIMIT $6",
    )
    .bind(filters.machine_id)
    .bind(&filters.status)
    .bind(filters.document_type_id)
    .bind(cursor_updated_at)
    .bind(cursor_id)
    .bind(i64::from(limit))
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM documents
         WHERE ($1::uuid IS NULL OR machine_id = $1)
           AND ($2::text IS NULL OR status = $2)
           AND ($3::uuid IS NULL OR document_type_id = $3)",
    )
    .bind(filters.machine_id)
    .bind(&filters.status)
    .bind(filters.document_type_id)
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Document>, sqlx::Error> {
    sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    data: &CreateDocumentRequest,
    storage_path: &str,
    revision: i32,
    uploader_id: Uuid,
) -> Result<Document, sqlx::Error> {
    sqlx::query_as::<_, Document>(
        "INSERT INTO documents (machine_id, document_type_id, title, storage_path, revision, status, uploader_id)
         VALUES ($1, $2, $3, $4, $5, 'draft', $6)
         RETURNING *",
    )
    .bind(data.machine_id)
    .bind(data.document_type_id)
    .bind(&data.title)
    .bind(storage_path)
    .bind(revision)
    .bind(uploader_id)
    .fetch_one(pool)
    .await
}

pub async fn update_metadata(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateDocumentRequest,
) -> Result<Option<Document>, sqlx::Error> {
    sqlx::query_as::<_, Document>(
        "UPDATE documents SET
            title = COALESCE($2, title),
            machine_id = COALESCE($3, machine_id)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.title)
    .bind(data.machine_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_status(
    pool: &PgPool,
    id: Uuid,
    status: &DocumentStatus,
) -> Result<Option<Document>, sqlx::Error> {
    let status_str = match status {
        DocumentStatus::Draft => "draft",
        DocumentStatus::Pending => "pending",
        DocumentStatus::Approved => "approved",
        DocumentStatus::Rejected => "rejected",
        DocumentStatus::Archived => "archived",
    };

    sqlx::query_as::<_, Document>(
        "UPDATE documents SET status = $2 WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(status_str)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Option<Document>, sqlx::Error> {
    sqlx::query_as::<_, Document>("DELETE FROM documents WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Get the latest revision number for a given machine + document_type combo.
pub async fn get_latest_revision(
    pool: &PgPool,
    machine_id: Option<Uuid>,
    document_type_id: Uuid,
) -> Result<i32, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT COALESCE(MAX(revision), 0) FROM documents
         WHERE ($1::uuid IS NULL AND machine_id IS NULL OR machine_id = $1)
           AND document_type_id = $2",
    )
    .bind(machine_id)
    .bind(document_type_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(0))
}
