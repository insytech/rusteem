use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use shared::dto::machines::{CreateMachineRequest, MachineDetail, MachineFilters, UpdateMachineRequest};
use shared::dto::pagination::PaginatedResponse;
use shared::Machine;

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::machines as service;
use crate::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(filters): Query<MachineFilters>,
) -> Result<Json<PaginatedResponse<MachineDetail>>, AppError> {
    let result = service::list(&state.pool, &filters).await?;
    Ok(Json(result))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MachineDetail>, AppError> {
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

pub async fn duplicate(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Machine>), AppError> {
    let machine = service::duplicate(&state.pool, id).await?;
    Ok((StatusCode::CREATED, Json(machine)))
}

#[derive(Debug, Serialize)]
pub struct PipelineStage {
    pub stage: String,
    pub status: String,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PipelineStatus {
    pub machine_id: Uuid,
    pub current_stage: String,
    pub overall_status: String,
    pub release_level: i32,
    pub stages: Vec<PipelineStage>,
}

pub async fn get_pipeline_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PipelineStatus>, AppError> {
    let stages = sqlx::query_as::<_, (String, String, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT stage::text, status::text, due_date, completed_at
         FROM machine_approval_stages
         WHERE machine_id = $1
         ORDER BY CASE stage
             WHEN 'scope_approval' THEN 1
             WHEN 'po_trail' THEN 2
             WHEN 'design' THEN 3
             WHEN 'run_off' THEN 4
             WHEN 'support_documents' THEN 5
             WHEN 'ramp_up' THEN 6
             WHEN 'release' THEN 7
         END",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    let pipeline_info = sqlx::query_as::<_, (String, String, i32)>(
        "SELECT current_stage::text, overall_status, release_level
         FROM machine_pipeline_status
         WHERE machine_id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?;

    let (current_stage, overall_status, release_level) = pipeline_info
        .unwrap_or(("scope_approval".to_string(), "not_started".to_string(), 0));

    let pipeline_stages: Vec<PipelineStage> = if stages.is_empty() {
        ["scope_approval", "po_trail", "design", "run_off", "support_documents", "ramp_up", "release"]
            .iter()
            .map(|s| PipelineStage {
                stage: s.to_string(),
                status: "not_started".to_string(),
                due_date: None,
                completed_at: None,
            })
            .collect()
    } else {
        stages
            .into_iter()
            .map(|(stage, status, due_date, completed_at)| PipelineStage {
                stage,
                status,
                due_date,
                completed_at,
            })
            .collect()
    };

    Ok(Json(PipelineStatus {
        machine_id: id,
        current_stage,
        overall_status,
        release_level,
        stages: pipeline_stages,
    }))
}
