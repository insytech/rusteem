use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub code: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
}
