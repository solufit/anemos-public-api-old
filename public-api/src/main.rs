use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use toml::Value;


pub mod api;

use crate::api::route::config;

/// Return anemos public api's Version
async fn index() -> impl Responder {
    let contents = fs::read_to_string("Cargo.toml").expect("Something went wrong reading the file");
    let parsed_value = contents.parse::<Value>().unwrap();
    let package = parsed_value.get("package").unwrap().get("version").unwrap().as_str().unwrap();

    HttpResponse::Ok().body(package.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();
    log::info!("Starting Anemos Public API Server at port 8080");
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