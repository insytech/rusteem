use shared::dto::purchase_rfqs::{CreatePurchaseRfqRequest, UpdatePurchaseRfqRequest};
use shared::PurchaseRfq;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_by_machine(
    pool: &PgPool,
    machine_id: Uuid,
) -> Result<Vec<PurchaseRfq>, sqlx::Error> {
    sqlx::query_as::<_, PurchaseRfq>(
        "SELECT * FROM purchase_rfqs WHERE machine_id = $1 ORDER BY created_at DESC",
    )
    .bind(machine_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<PurchaseRfq>, sqlx::Error> {
    sqlx::query_as::<_, PurchaseRfq>("SELECT * FROM purchase_rfqs WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    machine_id: Uuid,
    data: &CreatePurchaseRfqRequest,
) -> Result<PurchaseRfq, sqlx::Error> {
    sqlx::query_as::<_, PurchaseRfq>(
        "INSERT INTO purchase_rfqs (machine_id, rfq_number, purchase_order, tooling_agreement, tooling_number, notes)
         VALUES ($1, $2, $3, COALESCE($4, false), $5, $6)
         RETURNING *",
    )
    .bind(machine_id)
    .bind(&data.rfq_number)
    .bind(&data.purchase_order)
    .bind(data.tooling_agreement)
    .bind(&data.tooling_number)
    .bind(&data.notes)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdatePurchaseRfqRequest,
) -> Result<Option<PurchaseRfq>, sqlx::Error> {
    sqlx::query_as::<_, PurchaseRfq>(
        "UPDATE purchase_rfqs SET
            rfq_number = COALESCE($2, rfq_number),
            purchase_order = COALESCE($3, purchase_order),
            tooling_agreement = COALESCE($4, tooling_agreement),
            tooling_number = COALESCE($5, tooling_number),
            notes = COALESCE($6, notes)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.rfq_number)
    .bind(&data.purchase_order)
    .bind(data.tooling_agreement)
    .bind(&data.tooling_number)
    .bind(&data.notes)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM purchase_rfqs WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
