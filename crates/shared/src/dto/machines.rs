use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMachineRequest {
    pub name: String,
    pub asset_number: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub area: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub machine_type_id: Option<Uuid>,
    pub manufacturer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub responsible: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateMachineRequest {
    pub name: Option<String>,
    pub asset_number: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub area: Option<String>,
    pub active: Option<bool>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub machine_type_id: Option<Uuid>,
    pub manufacturer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub responsible: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MachineFilters {
    pub active: Option<bool>,
    pub area: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub machine_type_id: Option<Uuid>,
    pub manufacturer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub responsible: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct MachineDetail {
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
    pub machine_type_name: Option<String>,
    pub manufacturer_name: Option<String>,
    pub location_area: Option<String>,
    pub location_line: Option<String>,
    pub project_name: Option<String>,
}
