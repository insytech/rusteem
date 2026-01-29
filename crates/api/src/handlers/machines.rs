use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use shared::dto::machines::{CreateMachineRequest, MachineFilters, UpdateMachineRequest};
use shared::Machine;

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::machines as service;
use crate::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(filters): Query<MachineFilters>,
) -> Result<Json<Vec<Machine>>, AppError> {
    let machines = service::list(&state.pool, &filters).await?;
    Ok(Json(machines))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Machine>, AppError> {
    let machine = service::get_by_id(&state.pool, id).await?;
    Ok(Json(machine))
}

pub async fn create(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(data): Json<CreateMachineRequest>,
) -> Result<(StatusCode, Json<Machine>), AppError> {
    let machine = service::create(&state.pool, data).await?;
    Ok((StatusCode::CREATED, Json(machine)))
}

pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateMachineRequest>,
) -> Result<Json<Machine>, AppError> {
    let machine = service::update(&state.pool, id, data).await?;
    Ok(Json(machine))
}

pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::soft_delete(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
