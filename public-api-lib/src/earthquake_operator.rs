use std::collections::{HashMap, HashSet};
use anyhow::{Ok, Result, Error};
use log::{debug, info};
use crate::scheme::earthquake::EarthQuake;
use crate::redis::redisOperation;
use redis::Cmd;

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
async fn push_event_list_hourly_to_redis(event_id: String) -> Result<String, Error> {
    let mut redis_op = redisOperation::new().await?;

    let result = Cmd::set_ex(format!("earthquakee-expire-hour-{}", event_id), event_id, 3600)
        .query_async(&mut redis_op.multiplexed_connection).await.unwrap_or("".to_string());
    Ok(result)
}

async fn earthquake_data_submitter(earthquake: &Vec<EarthQuake>) -> Result<(), Error> {
    // Submit the data to the API

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
            .into_iter().collect::<Result<Vec<String>>>()?
            //filter out empty string
            .into_iter().filter(|x| x != &" ".to_string())
            //collect to HashSet
            .collect();

    //remove duplicates
    let event_id_list: Vec<String> = event_id_list.into_iter().filter(
        
            |x| if results.contains(x) {
                //remove duplicate from hash_earthquake
                hash_earthquake.remove(x);
                false
            } else {true}
        
    ).collect();

    //push to redis
    let cmds: Vec<_> = event_id_list.iter().map(|e| {
        push_event_list_hourly_to_redis(e.to_string())
    }).collect();
    


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

