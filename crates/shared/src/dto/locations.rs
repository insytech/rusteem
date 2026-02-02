use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLocationRequest {
    pub area: String,
    pub line: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateLocationRequest {
    pub area: Option<String>,
    pub line: Option<String>,
    pub active: Option<bool>,
}
