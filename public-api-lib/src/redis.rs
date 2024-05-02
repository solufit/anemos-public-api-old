use anyhow::{Ok, Result, Error};

use crate::get_env::PUBLIC_API_REDIS_HOST;

/// Represents a Redis operation.
pub struct redisOperation {
    pub client: redis::Client,
    pub con: redis::Connection,
    pub multiplexed_connection: redis::aio::MultiplexedConnection
}

impl redisOperation {
    /// Create a new Redis operation.
    pub async fn new() -> Result<Self, Error>  {
        let client = redis::Client::open(format!("redis://{}", *PUBLIC_API_REDIS_HOST))?;
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

#[cfg(test)]
mod tests {
    use redis::{Commands, ConnectionLike};

    use super::*;

    #[tokio::test]
    async fn test_new_redis_operation() -> Result<(), Error> {
        let redis_op = redisOperation::new().await?;
        assert!(redis_op.client.is_open());

        
        Ok(())
    }

    #[tokio::test]
    async fn test_redis_read_write() -> Result<(), Error> {
        // Create a new Redis operation
        let mut redis_op = redisOperation::new().await?;

        // Write a value to Redis
        let key = "single_my_key";
        let value = "single_my_value";
        redis_op.con.set(key, value)?;


        // Read the value from Redis
        let result: Option<String> = redis_op.con.get(key)?;

        // Assert that the value is correct
        assert_eq!(result, Some(value.to_string()));

        redis_op.con.del(key)?;

        Ok(())
        }

    #[tokio::test]
    async fn test_redis_async_single_read_write() -> Result<(), Error> {
        // Create a new Redis operation
        let mut redis_op = redisOperation::new().await?;


        // Write a value to Redis
        let key = "single_2-my_key";
        let value = "single_2-my_value";

        redis_op.multiplexed_connection.send_packed_command(
            &redis::Cmd::del(key)
        ).await?;

        redis_op.multiplexed_connection.send_packed_command(
            &redis::Cmd::set(key, value)
        ).await?;
    

        // Read the value from Redis
        let result_value = redis_op.multiplexed_connection.send_packed_command(
            &redis::Cmd::get(key)
        ).await?;

        let result: String = redis::from_redis_value(&result_value)?;

        // Assert that the value is correct
        assert_eq!(result, value.to_string());

        redis_op.multiplexed_connection.send_packed_command(
            &redis::Cmd::del(key)
        ).await?;

        Ok(())
        }

    #[tokio::test]
    async fn test_redis_async_multiple_read_write() -> Result<(), Error> {
        // Create a new Redis operation
        let mut redis_op = redisOperation::new().await?;

        // Write a value to Redis
        let key = "multi-my_key";
        let value = "multi-my_value";
        let key1 = "multi-my_key1";
        let value1 = "multi-my_value1";

        redis_op.multiplexed_connection.send_packed_commands(
            redis::Pipeline::new().del(key).del(key1),
            0, 2
        ).await?;

        redis_op.multiplexed_connection.send_packed_commands(
            redis::Pipeline::new().set(key, value).set(key1, value1),
            0, 2
        ).await?;
    

        // Read the value from Redis
        let result_value = redis_op.multiplexed_connection.send_packed_commands(
            redis::Pipeline::new().get(key).get(key1),
            0, 2
        ).await?;

        let result: String = redis::from_redis_value(&result_value[0])?;
        let result1: String = redis::from_redis_value(&result_value[1])?;

        // Assert that the value is correct
        assert_eq!(result, value.to_string());
        assert_eq!(result1, value1.to_string());

        redis_op.multiplexed_connection.send_packed_commands(
            redis::Pipeline::new().del(key).del(key1),
            0, 2
        ).await?;

        Ok(())
        }
    }
