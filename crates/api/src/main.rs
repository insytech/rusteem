use ax_um::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Inicializar logs
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "api=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Configurar conexión a BD
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Construir nuestra aplicación con una ruta y el estado (pool)
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool);

    // Ejecutarla
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK - RustEEM API is running"
}
