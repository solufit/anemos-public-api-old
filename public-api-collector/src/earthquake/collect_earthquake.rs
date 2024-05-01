use anyhow::{Error, Result};
use log::debug;
use once_cell::sync::Lazy;
use public_api_lib::scheme::earthquake::{self, EarthQuake};


pub async fn earthquake_operator() -> Result<(), Error> {

    let earthquakes_data: Vec<EarthQuake>;

    match collect_earthquake().await {
        Ok(earthquakes) => {
            earthquakes_data = earthquakes;
        },
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(())
        }
        
    }
    debug!("Earthquake Data: {:?}", earthquakes_data);
    

    Ok(())


}


pub async fn collect_earthquake() -> Result<Vec<public_api_lib::scheme::earthquake::EarthQuake>, Error> {

    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/get/earthquake?max=50", *public_api_lib::get_env::API_URL)
    });

    log::info!("Collecting Earthquake Data from: {}", &BASE_URL.to_string());

    let response = reqwest::get(BASE_URL.as_str())
        .await?
        .text()
        .await?;    

    debug!("Recieved Response: {}", response);

    let deserialized : Vec<public_api_lib::scheme::earthquake::EarthQuake> = serde_json::from_str(&response).unwrap();

    debug!("Converted: {:?}", deserialized);


    Ok(deserialized)
}





