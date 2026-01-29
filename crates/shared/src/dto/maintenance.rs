use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::maintenance::FrequencyUnit;

#[derive(Debug, Deserialize)]
pub struct CreateMaintenancePlanRequest {
    pub machine_id: Option<Uuid>,
    pub description: String,
    pub frequency_value: i32,
    pub frequency_unit: FrequencyUnit,
    pub next_due_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMaintenancePlanRequest {
    pub description: Option<String>,
    pub frequency_value: Option<i32>,
    pub frequency_unit: Option<FrequencyUnit>,
    pub next_due_at: Option<DateTime<Utc>>,
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CompletePlanRequest {
    pub performed_at: Option<DateTime<Utc>>,
}
