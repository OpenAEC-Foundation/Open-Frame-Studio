use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockLength {
    pub profile_id: String,
    pub material: String,
    pub available_length_mm: f64,
    pub cost_per_meter: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CutPlan {
    pub allocations: Vec<StockAllocation>,
    pub total_waste_mm: f64,
    pub waste_percentage: f64,
    pub total_bars_used: usize,
    pub total_stock_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockAllocation {
    pub stock_length_mm: f64,
    pub cuts: Vec<AllocatedCut>,
    pub remaining_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocatedCut {
    pub piece_id: String,
    pub kozijn_mark: String,
    pub length_mm: f64,
}

/// First Fit Decreasing algorithm for 1D cutting stock problem
pub fn optimize_cut_list(
    pieces: Vec<(String, String, f64)>,  // (piece_id, kozijn_mark, gross_length_mm)
    stock_length_mm: f64,                // standard bar length (e.g., 5800mm for wood)
    saw_kerf_mm: f64,                    // blade width (typically 4mm)
) -> CutPlan {
    // Sort pieces descending by length
    let mut sorted: Vec<_> = pieces.into_iter().collect();
    sorted.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut allocations: Vec<StockAllocation> = Vec::new();

    for (piece_id, mark, length) in sorted {
        let cut_length = length + saw_kerf_mm;

        // Try to fit in existing bar
        let mut placed = false;
        for alloc in &mut allocations {
            if alloc.remaining_mm >= cut_length {
                alloc.remaining_mm -= cut_length;
                alloc.cuts.push(AllocatedCut {
                    piece_id: piece_id.clone(),
                    kozijn_mark: mark.clone(),
                    length_mm: length,
                });
                placed = true;
                break;
            }
        }

        // Open new bar
        if !placed {
            allocations.push(StockAllocation {
                stock_length_mm,
                cuts: vec![AllocatedCut {
                    piece_id,
                    kozijn_mark: mark,
                    length_mm: length,
                }],
                remaining_mm: stock_length_mm - cut_length,
            });
        }
    }

    let total_waste: f64 = allocations.iter().map(|a| a.remaining_mm).sum();
    let total_stock: f64 = allocations.len() as f64 * stock_length_mm;
    let waste_pct = if total_stock > 0.0 { total_waste / total_stock * 100.0 } else { 0.0 };

    CutPlan {
        total_bars_used: allocations.len(),
        total_waste_mm: total_waste,
        waste_percentage: (waste_pct * 10.0).round() / 10.0,
        total_stock_cost: 0.0, // TODO: calculate from stock prices
        allocations,
    }
}
