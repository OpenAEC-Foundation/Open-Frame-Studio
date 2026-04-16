use tauri::State;
use crate::state::AppState;
use ofs_core::energy::{calculate_project_energy, ProjectEnergyResult};

#[tauri::command]
pub fn get_project_energy(
    state: State<'_, AppState>,
    max_uw: f64,
) -> Result<ProjectEnergyResult, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    let profiles = &project.custom_profiles;
    Ok(calculate_project_energy(&project, profiles, max_uw))
}
