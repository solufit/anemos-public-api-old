use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct WeatherWarning {
    pub status: Vec<status>,
    pub telegram: telegram
}


#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct status {
    pub prefCode: i32,
    pub prefName: String,
    pub warningName: Vec<String>,
    detail: Vec<detail>
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct detail {
    pub regionCode: i32,
    pub regionName: String,
    pub warningNames: Vec<String>,
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct telegram {
    pub id: String,
    pub Entryid: String,
    pub EditorialOffice: String,
    pub PublishingOffice: String,
    pub Category: String,
    pub Datetime: String,
    pub Headline: String,
    pub Pref:   String,
    pub Areas: Vec<Area>,
    pub Regions: Vec<Regions>,
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Area {
    pub Name: String,
    pub Code: String,
    pub Kind: Kind
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Kind {
    Name: Option<String>,
    Code: Option<String>,    
    Status: Option<String>
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Regions {
    pub Region: Region,
    pub Kind: Vec<RegionsKind>
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Region {
    pub Name: String,
    pub Code: String,
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RegionsKind {
    pub Name: String,
    pub Code: String,
    pub Status: String,
 //   pub Property: Option<Property>
}

//TODO add struct more
//#[allow(non_snake_case)]
//#[allow(non_camel_case_types)]
//#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
//pub struct Property {
//    pub Type: String,
//    pub AdvisoryPeriod: Option<AdvisoryPeriod>,
//    pub WindDirectionPart: Option<WindDirectionPart>,
//    pub WindSpeedPart: Option<WindSpeedPart>,
//    pub AdvisoryPeriod: Option<AdvisoryPeriod>,
//    pub WaveHeightPart: Option<WaveHeightPart>,

//}