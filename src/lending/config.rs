use serde::Deserialize;

#[derive(Deserialize)]
pub struct LendingConfig {
    pub interest_calculation: String,
    pub loan_duration_limits: LoanDurationLimits,
    pub max_ltv: f32,
    pub liquidation_threshold: f32,
}

#[derive(Deserialize)]
pub struct LoanDurationLimits {
    pub min_days: u32,
    pub max_days: u32,
}

// Default implementations
impl Default for LendingConfig {
    fn default() -> Self {
        Self {
            interest_calculation: "compound".to_string(),
            loan_duration_limits: LoanDurationLimits { min_days: 1, max_days: 365 },
            max_ltv: 0.8,
            liquidation_threshold: 0.9,
        }
    }
}