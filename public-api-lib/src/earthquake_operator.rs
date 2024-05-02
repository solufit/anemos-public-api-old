use std::fmt::format;
use std::collections::{HashMap, HashSet};
use anyhow::{Ok, Result, Error};
use futures::{task::waker, Future};
use log::{debug, info};
use crate::scheme::earthquake::EarthQuake;
use crate::redis::redisOperation;
use redis::{Pipeline, Cmd};

/// Extracts the event ID from an EarthQuake struct.
///
/// # Arguments
///
/// * `earthquake` - The EarthQuake struct from which to extract the event ID.
///
/// # Returns
///
/// The event ID as a string.
#[allow(unused)]
fn eventid_extractor(earthquake: &EarthQuake) -> String {
    earthquake.Eventid.to_string()
}


async fn get_from_redis(event_id: String) -> Result<String, Error> {
    let mut redis_op = redisOperation::new().await?;

    let result = Cmd::get(format!("earthquake-detail-{}", event_id))
        .query_async(&mut redis_op.multiplexed_connection).await.unwrap_or("".to_string());
    Ok(result)
}

async fn earthquake_data_submitter(earthquake: &Vec<EarthQuake>) -> Result<(), Error> {
    // Submit the data to the API

    let mut redis_op = redisOperation::new().await?;
    let mut event_id_list: Vec<String> = Vec::new();
    let mut hash_earthquake: HashMap<String, &EarthQuake> = HashMap::new();

    debug!("earthquake_data_id: {:?}", eventid_extractor(&earthquake[0]));

    // add to hashmap to check dubplicate
    // check if the event_id is already in the database

    // get cached data
    let cmds: Vec<_> = earthquake.iter().map(|e| {
        let event_id = eventid_extractor(e);
        event_id_list.push(event_id.clone());
        hash_earthquake.insert(event_id.clone(), e);
        get_from_redis(event_id)

    }).collect();

    let results : HashSet<String> = futures::future::join_all(cmds).await
            .into_iter().collect::<Result<Vec<String>>>()?.into_iter().collect();

    //check duplicates


    Ok(())
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

