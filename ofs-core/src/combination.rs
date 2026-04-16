use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombinationKozijn {
    pub id: Uuid,
    pub name: String,
    pub mark: String,
    pub members: Vec<CombinationMember>,
    pub couplings: Vec<Coupling>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombinationMember {
    pub kozijn_id: Uuid,
    pub offset_x: f64,
    pub offset_y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coupling {
    pub member_a_id: Uuid,
    pub member_b_id: Uuid,
    pub coupling_type: CouplingType,
    pub coupling_width: f64, // coupling profile width in mm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouplingType {
    SideToSide,   // stijlkoppeling (vertical)
    TopToBottom,  // dorpelkoppeling (horizontal)
    Corner,       // hoek
}

impl CombinationKozijn {
    pub fn new(name: &str, mark: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            mark: mark.to_string(),
            members: vec![],
            couplings: vec![],
        }
    }

    pub fn add_member(&mut self, kozijn_id: Uuid, offset_x: f64, offset_y: f64) {
        self.members.push(CombinationMember {
            kozijn_id,
            offset_x,
            offset_y,
        });
    }

    pub fn add_coupling(
        &mut self,
        a_id: Uuid,
        b_id: Uuid,
        coupling_type: CouplingType,
        width: f64,
    ) {
        self.couplings.push(Coupling {
            member_a_id: a_id,
            member_b_id: b_id,
            coupling_type,
            coupling_width: width,
        });
    }

    pub fn overall_width(&self, kozijnen: &[crate::kozijn::Kozijn]) -> f64 {
        let mut max_x = 0.0f64;
        for member in &self.members {
            if let Some(k) = kozijnen.iter().find(|k| k.id == member.kozijn_id) {
                let right = member.offset_x + k.frame.outer_width;
                max_x = max_x.max(right);
            }
        }
        max_x
    }

    pub fn overall_height(&self, kozijnen: &[crate::kozijn::Kozijn]) -> f64 {
        let mut max_y = 0.0f64;
        for member in &self.members {
            if let Some(k) = kozijnen.iter().find(|k| k.id == member.kozijn_id) {
                let bottom = member.offset_y + k.frame.outer_height;
                max_y = max_y.max(bottom);
            }
        }
        max_y
    }
}
