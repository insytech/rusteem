use shared::dto::locations::{CreateLocationRequest, UpdateLocationRequest};
use shared::Location;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::locations as repo;

pub async fn list(pool: &PgPool) -> Result<Vec<Location>, AppError> {
    repo::find_all(pool).await.map_err(AppError::from)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Location, AppError> {
    repo::find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Location {id} not found")))
}

pub async fn create(pool: &PgPool, data: CreateLocationRequest) -> Result<Location, AppError> {
    if data.area.trim().is_empty() {
        return Err(AppError::Validation(
            "Location area cannot be empty".to_string(),
        ));
    }
    if data.line.trim().is_empty() {
        return Err(AppError::Validation(
            "Location line cannot be empty".to_string(),
        ));
    }
    repo::create(pool, &data).await.map_err(AppError::from)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: UpdateLocationRequest,
) -> Result<Location, AppError> {
    if let Some(ref area) = data.area {
        if area.trim().is_empty() {
            return Err(AppError::Validation(
                "Location area cannot be empty".to_string(),
            ));
        }
    }
    if let Some(ref line) = data.line {
        if line.trim().is_empty() {
            return Err(AppError::Validation(
                "Location line cannot be empty".to_string(),
            ));
        }
    }
    repo::update(pool, id, &data)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Location {id} not found")))
}
