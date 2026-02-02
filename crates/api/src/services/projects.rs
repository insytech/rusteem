use shared::dto::projects::{CreateProjectRequest, UpdateProjectRequest};
use shared::Project;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::projects as repo;

pub async fn list(pool: &PgPool) -> Result<Vec<Project>, AppError> {
    repo::find_all(pool).await.map_err(AppError::from)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Project, AppError> {
    repo::find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Project {id} not found")))
}

pub async fn create(pool: &PgPool, data: CreateProjectRequest) -> Result<Project, AppError> {
    if data.name.trim().is_empty() {
        return Err(AppError::Validation(
            "Project name cannot be empty".to_string(),
        ));
    }
    repo::create(pool, &data).await.map_err(AppError::from)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: UpdateProjectRequest,
) -> Result<Project, AppError> {
    if let Some(ref name) = data.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Project name cannot be empty".to_string(),
            ));
        }
    }
    repo::update(pool, id, &data)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Project {id} not found")))
}
