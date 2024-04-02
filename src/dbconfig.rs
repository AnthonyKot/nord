pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        // let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
        // let port = env::var("DATABASE_PORT")
        //     .unwrap_or_else(|_| "5432".to_string())
        //     .parse()
        //     .expect("Invalid database port");
        // let user = env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
        // let password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "".to_string());
        // let database = env::var("DATABASE_NAME").unwrap_or_else(|_| "trading_platform".to_string());

        Self {
            host: "postgres".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "secret".to_string(),
            database: "nord".to_string(),
        }
    }
}

pub fn get_database_config() -> DatabaseConfig {
    DatabaseConfig::new()
}