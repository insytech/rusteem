use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct PurchaseRfq {
    pub id: Uuid,
    pub machine_id: Uuid,
    pub rfq_number: Option<String>,
    pub purchase_order: Option<String>,
    pub tooling_agreement: bool,
    pub tooling_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
