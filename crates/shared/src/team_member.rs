use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct TeamMember {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: Option<String>,
    pub department: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
