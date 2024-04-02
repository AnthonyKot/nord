# Nord
## Nord Trading and Lending System Template

### Run it
  ```sh
    docker-compose up --build
  ```

### Overview
This template provides a structured starting point for developing the Nord trading and lending system, designed to facilitate a comprehensive and risk-managed environment for users to engage in various trading and lending activities.

### Project Structure

- `src/`: Contains the main source code files.
  - `main.rs`: The entry point of the application.
  - `trading/`: Module for handling trading-related functionality.
    - `mod.rs`: Declares the trading module and its public items.
    - `engine.rs`: Implements the trading engine.
    - `order.rs`: Defines the Order struct and related functions.
    - `matching.rs`: Implements the order matching logic.
  - `lending/`: Module for handling lending-related functionality.
    - `mod.rs`: Declares the lending module and its public items.
    - `engine.rs`: Implements the lending engine.
    - `loan.rs`: Defines the Loan struct and related functions.
    - `interest.rs`: Implements interest rate calculation logic.
  - `margining/`: Module for handling margining and risk management.
    - `mod.rs`: Declares the margining module and its public items.
    - `system.rs`: Implements the margining system.
    - `account.rs`: Defines the Account struct and related functions.
    - `risk.rs`: Implements risk management logic.
  - `utils/`: Module for utility functions and shared functionality.
    - `mod.rs`: Declares the utils module and its public items.
    - `currency.rs`: Defines currency-related functions and constants.
    - `pricing.rs`: Implements pricing-related functions.
  - `db/`: Module for database and storage-related functionality.
    - `mod.rs`: Declares the db module and its public items.
    - `storage.rs`: Implements storage and database operations.

- `tests/`: Contains integration and unit tests for the project.
  - `trading_tests.rs`: Contains tests related to the trading module.
  - `lending_tests.rs`: Contains tests related to the lending module.
  - `margining_tests.rs`: Contains tests related to the margining module.

### Dependencies

The project relies on the following dependencies:

- `chrono`: A library for date and time handling.
- `rust_decimal`: A library for decimal arithmetic.
- `serde` and `serde_json`: Libraries for serialization and deserialization.
- `tokio`: An asynchronous runtime for Rust.
- `uuid`: A library for generating and handling UUIDs.

### Getting Started

To get started with the Nord system, ensure you have Rust and Cargo installed on your system. Clone the repository, navigate to the project directory, and run `cargo build` to compile the project. Use `cargo run` to execute the application.

### Contribution

Contributions to the Nord system are welcome. Please refer to the project's contribution guidelines for more details on how to contribute.

### General Configuration

- **System Operation Mode**: Define if the system is in production, staging, or development mode. This can affect logging levels, error reporting, and feature toggles.
  - `mode: "development"` | `"staging"` | `"production"`

## Trading Engine Configuration

- **Market Hours**: Define the hours during which trading is allowed.
  - `market_hours: { start: "09:00", end: "17:00" }`

- **Order Types Supported**: Specify which types of orders your system can handle (e.g., limit orders, market orders).
  - `order_types: ["limit", "market", "stop_loss"]`

- **Maximum Order Size**: Set limits on the maximum size of orders to prevent excessively large transactions.
  - `max_order_size: 10000`

- **Transaction Fee**: Configure the fee structure for trades.
  - `transaction_fee: { type: "fixed" | "percentage", value: 0.1 }`

## Lending Engine Configuration

- **Interest Calculation Method**: Define how interest on loans is calculated (e.g., simple interest, compound interest).
  - `interest_calculation: "compound"`

- **Loan Duration Limits**: Set minimum and maximum loan durations.
  - `loan_duration_limits: { min_days: 1, max_days: 365 }`

- **Maximum LTV (Loan to Value)**: Define the maximum loan to value ratio to control borrowing risk.
  - `max_ltv: 0.8`

- **Liquidation Threshold**: Set the LTV threshold at which loans are considered for liquidation.
  - `liquidation_threshold: 0.9`

## Margining System Configuration

- **Margin Call Thresholds**: Define thresholds for margin calls to manage leverage and risk.
  - `margin_call_thresholds: { imf: 0.1, cmf: 0.2, mmf: 0.3 }`

- **Liquidation Fees**: Configure fees applied during the liquidation process.
  - `liquidation_fees: { percentage: 0.05 }`

- **Insurance Fund Contribution**: Set the percentage of transaction fees or liquidation fees that go into the insurance fund.
  - `insurance_fund_contribution: 0.02`

## Risk Management Configuration

- **Volatility Adjustment Factor**: Adjust margin requirements based on market volatility.
  - `volatility_adjustment_factor: 1.2`

- **Risk Evaluation Period**: Define how frequently the system evaluates risk exposure.
  - `risk_evaluation_period: { every_n_hours: 4 }`

- **Socialized Loss Percentage**: Determine the percentage of losses socialized among users in extreme scenarios.
  - `socialized_loss_percentage: 0.1`

## Security and Compliance

- **Data Encryption Standards**: Specify encryption standards for sensitive data.
  - `encryption_standards: "AES-256"`

- **Compliance Reporting Frequency**: Set how often compliance reports are generated and reviewed.
  - `compliance_reporting_frequency: { every_n_days: 30 }`

- **Audit Log Retention**: Define the retention period for audit logs.
  - `audit_log_retention: { days: 180 }`

## Technical Configuration

- **Database Connection Details**: Include connection strings or configurations for databases.
  - `database: { type: "PostgreSQL", connection_string: "host=localhost dbname=nord_system user=postgres password=secret" }`

- **Cache Settings**: Define cache duration and size for frequently accessed data.
  - `cache: { duration_seconds: 600, max_size: 10000 }`

- **API Rate Limits**: Set rate limits for API usage to prevent abuse.
  - `api_rate_limits: { requests_per_minute: 100 }`
