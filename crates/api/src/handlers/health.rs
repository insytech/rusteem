use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::errors::AppError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub supabase: String,
}

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, AppError> {
    sqlx::query("SELECT 1")
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Health check: database unreachable");
            AppError::Internal("Database connection failed".to_string())
        })?;

    let supabase_status = if state.config.supabase_url.is_empty() {
        "not configured"
    } else {
        "configured"
    };

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        database: "connected".to_string(),
        supabase: supabase_status.to_string(),
    }))
}
