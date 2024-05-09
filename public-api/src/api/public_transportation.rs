use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
};
use serde::{Deserialize, Serialize};
use log::{debug, info};
use once_cell::sync::Lazy;
use utoipa::openapi::{response, security::Http};

//TODO! fix below teribble code
#[allow(clippy::needless_return)]
#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Public Transportation Area")
    )
)]
#[get("/v1/public-transportation/area")]
pub async fn get_public_transportation_area() -> impl Responder {
    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/get/weatherwarning?max=", *public_api_lib::get_env::API_URL)
    });

    match reqwest::get(BASE_URL.to_string()).await {
        Ok(response) => {

            match response.text().await {
                Ok(text) => {
                    debug!("Recieved Response: {}", text);

                    return HttpResponse::Ok().body(text)
                }
                Err(e) => {
                    log::error!("ERR! {}", e);
                    return HttpResponse::InternalServerError().body("ERR!")
                }
            }
            
        }
        Err(e) => {
            log::error!("ERR! {}",  e);
            return HttpResponse::InternalServerError().body("ERR!")
        }
    }


}