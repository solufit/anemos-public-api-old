use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::openapi::ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/docs/{_:.*}")
            .url("/v1/docs/openapi.json", ApiDoc::openapi())
    );
}
