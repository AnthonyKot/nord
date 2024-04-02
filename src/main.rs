mod config;
mod db;
mod lending;
mod margining;
mod riskmanagement;
mod trading;
mod logger;
mod dbconfig;

use tokio_postgres::NoTls;
use tokio_postgres::{Socket, tls::NoTlsStream};
use tokio_postgres::{Client, Connection};

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let (client, connection) = connect_postgres().await?;
    log::info!("Connecting to db.");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    log::info!("Application statring.");
    initialize_database(&client).await?;
    let cfg = config::load_config().expect("Failed to load config");
    println!("System operation mode: {}", cfg.system_operation_mode);
    println!("Trading market hours: {}", cfg.trading.market_hours);
    println!("Lending interest rate: {}", cfg.lending.interest_calculation);
    process_operations().await;

    log::info!("Application started.");
    log::warn!("This is a warning message.");

    Ok(())
}

async fn connect_postgres() -> Result<(Client, Connection<Socket, NoTlsStream>), tokio_postgres::Error> {
    let config = dbconfig::get_database_config();
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        config.host, config.port, config.user, config.password, config.database
    );
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;
    Ok((client, connection))
}

async fn initialize_database(client: &Client) -> Result<(), tokio_postgres::Error> {
    log::info!("Running query.");
    let queries = vec![
        "select * from tokens;"
    ];

    for query in queries {
        let rows = client.query(query, &[]).await?;
        log::info!("Query result.");
        for row in rows {
            let val: i32 = row.get(0);
            log::info!("Value: {}", val);
        }
    }

    Ok(())
}