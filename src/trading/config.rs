use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct TradingConfig {
    pub market_hours: String,
    pub order_types: Vec<String>,
    pub max_order_size: u32,
    pub transaction_fee: TransactionFee,
}

#[derive(Deserialize, Debug, Default)]
pub struct TransactionFee {
    pub value: f32,
}
