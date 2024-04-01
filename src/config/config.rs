use serde::Deserialize;

use crate::trading::TradingConfig;
use crate::lending::LendingConfig;
use crate::margining::MarginingConfig;
use crate::riskmanagement::RiskManagementConfig;

#[derive(Deserialize)]
pub struct AppConfig {
    pub system_operation_mode: String,
    pub trading: TradingConfig,
    pub lending: LendingConfig,
    pub margining: MarginingConfig,
    pub risk_management: RiskManagementConfig,
    // Add other configurations here
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            system_operation_mode: "development".to_string(),
            trading: TradingConfig::default(),
            lending: LendingConfig::default(),
            margining: MarginingConfig::default(),
            risk_management: RiskManagementConfig::default(),
            // Initialize other defaults here
        }
    }
}