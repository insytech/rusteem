use shared::dto::machines::{CreateMachineRequest, MachineFilters, UpdateMachineRequest};
use shared::Machine;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(pool: &PgPool, filters: &MachineFilters) -> Result<Vec<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>(
        "SELECT * FROM machines
         WHERE ($1::bool IS NULL OR active = $1)
           AND ($2::text IS NULL OR area = $2)
           AND ($3::text IS NULL OR line = $3)
           AND ($4::text IS NULL OR station = $4)
         ORDER BY name ASC",
    )
    .bind(filters.active)
    .bind(&filters.area)
    .bind(&filters.line)
    .bind(&filters.station)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>("SELECT * FROM machines WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &PgPool, data: &CreateMachineRequest) -> Result<Machine, sqlx::Error> {
    sqlx::query_as::<_, Machine>(
        "INSERT INTO machines (name, asset_number, line, station, area)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.asset_number)
    .bind(&data.line)
    .bind(&data.station)
    .bind(&data.area)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateMachineRequest,
) -> Result<Option<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>(
        "UPDATE machines SET
            name = COALESCE($2, name),
            asset_number = COALESCE($3, asset_number),
            line = COALESCE($4, line),
            station = COALESCE($5, station),
            area = COALESCE($6, area),
            active = COALESCE($7, active)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.name)
    .bind(&data.asset_number)
    .bind(&data.line)
    .bind(&data.station)
    .bind(&data.area)
    .bind(data.active)
    .fetch_optional(pool)
    .await
}

pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE machines SET active = false WHERE id = $1 AND active = true")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_by_asset_number(
    pool: &PgPool,
    asset_number: &str,
) -> Result<Option<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>("SELECT * FROM machines WHERE asset_number = $1")
        .bind(asset_number)
        .fetch_optional(pool)
        .await
}
