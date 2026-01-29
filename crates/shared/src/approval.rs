use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct ApprovalWorkflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct ApprovalStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_order: i32,
    pub role_name: String,
    pub is_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Approval {
    pub id: Uuid,
    pub document_id: Uuid,
    pub workflow_id: Option<Uuid>,
    pub step_id: Uuid,
    pub approver_id: Option<Uuid>,
    pub decision: ApprovalDecision,
    pub comments: Option<String>,
    pub decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "backend", sqlx(type_name = "text", rename_all = "lowercase"))]
pub enum ApprovalDecision {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct ApprovalHistory {
    pub id: Uuid,
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
