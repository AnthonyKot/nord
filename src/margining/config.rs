use serde::Deserialize;

#[derive(Deserialize)]
pub struct MarginingConfig {
    pub margin_call_thresholds: MarginCallThresholds,
    pub liquidation_fees: f32,
    pub insurance_fund_contribution: f32,
}

#[derive(Deserialize)]
pub struct MarginCallThresholds {
    pub imf: f32,
    pub cmf: f32,
    pub mmf: f32,
}

// Default implementations
impl Default for MarginingConfig {
    fn default() -> Self {
        Self {
            margin_call_thresholds: MarginCallThresholds { imf: 0.1, cmf: 0.2, mmf: 0.3 },
            liquidation_fees: 0.05,
            insurance_fund_contribution: 0.02,
        }
    }
}