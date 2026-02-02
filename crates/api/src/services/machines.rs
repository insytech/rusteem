use shared::dto::machines::{CreateMachineRequest, MachineDetail, MachineFilters, UpdateMachineRequest};
use shared::dto::pagination::PaginatedResponse;
use shared::Machine;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::machines as repo;
use crate::repositories::pagination::{clamp_limit, decode_cursor, encode_cursor};

pub async fn list(
    pool: &PgPool,
    filters: &MachineFilters,
) -> Result<PaginatedResponse<MachineDetail>, AppError> {
    let limit = clamp_limit(filters.limit);
    let (cursor_name, cursor_id) = match &filters.cursor {
        Some(c) => {
            let (name, id) = decode_cursor(c)?;
            (Some(name), Some(id))
        }
        None => (None, None),
    };

    let (mut rows, total) =
        repo::find_all_with_details(pool, filters, cursor_name.as_deref(), cursor_id, limit + 1)
            .await?;

    let has_more = rows.len() > limit as usize;
    if has_more {
        rows.pop();
    }

    let next_cursor = if has_more {
        rows.last()
            .map(|m| encode_cursor(&m.name, m.id))
    } else {
        None
    };

    Ok(PaginatedResponse {
        items: rows,
        next_cursor,
        total,
    })
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<MachineDetail, AppError> {
    repo::find_detail_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Machine {id} not found")))
}

pub async fn create(pool: &PgPool, data: CreateMachineRequest) -> Result<Machine, AppError> {
    if data.name.trim().is_empty() {
        return Err(AppError::Validation("Machine name cannot be empty".to_string()));
    }

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

pub async fn duplicate(pool: &PgPool, source_id: Uuid) -> Result<Machine, AppError> {
    let source = repo::find_by_id(pool, source_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Machine {source_id} not found")))?;

    let new_name = format!("{} (Copy)", source.name);
    repo::duplicate(pool, source_id, &new_name)
        .await?
        .ok_or_else(|| AppError::Internal("Failed to duplicate machine".to_string()))
}
