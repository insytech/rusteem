use chrono::{DateTime, Utc};
use shared::dto::maintenance::{CreateMaintenancePlanRequest, UpdateMaintenancePlanRequest};
use shared::MaintenancePlan;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_by_machine(
    pool: &PgPool,
    machine_id: Uuid,
) -> Result<Vec<MaintenancePlan>, sqlx::Error> {
    sqlx::query_as::<_, MaintenancePlan>(
        "SELECT * FROM maintenance_plans WHERE machine_id = $1 ORDER BY next_due_at",
    )
    .bind(machine_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<MaintenancePlan>, sqlx::Error> {
    sqlx::query_as::<_, MaintenancePlan>("SELECT * FROM maintenance_plans WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &PgPool,
    data: &CreateMaintenancePlanRequest,
) -> Result<MaintenancePlan, sqlx::Error> {
    let freq_unit = match data.frequency_unit {
        shared::FrequencyUnit::Hours => "hours",
        shared::FrequencyUnit::Days => "days",
        shared::FrequencyUnit::Months => "months",
        shared::FrequencyUnit::Cycles => "cycles",
    };

    sqlx::query_as::<_, MaintenancePlan>(
        "INSERT INTO maintenance_plans (machine_id, description, frequency_value, frequency_unit, next_due_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
    )
    .bind(data.machine_id)
    .bind(&data.description)
    .bind(data.frequency_value)
    .bind(freq_unit)
    .bind(data.next_due_at)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateMaintenancePlanRequest,
) -> Result<Option<MaintenancePlan>, sqlx::Error> {
    let freq_unit = data.frequency_unit.as_ref().map(|u| match u {
        shared::FrequencyUnit::Hours => "hours",
        shared::FrequencyUnit::Days => "days",
        shared::FrequencyUnit::Months => "months",
        shared::FrequencyUnit::Cycles => "cycles",
    });

    sqlx::query_as::<_, MaintenancePlan>(
        "UPDATE maintenance_plans SET
            description = COALESCE($2, description),
            frequency_value = COALESCE($3, frequency_value),
            frequency_unit = COALESCE($4, frequency_unit),
            next_due_at = COALESCE($5, next_due_at),
            is_enabled = COALESCE($6, is_enabled)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.description)
    .bind(data.frequency_value)
    .bind(freq_unit)
    .bind(data.next_due_at)
    .bind(data.is_enabled)
    .fetch_optional(pool)
    .await
}

pub async fn mark_completed(
    pool: &PgPool,
    id: Uuid,
    performed_at: DateTime<Utc>,
    next_due_at: DateTime<Utc>,
) -> Result<Option<MaintenancePlan>, sqlx::Error> {
    sqlx::query_as::<_, MaintenancePlan>(
        "UPDATE maintenance_plans SET
            last_performed_at = $2,
            next_due_at = $3
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(performed_at)
    .bind(next_due_at)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM maintenance_plans WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn find_upcoming(
    pool: &PgPool,
    days_ahead: i32,
    cursor_next_due: Option<DateTime<Utc>>,
    cursor_id: Option<Uuid>,
    limit: i32,
) -> Result<(Vec<MaintenancePlan>, i64), sqlx::Error> {
    let rows = sqlx::query_as::<_, MaintenancePlan>(
        "SELECT * FROM maintenance_plans
         WHERE is_enabled = true
           AND next_due_at IS NOT NULL
           AND next_due_at <= now() + ($1 || ' days')::interval
           AND next_due_at >= now()
           AND (
               $2::timestamptz IS NULL
               OR (next_due_at, id) > ($2, $3::uuid)
           )
         ORDER BY next_due_at ASC, id ASC
         LIMIT $4",
    )
    .bind(days_ahead)
    .bind(cursor_next_due)
    .bind(cursor_id)
    .bind(i64::from(limit))
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM maintenance_plans
         WHERE is_enabled = true
           AND next_due_at IS NOT NULL
           AND next_due_at <= now() + ($1 || ' days')::interval
           AND next_due_at >= now()",
    )
    .bind(days_ahead)
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0))
}

pub async fn find_overdue(
    pool: &PgPool,
    cursor_next_due: Option<DateTime<Utc>>,
    cursor_id: Option<Uuid>,
    limit: i32,
) -> Result<(Vec<MaintenancePlan>, i64), sqlx::Error> {
    let rows = sqlx::query_as::<_, MaintenancePlan>(
        "SELECT * FROM maintenance_plans
         WHERE is_enabled = true
           AND next_due_at IS NOT NULL
           AND next_due_at < now()
           AND (
               $1::timestamptz IS NULL
               OR (next_due_at, id) > ($1, $2::uuid)
           )
         ORDER BY next_due_at ASC, id ASC
         LIMIT $3",
    )
    .bind(cursor_next_due)
    .bind(cursor_id)
    .bind(i64::from(limit))
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM maintenance_plans
         WHERE is_enabled = true
           AND next_due_at IS NOT NULL
           AND next_due_at < now()",
    )
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0))
}
