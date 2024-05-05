use actix_web::{
    get,
    HttpResponse,
    Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


/// Represents the response structure for the version endpoint.
#[derive(Deserialize,Serialize, Debug, ToSchema)]
pub struct VersionResponse {
    app: String,
    version: String,
}

/// Retrieves the version information of the API.
///
/// # Returns
///
/// The version information of the API.
#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Version information", body = VersionResponse),
    ),
)]
#[get("/v1/")]
pub async fn versions() -> impl Responder {
    let response = VersionResponse {
        app: "Anemos Public API".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    log::debug!("Version endpoint called: {:?}", response);
    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, http::StatusCode};

    use super::*;
    #[actix_rt::test]
    async fn test_versions() {
        let app = test::init_service(App::new().service(versions)).await;

        let req = test::TestRequest::get().uri("/v1/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let response: VersionResponse = serde_json::from_slice(&body[..]).unwrap();

        assert_eq!(response.app, "Anemos Public API");
        assert_eq!(response.version, env!("CARGO_PKG_VERSION"));
    }
}
