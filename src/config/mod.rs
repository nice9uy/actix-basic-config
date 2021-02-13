use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing_subscriber::EnvFilter;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port : i32,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        info!("Loading Configuration for server");

        let mut cnf = config::Config::new();
        cnf.merge(config::Environment::default())?;
        cnf.try_into()
            .context("Loading Configuration from Enverontment")
    }
}