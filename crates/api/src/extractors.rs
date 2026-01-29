use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::{decode_token, extract_bearer_token};
use crate::state::AppState;

/// Authenticated user extracted from a valid JWT token.
///
/// Use as an extractor in handlers that require authentication:
/// ```ignore
/// async fn create_machine(user: AuthUser, ...) -> Result<..., AppError> { ... }
/// ```
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: String,
    pub email: Option<String>,
}

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                AppError::Unauthorized("Missing Authorization header".to_string())
            })?;

        let token = extract_bearer_token(auth_header).ok_or_else(|| {
            AppError::Unauthorized("Invalid Authorization header format".to_string())
        })?;

        let claims = decode_token(token, &state.config.jwt_secret)?;

        let id = claims.sub.parse::<Uuid>().map_err(|_| {
            AppError::Unauthorized("Invalid user ID in token".to_string())
        })?;

        Ok(AuthUser {
            id,
            role: claims.role,
            email: claims.email,
        })
    }
}
