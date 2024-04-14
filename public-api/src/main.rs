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
    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}