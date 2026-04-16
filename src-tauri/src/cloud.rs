use serde::{Deserialize, Serialize};

/// Configuration for connecting to the OFS Cloud API.
/// Actual HTTP calls are performed from the Svelte frontend using fetch().
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub api_url: String,
    pub api_key: Option<String>,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:3456/api/v1".to_string(),
            api_key: None,
        }
    }
}
