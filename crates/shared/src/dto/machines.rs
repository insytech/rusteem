use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMachineRequest {
    pub name: String,
    pub asset_number: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub area: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateMachineRequest {
    pub name: Option<String>,
    pub asset_number: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub area: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MachineFilters {
    pub active: Option<bool>,
    pub area: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}
