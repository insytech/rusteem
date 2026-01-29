use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
    pub asset_number: Option<String>,
    pub line: Option<String>,
    pub station: Option<String>,
    pub area: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub required_for_release: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: Uuid,
    pub machine_id: Option<Uuid>,
    pub document_type_id: Uuid,
    pub title: String,
    pub storage_path: String,
    pub revision: i32,
    pub status: DocumentStatus,
    pub uploader_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DocumentStatus {
    Draft,
    Pending,
    Approved,
    Rejected,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_order: i32,
    pub role_name: String,
    pub is_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApprovalDecision {
    Pending,
    Approved,
    Rejected,
}
