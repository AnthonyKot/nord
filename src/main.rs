mod config;
mod db;
mod lending;
mod margining;
mod riskmanagement;
mod trading;

async fn process_operations() {
    // let trading_task = tokio::spawn(async {
    //     trading_engine.process_orders().await;
    // });
    // let lending_task = tokio::spawn(async {
    //     lending_engine.process_loans().await;
    // });
    // let margining_task = tokio::spawn(async {
    //     margining_system.update_and_manage_risk().await;
    // });

    // tokio::try_join!(trading_task, lending_task, margining_task).unwrap();
}

#[tokio::main]
async fn main() {
    // logger::init();
    let cfg = config::load_config().expect("Failed to load config");
    println!("System operation mode: {}", cfg.system_operation_mode);
    println!("Trading market hours: {}", cfg.trading.market_hours);
    println!("Lending interest rate: {}", cfg.lending.interest_calculation);
    process_operations().await;
}