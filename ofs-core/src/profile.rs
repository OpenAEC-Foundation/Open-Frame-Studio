use serde::{Deserialize, Serialize};

/// Reference to a profile in the library
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileRef {
    pub id: String,
    pub name: String,
}

impl ProfileRef {
    pub fn default_frame() -> Self {
        Self {
            id: "wood-meranti-67x114".into(),
            name: "Meranti 67x114mm".into(),
        }
    }

    pub fn default_sill() -> Self {
        Self {
            id: "wood-meranti-67x150".into(),
            name: "Meranti 67x150mm (dorpel)".into(),
        }
    }

    pub fn default_divider() -> Self {
        Self {
            id: "wood-meranti-67x114".into(),
            name: "Meranti 67x114mm".into(),
        }
    }
}

/// A profile definition from the library
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDefinition {
    pub id: String,
    pub name: String,
    pub material: String,
    pub material_subtype: Option<String>,
    /// Face width in mm
    pub width: f64,
    /// Depth in mm
    pub depth: f64,
    /// Visible face width after glazing
    pub sightline: f64,
    /// Glazing rebate depth in mm
    pub glazing_rebate: f64,
    /// 2D cross-section polygon points
    pub cross_section: Vec<[f64; 2]>,
    /// Thermal transmittance of the frame (W/m²K)
    pub uf_value: f64,
    /// Where this profile can be used
    pub applicable_as: Vec<ProfileApplication>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileApplication {
    Frame,
    Sash,
    Divider,
    Sill,
}
