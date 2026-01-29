use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use uuid::Uuid;

use crate::errors::AppError;

const DEFAULT_LIMIT: i32 = 50;
const MAX_LIMIT: i32 = 100;

pub fn encode_cursor(field_value: &str, id: Uuid) -> String {
    let raw = format!("{field_value}|{id}");
    URL_SAFE_NO_PAD.encode(raw.as_bytes())
}

pub fn decode_cursor(cursor: &str) -> Result<(String, Uuid), AppError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| AppError::Validation("Invalid cursor".to_string()))?;
    let raw = String::from_utf8(bytes)
        .map_err(|_| AppError::Validation("Invalid cursor encoding".to_string()))?;
    let (field, id_str) = raw
        .rsplit_once('|')
        .ok_or_else(|| AppError::Validation("Malformed cursor".to_string()))?;
    let id = id_str
        .parse::<Uuid>()
        .map_err(|_| AppError::Validation("Invalid cursor ID".to_string()))?;
    Ok((field.to_string(), id))
}

pub fn clamp_limit(limit: Option<i32>) -> i32 {
    limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT)
}
