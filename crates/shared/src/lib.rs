pub mod document;
pub mod dto;
pub mod machine;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use document::{Document, DocumentStatus, DocumentType};
pub use machine::Machine;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ApprovalStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_order: i32,
    pub role_name: String,
    pub is_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Approval {
    pub id: Uuid,
    pub document_id: Uuid,
    pub step_id: Uuid,
    pub approver_id: Option<Uuid>,
    pub decision: ApprovalDecision,
    pub comments: Option<String>,
    pub decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum ApprovalDecision {
    Pending,
    Approved,
    Rejected,
}
