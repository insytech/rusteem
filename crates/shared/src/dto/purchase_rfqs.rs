use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePurchaseRfqRequest {
    pub rfq_number: Option<String>,
    pub purchase_order: Option<String>,
    pub tooling_agreement: Option<bool>,
    pub tooling_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePurchaseRfqRequest {
    pub rfq_number: Option<String>,
    pub purchase_order: Option<String>,
    pub tooling_agreement: Option<bool>,
    pub tooling_number: Option<String>,
    pub notes: Option<String>,
}
