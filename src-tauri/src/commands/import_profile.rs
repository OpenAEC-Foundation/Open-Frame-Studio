use tauri::State;
use crate::state::AppState;

#[tauri::command]
pub async fn import_dxf_profile(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<String, String> {
    let python_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Cannot find exe directory")?
        .join("../python");

    let python_dir = if python_dir.exists() {
        python_dir
    } else {
        std::path::PathBuf::from("python")
    };

    let output = crate::state::python_command()
        .arg(python_dir.join("main.py"))
        .arg("import-dxf-profile")
        .arg("--file")
        .arg(&file_path)
        .output()
        .await
        .map_err(|e| format!("Python sidecar fout: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse the JSON response to extract just the profile
        let response: serde_json::Value =
            serde_json::from_str(&stdout).map_err(|e| format!("JSON parse fout: {}", e))?;
        let profile = response
            .get("profile")
            .ok_or("Geen profiel in response")?;
        Ok(profile.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("DXF profiel import mislukt: {}", stderr))
    }
}

#[tauri::command]
pub async fn import_catalog(
    _state: State<'_, AppState>,
    file_path: String,
    supplier: Option<String>,
) -> Result<String, String> {
    let python_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Cannot find exe directory")?
        .join("../python");

    let python_dir = if python_dir.exists() {
        python_dir
    } else {
        std::path::PathBuf::from("python")
    };

    let mut cmd = crate::state::python_command();
    cmd.arg(python_dir.join("main.py"))
        .arg("import-catalog")
        .arg("--file")
        .arg(&file_path);

    if let Some(ref s) = supplier {
        cmd.arg("--supplier").arg(s);
    }

    let output = cmd
        .output()
        .await
        .map_err(|e| format!("Python sidecar fout: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let response: serde_json::Value =
            serde_json::from_str(&stdout).map_err(|e| format!("JSON parse fout: {}", e))?;
        let profiles = response
            .get("profiles")
            .ok_or("Geen profielen in response")?;
        Ok(profiles.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Catalogus import mislukt: {}", stderr))
    }
}
