use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RedisSetRequest {
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct RedisGetResponse {
    pub value: String,
}