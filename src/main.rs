mod config;
mod handler;

use color_eyre::Result;
use crate::config::Config;
use actix_web::{App, HttpServer , middleware::Logger};
use tracing::info;
use crate::handler::app_config;

#[actix_web::main]
async fn main() ->Result<()> {
    let config = Config::from_env()
        .expect("Server Configuration");

    info!("Starting Server at http://{}:{}/", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(app_config)
    })
        .bind(format!("{}:{}", config.host , config.port))?
        .run()
        .await?;
    Ok(())
}
