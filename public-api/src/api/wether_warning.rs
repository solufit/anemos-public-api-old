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



#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Weather Warning", body = WeatherWarning),
    ),
)]
#[get("/v1/weather_warning/get/{num}")]
pub async fn get_weather_warning(num: web::Path<u32>) -> impl Responder {
    let max_num = num.into_inner();
    
    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/get/weatherwarning?max=", *public_api_lib::get_env::API_URL)
    });
    let base_url_system = BASE_URL.to_string() + &max_num.to_string();

    log::info!("Collecting Weather Warning Data from: {}", &base_url_system);

    let response = reqwest::get(base_url_system.as_str())
        .await.unwrap()
        .text()
        .await.unwrap();  

    let deserialized: WeatherWarning = serde_json::from_str(&response).unwrap();

    debug!("Converted: {:?}", deserialized);

    HttpResponse::Ok().json(deserialized)
}

