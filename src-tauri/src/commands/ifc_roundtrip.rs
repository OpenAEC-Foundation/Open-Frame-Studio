#[tauri::command]
pub fn compare_ifc_files(
    old_path: String,
    new_path: String,
) -> Result<String, String> {
    let old = ofs_core::import::ifc_import::parse_ifc_file(&old_path)?;
    let new = ofs_core::import::ifc_import::parse_ifc_file(&new_path)?;
    let diff = ofs_core::ifc_roundtrip::compare_ifc_imports(&old, &new);
    serde_json::to_string(&diff).map_err(|e| e.to_string())
}
