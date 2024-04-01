system_operation_mode = "development"

[trading]
market_hours = "09:00-17:00"
order_types = ["limit", "market", "stop_loss"]
max_order_size = 10000
transaction_fee = { type = "percentage", value = 0.1 }

[lending]
interest_calculation = "compound"
loan_duration_limits = { min_days = 1, max_days = 365 }
max_ltv = 0.8
liquidation_threshold = 0.9

[margining]
margin_call_thresholds = { imf = 0.1, cmf = 0.2, mmf = 0.3 }
liquidation_fees = 0.05
insurance_fund_contribution = 0.02

[risk_management]
volatility_adjustment_factor = 1.2
risk_evaluation_period = 4 # in hours
socialized_loss_percentage = 0.1

[database]
type = "PostgreSQL"
connection_string = "host=localhost dbname=nord_system user=postgres password=secret"

[cache]
duration_seconds = 600
max_size = 10000

[api]
rate_limits = { requests_per_minute = 100 }