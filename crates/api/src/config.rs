use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_service_role_key: String,
    pub jwt_secret: String,
    pub allowed_origins: Vec<String>,
    pub max_db_connections: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let max_db_connections = env::var("MAX_DB_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .unwrap_or(10);

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            supabase_url: env::var("SUPABASE_URL")?,
            supabase_anon_key: env::var("SUPABASE_ANON_KEY")?,
            supabase_service_role_key: env::var("SUPABASE_SERVICE_ROLE_KEY")?,
            jwt_secret: env::var("JWT_SECRET")?,
            allowed_origins,
            max_db_connections,
        })
    }
}
