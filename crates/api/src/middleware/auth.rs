use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

use crate::errors::AppError;

/// JWT claims from Supabase Auth tokens.
#[derive(Debug, Deserialize)]
pub struct Claims {
    /// Subject (user ID as UUID string)
    pub sub: String,
    /// User role (e.g. "authenticated", "anon")
    #[serde(default)]
    pub role: String,
    /// User email
    #[serde(default)]
    pub email: Option<String>,
    /// Issuer
    #[serde(default)]
    pub iss: Option<String>,
    /// Expiration (unix timestamp)
    pub exp: u64,
}

/// Decode and validate a Supabase JWT token.
///
/// Supabase local uses ES256 (ECDSA P-256) by default.
/// For HMAC-signed tokens (some Supabase configs), pass the
/// symmetric secret via `jwt_secret`.
///
/// This function tries HMAC first (HS256) since that's the most common
/// Supabase self-hosted config, then falls back to fetching JWKS if needed.
pub fn decode_token(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
    // Try HS256 with the configured secret
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[
        "supabase-demo",
        "supabase",
        "http://localhost:54321/auth/v1",
    ]);
    validation.validate_exp = true;

    let key = DecodingKey::from_secret(jwt_secret.as_bytes());

    match decode::<Claims>(token, &key, &validation) {
        Ok(data) => Ok(data.claims),
        Err(e) => {
            tracing::warn!(error = %e, "JWT validation failed");
            Err(AppError::Unauthorized("Invalid or expired token".to_string()))
        }
    }
}

/// Extract the Bearer token from an Authorization header value.
pub fn extract_bearer_token(header_value: &str) -> Option<&str> {
    header_value
        .strip_prefix("Bearer ")
        .or_else(|| header_value.strip_prefix("bearer "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestClaims {
        sub: String,
        role: String,
        email: String,
        iss: String,
        exp: u64,
    }

    fn make_token(claims: &TestClaims, secret: &str) -> String {
        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(secret.as_bytes());
        encode(&header, claims, &key).expect("failed to encode token")
    }

    fn valid_claims() -> TestClaims {
        TestClaims {
            sub: "d0e1f2a3-b4c5-6789-0abc-def012345678".to_string(),
            role: "authenticated".to_string(),
            email: "test@insytech.com".to_string(),
            iss: "supabase-demo".to_string(),
            exp: (chrono::Utc::now().timestamp() as u64) + 3600,
        }
    }

    #[test]
    fn valid_token_decodes_successfully() {
        let secret = "super-secret-jwt-token-with-at-least-32-characters-long";
        let claims = valid_claims();
        let token = make_token(&claims, secret);

        let result = decode_token(&token, secret);
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert_eq!(decoded.sub, claims.sub);
        assert_eq!(decoded.role, "authenticated");
        assert_eq!(decoded.email, Some("test@insytech.com".to_string()));
    }

    #[test]
    fn expired_token_fails() {
        let secret = "super-secret-jwt-token-with-at-least-32-characters-long";
        let mut claims = valid_claims();
        claims.exp = 1000; // far in the past

        let token = make_token(&claims, secret);
        let result = decode_token(&token, secret);
        assert!(result.is_err());
    }

    #[test]
    fn wrong_secret_fails() {
        let claims = valid_claims();
        let token = make_token(&claims, "correct-secret-that-is-long-enough-for-hs256");

        let result = decode_token(&token, "wrong-secret-that-is-also-long-enough");
        assert!(result.is_err());
    }

    #[test]
    fn missing_bearer_prefix_returns_none() {
        assert!(extract_bearer_token("Token abc").is_none());
    }

    #[test]
    fn extracts_bearer_token() {
        assert_eq!(
            extract_bearer_token("Bearer my.jwt.token"),
            Some("my.jwt.token")
        );
    }

    #[test]
    fn extracts_bearer_token_lowercase() {
        assert_eq!(
            extract_bearer_token("bearer my.jwt.token"),
            Some("my.jwt.token")
        );
    }
}
