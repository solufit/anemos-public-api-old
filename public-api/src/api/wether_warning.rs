use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
};
use serde::{Deserialize, Serialize};
use public_api_lib::scheme::weather_warning::WeatherWarning;



#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Weather Warning", body = WeatherWarning),
    ),
)]
#[get("/v1/weather_warning/get/{num}")]
pub async fn get_weather_warning(num: web::Path<u32>) -> impl Responder {
    
    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/get/weatherwarning?max={}", *public_api_lib::get_env::API_URL, num)
    });

    log::info!("Collecting Weather Warning Data from: {}", &BASE_URL.to_string());

    let response = reqwest::get(BASE_URL.as_str())
        .await.unwrap()
        .text()
        .await.unwrap();  

    let deserialized: WeatherWarning = serde_json::from_str(&response).unwrap();

    debug!("Converted: {:?}", deserialized);

    HttpResponse::Ok().json(deserialized)
}

