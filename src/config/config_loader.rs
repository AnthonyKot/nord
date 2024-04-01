use config::{Config, ConfigError, File, Environment};
use super::AppConfig;

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let mut cfg = Config::default();
    cfg.merge(File::with_name("Default.toml"))?.
        merge(File::with_name("Settings.toml").required(false))?.
        merge(Environment::with_prefix("APP").separator("__"))?;

    cfg.try_into()
}