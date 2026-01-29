use shared::dto::machines::{CreateMachineRequest, MachineFilters, UpdateMachineRequest};
use shared::Machine;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::machines as repo;

pub async fn list(pool: &PgPool, filters: &MachineFilters) -> Result<Vec<Machine>, AppError> {
    repo::find_all(pool, filters).await.map_err(AppError::from)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Machine, AppError> {
    repo::find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Machine {id} not found")))
}

pub async fn create(pool: &PgPool, data: CreateMachineRequest) -> Result<Machine, AppError> {
    if data.name.trim().is_empty() {
        return Err(AppError::Validation("Machine name cannot be empty".to_string()));
    }

    // Validate asset_number uniqueness if provided
    if let Some(ref asset_number) = data.asset_number {
        if let Some(existing) = repo::find_by_asset_number(pool, asset_number).await? {
            return Err(AppError::Validation(format!(
                "Asset number '{}' is already assigned to machine '{}'",
                asset_number, existing.name
            )));
        }
    }

    repo::create(pool, &data).await.map_err(AppError::from)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: UpdateMachineRequest,
) -> Result<Machine, AppError> {
    if let Some(ref name) = data.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation("Machine name cannot be empty".to_string()));
        }
    }

    // Validate asset_number uniqueness if changing it
    if let Some(ref asset_number) = data.asset_number {
        if let Some(existing) = repo::find_by_asset_number(pool, asset_number).await? {
            if existing.id != id {
                return Err(AppError::Validation(format!(
                    "Asset number '{}' is already assigned to machine '{}'",
                    asset_number, existing.name
                )));
            }
        }
    }

    repo::update(pool, id, &data)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Machine {id} not found")))
}

pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let deleted = repo::soft_delete(pool, id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!(
            "Machine {id} not found or already inactive"
        )));
    }
    Ok(())
}
