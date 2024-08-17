use bb8_redis::{bb8::Pool, RedisConnectionManager};

#[derive(Debug)]
pub struct ServiceState {
    pub redis_pool: Pool<RedisConnectionManager>
}