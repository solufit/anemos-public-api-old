use utoipa::OpenApi; 

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Utoipa API",
        version = "0.1.0",
        description = "API for"
    ),
    paths(),
    components()
)]
//TODO Add Info to auto generate version
pub struct ApiDoc;
