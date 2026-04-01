#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod state;
mod blender;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Project commands
            commands::project::new_project,
            commands::project::open_project,
            commands::project::save_project,
            commands::project::get_project,
            // Kozijn commands
            commands::kozijn::create_kozijn,
            commands::kozijn::create_kozijn_from_template,
            commands::kozijn::get_kozijn,
            commands::kozijn::get_all_kozijnen,
            commands::kozijn::update_kozijn_dimensions,
            commands::kozijn::update_cell_type,
            commands::kozijn::add_column,
            commands::kozijn::add_row,
            commands::kozijn::remove_kozijn,
            commands::kozijn::get_kozijn_geometry,
            // Export commands
            commands::export_ifc::export_ifc,
            commands::export_dxf::export_dxf,
            commands::export_pdf::export_kozijnstaat,
            commands::export_workshop::export_workshop_drawing,
            // Blender commands
            commands::blender::send_to_blender,
            commands::blender::check_blender_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Open Frame Studio");
}
