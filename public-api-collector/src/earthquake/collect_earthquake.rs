use anyhow::{Result, Error};
use log::debug;
use once_cell::sync::Lazy;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ReturnEarthQuake{
    earth_quake: Vec<public_api_lib::scheme::earthquake::EarthQuake>
}


pub async fn collect_earthquake() -> Result<(), Error> {

    static BASE_URL: Lazy<std::string::String> = Lazy::new(|| {
        format!("{}/get/earthquake?max=50", *public_api_lib::get_env::API_URL)
    });

    log::info!("Collecting Earthquake Data from: {}", &BASE_URL.to_string());

    let response = reqwest::get(BASE_URL.as_str())
        .await?
        .text()
        .await?;    

    debug!("{}", response);

    let deserialized = serde_json::from_str::<ReturnEarthQuake>(&response)?;


    Ok(())
}





