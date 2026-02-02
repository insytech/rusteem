use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use shared::dto::team_members::{CreateTeamMemberRequest, UpdateTeamMemberRequest};
use shared::TeamMember;

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::team_members as service;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<TeamMember>>, AppError> {
    let result = service::list(&state.pool).await?;
    Ok(Json(result))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TeamMember>, AppError> {
    let item = service::get_by_id(&state.pool, id).await?;
    Ok(Json(item))
}

pub async fn create(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(data): Json<CreateTeamMemberRequest>,
) -> Result<(StatusCode, Json<TeamMember>), AppError> {
    let item = service::create(&state.pool, data).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateTeamMemberRequest>,
) -> Result<Json<TeamMember>, AppError> {
    let item = service::update(&state.pool, id, data).await?;
    Ok(Json(item))
}
