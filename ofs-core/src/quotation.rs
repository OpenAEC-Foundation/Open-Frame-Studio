use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quotation {
    pub id: String,
    pub version: u32,
    pub status: QuotationStatus,
    pub created_at: String,
    pub valid_until: String,
    pub kozijn_marks: Vec<String>,
    pub total_incl_btw: f64,
    pub notes: String,
    pub change_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotationStatus {
    Draft,
    Sent,
    Accepted,
    Rejected,
    Expired,
}

impl Quotation {
    pub fn new_draft(kozijn_marks: Vec<String>, total: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            version: 1,
            status: QuotationStatus::Draft,
            created_at: chrono::Utc::now().to_rfc3339(),
            valid_until: (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339(),
            kozijn_marks,
            total_incl_btw: total,
            notes: String::new(),
            change_description: "Eerste versie".into(),
        }
    }

    pub fn create_revision(&self, new_total: f64, change_desc: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            version: self.version + 1,
            status: QuotationStatus::Draft,
            created_at: chrono::Utc::now().to_rfc3339(),
            valid_until: (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339(),
            kozijn_marks: self.kozijn_marks.clone(),
            total_incl_btw: new_total,
            notes: String::new(),
            change_description: change_desc.to_string(),
        }
    }
}
