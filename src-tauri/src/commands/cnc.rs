use tauri::State;
use crate::state::AppState;

#[tauri::command]
pub fn export_cnc_gcode(
    state: State<'_, AppState>,
    id: String,
    output_dir: String,
) -> Result<Vec<String>, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    let id: uuid::Uuid = id.parse().map_err(|e: uuid::Error| e.to_string())?;
    let kozijn = project.kozijnen.iter().find(|k| k.id == id)
        .ok_or("Kozijn niet gevonden")?;

    let parts = ofs_core::cnc::generate_cnc_parts(kozijn);
    let gcode = ofs_core::cnc::gcode::GenericGCode;
    use ofs_core::cnc::postprocessor::CncPostProcessor;
    let files = gcode.generate(&parts)?;

    let mut written = Vec::new();
    for (filename, content) in files {
        let path = std::path::Path::new(&output_dir).join(&filename);
        std::fs::write(&path, &content).map_err(|e| format!("Write error: {}", e))?;
        written.push(filename);
    }
    Ok(written)
}

#[tauri::command]
pub fn get_cnc_parts(
    state: State<'_, AppState>,
    id: String,
) -> Result<Vec<ofs_core::cnc::CncPart>, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    let id: uuid::Uuid = id.parse().map_err(|e: uuid::Error| e.to_string())?;
    let kozijn = project.kozijnen.iter().find(|k| k.id == id)
        .ok_or("Kozijn niet gevonden")?;
    Ok(ofs_core::cnc::generate_cnc_parts(kozijn))
}
