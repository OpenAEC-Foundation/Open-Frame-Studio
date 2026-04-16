use axum::{extract::{Path, State}, Json};
use std::sync::Arc;
use crate::storage::{CloudState, CloudProject};

pub async fn list_projects(
    State(state): State<Arc<CloudState>>,
) -> Json<Vec<CloudProject>> {
    let projects = state.projects.lock().unwrap();
    Json(projects.values().cloned().collect())
}

pub async fn create_project(
    State(state): State<Arc<CloudState>>,
    Json(body): Json<serde_json::Value>,
) -> Json<CloudProject> {
    let id = uuid::Uuid::new_v4().to_string();
    let project = CloudProject {
        id: id.clone(),
        name: body["name"].as_str().unwrap_or("Nieuw project").to_string(),
        number: body["number"].as_str().unwrap_or("").to_string(),
        client: body["client"].as_str().unwrap_or("").to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        data_json: body["data"].as_str().unwrap_or("{}").to_string(),
    };
    state.projects.lock().unwrap().insert(id, project.clone());
    Json(project)
}

pub async fn get_project(
    State(state): State<Arc<CloudState>>,
    Path(id): Path<String>,
) -> Json<Option<CloudProject>> {
    let projects = state.projects.lock().unwrap();
    Json(projects.get(&id).cloned())
}

pub async fn delete_project(
    State(state): State<Arc<CloudState>>,
    Path(id): Path<String>,
) -> Json<bool> {
    let mut projects = state.projects.lock().unwrap();
    Json(projects.remove(&id).is_some())
}

pub async fn get_kozijnen(
    State(state): State<Arc<CloudState>>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let projects = state.projects.lock().unwrap();
    if let Some(project) = projects.get(&id) {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&project.data_json) {
            return Json(data["kozijnen"].clone());
        }
    }
    Json(serde_json::json!([]))
}

pub async fn export_ifc(
    State(_state): State<Arc<CloudState>>,
    Path(_id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "not_implemented", "message": "IFC export via cloud API — coming soon"}))
}

pub async fn get_quotation(
    State(_state): State<Arc<CloudState>>,
    Path(_id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "not_implemented"}))
}

pub async fn get_production(
    State(_state): State<Arc<CloudState>>,
    Path(_id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "not_implemented"}))
}
