use crate::state::AppState;
use ofs_core::planning::ProductionPlan;
use tauri::State;

#[tauri::command]
pub fn get_production_plan(
    state: State<'_, AppState>,
    hours_per_day: Option<f64>,
    workers: Option<u32>,
) -> Result<ProductionPlan, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    Ok(ofs_core::planning::generate_production_plan(
        &project,
        hours_per_day.unwrap_or(8.0),
        workers.unwrap_or(2),
    ))
}
