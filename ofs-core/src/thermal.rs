use serde::{Deserialize, Serialize};
use crate::kozijn::Kozijn;
use crate::profile::ProfileDefinition;

/// Thermal performance result per EN ISO 10077-1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThermalResult {
    /// Window U-value Uw (W/m²K)
    pub uw_value: f64,
    /// Average frame U-value Uf (W/m²K)
    pub uf_value: f64,
    /// Average glass U-value Ug (W/m²K)
    pub ug_value: f64,
    /// Linear thermal transmittance at glass edge Ψg (W/mK)
    pub psi_value: f64,
    /// Total frame area (m²)
    pub area_frame_m2: f64,
    /// Total glass area (m²)
    pub area_glass_m2: f64,
    /// Total window area (m²)
    pub area_total_m2: f64,
    /// Glass perimeter for Ψg calculation (m)
    pub glass_perimeter_m: f64,
    /// Rating: "A" (< 1.3), "B" (1.3-1.8), "C" (1.8-2.5), "D" (> 2.5)
    pub rating: String,
}

/// Calculate Uw-value according to EN ISO 10077-1
///
/// Formula: Uw = (Af×Uf + Ag×Ug + lg×Ψg) / (Af + Ag)
///
/// Where:
/// - Af = frame area (m²)
/// - Uf = frame U-value (W/m²K)
/// - Ag = glass area (m²)
/// - Ug = glass U-value (W/m²K)
/// - lg = glass perimeter (m)
/// - Ψg = linear thermal transmittance at glass edge (W/mK)
pub fn calculate_uw(kozijn: &Kozijn, profiles: &[ProfileDefinition]) -> ThermalResult {
    let outer_w = kozijn.frame.outer_width;
    let outer_h = kozijn.frame.outer_height;
    let frame_w = kozijn.frame.frame_width;

    // Total window area
    let area_total = (outer_w * outer_h) / 1_000_000.0; // mm² to m²

    // Frame area: outer - inner (simplified as frame border area)
    let inner_w = outer_w - 2.0 * frame_w;
    let inner_h = outer_h - 2.0 * frame_w;
    let area_inner = (inner_w * inner_h) / 1_000_000.0;

    // Account for dividers
    let num_v_dividers = if kozijn.grid.columns.len() > 1 {
        kozijn.grid.columns.len() - 1
    } else {
        0
    };
    let num_h_dividers = if kozijn.grid.rows.len() > 1 {
        kozijn.grid.rows.len() - 1
    } else {
        0
    };

    let divider_area = (num_v_dividers as f64 * frame_w * inner_h
        + num_h_dividers as f64 * frame_w * inner_w) / 1_000_000.0;

    let area_frame = area_total - area_inner + divider_area;
    let area_glass = area_total - area_frame;

    // Look up Uf from profile library
    let uf_value = profiles
        .iter()
        .find(|p| p.id == kozijn.frame.profile.id)
        .map(|p| p.uf_value)
        .unwrap_or(1.8); // default wood

    // Average Ug from all cells
    let total_ug: f64 = kozijn.cells.iter().map(|c| c.glazing.ug_value).sum();
    let ug_value = if kozijn.cells.is_empty() {
        1.0
    } else {
        total_ug / kozijn.cells.len() as f64
    };

    // Ψg: linear thermal transmittance at glass edge
    // Typical values: aluminum spacer = 0.08, warm-edge = 0.04, super-spacer = 0.03
    let psi_value = estimate_psi(&kozijn.cells);

    // Glass perimeter: sum of all cell perimeters
    let cols = &kozijn.grid.columns;
    let rows = &kozijn.grid.rows;
    let mut glass_perimeter = 0.0;
    for col in cols.iter() {
        for row in rows.iter() {
            let w = col.size - frame_w; // approximate glass width
            let h = row.size - frame_w;
            if w > 0.0 && h > 0.0 {
                glass_perimeter += 2.0 * (w + h) / 1000.0; // mm to m
            }
        }
    }

    // EN ISO 10077-1 formula
    let uw_value = if area_total > 0.0 {
        (area_frame * uf_value + area_glass * ug_value + glass_perimeter * psi_value) / area_total
    } else {
        0.0
    };

    let rating = match uw_value {
        v if v < 1.3 => "A",
        v if v < 1.8 => "B",
        v if v < 2.5 => "C",
        _ => "D",
    }
    .to_string();

    ThermalResult {
        uw_value: round2(uw_value),
        uf_value: round2(uf_value),
        ug_value: round2(ug_value),
        psi_value: round3(psi_value),
        area_frame_m2: round3(area_frame),
        area_glass_m2: round3(area_glass),
        area_total_m2: round3(area_total),
        glass_perimeter_m: round2(glass_perimeter),
        rating,
    }
}

fn estimate_psi(cells: &[crate::kozijn::Cell]) -> f64 {
    // Use first cell's spacer type to determine Ψg
    let spacer = cells
        .first()
        .map(|c| c.glazing.spacer_type.as_str())
        .unwrap_or("warm-edge");

    match spacer {
        "aluminium" => 0.08,
        "warm-edge" | "warm_edge" => 0.04,
        "super-spacer" | "super_spacer" => 0.03,
        _ => 0.06, // generic default
    }
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}
