use chrono::{DateTime, Months, Utc};
use shared::dto::maintenance::{
    CreateMaintenancePlanRequest, UpdateMaintenancePlanRequest,
};
use shared::{FrequencyUnit, MaintenancePlan};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::maintenance as repo;

pub async fn get_by_machine(
    pool: &PgPool,
    machine_id: Uuid,
) -> Result<Vec<MaintenancePlan>, AppError> {
    repo::find_by_machine(pool, machine_id)
        .await
        .map_err(AppError::from)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<MaintenancePlan, AppError> {
    repo::find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Maintenance plan {id} not found")))
}

pub async fn create(
    pool: &PgPool,
    data: CreateMaintenancePlanRequest,
) -> Result<MaintenancePlan, AppError> {
    if data.description.trim().is_empty() {
        return Err(AppError::Validation(
            "Description cannot be empty".to_string(),
        ));
    }
    if data.frequency_value <= 0 {
        return Err(AppError::Validation(
            "Frequency value must be positive".to_string(),
        ));
    }

    repo::create(pool, &data).await.map_err(AppError::from)
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: UpdateMaintenancePlanRequest,
) -> Result<MaintenancePlan, AppError> {
    if let Some(ref desc) = data.description {
        if desc.trim().is_empty() {
            return Err(AppError::Validation(
                "Description cannot be empty".to_string(),
            ));
        }
    }
    if let Some(val) = data.frequency_value {
        if val <= 0 {
            return Err(AppError::Validation(
                "Frequency value must be positive".to_string(),
            ));
        }
    }

    repo::update(pool, id, &data)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Maintenance plan {id} not found")))
}

pub async fn complete(pool: &PgPool, id: Uuid, performed_at: DateTime<Utc>) -> Result<MaintenancePlan, AppError> {
    let plan = get_by_id(pool, id).await?;
    let next_due = calculate_next_due(performed_at, plan.frequency_value, &plan.frequency_unit);

    repo::mark_completed(pool, id, performed_at, next_due)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Maintenance plan {id} not found")))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    if !repo::delete(pool, id).await? {
        return Err(AppError::NotFound(format!(
            "Maintenance plan {id} not found"
        )));
    }
    Ok(())
}

pub async fn get_upcoming(
    pool: &PgPool,
    days_ahead: i32,
) -> Result<Vec<MaintenancePlan>, AppError> {
    repo::find_upcoming(pool, days_ahead)
        .await
        .map_err(AppError::from)
}

pub async fn get_overdue(pool: &PgPool) -> Result<Vec<MaintenancePlan>, AppError> {
    repo::find_overdue(pool).await.map_err(AppError::from)
}

/// Calculate the next due date based on the frequency unit and value.
fn calculate_next_due(
    from: DateTime<Utc>,
    value: i32,
    unit: &FrequencyUnit,
) -> DateTime<Utc> {
    match unit {
        FrequencyUnit::Hours => from + chrono::Duration::hours(i64::from(value)),
        FrequencyUnit::Days => from + chrono::Duration::days(i64::from(value)),
        FrequencyUnit::Months => {
            from + Months::new(value as u32)
        }
        // Cycles are tracked manually; default to 30 days as a reminder
        FrequencyUnit::Cycles => from + chrono::Duration::days(30),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_calculate_next_due_hours() {
        let from = Utc.with_ymd_and_hms(2026, 1, 15, 10, 0, 0).unwrap();
        let next = calculate_next_due(from, 8, &FrequencyUnit::Hours);
        assert_eq!(next, Utc.with_ymd_and_hms(2026, 1, 15, 18, 0, 0).unwrap());
    }

    #[test]
    fn test_calculate_next_due_days() {
        let from = Utc.with_ymd_and_hms(2026, 1, 15, 0, 0, 0).unwrap();
        let next = calculate_next_due(from, 7, &FrequencyUnit::Days);
        assert_eq!(next, Utc.with_ymd_and_hms(2026, 1, 22, 0, 0, 0).unwrap());
    }

    #[test]
    fn test_calculate_next_due_months() {
        let from = Utc.with_ymd_and_hms(2026, 1, 31, 0, 0, 0).unwrap();
        let next = calculate_next_due(from, 1, &FrequencyUnit::Months);
        // chrono Months handles end-of-month correctly
        assert_eq!(next, Utc.with_ymd_and_hms(2026, 2, 28, 0, 0, 0).unwrap());
    }

    #[test]
    fn test_calculate_next_due_cycles() {
        let from = Utc.with_ymd_and_hms(2026, 1, 15, 0, 0, 0).unwrap();
        let next = calculate_next_due(from, 100, &FrequencyUnit::Cycles);
        // Cycles default to 30-day reminder
        assert_eq!(next, Utc.with_ymd_and_hms(2026, 2, 14, 0, 0, 0).unwrap());
    }
}
