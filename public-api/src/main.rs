use std::env;

use actix_web::{App, HttpServer};

pub mod api;

use crate::api::route::config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();
    log::info!("Starting Anemos Public API Server at port 8080");
    log::info!("Starting Anemos Public API Server Version: {} at port 8080", env!("CARGO_PKG_VERSION"));
    log::debug!("Debugging mode enabled");
    

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}