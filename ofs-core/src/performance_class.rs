use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceClassification {
    pub air_permeability: String,     // "Klasse 3" etc
    pub water_tightness: String,
    pub wind_resistance: String,
    pub thermal_transmittance: String,
    pub sound_insulation: String,
    pub burglar_resistance: String,
}

pub fn classify_performance(
    kozijn: &crate::kozijn::Kozijn,
    profiles: &[crate::profile::ProfileDefinition],
) -> PerformanceClassification {
    let uw = crate::thermal::calculate_uw(kozijn, profiles).uw_value;

    // Simplified classification based on Dutch market norms
    let area_m2 = (kozijn.frame.outer_width / 1000.0) * (kozijn.frame.outer_height / 1000.0);

    // Air permeability (EN 12207) -- depends on gasket quality
    let air = if area_m2 < 2.0 { "Klasse 4" } else { "Klasse 3" };

    // Water tightness (EN 12208) -- depends on sill design and gasket
    let water = if kozijn.frame.sill.is_some() { "Klasse 7A" } else { "Klasse 5A" };

    // Wind resistance (EN 12210) -- depends on frame size and profile section modulus
    let wind = if area_m2 < 3.0 && kozijn.frame.frame_width >= 67.0 { "Klasse C4" }
        else if area_m2 < 5.0 { "Klasse C3" }
        else { "Klasse C2" };

    // Thermal
    let thermal = format!("Uw = {:.2} W/m\u{00b2}K", uw);

    // Sound -- depends on glass type
    let sound = "Rw >= 30 dB (indicatief)".to_string();

    // Burglar resistance -- from hardware security class
    let rc = kozijn.cells.first()
        .and_then(|c| c.hardware_set.as_ref())
        .map(|h| format!("{:?}", h.security_class))
        .unwrap_or_else(|| "Niet geclassificeerd".to_string());

    PerformanceClassification {
        air_permeability: air.into(),
        water_tightness: water.into(),
        wind_resistance: wind.into(),
        thermal_transmittance: thermal,
        sound_insulation: sound,
        burglar_resistance: rc,
    }
}
