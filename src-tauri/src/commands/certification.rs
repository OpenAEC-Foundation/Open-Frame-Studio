use tauri::State;
use crate::state::AppState;
use ofs_core::certification::{CertificationResult, check_ce_marking, check_skh_komo};
use ofs_core::performance_class::{PerformanceClassification, classify_performance};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullCertificationResult {
    pub ce_marking: CertificationResult,
    pub skh_komo: CertificationResult,
    pub performance_class: PerformanceClassification,
}

#[tauri::command]
pub fn check_certification(
    state: State<'_, AppState>,
    id: String,
) -> Result<FullCertificationResult, String> {
    let project = state.project.lock().map_err(|e| e.to_string())?;
    let id: uuid::Uuid = id.parse().map_err(|e: uuid::Error| e.to_string())?;
    let kozijn = project.kozijnen.iter().find(|k| k.id == id)
        .ok_or("Kozijn niet gevonden")?;
    let profiles = &project.custom_profiles;

    Ok(FullCertificationResult {
        ce_marking: check_ce_marking(kozijn, profiles),
        skh_komo: check_skh_komo(kozijn),
        performance_class: classify_performance(kozijn, profiles),
    })
}
