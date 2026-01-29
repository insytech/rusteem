mod config;
mod errors;
mod extractors;
mod handlers;
mod middleware;
mod state;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::handlers::health::health_check;
use crate::middleware::cors::build_cors_layer;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env().expect("Failed to load configuration from environment");

    let pool = PgPoolOptions::new()
        .max_connections(config.max_db_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let cors = build_cors_layer(&config);
    let state = AppState { pool, config };

    let app = Router::new()
        .route("/health", get(health_check))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
