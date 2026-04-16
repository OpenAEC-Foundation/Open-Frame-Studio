use tauri::State;
use crate::state::AppState;

#[tauri::command]
pub fn optimize_project_cut_list(
    state: State<'_, AppState>,
    stock_length_mm: Option<f64>,
) -> Result<ofs_core::optimization::CutPlan, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    let stock = stock_length_mm.unwrap_or(5800.0);

    // Collect all cut pieces from all kozijnen
    let mut pieces = Vec::new();
    for kozijn in &project.kozijnen {
        let prod = ofs_core::production::compute_production_data(kozijn);
        for item in &prod.cut_list {
            pieces.push((
                item.piece_id.clone(),
                prod.kozijn_mark.clone(),
                item.gross_length_mm,
            ));
        }
    }

    Ok(ofs_core::optimization::optimize_cut_list(pieces, stock, 4.0))
}
