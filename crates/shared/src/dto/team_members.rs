use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTeamMemberRequest {
    pub name: String,
    pub email: String,
    pub role: Option<String>,
    pub department: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTeamMemberRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub department: Option<String>,
    pub active: Option<bool>,
}
