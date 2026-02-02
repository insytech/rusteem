use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use shared::dto::manufacturers::{CreateManufacturerRequest, UpdateManufacturerRequest};
use shared::Manufacturer;

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::manufacturers as service;
use crate::state::AppState;

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<Manufacturer>>, AppError> {
    let result = service::list(&state.pool).await?;
    Ok(Json(result))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Manufacturer>, AppError> {
    let item = service::get_by_id(&state.pool, id).await?;
    Ok(Json(item))
}

pub async fn create(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(data): Json<CreateManufacturerRequest>,
) -> Result<(StatusCode, Json<Manufacturer>), AppError> {
    let item = service::create(&state.pool, data).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateManufacturerRequest>,
) -> Result<Json<Manufacturer>, AppError> {
    let item = service::update(&state.pool, id, data).await?;
    Ok(Json(item))
}
