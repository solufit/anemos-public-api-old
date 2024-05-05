use utoipa::OpenApi; 

use super::earthquake;
use super::versions;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Anemos API",
        description = "Anemos API made by solufit. For detail, please visit the web site. https://anemos.solufit.net/api",
        version = "0.1.0"
    ),
    paths(
        versions::versions,
        earthquake::earthquake_eventids_hourly,
        earthquake::earthquake_eventids_daily
    ),
    components(
        schemas(
            versions::VersionResponse,
            earthquake::EarthQuakeEventIDList
        )
    )
)]
//TODO Add Info to auto generate version
pub struct ApiDoc;
