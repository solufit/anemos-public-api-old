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





