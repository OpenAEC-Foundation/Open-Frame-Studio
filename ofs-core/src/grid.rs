use crate::kozijn::{GridDivision, Kozijn, PanelType, OpeningDirection};
use crate::profile::ProfileRef;
use crate::template::KozijnSjabloon;

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

// ── Sjabloon-aware templates ──────────────────────────────────

pub fn template_single_turn_tilt_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let mut k = Kozijn::new_with_sjabloon("Draaikiepraam", "K01", width, height, sj);
    k.cells[0].panel_type = PanelType::TurnTilt;
    k.cells[0].opening_direction = Some(OpeningDirection::Left);
    k.cells[0].assign_sash_from_sjabloon(sj);
    k
}

pub fn template_double_turn_tilt_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let fw = sj.frame_width;
    let inner_w = width - 2.0 * fw;
    let half = (inner_w - fw) / 2.0;

    let mut k = Kozijn::new_with_sjabloon("Dubbel draaikiepraam", "K01", width, height, sj);
    k.grid.columns = vec![
        GridDivision { size: half, divider_profile: None },
        GridDivision { size: half, divider_profile: Some(sj.tussenstijl_profile.clone()) },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = PanelType::TurnTilt;
    k.cells[0].opening_direction = Some(OpeningDirection::Left);
    k.cells[0].assign_sash_from_sjabloon(sj);
    k.cells[1].panel_type = PanelType::TurnTilt;
    k.cells[1].opening_direction = Some(OpeningDirection::Right);
    k.cells[1].assign_sash_from_sjabloon(sj);
    k
}

pub fn template_sliding_door_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let fw = sj.frame_width;
    let inner_w = width - 2.0 * fw;
    let half = (inner_w - fw) / 2.0;

    let mut k = Kozijn::new_with_sjabloon("Schuifpui", "P01", width, height, sj);
    k.grid.columns = vec![
        GridDivision { size: half, divider_profile: None },
        GridDivision { size: half, divider_profile: Some(sj.tussenstijl_profile.clone()) },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = PanelType::FixedGlass;
    k.cells[1].panel_type = PanelType::Sliding;
    k.cells[1].assign_sash_from_sjabloon(sj);
    k
}

pub fn template_front_door_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let fw = sj.frame_width;
    let inner_h = height - 2.0 * fw;
    let top_light = 400.0;
    let door_h = inner_h - top_light - fw;

    let mut k = Kozijn::new_with_sjabloon("Voordeur", "D01", width, height, sj);
    k.grid.rows = vec![
        GridDivision { size: top_light, divider_profile: None },
        GridDivision { size: door_h, divider_profile: Some(sj.tussendorpel_profile.clone()) },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = PanelType::FixedGlass;
    k.cells[1].panel_type = PanelType::Door;
    k.cells[1].opening_direction = Some(OpeningDirection::Inward);
    k.cells[1].assign_sash_from_sjabloon(sj);
    k
}

pub fn template_top_hung_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let mut k = Kozijn::new_with_sjabloon("Klapraam", "K01", width, height, sj);
    k.cells[0].panel_type = PanelType::TopHung;
    k.cells[0].opening_direction = Some(OpeningDirection::Outward);
    k.cells[0].assign_sash_from_sjabloon(sj);
    k
}

pub fn template_lift_slide_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let fw = sj.frame_width;
    let inner_w = width - 2.0 * fw;
    let half = (inner_w - fw) / 2.0;

    let mut k = Kozijn::new_with_sjabloon("Hefschuifpui", "P01", width, height, sj);
    k.grid.columns = vec![
        GridDivision { size: half, divider_profile: None },
        GridDivision { size: half, divider_profile: Some(sj.tussenstijl_profile.clone()) },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = PanelType::FixedGlass;
    k.cells[1].panel_type = PanelType::LiftSlide;
    k.cells[1].assign_sash_from_sjabloon(sj);
    k
}

pub fn template_pivot_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let mut k = Kozijn::new_with_sjabloon("Pivotraam", "K01", width, height, sj);
    k.cells[0].panel_type = PanelType::Pivot;
    k.cells[0].opening_direction = Some(OpeningDirection::Inward);
    k.cells[0].assign_sash_from_sjabloon(sj);
    k
}

pub fn template_stolp_sj(width: f64, height: f64, sj: &KozijnSjabloon) -> Kozijn {
    let fw = sj.frame_width;
    let inner_w = width - 2.0 * fw;
    let half = (inner_w - fw) / 2.0;

    let mut k = Kozijn::new_with_sjabloon("Stolpraam", "K01", width, height, sj);
    k.grid.columns = vec![
        GridDivision { size: half, divider_profile: None },
        GridDivision { size: half, divider_profile: Some(sj.tussenstijl_profile.clone()) },
    ];
    k.rebuild_cells();
    k.cells[0].panel_type = PanelType::Turn;
    k.cells[0].opening_direction = Some(OpeningDirection::Left);
    k.cells[0].assign_sash_from_sjabloon(sj);
    k.cells[1].panel_type = PanelType::Turn;
    k.cells[1].opening_direction = Some(OpeningDirection::Right);
    k.cells[1].assign_sash_from_sjabloon(sj);
    k
}

// ── Legacy templates (backward compatible, use default sjabloon) ──

pub fn template_single_turn_tilt(width: f64, height: f64) -> Kozijn {
    template_single_turn_tilt_sj(width, height, &crate::template::default_sjabloon())
}

pub fn template_double_turn_tilt(width: f64, height: f64) -> Kozijn {
    template_double_turn_tilt_sj(width, height, &crate::template::default_sjabloon())
}

pub fn template_sliding_door(width: f64, height: f64) -> Kozijn {
    template_sliding_door_sj(width, height, &crate::template::default_sjabloon())
}

pub fn template_front_door(width: f64, height: f64) -> Kozijn {
    template_front_door_sj(width, height, &crate::template::default_sjabloon())
}

pub fn template_top_hung(width: f64, height: f64) -> Kozijn {
    template_top_hung_sj(width, height, &crate::template::default_sjabloon())
}

pub fn template_lift_slide(width: f64, height: f64) -> Kozijn {
    template_lift_slide_sj(width, height, &crate::template::default_sjabloon())
}

pub fn template_pivot(width: f64, height: f64) -> Kozijn {
    template_pivot_sj(width, height, &crate::template::default_sjabloon())
}

pub fn template_stolp(width: f64, height: f64) -> Kozijn {
    template_stolp_sj(width, height, &crate::template::default_sjabloon())
}
