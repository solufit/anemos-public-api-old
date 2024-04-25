use utoipa::OpenApi; 


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Anemos API",
        description = "Anemos API made by solufit. For detail, please visit the web site. https://anemos.solufit.net/api",
        version = "0.1.0"
    ),
    paths(
        super::versions::versions,
    ),
    components(
        schemas(
            super::versions::VersionResponse
        )
    )
)]
//TODO Add Info to auto generate version
pub struct ApiDoc;
