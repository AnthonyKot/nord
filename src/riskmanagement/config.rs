use serde::Deserialize;

#[derive(Deserialize)]
pub struct RiskManagementConfig {
    pub volatility_adjustment_factor: f32,
    pub risk_evaluation_period: u32,
    pub socialized_loss_percentage: f32,
}

// Default implementations
impl Default for RiskManagementConfig {
    fn default() -> Self {
        Self {
            volatility_adjustment_factor: 1.2,
            risk_evaluation_period: 4, // hours
            socialized_loss_percentage: 0.1,
        }
    }
}