use crate::state::AppState;
use ofs_core::ids::{IdsCheckResult, IdsRequirement};
use tauri::State;

#[tauri::command]
pub fn validate_project_ids(
    state: State<'_, AppState>,
    requirements_json: Option<String>,
) -> Result<Vec<IdsCheckResult>, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;

    let requirements: Vec<IdsRequirement> = if let Some(json) = requirements_json {
        serde_json::from_str(&json).map_err(|e| format!("Ongeldige IDS requirements: {}", e))?
    } else {
        ofs_core::ids::default_ids_requirements()
    };

    let profiles = &project.custom_profiles;
    let mut all_results = Vec::new();

    for kozijn in &project.kozijnen {
        let results = ofs_core::ids::validate_kozijn(kozijn, profiles, &requirements);
        all_results.extend(results);
    }

    Ok(all_results)
}
