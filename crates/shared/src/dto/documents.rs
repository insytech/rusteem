use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::document::DocumentStatus;

#[derive(Debug, Deserialize)]
pub struct CreateDocumentRequest {
    pub machine_id: Option<Uuid>,
    pub document_type_id: Uuid,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub machine_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: DocumentStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentFilters {
    pub machine_id: Option<Uuid>,
    pub status: Option<String>,
    pub document_type_id: Option<Uuid>,
}
