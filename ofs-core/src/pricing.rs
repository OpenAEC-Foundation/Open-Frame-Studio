use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PricingConfig {
    pub discount_percentage: f64,
    pub btw_percentage: f64,        // default 21.0
    pub btw_verlegd: bool,          // reverse charge
    pub transport_cost: f64,
    pub montage_cost_per_hour: f64,
    pub montage_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotationPrice {
    pub subtotal: f64,
    pub discount_amount: f64,
    pub transport: f64,
    pub montage: f64,
    pub subtotal_after_extras: f64,
    pub btw_amount: f64,
    pub total_incl_btw: f64,
}

impl PricingConfig {
    pub fn calculate(&self, material_subtotal: f64) -> QuotationPrice {
        let discount_amount = material_subtotal * self.discount_percentage / 100.0;
        let after_discount = material_subtotal - discount_amount;
        let montage = self.montage_cost_per_hour * self.montage_hours;
        let subtotal_after_extras = after_discount + self.transport_cost + montage;
        let btw_amount = if self.btw_verlegd { 0.0 } else { subtotal_after_extras * self.btw_percentage / 100.0 };
        let total = subtotal_after_extras + btw_amount;

        QuotationPrice {
            subtotal: material_subtotal,
            discount_amount,
            transport: self.transport_cost,
            montage,
            subtotal_after_extras,
            btw_amount,
            total_incl_btw: total,
        }
    }
}
