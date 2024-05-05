use actix_web::{
    get,
    HttpResponse,
    Responder,
};
use serde::{Deserialize, Serialize};


use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct EarthQuakeEventIDList {
    pub event_ids: Vec<String>,
}

/// Retrieves the earthquake eventids in hours.
/// 
/// # Returns
/// 
/// The earthquake eventids in hours.
/// 
/// # Errors
/// 
/// If an error occurs while retrieving the earthquake eventids, an internal server error will be returned.
/// 
/// # Example
/// 
/// ```json
/// 
/// {
///    "event_ids": [
///       "us6000d3z5",
///       "us6000d3z4",
///       "us6000d3z3",
///       "us6000d3z2",
///       "us6000d3z1",
///       "us6000d3z0"
/// ]
/// }
/// 
/// ```
#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Earthquake eventids in hours", body = EarthQuakeEventIDList),
    ),
    
)]
#[get("/v1/earthquake/eventids/hourly")]
pub async fn earthquake_eventids_hourly() -> impl Responder {
    let event_ids = match public_api_lib::earthquake_operator::get_earthquake_trend_hour().await {
        Ok(event_ids) => event_ids,
        Err(e) => {
            log::error!("Failed to get earthquake trend: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let response = EarthQuakeEventIDList {
        event_ids,
    };
    log::debug!("Earthquake eventids hourly endpoint called: {:?}", response);
    HttpResponse::Ok().json(response)
}