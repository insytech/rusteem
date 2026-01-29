use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::approval::ApprovalDecision;

#[derive(Debug, Deserialize)]
pub struct SubmitDecisionRequest {
    pub decision: ApprovalDecision,
    pub comments: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct PendingApproval {
    pub approval_id: Uuid,
    pub document_id: Uuid,
    pub document_title: String,
    pub step_order: i32,
    pub role_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
