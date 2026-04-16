use crate::state::AppState;
use ofs_core::bcf::{BcfComment, BcfTopic};
use tauri::State;

#[tauri::command]
pub fn get_bcf_topics(state: State<'_, AppState>) -> Result<Vec<BcfTopic>, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    Ok(project.bcf_topics.clone())
}

#[tauri::command]
pub fn create_bcf_topic(
    state: State<'_, AppState>,
    title: String,
    description: String,
) -> Result<BcfTopic, String> {
    let mut project = state.project.lock().map_err(|e| e.to_string())?;
    let topic = ofs_core::bcf::create_topic(&title, &description);
    project.bcf_topics.push(topic.clone());
    Ok(topic)
}

#[tauri::command]
pub fn update_bcf_topic_status(
    state: State<'_, AppState>,
    guid: String,
    status: String,
) -> Result<BcfTopic, String> {
    let mut project = state.project.lock().map_err(|e| e.to_string())?;
    let topic = project
        .bcf_topics
        .iter_mut()
        .find(|t| t.guid == guid)
        .ok_or("BCF topic niet gevonden")?;
    topic.status = status;
    topic.modified_date = chrono::Utc::now().to_rfc3339();
    Ok(topic.clone())
}

#[tauri::command]
pub fn add_bcf_comment(
    state: State<'_, AppState>,
    guid: String,
    author: String,
    comment: String,
) -> Result<BcfTopic, String> {
    let mut project = state.project.lock().map_err(|e| e.to_string())?;
    let topic = project
        .bcf_topics
        .iter_mut()
        .find(|t| t.guid == guid)
        .ok_or("BCF topic niet gevonden")?;

    topic.comments.push(BcfComment {
        guid: uuid::Uuid::new_v4().to_string(),
        author,
        date: chrono::Utc::now().to_rfc3339(),
        comment,
    });
    topic.modified_date = chrono::Utc::now().to_rfc3339();

    Ok(topic.clone())
}
