use anyhow::{Ok, Result, Error};
use log::{debug, info};
use crate::scheme::earthquake::EarthQuake;
use crate::redis::redisOperation;

/// Extracts the event ID from an EarthQuake struct.
///
/// # Arguments
///
/// * `earthquake` - The EarthQuake struct from which to extract the event ID.
///
/// # Returns
///
/// The event ID as a string.
fn eventid_extractor(earthquake: &EarthQuake) -> String {
    earthquake.Eventid.to_string()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eventid_extractor() {
        let earthquake = EarthQuake {
            Eventid: 12345.to_string(),
            id: "id".to_string(),
            EditorialOffice: "EditorialOffice".to_string(),
            PublishingOffice: "PublishingOffice".to_string(),
            Category: "Category".to_string(),
            Datetime: "Datetime".to_string(),
            Headline: "Headline".to_string(),
            Hypocenter: "Hypocenter".to_string(),
            RegionCode: "RegionCode".to_string(),
            MaxInt: "MaxInt".to_string(),
            Magnitude: 1.0,
            Intensity: None,

            
        };

        let event_id = eventid_extractor(&earthquake);
        assert_eq!(event_id, "12345");
    }
}

