use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BcfTopic {
    pub guid: String,
    pub title: String,
    pub description: String,
    pub status: String,        // "Open", "Closed", "InProgress"
    pub priority: String,      // "Critical", "Major", "Normal", "Minor"
    pub creation_date: String,
    pub modified_date: String,
    pub assigned_to: Option<String>,
    pub related_kozijn_ids: Vec<String>,
    pub comments: Vec<BcfComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfComment {
    pub guid: String,
    pub author: String,
    pub date: String,
    pub comment: String,
}

/// Simple BCF topic management (full ZIP import/export deferred — needs quick-xml + zip crates)
pub fn create_topic(title: &str, description: &str) -> BcfTopic {
    BcfTopic {
        guid: uuid::Uuid::new_v4().to_string(),
        title: title.to_string(),
        description: description.to_string(),
        status: "Open".to_string(),
        priority: "Normal".to_string(),
        creation_date: chrono::Utc::now().to_rfc3339(),
        modified_date: chrono::Utc::now().to_rfc3339(),
        assigned_to: None,
        related_kozijn_ids: vec![],
        comments: vec![],
    }
}
