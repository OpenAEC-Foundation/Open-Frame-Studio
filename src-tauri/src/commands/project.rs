use crate::state::AppState;
use ofs_core::kozijn::Project;
use tauri::State;

#[tauri::command]
pub fn new_project(
    state: State<'_, AppState>,
    name: String,
    number: String,
) -> Result<Project, String> {
    let project = Project::new(&name, &number);
    let mut current = state.project.lock().map_err(|e| e.to_string())?;
    *current = project.clone();
    let mut path = state.project_path.lock().map_err(|e| e.to_string())?;
    *path = None;
    Ok(project)
}

#[tauri::command]
pub fn get_project(state: State<'_, AppState>) -> Result<Project, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    Ok(project.clone())
}

#[tauri::command]
pub fn open_project(state: State<'_, AppState>, file_path: String) -> Result<Project, String> {
    let contents = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let project: Project = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
    let mut current = state.project.lock().map_err(|e| e.to_string())?;
    *current = project.clone();
    let mut path = state.project_path.lock().map_err(|e| e.to_string())?;
    *path = Some(file_path);
    Ok(project)
}

#[tauri::command]
pub fn save_project(state: State<'_, AppState>, file_path: String) -> Result<(), String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&*project).map_err(|e| e.to_string())?;
    std::fs::write(&file_path, json).map_err(|e| e.to_string())?;
    drop(project);
    let mut path = state.project_path.lock().map_err(|e| e.to_string())?;
    *path = Some(file_path);
    Ok(())
}
