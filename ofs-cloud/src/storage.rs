use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProject {
    pub id: String,
    pub name: String,
    pub number: String,
    pub client: String,
    pub created_at: String,
    pub updated_at: String,
    pub data_json: String, // full .ofs project JSON
}

pub struct CloudState {
    pub projects: Mutex<HashMap<String, CloudProject>>,
}

impl CloudState {
    pub fn new() -> Self {
        Self {
            projects: Mutex::new(HashMap::new()),
        }
    }
}
