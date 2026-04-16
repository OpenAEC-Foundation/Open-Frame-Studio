#[tauri::command]
pub fn get_glass_library() -> Vec<ofs_core::glass_library::GlassProduct> {
    ofs_core::glass_library::builtin_glass_library()
}
