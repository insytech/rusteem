use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use tower::ServiceExt;

/// Build the application router for testing.
/// Requires DATABASE_URL and Supabase env vars to be set.
pub async fn test_app() -> Router {
    dotenvy::dotenv().ok();

    let config =
        api::config::AppConfig::from_env().expect("Test requires env vars (run supabase start)");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to test database");

    let state = api::state::AppState { pool, config };

    api::build_router(state)
}

/// Generate a test JWT token using HS256 with the configured secret.
pub fn test_jwt() -> String {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        "super-secret-jwt-token-with-at-least-32-characters-long".to_string()
    });

    let claims = serde_json::json!({
        "sub": "00000000-0000-0000-0000-000000000099",
        "role": "Engineering Lead",
        "email": "test@insytech.com",
        "exp": chrono::Utc::now().timestamp() + 3600,
        "iss": "test"
    });

    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to encode test JWT")
}

/// Send an authenticated JSON request and return status + deserialized body.
pub async fn send_json<T: DeserializeOwned>(
    app: &Router,
    method: Method,
    uri: &str,
    body: Option<serde_json::Value>,
) -> (StatusCode, Option<T>) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("Authorization", format!("Bearer {}", test_jwt()));

    let req = if let Some(ref json) = body {
        builder = builder.header("Content-Type", "application/json");
        builder
            .body(Body::from(serde_json::to_string(json).unwrap()))
            .unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    };

    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let parsed = if bytes.is_empty() {
        None
    } else {
        serde_json::from_slice(&bytes).ok()
    };

    (status, parsed)
}

/// Send an unauthenticated request and return just the status.
pub async fn send_no_auth(app: &Router, method: Method, uri: &str) -> StatusCode {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    app.clone().oneshot(req).await.unwrap().status()
}
