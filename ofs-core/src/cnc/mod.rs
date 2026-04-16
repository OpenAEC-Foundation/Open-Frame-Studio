pub mod postprocessor;
pub mod gcode;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CncPart {
    pub piece_id: String,
    pub kozijn_mark: String,
    pub profile_name: String,
    pub material: String,
    pub gross_length_mm: f64,
    pub operations: Vec<CncOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CncOperation {
    CrossCut { position_mm: f64, angle_deg: f64 },
    Mortise { x: f64, y: f64, width: f64, depth: f64, length: f64 },
    Tenon { x: f64, y: f64, width: f64, length: f64 },
    Drill { x: f64, y: f64, diameter: f64, depth: f64 },
    Groove { start_x: f64, end_x: f64, depth: f64, width: f64 },
}

pub fn generate_cnc_parts(kozijn: &crate::kozijn::Kozijn) -> Vec<CncPart> {
    let prod = crate::production::compute_production_data(kozijn);
    let mut parts = Vec::new();

    for cut in &prod.cut_list {
        let mut ops = Vec::new();
        // Cross cuts at both ends
        ops.push(CncOperation::CrossCut { position_mm: 0.0, angle_deg: cut.miter_left_deg });
        ops.push(CncOperation::CrossCut { position_mm: cut.gross_length_mm, angle_deg: cut.miter_right_deg });

        // Mortise/tenon for pen/slis joints
        if cut.miter_left_deg.abs() < 1.0 || cut.miter_right_deg.abs() < 1.0 {
            // Horizontal members get tenons
            let mt = cut.member_type.label_nl();
            if mt.contains("dorpel") || mt.contains("Dorpel") || mt.contains("boven") || mt.contains("Boven") {
                let pw = 20.0; // pen length
                ops.push(CncOperation::Tenon { x: 0.0, y: 0.0, width: pw, length: cut.gross_length_mm.min(20.0) });
                ops.push(CncOperation::Tenon { x: cut.gross_length_mm - pw, y: 0.0, width: pw, length: 20.0 });
            }
        }

        // Sponning groove along length
        ops.push(CncOperation::Groove {
            start_x: 20.0,
            end_x: cut.gross_length_mm - 20.0,
            depth: 17.0,
            width: 12.0,
        });

        parts.push(CncPart {
            piece_id: cut.piece_id.clone(),
            kozijn_mark: prod.kozijn_mark.clone(),
            profile_name: cut.profile_name.clone(),
            material: cut.material.clone(),
            gross_length_mm: cut.gross_length_mm,
            operations: ops,
        });
    }
    parts
}
