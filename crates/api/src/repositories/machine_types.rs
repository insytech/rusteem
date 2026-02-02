use shared::dto::machine_types::{CreateMachineTypeRequest, UpdateMachineTypeRequest};
use shared::MachineType;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(pool: &PgPool) -> Result<Vec<MachineType>, sqlx::Error> {
    sqlx::query_as::<_, MachineType>("SELECT * FROM machine_types ORDER BY name ASC")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<MachineType>, sqlx::Error> {
    sqlx::query_as::<_, MachineType>("SELECT * FROM machine_types WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    data: &CreateMachineTypeRequest,
) -> Result<MachineType, sqlx::Error> {
    sqlx::query_as::<_, MachineType>(
        "INSERT INTO machine_types (name, description) VALUES ($1, $2) RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.description)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateMachineTypeRequest,
) -> Result<Option<MachineType>, sqlx::Error> {
    sqlx::query_as::<_, MachineType>(
        "UPDATE machine_types SET
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            active = COALESCE($4, active)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.name)
    .bind(&data.description)
    .bind(data.active)
    .fetch_optional(pool)
    .await
}
