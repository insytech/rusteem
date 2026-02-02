use shared::dto::projects::{CreateProjectRequest, UpdateProjectRequest};
use shared::Project;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY name ASC")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &PgPool, data: &CreateProjectRequest) -> Result<Project, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "INSERT INTO projects (name, code, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.code)
    .bind(&data.description)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateProjectRequest,
) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "UPDATE projects SET
            name = COALESCE($2, name),
            code = COALESCE($3, code),
            description = COALESCE($4, description),
            active = COALESCE($5, active)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.name)
    .bind(&data.code)
    .bind(&data.description)
    .bind(data.active)
    .fetch_optional(pool)
    .await
}
