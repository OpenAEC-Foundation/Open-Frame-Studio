use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionPlan {
    pub jobs: Vec<ProductionJob>,
    pub total_hours: f64,
    pub estimated_days: f64,
    pub delivery_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionJob {
    pub kozijn_mark: String,
    pub kozijn_name: String,
    pub phases: Vec<ProductionPhase>,
    pub total_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionPhase {
    pub name: String,
    pub estimated_hours: f64,
}

/// Estimate labor hours for a single kozijn based on complexity.
pub fn estimate_labor_hours(kozijn: &crate::kozijn::Kozijn) -> f64 {
    let num_cells = kozijn.cells.len();
    let num_operable = kozijn.cells.iter()
        .filter(|c| c.panel_type.is_operable())
        .count();

    // Base: 1 hour per kozijn + 0.5 per cell + 1.0 per operable cell
    let fabrication = 1.0 + (num_cells as f64 * 0.5) + (num_operable as f64 * 1.0);

    // Surface treatment: 0.3 hours per m2
    let outer_area_m2 = (kozijn.frame.outer_width * kozijn.frame.outer_height) / 1e6;
    let treatment = if matches!(kozijn.frame.material, crate::kozijn::Material::Wood(_) | crate::kozijn::Material::WoodAluminum) {
        outer_area_m2 * 0.3
    } else {
        0.0
    };

    // Assembly: 0.5 hours per kozijn + 0.25 per cell
    let assembly = 0.5 + (num_cells as f64 * 0.25);

    fabrication + treatment + assembly
}

pub fn generate_production_plan(
    project: &crate::kozijn::Project,
    hours_per_day: f64,
    workers: u32,
) -> ProductionPlan {
    let h_per_day = if hours_per_day > 0.0 { hours_per_day } else { 8.0 };
    let w = if workers > 0 { workers } else { 2 };

    let mut jobs = Vec::new();
    let mut total_hours = 0.0;

    for kozijn in &project.kozijnen {
        let est = estimate_labor_hours(kozijn);
        let phases = vec![
            ProductionPhase { name: "Zagen".into(), estimated_hours: est * 0.15 },
            ProductionPhase { name: "Bewerken (frezen/boren)".into(), estimated_hours: est * 0.20 },
            ProductionPhase { name: "Oppervlaktebehandeling".into(), estimated_hours: est * 0.15 },
            ProductionPhase { name: "Assemblage".into(), estimated_hours: est * 0.25 },
            ProductionPhase { name: "Beglazing".into(), estimated_hours: est * 0.10 },
            ProductionPhase { name: "Afwerking & controle".into(), estimated_hours: est * 0.15 },
        ];
        total_hours += est;
        jobs.push(ProductionJob {
            kozijn_mark: kozijn.mark.clone(),
            kozijn_name: kozijn.name.clone(),
            phases,
            total_hours: est,
        });
    }

    let days = total_hours / (h_per_day * w as f64);
    let delivery = chrono::Utc::now() + chrono::Duration::days(days.ceil() as i64 + 2); // +2 buffer

    ProductionPlan {
        jobs,
        total_hours: (total_hours * 10.0).round() / 10.0,
        estimated_days: (days * 10.0).round() / 10.0,
        delivery_date: delivery.format("%Y-%m-%d").to_string(),
    }
}
