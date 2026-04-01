use crate::kozijn::{GridDivision, Kozijn};
use crate::profile::ProfileRef;

/// Distribute grid divisions evenly across available space
pub fn distribute_evenly(count: usize, total_size: f64, divider_width: f64) -> Vec<GridDivision> {
    let total_divider_space = if count > 1 {
        (count - 1) as f64 * divider_width
    } else {
        0.0
    };
    let cell_size = (total_size - total_divider_space) / count as f64;

    (0..count)
        .map(|i| GridDivision {
            size: cell_size,
            divider_profile: if i > 0 {
                Some(ProfileRef::default_divider())
            } else {
                None
            },
        })
        .collect()
}

/// Create a kozijn from a common template
pub fn template_single_turn_tilt(width: f64, height: f64) -> Kozijn {
    let mut k = Kozijn::new("Draaikiepraam", "K01", width, height);
    k.cells[0].panel_type = crate::kozijn::PanelType::TurnTilt;
    k.cells[0].opening_direction = Some(crate::kozijn::OpeningDirection::Left);
    k
}

pub fn template_double_turn_tilt(width: f64, height: f64) -> Kozijn {
    let fw = 67.0;
    let inner_w = width - 2.0 * fw;
    let half = (inner_w - fw) / 2.0; // minus one divider

    let mut k = Kozijn::new("Dubbel draaikiepraam", "K01", width, height);
    k.grid.columns = vec![
        GridDivision {
            size: half,
            divider_profile: None,
        },
        GridDivision {
            size: half,
            divider_profile: Some(ProfileRef::default_divider()),
        },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = crate::kozijn::PanelType::TurnTilt;
    k.cells[0].opening_direction = Some(crate::kozijn::OpeningDirection::Left);
    k.cells[1].panel_type = crate::kozijn::PanelType::TurnTilt;
    k.cells[1].opening_direction = Some(crate::kozijn::OpeningDirection::Right);
    k
}

pub fn template_sliding_door(width: f64, height: f64) -> Kozijn {
    let fw = 67.0;
    let inner_w = width - 2.0 * fw;
    let half = (inner_w - fw) / 2.0;

    let mut k = Kozijn::new("Schuifpui", "P01", width, height);
    k.grid.columns = vec![
        GridDivision {
            size: half,
            divider_profile: None,
        },
        GridDivision {
            size: half,
            divider_profile: Some(ProfileRef::default_divider()),
        },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = crate::kozijn::PanelType::FixedGlass;
    k.cells[1].panel_type = crate::kozijn::PanelType::Sliding;
    k
}

/// Melkmeisje kozijn: deur met zijlichten (links en rechts een zijlicht met
/// borstwering eronder). Het is een 3-koloms, 2-rij kozijn:
///   Bovenlicht links | Bovenlicht midden  | Bovenlicht rechts   (rij 1: bovenlicht)
///   Glas links       | Deur               | Glas rechts          (rij 2: boven borstwering)
///
/// Of bij standaard: 3 kolommen x 2 rijen
///   [zijlicht] [bovenlicht] [zijlicht]
///   [paneel]   [deur]       [paneel]
///
/// Typisch melkmeisje: de borstwering zit ONDER de zijlichten.
/// Dat geeft: 2 rijen (bovenlicht + onderzone), 3 kolommen (links, deur, rechts)
/// - Rij 1 boven: vast glas links, vast glas midden, vast glas rechts
/// - Rij 2 onder: paneel links, deur midden, paneel rechts
pub fn template_melkmeisje(width: f64, height: f64) -> Kozijn {
    let fw = 67.0;
    let inner_w = width - 2.0 * fw;
    let inner_h = height - 2.0 * fw;

    // Zijlichten zijn typisch ~400mm breed, deur is de rest
    let side_width = 400.0_f64.min((inner_w - 2.0 * fw) / 4.0); // max 1/4
    let door_width = inner_w - 2.0 * side_width - 2.0 * fw; // minus 2 dividers

    // Borstwering is typisch ~600mm hoog (onder zijlichten)
    let borstwering_h = 600.0;
    let top_h = inner_h - borstwering_h - fw; // minus 1 horizontal divider

    let mut k = Kozijn::new("Melkmeisje", "M01", width, height);

    k.grid.columns = vec![
        GridDivision {
            size: side_width,
            divider_profile: None,
        },
        GridDivision {
            size: door_width,
            divider_profile: Some(ProfileRef::default_divider()),
        },
        GridDivision {
            size: side_width,
            divider_profile: Some(ProfileRef::default_divider()),
        },
    ];

    k.grid.rows = vec![
        GridDivision {
            size: top_h,
            divider_profile: None,
        },
        GridDivision {
            size: borstwering_h,
            divider_profile: Some(ProfileRef::default_divider()),
        },
    ];

    k.rebuild_cells();

    // Rij 1 (boven): vast glas | vast glas | vast glas
    // cells[0]=links-boven, cells[1]=midden-boven, cells[2]=rechts-boven
    k.cells[0].panel_type = crate::kozijn::PanelType::FixedGlass;
    k.cells[1].panel_type = crate::kozijn::PanelType::FixedGlass;
    k.cells[2].panel_type = crate::kozijn::PanelType::FixedGlass;

    // Rij 2 (onder): paneel | deur | paneel
    // cells[3]=links-onder, cells[4]=midden-onder, cells[5]=rechts-onder
    k.cells[3].panel_type = crate::kozijn::PanelType::Panel;
    k.cells[4].panel_type = crate::kozijn::PanelType::Door;
    k.cells[4].opening_direction = Some(crate::kozijn::OpeningDirection::Inward);
    k.cells[5].panel_type = crate::kozijn::PanelType::Panel;

    k
}

/// Melkmeisje met bovenlicht: zelfde als melkmeisje maar met extra rij
/// bovenlicht over de volle breedte
/// Rij 1: [bovenlicht over volle breedte] (via 3 vast glas cellen)
/// Rij 2: [glas links] [glas midden] [glas rechts]
/// Rij 3: [paneel]     [deur]        [paneel]
pub fn template_melkmeisje_met_bovenlicht(width: f64, height: f64) -> Kozijn {
    let fw = 67.0;
    let inner_w = width - 2.0 * fw;
    let inner_h = height - 2.0 * fw;

    let side_width = 400.0_f64.min((inner_w - 2.0 * fw) / 4.0);
    let door_width = inner_w - 2.0 * side_width - 2.0 * fw;

    let bovenlicht_h = 350.0;
    let borstwering_h = 600.0;
    let midden_h = inner_h - bovenlicht_h - borstwering_h - 2.0 * fw; // minus 2 dividers

    let mut k = Kozijn::new("Melkmeisje met bovenlicht", "M01", width, height);

    k.grid.columns = vec![
        GridDivision {
            size: side_width,
            divider_profile: None,
        },
        GridDivision {
            size: door_width,
            divider_profile: Some(ProfileRef::default_divider()),
        },
        GridDivision {
            size: side_width,
            divider_profile: Some(ProfileRef::default_divider()),
        },
    ];

    k.grid.rows = vec![
        GridDivision {
            size: bovenlicht_h,
            divider_profile: None,
        },
        GridDivision {
            size: midden_h,
            divider_profile: Some(ProfileRef::default_divider()),
        },
        GridDivision {
            size: borstwering_h,
            divider_profile: Some(ProfileRef::default_divider()),
        },
    ];

    k.rebuild_cells();

    // Rij 1 (bovenlicht): allemaal vast glas
    k.cells[0].panel_type = crate::kozijn::PanelType::FixedGlass;
    k.cells[1].panel_type = crate::kozijn::PanelType::FixedGlass;
    k.cells[2].panel_type = crate::kozijn::PanelType::FixedGlass;

    // Rij 2 (midden): glas zijlichten + deur
    k.cells[3].panel_type = crate::kozijn::PanelType::FixedGlass;
    k.cells[4].panel_type = crate::kozijn::PanelType::Door;
    k.cells[4].opening_direction = Some(crate::kozijn::OpeningDirection::Inward);
    k.cells[5].panel_type = crate::kozijn::PanelType::FixedGlass;

    // Rij 3 (borstwering): panelen
    k.cells[6].panel_type = crate::kozijn::PanelType::Panel;
    k.cells[7].panel_type = crate::kozijn::PanelType::Panel;
    k.cells[8].panel_type = crate::kozijn::PanelType::Panel;

    k
}

pub fn template_front_door(width: f64, height: f64) -> Kozijn {
    let fw = 67.0;
    let inner_h = height - 2.0 * fw;
    let top_light = 400.0;
    let door_h = inner_h - top_light - fw;

    let mut k = Kozijn::new("Voordeur", "D01", width, height);
    k.grid.rows = vec![
        GridDivision {
            size: top_light,
            divider_profile: None,
        },
        GridDivision {
            size: door_h,
            divider_profile: Some(ProfileRef::default_divider()),
        },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = crate::kozijn::PanelType::FixedGlass;
    k.cells[1].panel_type = crate::kozijn::PanelType::Door;
    k.cells[1].opening_direction = Some(crate::kozijn::OpeningDirection::Inward);
    k
}
