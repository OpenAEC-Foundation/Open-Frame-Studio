use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdsRequirement {
    pub entity_type: String,      // "IfcWindow", "IfcDoor"
    pub pset_name: String,        // "Pset_WindowCommon"
    pub property_name: String,    // "ThermalTransmittance"
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdsCheckResult {
    pub requirement: String,
    pub kozijn_mark: String,
    pub passed: bool,
    pub actual_value: Option<String>,
    pub message: String,
}

/// Validate a kozijn against IDS requirements
pub fn validate_kozijn(
    kozijn: &crate::kozijn::Kozijn,
    profiles: &[crate::profile::ProfileDefinition],
    requirements: &[IdsRequirement],
) -> Vec<IdsCheckResult> {
    let mut results = Vec::new();

    for req in requirements {
        let check = match req.property_name.as_str() {
            "ThermalTransmittance" | "Uw" => {
                let uw = crate::thermal::calculate_uw(kozijn, profiles);
                IdsCheckResult {
                    requirement: format!("{}.{}", req.pset_name, req.property_name),
                    kozijn_mark: kozijn.mark.clone(),
                    passed: uw.uw_value > 0.0,
                    actual_value: Some(format!("{:.2}", uw.uw_value)),
                    message: if uw.uw_value > 0.0 { "OK".into() } else { "Uw niet berekend".into() },
                }
            },
            "FrameDepth" => {
                IdsCheckResult {
                    requirement: format!("{}.{}", req.pset_name, req.property_name),
                    kozijn_mark: kozijn.mark.clone(),
                    passed: kozijn.frame.frame_depth > 0.0,
                    actual_value: Some(format!("{:.0}mm", kozijn.frame.frame_depth)),
                    message: "OK".into(),
                }
            },
            _ => {
                IdsCheckResult {
                    requirement: format!("{}.{}", req.pset_name, req.property_name),
                    kozijn_mark: kozijn.mark.clone(),
                    passed: false,
                    actual_value: None,
                    message: format!("Property '{}' niet gecontroleerd", req.property_name),
                }
            }
        };
        results.push(check);
    }
    results
}

/// Built-in IDS requirements for common Dutch standards
pub fn default_ids_requirements() -> Vec<IdsRequirement> {
    vec![
        IdsRequirement { entity_type: "IfcWindow".into(), pset_name: "Pset_WindowCommon".into(), property_name: "ThermalTransmittance".into(), required: true },
        IdsRequirement { entity_type: "IfcWindow".into(), pset_name: "Pset_WindowCommon".into(), property_name: "FrameDepth".into(), required: true },
        IdsRequirement { entity_type: "IfcDoor".into(), pset_name: "Pset_DoorCommon".into(), property_name: "ThermalTransmittance".into(), required: true },
    ]
}
