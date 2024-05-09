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
        (status = 200, description = "Public Transportation Area", body =  String)
    )
)]
#[get("/v1/public-transportation/get-area")]
pub async fn get_public_transportation_area() -> impl Responder {
    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/config/pti/area", *public_api_lib::get_env::API_URL)
    });

    match reqwest::get(BASE_URL.to_string()).await {
        Ok(response) => {

            match response.text().await {
                Ok(text) => {
                    debug!("Recieved Response: {}", text);

                    return HttpResponse::Ok().json(text)
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

#[allow(clippy::needless_return)]
#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Public Transportation line")
    )
)]
#[get("/v1/public-transportation/get-line/{area}")]
pub async fn get_public_transportation_area_to_line(area: web::Path<String>) -> impl Responder {
    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/config/pti/line", *public_api_lib::get_env::API_URL)
    });
    let base_url = format!("{}?area={}", *BASE_URL, area);

    match reqwest::get(base_url).await {
        Ok(response) => {

            match response.text().await {
                Ok(text) => {
                    debug!("Recieved Response: {}", text);

                    return HttpResponse::Ok().json(text)
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

#[allow(clippy::needless_return)]
#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Public Transportation line info")
    )
)]
#[get("/v1/public-transportation/get-line-info/{line}")]
pub async fn get_public_transportation_line(line: web::Path<String>) -> impl Responder {
    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/get/pti?ptid=", *public_api_lib::get_env::API_URL)
    });
    log::debug!("line: {}", line);
    let base_url = format!("{}{}", *BASE_URL, line);

    log::debug!("base_url: {}", base_url);

    match reqwest::get(base_url).await {
        Ok(response) => {

            match response.text().await {
                Ok(text) => {
                    debug!("Recieved Response: {}", text);

                    return HttpResponse::Ok().json(text)
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