#[tauri::command]
pub fn import_ifc_file(file_path: String) -> Result<String, String> {
    let result = ofs_core::import::ifc_import::parse_ifc_file(&file_path)?;
    serde_json::to_string(&result).map_err(|e| e.to_string())
}
