use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StageCount {
    pub stage: String,
    pub label: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MachineAlert {
    pub machine_id: Uuid,
    pub machine_name: String,
    pub area: Option<String>,
    pub current_stage: String,
    pub status: String,
    pub days_overdue: i64,
    pub responsible: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityEntry {
    pub id: Uuid,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardSummary {
    pub pipeline: Vec<StageCount>,
    pub total_active: i64,
    pub total_released: i64,
    pub total_in_progress: i64,
    pub total_overdue: i64,
    pub total_breaches: i64,
    pub released_this_month: i64,
    pub needs_attention: Vec<MachineAlert>,
    pub recent_activity: Vec<ActivityEntry>,
}
