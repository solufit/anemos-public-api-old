use serde::{Deserialize, Serialize};
use chrono::DateTime;

/// Represents an intensity station.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct IntensityStation {
    Name: String,
    Code: String,
    Int: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
#[serde(untagged)]
pub enum IntensityStationEnum {
    IntensityStation(IntensityStation),
    IntensityStationVec(Vec<IntensityStation>)
}

/// Represents a city.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct City {
    Name: String,
    Code: String,
    MaxInt: String,
    IntensityStation: Option<IntensityStationEnum>
}

/// Represents a Area
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Area {
    Name: String,
    Code: String,
    MaxInt: String,
    City: Option<Vec<City>>
}

/// Represents an intensity.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Intensity {
    Name: String,
    Code: String,
    MaxInt: String,
    Area: Option<Vec<Area>>
}

/// Represents an earthquake.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct EarthQuake {
    id: String,
    Eventid: String,
    EditorialOffice: String,
    PublishingOffice: String,
    Category: String,
    //Datetime: DateTime<chrono::Utc>,
    Datetime: String,
    Headline: String,
    Hypocenter: String,
    RegionCode: String,
    MaxInt: String,
    Magnitude: f64,
    Intensity: Option<Vec<Intensity>>
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_JSON :&str = r#"
    {
        "id": "662b1c5765b4c9846ee7b6db",
        "Eventid": "20240426103300",
        "EditorialOffice": "大阪管区気象台",
        "PublishingOffice": "気象庁",
        "Category": "地震情報",
        "Datetime": "2024-04-26T01:36:00",
        "Headline": "２６日１０時３３分ころ、地震がありました。",
        "Hypocenter": "熊本県阿蘇地方",
        "RegionCode": "740",
        "MaxInt": "3",
        "Magnitude": 3.3,
        "Intensity": [
          {
            "Name": "熊本県",
            "Code": "43",
            "MaxInt": "3",
            "Area": [
              {
                "Name": "熊本県阿蘇",
                "Code": "740",
                "MaxInt": "3",
                "City": [
                  {
                    "Name": "産山村",
                    "Code": "4342500",
                    "MaxInt": "3",
                    "IntensityStation": {
                      "Name": "産山村山鹿＊",
                      "Code": "4342531",
                      "Int": "3"
                    }
                  }
                ]
              }
            ]
          }
        ]
      }
    "#;


    #[test]
    fn test_convert_json() -> anyhow::Result<(), anyhow::Error> {
        println!("TEST_JSON: {}", TEST_JSON);
        let deserialized = serde_json::from_str::<EarthQuake>(TEST_JSON).unwrap();

        println!("{:?}", deserialized);
        
        Ok(())
    }

}

