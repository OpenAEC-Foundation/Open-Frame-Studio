use crate::state::AppState;
use ofs_core::quotation::{Quotation, QuotationStatus};
use tauri::State;

#[tauri::command]
pub fn get_quotations(state: State<'_, AppState>) -> Result<Vec<Quotation>, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    Ok(project.quotations.clone())
}

#[tauri::command]
pub fn create_quotation(
    state: State<'_, AppState>,
    kozijn_marks: Vec<String>,
    total_incl_btw: f64,
) -> Result<Quotation, String> {
    let mut project = state.project.lock().map_err(|e| e.to_string())?;
    let quotation = Quotation::new_draft(kozijn_marks, total_incl_btw);
    project.quotations.push(quotation.clone());
    Ok(quotation)
}

#[tauri::command]
pub fn update_quotation_status(
    state: State<'_, AppState>,
    quotation_id: String,
    status: QuotationStatus,
) -> Result<Quotation, String> {
    let mut project = state.project.lock().map_err(|e| e.to_string())?;
    let quotation = project
        .quotations
        .iter_mut()
        .find(|q| q.id == quotation_id)
        .ok_or("Offerte niet gevonden")?;
    quotation.status = status;
    Ok(quotation.clone())
}

#[tauri::command]
pub fn create_quotation_revision(
    state: State<'_, AppState>,
    quotation_id: String,
    new_total: f64,
    change_description: String,
) -> Result<Quotation, String> {
    let mut project = state.project.lock().map_err(|e| e.to_string())?;
    let original = project
        .quotations
        .iter()
        .find(|q| q.id == quotation_id)
        .ok_or("Offerte niet gevonden")?
        .clone();
    let revision = original.create_revision(new_total, &change_description);
    project.quotations.push(revision.clone());
    Ok(revision)
}
