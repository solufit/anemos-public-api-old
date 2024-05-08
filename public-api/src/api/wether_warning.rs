use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
};
use serde::{Deserialize, Serialize};
use public_api_lib::scheme::weather_warning::WeatherWarning;
use log::{debug, info};
use once_cell::sync::Lazy;
use utoipa::openapi::response;



#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Weather Warning", body = WeatherWarning),
    ),
)]
#[get("/v1/weather_warning/get/{num}")]
#[allow(clippy::needless_return)]
pub async fn get_weather_warning(num: web::Path<u32>) -> impl Responder {
    let max_num = num.into_inner();
    
    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/get/weatherwarning?max=", *public_api_lib::get_env::API_URL)
    });
    let base_url_system = BASE_URL.to_string() + &max_num.to_string();

    log::info!("Collecting Weather Warning Data from: {}", &base_url_system);

    match reqwest::get(base_url_system.as_str()).await {
        Ok(response) => {

            match response.text().await  {
                Ok(text) => {
                    debug!("Recieved Response: {}", text);
                    //let deserialized: WeatherWarning = serde_json::from_str(&text).unwrap();
                    //debug!("Converted: {:?}", deserialized);
                    //return HttpResponse::Ok().json(deserialized);

                    //return raw data for wip
                    return HttpResponse::Ok().json(text);
                }
                Err(e) => {
                    log::error!("Error: {}", e);
                    return HttpResponse::InternalServerError().body("ERR");
            }
        }
            
        }
        Err(e) => {
            log::error!("Error: {}", e);
            return HttpResponse::InternalServerError().body("ERR");
        }
    }


}

