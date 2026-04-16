use super::postprocessor::CncPostProcessor;
use super::{CncPart, CncOperation};

pub struct GenericGCode;

impl CncPostProcessor for GenericGCode {
    fn name(&self) -> &str { "Generic G-Code" }
    fn extension(&self) -> &str { "nc" }

    fn generate(&self, parts: &[CncPart]) -> Result<Vec<(String, String)>, String> {
        let mut files = Vec::new();
        for part in parts {
            let mut code = String::new();
            code.push_str(&format!("( Part: {} - {} )\n", part.piece_id, part.kozijn_mark));
            code.push_str(&format!("( Profile: {} L={:.0}mm )\n", part.profile_name, part.gross_length_mm));
            code.push_str("G90 G21\n"); // absolute, mm
            code.push_str("G28\n"); // home

            for op in &part.operations {
                match op {
                    CncOperation::CrossCut { position_mm, angle_deg } => {
                        code.push_str(&format!("( CrossCut at {:.1}mm, angle {:.1}deg )\n", position_mm, angle_deg));
                        code.push_str(&format!("G0 X{:.1}\n", position_mm));
                        code.push_str("G1 Z-50 F500\n");
                        code.push_str("G0 Z5\n");
                    }
                    CncOperation::Mortise { x, y, width: _, depth, length } => {
                        code.push_str(&format!("( Mortise at X{:.1} Y{:.1} D{:.1} L{:.1} )\n", x, y, depth, length));
                        code.push_str(&format!("G0 X{:.1} Y{:.1}\n", x, y));
                        code.push_str(&format!("G1 Z-{:.1} F300\n", depth));
                        code.push_str(&format!("G1 X{:.1}\n", x + length));
                        code.push_str("G0 Z5\n");
                    }
                    CncOperation::Tenon { x, y, width, length } => {
                        code.push_str(&format!("( Tenon at X{:.1} Y{:.1} W{:.1} L{:.1} )\n", x, y, width, length));
                    }
                    CncOperation::Drill { x, y, diameter, depth } => {
                        code.push_str(&format!("G0 X{:.1} Y{:.1}\n", x, y));
                        code.push_str(&format!("G83 Z-{:.1} Q2 R1 F200 ( drill d={:.1} )\n", depth, diameter));
                        code.push_str("G0 Z5\n");
                    }
                    CncOperation::Groove { start_x, end_x, depth, width } => {
                        code.push_str(&format!("( Groove {:.1}-{:.1} D{:.1} W{:.1} )\n", start_x, end_x, depth, width));
                        code.push_str(&format!("G0 X{:.1}\n", start_x));
                        code.push_str(&format!("G1 Z-{:.1} F400\n", depth));
                        code.push_str(&format!("G1 X{:.1}\n", end_x));
                        code.push_str("G0 Z5\n");
                    }
                }
            }
            code.push_str("G28\nM30\n");
            files.push((format!("{}.nc", part.piece_id), code));
        }
        Ok(files)
    }
}
