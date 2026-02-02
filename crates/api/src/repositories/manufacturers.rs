use shared::dto::manufacturers::{CreateManufacturerRequest, UpdateManufacturerRequest};
use shared::Manufacturer;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all(pool: &PgPool) -> Result<Vec<Manufacturer>, sqlx::Error> {
    sqlx::query_as::<_, Manufacturer>("SELECT * FROM manufacturers ORDER BY name ASC")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Manufacturer>, sqlx::Error> {
    sqlx::query_as::<_, Manufacturer>("SELECT * FROM manufacturers WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    data: &CreateManufacturerRequest,
) -> Result<Manufacturer, sqlx::Error> {
    sqlx::query_as::<_, Manufacturer>(
        "INSERT INTO manufacturers (name, contact_name, contact_email, contact_phone, website)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.contact_name)
    .bind(&data.contact_email)
    .bind(&data.contact_phone)
    .bind(&data.website)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateManufacturerRequest,
) -> Result<Option<Manufacturer>, sqlx::Error> {
    sqlx::query_as::<_, Manufacturer>(
        "UPDATE manufacturers SET
            name = COALESCE($2, name),
            contact_name = COALESCE($3, contact_name),
            contact_email = COALESCE($4, contact_email),
            contact_phone = COALESCE($5, contact_phone),
            website = COALESCE($6, website),
            active = COALESCE($7, active)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.name)
    .bind(&data.contact_name)
    .bind(&data.contact_email)
    .bind(&data.contact_phone)
    .bind(&data.website)
    .bind(data.active)
    .fetch_optional(pool)
    .await
}
