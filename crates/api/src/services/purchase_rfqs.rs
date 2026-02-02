use shared::dto::purchase_rfqs::{CreatePurchaseRfqRequest, UpdatePurchaseRfqRequest};
use shared::PurchaseRfq;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::purchase_rfqs as repo;

pub async fn list_by_machine(
    pool: &PgPool,
    machine_id: Uuid,
) -> Result<Vec<PurchaseRfq>, AppError> {
    repo::find_by_machine(pool, machine_id)
        .await
        .map_err(AppError::from)
}

pub async fn create(
    pool: &PgPool,
    machine_id: Uuid,
    data: CreatePurchaseRfqRequest,
) -> Result<PurchaseRfq, AppError> {
    repo::create(pool, machine_id, &data)
        .await
        .map_err(AppError::from)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: UpdatePurchaseRfqRequest,
) -> Result<PurchaseRfq, AppError> {
    repo::update(pool, id, &data)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Purchase RFQ {id} not found")))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let deleted = repo::delete(pool, id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!(
            "Purchase RFQ {id} not found"
        )));
    }
    Ok(())
}
