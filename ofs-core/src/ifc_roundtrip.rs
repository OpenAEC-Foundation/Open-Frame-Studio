use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcDiff {
    pub added: Vec<DiffItem>,
    pub removed: Vec<DiffItem>,
    pub modified: Vec<DiffModification>,
    pub unchanged: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffItem {
    pub guid: String,
    pub name: String,
    pub entity_type: String,
    pub width_mm: f64,
    pub height_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffModification {
    pub guid: String,
    pub name: String,
    pub changes: Vec<PropertyChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyChange {
    pub property: String,
    pub old_value: String,
    pub new_value: String,
}

/// Compare two IFC import results and produce a diff
pub fn compare_ifc_imports(
    old: &crate::import::ifc_import::IfcImportResult,
    new: &crate::import::ifc_import::IfcImportResult,
) -> IfcDiff {
    let old_guids: HashMap<&str, (&str, f64, f64)> = old
        .windows
        .iter()
        .map(|w| {
            (
                w.guid.as_str(),
                (w.name.as_str(), w.width_mm, w.height_mm),
            )
        })
        .chain(old.doors.iter().map(|d| {
            (
                d.guid.as_str(),
                (d.name.as_str(), d.width_mm, d.height_mm),
            )
        }))
        .collect();

    let new_guids: HashMap<&str, (&str, f64, f64)> = new
        .windows
        .iter()
        .map(|w| {
            (
                w.guid.as_str(),
                (w.name.as_str(), w.width_mm, w.height_mm),
            )
        })
        .chain(new.doors.iter().map(|d| {
            (
                d.guid.as_str(),
                (d.name.as_str(), d.width_mm, d.height_mm),
            )
        }))
        .collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    let mut unchanged = 0;

    // Find added and modified
    for (guid, (name, w, h)) in &new_guids {
        if let Some((_, old_w, old_h)) = old_guids.get(guid) {
            let mut changes = Vec::new();
            if (w - old_w).abs() > 1.0 {
                changes.push(PropertyChange {
                    property: "Breedte".into(),
                    old_value: format!("{:.0}mm", old_w),
                    new_value: format!("{:.0}mm", w),
                });
            }
            if (h - old_h).abs() > 1.0 {
                changes.push(PropertyChange {
                    property: "Hoogte".into(),
                    old_value: format!("{:.0}mm", old_h),
                    new_value: format!("{:.0}mm", h),
                });
            }
            if changes.is_empty() {
                unchanged += 1;
            } else {
                modified.push(DiffModification {
                    guid: guid.to_string(),
                    name: name.to_string(),
                    changes,
                });
            }
        } else {
            added.push(DiffItem {
                guid: guid.to_string(),
                name: name.to_string(),
                entity_type: "Window/Door".into(),
                width_mm: *w,
                height_mm: *h,
            });
        }
    }

    // Find removed
    for (guid, (name, w, h)) in &old_guids {
        if !new_guids.contains_key(guid) {
            removed.push(DiffItem {
                guid: guid.to_string(),
                name: name.to_string(),
                entity_type: "Window/Door".into(),
                width_mm: *w,
                height_mm: *h,
            });
        }
    }

    IfcDiff {
        added,
        removed,
        modified,
        unchanged,
    }
}
