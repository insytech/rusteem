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
    pub responsible_id: Option<Uuid>,
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
    pub responsible_id: Option<Uuid>,
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
    pub responsible_id: Option<Uuid>,
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
    pub responsible_id: Option<Uuid>,
    pub machine_type_name: Option<String>,
    pub manufacturer_name: Option<String>,
    pub location_area: Option<String>,
    pub location_line: Option<String>,
    pub project_name: Option<String>,
    pub responsible_name: Option<String>,
    pub responsible_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MachineStats {
    pub total: i64,
    pub active: i64,
    pub by_type: Vec<GroupCount>,
    pub by_area: Vec<GroupCount>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupCount {
    pub name: String,
    pub count: i64,
}
