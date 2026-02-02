use shared::dto::team_members::{CreateTeamMemberRequest, UpdateTeamMemberRequest};
use shared::TeamMember;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(pool: &PgPool) -> Result<Vec<TeamMember>, sqlx::Error> {
    sqlx::query_as::<_, TeamMember>(
        "SELECT * FROM team_members WHERE active = true ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<TeamMember>, sqlx::Error> {
    sqlx::query_as::<_, TeamMember>("SELECT * FROM team_members WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<TeamMember>, sqlx::Error> {
    sqlx::query_as::<_, TeamMember>("SELECT * FROM team_members WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    data: &CreateTeamMemberRequest,
) -> Result<TeamMember, sqlx::Error> {
    sqlx::query_as::<_, TeamMember>(
        "INSERT INTO team_members (name, email, role, department)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.email)
    .bind(&data.role)
    .bind(&data.department)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateTeamMemberRequest,
) -> Result<Option<TeamMember>, sqlx::Error> {
    sqlx::query_as::<_, TeamMember>(
        "UPDATE team_members SET
            name = COALESCE($2, name),
            email = COALESCE($3, email),
            role = COALESCE($4, role),
            department = COALESCE($5, department),
            active = COALESCE($6, active)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.name)
    .bind(&data.email)
    .bind(&data.role)
    .bind(&data.department)
    .bind(data.active)
    .fetch_optional(pool)
    .await
}
