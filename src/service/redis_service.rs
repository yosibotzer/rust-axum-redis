
use std::sync::Arc;
use tracing::error;

use redis::AsyncCommands;

use bb8_redis::bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;

use crate::model::{api::{RedisGetResponse, RedisSetRequest}, service_state::ServiceState};

use super::error::RepoError::{self, RedisConnectionError, RedisGetError, RedisSetError};


pub async fn redis_get(redis_state: Arc<ServiceState>, key: &str) -> Result<Option<RedisGetResponse>, RepoError> {

    let mut conn = get_connection(&redis_state).await?;

    let result: Option<String> = conn
        .get(key)
        .await
        .map_err(|e| map_redis_get_error(e, key))?;

    match result {
        Some(value) => Ok(Some(RedisGetResponse { value })),
        None => Ok(None)
    }
}

pub async fn redis_set(redis_state: Arc<ServiceState>, redis_set: &RedisSetRequest) -> Result<(), RepoError> {

    let mut conn = get_connection(&redis_state).await?;

    conn
        .set(&redis_set.key, &redis_set.value)
        .await
        .map_err(|e| map_redis_set_error(e, &redis_set))?;

    Ok(())
}

async fn get_connection(redis_state: &Arc<ServiceState>) -> Result<PooledConnection<RedisConnectionManager>, RepoError> {
   
    let conn = redis_state.redis_pool
        .get()
        .await
        .map_err(|e| map_redis_connection_error(e))?;
    
    Ok(conn)
}

fn map_redis_connection_error(e: bb8_redis::bb8::RunError<redis::RedisError>) -> RepoError {
    error!("Redis connection error: {:?}", e);
    RedisConnectionError
}                               

fn map_redis_set_error(e: redis::RedisError, redis_set: &RedisSetRequest) -> RepoError {
    error!("Redis set error: {:?} for key: {:?} and value: {:?}", e, redis_set.key, redis_set.value);
    RedisSetError
}

fn map_redis_get_error(e: redis::RedisError, key: &str) -> RepoError {
    error!("Redis get error: {:?} for key: {:?}", e, key);
    RedisGetError
}