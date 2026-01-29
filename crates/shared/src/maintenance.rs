use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MaintenancePlan {
    pub id: Uuid,
    pub machine_id: Option<Uuid>,
    pub description: String,
    pub frequency_value: i32,
    pub frequency_unit: FrequencyUnit,
    pub last_performed_at: Option<DateTime<Utc>>,
    pub next_due_at: Option<DateTime<Utc>>,
    pub is_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum FrequencyUnit {
    Hours,
    Days,
    Months,
    Cycles,
}
