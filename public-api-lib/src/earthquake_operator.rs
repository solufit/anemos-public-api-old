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

/// Return the earthquake data future treat from the Redis cache.
/// 
/// # Arguments
/// 
/// * `event_id` - The event ID of the earthquake.
/// 
/// # Returns
/// 
/// The earthquake future 
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
async fn get_from_redis(event_id: String) -> Result<String, Error> {
    let mut redis_op = redisOperation::new().await?;

    let result = Cmd::get(format!("earthquake-detail-{}", event_id))
        .query_async(&mut redis_op.multiplexed_connection).await.unwrap_or("".to_string());
    Ok(result)
}

/// Push the event list hourly to the Redis cache.
/// 
/// # Arguments
/// 
/// * `event_id` - The event ID of the earthquake.
/// 
/// # Returns
/// 
/// return async future.
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The event list is stored in the Redis cache for one hour.
/// 
async fn push_event_hourly_to_redis(event_id: String) -> Result<String, Error> {
    let mut redis_op = redisOperation::new().await?;

    let result = Cmd::set_ex(format!("earthquake-expire-hour-{}", event_id), event_id, 3600)
        .query_async(&mut redis_op.multiplexed_connection).await?;
    Ok(result)
}

/// Push the event list hourly to the Redis cache.
/// 
/// # Arguments
/// 
/// * `event_id` - The event ID List of the earthquake.
/// 
/// # Returns
/// 
/// return async future.
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The event list is stored in the Redis cache for one hour.
/// 
async fn push_event_list_hourly_to_redis(event_id: Vec<String>) -> Result<(), Error> {
    let mut redis_op = redisOperation::new().await?;

    Cmd::lpush("earthquake_eventid_hour", event_id)
        .query_async(&mut redis_op.multiplexed_connection).await?;
    Ok(())
}

/// Push the event list daily to the Redis cache.
/// 
/// # Arguments
/// 
/// * `event_id` - The event ID List of the earthquake.
/// 
/// # Returns
/// 
/// return async future.
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The event list is stored in the Redis cache for one hour.
/// 
async fn push_event_list_daily_to_redis(event_id: Vec<String>) -> Result<(), Error> {
    let mut redis_op = redisOperation::new().await?;

    Cmd::lpush("earthquake_eventid_day", event_id)
        .query_async(&mut redis_op.multiplexed_connection).await?;
    Ok(())
}
/// Push the event list daily to the Redis cache.
/// 
/// # Arguments
/// 
/// * `event_id` - The event ID of the earthquake.
/// 
/// # Returns
/// 
/// return async future.
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The event list is stored in the Redis cache for one day.
/// 
async fn push_event_daily_to_redis(event_id: String) -> Result<String, Error> {
    let mut redis_op = redisOperation::new().await?;

    let result = Cmd::set_ex(format!("earthquake-expire-day-{}", event_id), event_id, 3600 * 24)
        .query_async(&mut redis_op.multiplexed_connection).await?;
    Ok(result)
}

/// Push the event detail to the Redis cache.
/// 
/// # Arguments
/// 
/// * `event` - The EarthQuake struct of the earthquake.
/// 
/// * `event_id` - The event ID of the earthquake.
/// 
/// # Returns
/// 
/// return async future.
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The event detail is stored in the Redis cache for two days.
/// 
async fn push_event_detail_to_redis(event: &EarthQuake, event_id: String) -> Result<String, Error> {
    let mut redis_op = redisOperation::new().await?;

    //convert EarthQuake to json strings
    let event = serde_json::to_string(event).unwrap();

    let result = Cmd::set_ex(format!("earthquake-detail-{}", event_id), event, 3600 * 24 * 2)
        .query_async(&mut redis_op.multiplexed_connection).await?;
    Ok(result)
}

/// Submits the earthquake data to the API.
/// 
/// # Arguments
/// 
/// * `earthquake` - The EarthQuake struct of the earthquake.
/// 
/// # Returns
/// 
/// return async future.
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The earthquake data is submitted to the API.
/// 
/// The event list is stored in the Redis cache for one hour or one day.
/// 
pub async fn earthquake_data_submitter(earthquake: &[EarthQuake]) -> Result<(), Error> {
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

    info!("New EarthQuake Info Count: {}", event_id_list.len());

    //push to redis

    //create tasks
    let cmds: Vec<_> = event_id_list.iter().map(|e| {
        push_event_hourly_to_redis(e.to_string())
    }).collect();

    let cmd1: Vec<_> = event_id_list.iter().map(|e| {
        push_event_daily_to_redis(e.to_string())
    }).collect::<Vec<_>>();

    let cmd2: Vec<_> = event_id_list.iter().map(|e| {
        push_event_detail_to_redis(hash_earthquake.get(e).unwrap(), e.to_string())
    }).collect::<Vec<_>>();

    //join all tasks
    futures::future::try_join_all(cmds).await?;
    futures::future::try_join_all(cmd1).await?;
    futures::future::try_join_all(cmd2).await?;

    push_event_list_daily_to_redis(event_id_list.clone()).await?;
    push_event_list_hourly_to_redis(event_id_list.clone()).await?;

    Ok(())
}

///
/// Get the earthquake trend for the hour.
/// 
/// # Returns
/// 
/// return EventID List
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The earthquake trend is stored in the Redis cache.
/// 
/// The event list is stored in the Redis cache for one hour.
/// 
pub async fn get_earthquake_trend_hour() -> Result<Vec<String>, Error> {
    let mut redis_op = redisOperation::new().await?;

    let result: Vec<String> = Cmd::lrange("earthquake_eventid_hour", 0, -1)
        .query_async(&mut redis_op.multiplexed_connection).await?;

    Ok(result)

}

///
/// Get the earthquake trend for the hour.
/// 
/// # Returns
/// 
/// return EventID List
/// 
/// # Errors
/// 
/// Returns an error if the Redis operation fails.
/// 
/// # Remarks
/// 
/// The earthquake trend is stored in the Redis cache.
/// 
/// The event list is stored in the Redis cache for one hour.
/// 
pub async fn get_earthquake_trend_day() -> Result<Vec<String>, Error> {
    let mut redis_op = redisOperation::new().await?;

    let result: Vec<String> = Cmd::lrange("earthquake_eventid_day", 0, -1)
        .query_async(&mut redis_op.multiplexed_connection).await?;

    Ok(result)

}
#[cfg(test)]
mod tests {
    use super::*;
        #[tokio::test]
        async fn test_earthquake_trend_hour() -> Result<(), Error> {
            // push event list to redis
            push_event_list_hourly_to_redis(
                vec!["12345".to_string(), "12346".to_string()]
            ).await.unwrap();
            // Call the get_earthquake_trend_hour function
            let mut result = get_earthquake_trend_hour().await?;

            result.sort();

            // Assert that the function returns Ok
            assert_eq!(result, vec!["12345".to_string(), "12346".to_string()]);

            //delete exists
            let mut redis_op = redisOperation::new().await.unwrap();
            let _ : () = redis::cmd("flushall").query(&mut redis_op.con).unwrap();
            
            return Ok(());

        }

        #[tokio::test]
        async fn test_earthquake_trend_daily() -> Result<(), Error> {
            // push event list to redis
            push_event_list_daily_to_redis(
                vec!["12345".to_string(), "12346".to_string()]
            ).await.unwrap();
            // Call the get_earthquake_trend_daily function
            let mut result = get_earthquake_trend_day().await?;

            result.sort();

            // Assert that the function returns Ok
            assert_eq!(result, vec!["12345".to_string(), "12346".to_string()]);

            //delete exists
            let mut redis_op = redisOperation::new().await.unwrap();
            let _ : () = redis::cmd("flushall").query(&mut redis_op.con).unwrap();
            
            return Ok(());

        }

        #[tokio::test]
        async fn test_get_earthquake_trend_hour() {
            // Call the get_earthquake_trend_day function
            let result = get_earthquake_trend_hour().await;

            // Assert that the function returns Ok
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_get_earthquake_trend_day() {
            // Call the get_earthquake_trend_day function
            let result = get_earthquake_trend_day().await;

            // Assert that the function returns Ok
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_earthquake_data_submitter() {
            // Create a sample earthquake data
            let earthquake_sample_before = EarthQuake {
                Eventid: "before".to_string(),
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
            let earthquake_sample_after = EarthQuake {
                Eventid: "after".to_string(),
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

            let mut event_id_list: Vec<String> = vec!["before".to_string(), "after".to_string()];
            event_id_list.sort();

            let mut redis_op = redisOperation::new().await.unwrap();

            // prepare data
            push_event_daily_to_redis("before".to_string()).await.unwrap();
            push_event_hourly_to_redis("before".to_string()).await.unwrap();
            push_event_detail_to_redis(&earthquake_sample_before, "before".to_string()).await.unwrap();

            // Call the earthquake_data_submitter function
            let _ = earthquake_data_submitter(&[earthquake_sample_before.clone(), earthquake_sample_after.clone()]).await;

            // check execute success
            let result_before = get_from_redis("before".to_string()).await.unwrap();
            let result_after = get_from_redis("after".to_string()).await.unwrap();
            let mut result_list_hourly = get_earthquake_trend_hour().await.unwrap();
            result_list_hourly.sort();
            let mut result_list_daily = get_earthquake_trend_day().await.unwrap();
            result_list_daily.sort();

            // Assert that the function returns Ok
            assert_eq!(result_before, serde_json::to_string(&earthquake_sample_before).unwrap());
            assert_eq!(result_after, serde_json::to_string(&earthquake_sample_after).unwrap());
            assert_eq!(result_list_hourly, event_id_list);
            assert_eq!(result_list_daily, event_id_list);

           
            // clean redis
            let _ : () = redis::cmd("flushall").query(&mut redis_op.con).unwrap();


        }

    #[test]
    fn test_eventid_extractor() {
        let earthquake = EarthQuake {
            Eventid: "12345".to_string(),
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

