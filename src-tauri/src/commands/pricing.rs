use tauri::State;
use crate::state::AppState;

#[tauri::command]
pub fn get_pricing_config(state: State<'_, AppState>) -> Option<ofs_core::pricing::PricingConfig> {
    let project = state.project.lock().ok()?;
    project.pricing_config.clone()
}

#[tauri::command]
pub fn update_pricing_config(
    state: State<'_, AppState>,
    config_json: String,
) -> Result<(), String> {
    let config: ofs_core::pricing::PricingConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Invalid pricing config: {}", e))?;
    let mut project = state.project.lock().map_err(|e| e.to_string())?;
    project.pricing_config = Some(config);
    Ok(())
}
