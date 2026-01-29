use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use shared::dto::approvals::{PendingApproval, SubmitDecisionRequest};
use shared::dto::pagination::{PaginatedResponse, PaginationParams};
use shared::{Approval, ApprovalHistory};

use crate::errors::AppError;
use crate::extractors::AuthUser;
use crate::services::approvals as service;
use crate::state::AppState;

pub async fn initiate_workflow(
    State(state): State<AppState>,
    user: AuthUser,
    Path((document_id, workflow_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<Vec<Approval>>), AppError> {
    let approvals =
        service::initiate_workflow(&state.pool, document_id, workflow_id, user.id).await?;
    Ok((StatusCode::CREATED, Json(approvals)))
}

pub async fn submit_decision(
    State(state): State<AppState>,
    user: AuthUser,
    Path(approval_id): Path<Uuid>,
    Json(request): Json<SubmitDecisionRequest>,
) -> Result<Json<Approval>, AppError> {
    let approval =
        service::submit_decision(&state.pool, approval_id, user.id, request).await?;
    Ok(Json(approval))
}

pub async fn get_pending(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<PendingApproval>>, AppError> {
    let result = service::get_pending_for_role(&state.pool, &user.role, &pagination).await?;
    Ok(Json(result))
}

pub async fn get_document_history(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<Vec<ApprovalHistory>>, AppError> {
    let history = service::get_document_history(&state.pool, document_id).await?;
    Ok(Json(history))
}

pub async fn get_document_approvals(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
) -> Result<Json<Vec<Approval>>, AppError> {
    let approvals = service::get_document_approvals(&state.pool, document_id).await?;
    Ok(Json(approvals))
}
