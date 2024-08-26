mod model;
mod service;
mod controller;

use std::env;

use axum::Router;
use bb8_redis::{bb8::Pool, RedisConnectionManager};

use strum_macros::EnumString;
use tokio::net::TcpListener;

use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use model::{service_config::ServiceConfig, service_state::ServiceState};
use controller::redis_controller::get_service_routes;


#[derive(Debug, EnumString, strum_macros::Display)]
enum RunMode {
    Dev,
    Prod,
    Test,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let run_mode_str = env::var("RUN_MODE").unwrap_or_else(|_| RunMode::Dev.to_string());

    let run_mode: RunMode = run_mode_str.parse()?;

    set_tracing(&run_mode)?;

    let service_config = ServiceConfig::new(&run_mode)?;
    
    let pool = create_redis_pool(&service_config).await?;

    let service_state = ServiceState {
        redis_pool: pool
    };
    
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    
    let app = Router::new()
        .merge(get_service_routes(service_state));

    axum::serve(listener, app).await?;

    Ok(())
}


fn set_tracing(run_mode: &RunMode) -> Result<(), Box<dyn std::error::Error>> {
    
    match run_mode {
        
        RunMode::Dev | RunMode::Test => {
            
            let env_filter = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(LevelFilter::INFO.to_string()))?;
            
            let simple_collector = tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer())
                .with(env_filter);
            
            tracing::subscriber::set_global_default(simple_collector)?;
            
            Ok(())
        }
        RunMode::Prod => {
            
            let env_filter = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(LevelFilter::WARN.to_string()))?;
            
            let logger = tracing_logstash::Layer::default().event_format(
                tracing_logstash::logstash::LogstashFormat::default()
                    .with_constants(vec![("service.name", "rust-axum-postgres".to_owned())]),
            );

            let collector = Registry::default()
                .with(logger)
                .with(env_filter);
            
            tracing::subscriber::set_global_default(collector)?;
            
            Ok(())
        }
    }
}

async fn create_redis_pool(service_config: &ServiceConfig) -> Result<Pool<RedisConnectionManager>, Box<dyn std::error::Error>> {
    
    info!("connecting to redis");
    
    let redis_manager = RedisConnectionManager::new(service_config.url.to_string())?;

    let redis_pool: Pool<RedisConnectionManager> = Pool::builder()
        .build(redis_manager)
        .await?;

    Ok(redis_pool)
}