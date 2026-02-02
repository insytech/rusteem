use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use shared::dto::purchase_rfqs::{CreatePurchaseRfqRequest, UpdatePurchaseRfqRequest};
use shared::PurchaseRfq;

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::purchase_rfqs as service;
use crate::state::AppState;

pub async fn list_by_machine(
    State(state): State<AppState>,
    Path(machine_id): Path<Uuid>,
) -> Result<Json<Vec<PurchaseRfq>>, AppError> {
    let result = service::list_by_machine(&state.pool, machine_id).await?;
    Ok(Json(result))
}

pub async fn create(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(machine_id): Path<Uuid>,
    Json(data): Json<CreatePurchaseRfqRequest>,
) -> Result<(StatusCode, Json<PurchaseRfq>), AppError> {
    let item = service::create(&state.pool, machine_id, data).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdatePurchaseRfqRequest>,
) -> Result<Json<PurchaseRfq>, AppError> {
    let item = service::update(&state.pool, id, data).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
