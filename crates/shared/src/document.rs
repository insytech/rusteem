use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct DocumentType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub required_for_release: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum DocumentStatus {
    Draft,
    Pending,
    Approved,
    Rejected,
    Archived,
}

impl DocumentStatus {
    /// Validate whether a status transition is allowed.
    pub fn can_transition_to(&self, target: &DocumentStatus) -> bool {
        matches!(
            (self, target),
            (DocumentStatus::Draft, DocumentStatus::Pending)
                | (DocumentStatus::Pending, DocumentStatus::Approved)
                | (DocumentStatus::Pending, DocumentStatus::Rejected)
                | (DocumentStatus::Rejected, DocumentStatus::Draft)
                | (DocumentStatus::Approved, DocumentStatus::Archived)
        )
    }
}
