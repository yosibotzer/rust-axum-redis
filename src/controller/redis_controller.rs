
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::get;

use crate::model::api::RedisSetRequest;
use crate::model::service_state::ServiceState;
use crate::service::error::RepoError;
use crate::service::redis_service;

pub fn get_service_routes(service_state: ServiceState) -> Router {

    Router::new()
        .route("/redis/set/:key/:value", get(redis_set))
        .route("/redis/get/:key", get(redis_get))
        .with_state(Arc::new(service_state))
}

async fn redis_get(State(service_state): State<Arc<ServiceState>>, Path(key): Path<String>) 
-> Result<Response, RepoError> {
    
    let result = redis_service::redis_get(service_state, key.as_str()).await?;

    match result {
        Some(resredis_get_response) => Ok(Json(resredis_get_response).into_response()),
        None => Ok(StatusCode::NO_CONTENT.into_response())
    }
}

async fn redis_set(State(service_state): State<Arc<ServiceState>>, Path(redis_put): Path<RedisSetRequest>) 
-> Result<(), RepoError> {

    redis_service::redis_set(service_state, &redis_put).await
}

impl IntoResponse for RepoError {

    fn into_response(self) -> Response {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}