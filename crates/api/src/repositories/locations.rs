use shared::dto::locations::{CreateLocationRequest, UpdateLocationRequest};
use shared::Location;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(pool: &PgPool) -> Result<Vec<Location>, sqlx::Error> {
    sqlx::query_as::<_, Location>("SELECT * FROM locations ORDER BY area ASC, line ASC")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Location>, sqlx::Error> {
    sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    data: &CreateLocationRequest,
) -> Result<Location, sqlx::Error> {
    sqlx::query_as::<_, Location>(
        "INSERT INTO locations (area, line) VALUES ($1, $2) RETURNING *",
    )
    .bind(&data.area)
    .bind(&data.line)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateLocationRequest,
) -> Result<Option<Location>, sqlx::Error> {
    sqlx::query_as::<_, Location>(
        "UPDATE locations SET
            area = COALESCE($2, area),
            line = COALESCE($3, line),
            active = COALESCE($4, active)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.area)
    .bind(&data.line)
    .bind(data.active)
    .fetch_optional(pool)
    .await
}
