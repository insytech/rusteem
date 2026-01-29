use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

const API_BASE: &str = "/api";
const TOKEN_KEY: &str = "rusteem_token";

pub fn get_token() -> Option<String> {
    LocalStorage::get::<String>(TOKEN_KEY).ok()
}

#[allow(dead_code)]
pub fn set_token(token: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token.to_string());
}

#[allow(dead_code)]
pub fn clear_token() {
    LocalStorage::delete(TOKEN_KEY);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.status)
    }
}

fn auth_header_value() -> Option<String> {
    get_token().map(|t| format!("Bearer {t}"))
}

pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    let url = format!("{API_BASE}{path}");
    let mut req = Request::get(&url);

    if let Some(auth) = auth_header_value() {
        req = req.header("Authorization", &auth);
    }

    let resp = req.send().await.map_err(|e| ApiError {
        status: 0,
        message: format!("Network error: {e}"),
    })?;

    if !resp.ok() {
        return Err(ApiError {
            status: resp.status(),
            message: resp.text().await.unwrap_or_default(),
        });
    }

    resp.json::<T>().await.map_err(|e| ApiError {
        status: 0,
        message: format!("Parse error: {e}"),
    })
}

pub async fn post<T: DeserializeOwned, B: Serialize>(
    path: &str,
    body: &B,
) -> Result<T, ApiError> {
    let url = format!("{API_BASE}{path}");
    let json_body = serde_json::to_string(body).unwrap_or_default();

    let mut req = Request::post(&url).header("Content-Type", "application/json");

    if let Some(auth) = auth_header_value() {
        req = req.header("Authorization", &auth);
    }

    let resp = req
        .body(json_body)
        .map_err(|e| ApiError {
            status: 0,
            message: format!("Request build error: {e:?}"),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            status: 0,
            message: format!("Network error: {e}"),
        })?;

    if !resp.ok() {
        return Err(ApiError {
            status: resp.status(),
            message: resp.text().await.unwrap_or_default(),
        });
    }

    resp.json::<T>().await.map_err(|e| ApiError {
        status: 0,
        message: format!("Parse error: {e}"),
    })
}

#[allow(dead_code)]
pub async fn put<T: DeserializeOwned, B: Serialize>(
    path: &str,
    body: &B,
) -> Result<T, ApiError> {
    let url = format!("{API_BASE}{path}");
    let json_body = serde_json::to_string(body).unwrap_or_default();

    let mut req = Request::put(&url).header("Content-Type", "application/json");

    if let Some(auth) = auth_header_value() {
        req = req.header("Authorization", &auth);
    }

    let resp = req
        .body(json_body)
        .map_err(|e| ApiError {
            status: 0,
            message: format!("Request build error: {e:?}"),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            status: 0,
            message: format!("Network error: {e}"),
        })?;

    if !resp.ok() {
        return Err(ApiError {
            status: resp.status(),
            message: resp.text().await.unwrap_or_default(),
        });
    }

    resp.json::<T>().await.map_err(|e| ApiError {
        status: 0,
        message: format!("Parse error: {e}"),
    })
}

pub async fn fetch_dashboard_summary() -> Result<shared::dto::dashboard::DashboardSummary, ApiError>
{
    get::<shared::dto::dashboard::DashboardSummary>("/dashboard/summary").await
}

pub async fn delete_req(path: &str) -> Result<(), ApiError> {
    let url = format!("{API_BASE}{path}");
    let mut req = Request::delete(&url);

    if let Some(auth) = auth_header_value() {
        req = req.header("Authorization", &auth);
    }

    let resp = req.send().await.map_err(|e| ApiError {
        status: 0,
        message: format!("Network error: {e}"),
    })?;

    if !resp.ok() {
        return Err(ApiError {
            status: resp.status(),
            message: resp.text().await.unwrap_or_default(),
        });
    }

    Ok(())
}
