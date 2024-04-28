use anyhow::{Ok, Result, Error};

use crate::get_env::PUBLIC_API_REDIS_HOST;

/// Represents a Redis operation.
pub struct redisOperation {
    client: redis::Client,
    con: redis::Connection,
    multiplexed_connection: redis::aio::MultiplexedConnection
}

impl redisOperation {
    /// Create a new Redis operation.
    pub async fn new() -> Result<Self, Error>  {
        let client = redis::Client::open(format!("redis://{}", PUBLIC_API_REDIS_HOST.to_string()))?;
        let con = client.get_connection()?;
        let multiplexed = client.get_multiplexed_tokio_connection().await?;

        Ok(
            Self{
                client,
                con,
                multiplexed_connection: multiplexed
            }
        )
    }
} 