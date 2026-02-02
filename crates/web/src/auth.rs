use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const SUPABASE_URL: &str = "http://localhost:54321";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0";
const TOKEN_KEY: &str = "rusteem_token";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    #[allow(dead_code)]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
struct LoginBody {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct SignupBody {
    email: String,
    password: String,
}

pub async fn login(email: &str, password: &str) -> Result<AuthSession, String> {
    let body = LoginBody {
        email: email.to_string(),
        password: password.to_string(),
    };

    let resp = Request::post(&format!("{SUPABASE_URL}/auth/v1/token?grant_type=password"))
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap_or_default())
        .map_err(|e| format!("Request build error: {e:?}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !resp.ok() {
        let msg = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Login failed: {msg}"));
    }

    let session: AuthSession = resp
        .json()
        .await
        .map_err(|e| format!("Parse error: {e}"))?;

    let _ = LocalStorage::set(TOKEN_KEY, &session.access_token);
    // Also set it in the api module's key so authenticated requests work
    let _ = LocalStorage::set("rusteem_token", &session.access_token);

    Ok(session)
}

pub async fn signup(email: &str, password: &str) -> Result<(), String> {
    let body = SignupBody {
        email: email.to_string(),
        password: password.to_string(),
    };

    let resp = Request::post(&format!("{SUPABASE_URL}/auth/v1/signup"))
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap_or_default())
        .map_err(|e| format!("Request build error: {e:?}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !resp.ok() {
        let msg = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Signup failed: {msg}"));
    }

    Ok(())
}

pub fn logout() {
    LocalStorage::delete(TOKEN_KEY);
}

pub fn is_authenticated() -> bool {
    LocalStorage::get::<String>(TOKEN_KEY).is_ok()
}
