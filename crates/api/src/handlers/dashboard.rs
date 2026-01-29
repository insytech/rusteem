use axum::extract::State;
use axum::Json;

use shared::dto::dashboard::DashboardSummary;

use crate::errors::AppError;
use crate::services::dashboard as service;
use crate::state::AppState;

pub async fn summary(
    State(state): State<AppState>,
) -> Result<Json<DashboardSummary>, AppError> {
    let summary = service::get_summary(&state.pool).await?;
    Ok(Json(summary))
}
