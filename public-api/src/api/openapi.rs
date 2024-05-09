use utoipa::OpenApi; 

use super::{
    earthquake,
    versions,
    wether_warning,
    public_transportation
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Anemos API",
        description = r#"
        Anemos API made by solufit. For detail, please visit the web site. https://lp.anemos.solufit.net/

        # Warning!!! This API is under development. The specification may change without notice.
        "#,
        version = "0.1.0"
    ),
    paths(
        versions::versions,
        earthquake::earthquake_eventids_hourly,
        earthquake::earthquake_eventids_daily,
        earthquake::get_earthquake_detail,
        wether_warning::get_weather_warning,
        public_transportation::get_public_transportation_area,
        public_transportation::get_public_transportation_area_to_line,
        public_transportation::get_public_transportation_line
    ),
    components(
        schemas(
            versions::VersionResponse,
            earthquake::EarthQuakeEventIDList,
            earthquake::NotFound,
            public_api_lib::scheme::earthquake::EarthQuake,
            public_api_lib::scheme::earthquake::Intensity,
            public_api_lib::scheme::earthquake::Area,
            public_api_lib::scheme::earthquake::City,
            public_api_lib::scheme::earthquake::IntensityStationEnum,
            public_api_lib::scheme::earthquake::IntensityStation,
            public_api_lib::scheme::weather_warning::WeatherWarning,
            

        )
    )
)]
//TODO Add Info to auto generate version
pub struct ApiDoc;
