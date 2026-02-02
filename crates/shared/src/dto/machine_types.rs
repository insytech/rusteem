use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMachineTypeRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateMachineTypeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
}
