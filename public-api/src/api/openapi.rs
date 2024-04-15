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
pub struct ApiDoc;
