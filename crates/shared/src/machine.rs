use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
    pub asset_number: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub area: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub machine_type_id: Option<Uuid>,
    pub manufacturer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub responsible: Option<String>,
}
