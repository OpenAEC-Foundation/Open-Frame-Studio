use crate::state::AppState;
use ofs_core::procurement::PurchaseProposal;
use tauri::State;

#[tauri::command]
pub fn generate_purchase_proposals(
    state: State<'_, AppState>,
) -> Result<Vec<PurchaseProposal>, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    Ok(ofs_core::procurement::generate_purchase_proposals(&project))
}
