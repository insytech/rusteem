use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use shared::dto::maintenance::{
    CompletePlanRequest, CreateMaintenancePlanRequest, UpdateMaintenancePlanRequest,
};
use shared::dto::pagination::{PaginatedResponse, PaginationParams};
use shared::MaintenancePlan;

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::maintenance as service;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct UpcomingQuery {
    pub days: Option<i32>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

pub async fn get_by_machine(
    State(state): State<AppState>,
    Path(machine_id): Path<Uuid>,
) -> Result<Json<Vec<MaintenancePlan>>, AppError> {
    let plans = service::get_by_machine(&state.pool, machine_id).await?;
    Ok(Json(plans))
}

pub async fn get_upcoming(
    State(state): State<AppState>,
    Query(query): Query<UpcomingQuery>,
) -> Result<Json<PaginatedResponse<MaintenancePlan>>, AppError> {
    let days = query.days.unwrap_or(7);
    let pagination = PaginationParams {
        cursor: query.cursor,
        limit: query.limit,
    };
    let result = service::get_upcoming(&state.pool, days, &pagination).await?;
    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct OverdueQuery {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

pub async fn get_overdue(
    State(state): State<AppState>,
    Query(query): Query<OverdueQuery>,
) -> Result<Json<PaginatedResponse<MaintenancePlan>>, AppError> {
    let pagination = PaginationParams {
        cursor: query.cursor,
        limit: query.limit,
    };
    let result = service::get_overdue(&state.pool, &pagination).await?;
    Ok(Json(result))
}

pub async fn create(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(data): Json<CreateMaintenancePlanRequest>,
) -> Result<(StatusCode, Json<MaintenancePlan>), AppError> {
    let plan = service::create(&state.pool, data).await?;
    Ok((StatusCode::CREATED, Json(plan)))
}

pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateMaintenancePlanRequest>,
) -> Result<Json<MaintenancePlan>, AppError> {
    let plan = service::update(&state.pool, id, data).await?;
    Ok(Json(plan))
}

pub async fn complete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<CompletePlanRequest>,
) -> Result<Json<MaintenancePlan>, AppError> {
    let performed_at = data.performed_at.unwrap_or_else(Utc::now);
    let plan = service::complete(&state.pool, id, performed_at).await?;
    Ok(Json(plan))
}

pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
