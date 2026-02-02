use shared::dto::team_members::{CreateTeamMemberRequest, UpdateTeamMemberRequest};
use shared::TeamMember;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::team_members as repo;

pub async fn list(pool: &PgPool) -> Result<Vec<TeamMember>, AppError> {
    repo::find_all(pool).await.map_err(AppError::from)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<TeamMember, AppError> {
    repo::find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Team member {id} not found")))
}

pub async fn create(pool: &PgPool, data: CreateTeamMemberRequest) -> Result<TeamMember, AppError> {
    if data.name.trim().is_empty() {
        return Err(AppError::Validation(
            "Team member name cannot be empty".to_string(),
        ));
    }
    if data.email.trim().is_empty() {
        return Err(AppError::Validation(
            "Team member email cannot be empty".to_string(),
        ));
    }
    if let Some(existing) = repo::find_by_email(pool, &data.email).await? {
        return Err(AppError::Validation(format!(
            "Email '{}' is already used by '{}'",
            data.email, existing.name
        )));
    }
    repo::create(pool, &data).await.map_err(AppError::from)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: UpdateTeamMemberRequest,
) -> Result<TeamMember, AppError> {
    if let Some(ref name) = data.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Team member name cannot be empty".to_string(),
            ));
        }
    }
    if let Some(ref email) = data.email {
        if email.trim().is_empty() {
            return Err(AppError::Validation(
                "Team member email cannot be empty".to_string(),
            ));
        }
        if let Some(existing) = repo::find_by_email(pool, email).await? {
            if existing.id != id {
                return Err(AppError::Validation(format!(
                    "Email '{}' is already used by '{}'",
                    email, existing.name
                )));
            }
        }
    }
    repo::update(pool, id, &data)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Team member {id} not found")))
}
