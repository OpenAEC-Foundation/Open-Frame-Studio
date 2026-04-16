use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BengContribution {
    pub kozijn_mark: String,
    pub uw_value: f64,
    pub g_value: f64,
    pub area_m2: f64,
    pub orientation: String,
    pub transmission_loss_w_per_k: f64,  // U x A
    pub solar_gain_factor: f64,          // g x A
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectEnergyResult {
    pub kozijn_contributions: Vec<BengContribution>,
    pub total_transmission_loss: f64,
    pub total_solar_gain_factor: f64,
    pub average_uw: f64,
    pub bouwbesluit_max_uw: f64,  // 1.65 for renovation, 1.4 for new build
    pub compliant: bool,
}

pub fn calculate_project_energy(
    project: &crate::kozijn::Project,
    profiles: &[crate::profile::ProfileDefinition],
    max_uw: f64,  // Bouwbesluit limit
) -> ProjectEnergyResult {
    let mut contributions = Vec::new();
    let mut total_ua = 0.0;
    let mut total_ga = 0.0;
    let mut total_area = 0.0;

    for kozijn in &project.kozijnen {
        let uw_result = crate::thermal::calculate_uw(kozijn, profiles);
        let uw = uw_result.uw_value;

        let ow = kozijn.frame.outer_width / 1000.0;
        let oh = kozijn.frame.outer_height / 1000.0;
        let area = ow * oh;

        // g-value: default for HR++ glass
        let g = 0.60;

        let ua = uw * area;
        let ga = g * area;
        total_ua += ua;
        total_ga += ga;
        total_area += area;

        contributions.push(BengContribution {
            kozijn_mark: kozijn.mark.clone(),
            uw_value: (uw * 100.0).round() / 100.0,
            g_value: g,
            area_m2: (area * 100.0).round() / 100.0,
            orientation: "Onbekend".into(),
            transmission_loss_w_per_k: (ua * 100.0).round() / 100.0,
            solar_gain_factor: (ga * 100.0).round() / 100.0,
        });
    }

    let avg_uw = if total_area > 0.0 { total_ua / total_area } else { 0.0 };

    ProjectEnergyResult {
        kozijn_contributions: contributions,
        total_transmission_loss: (total_ua * 100.0).round() / 100.0,
        total_solar_gain_factor: (total_ga * 100.0).round() / 100.0,
        average_uw: (avg_uw * 100.0).round() / 100.0,
        bouwbesluit_max_uw: max_uw,
        compliant: avg_uw <= max_uw,
    }
}
