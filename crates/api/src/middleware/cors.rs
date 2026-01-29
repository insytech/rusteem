use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::config::AppConfig;

/// Build a CORS layer from the configured allowed origins.
pub fn build_cors_layer(config: &AppConfig) -> CorsLayer {
    let origins: Vec<HeaderValue> = config
        .allowed_origins
        .iter()
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false)
}
