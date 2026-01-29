use crate::config::AppConfig;
use crate::errors::AppError;

/// Upload a file to Supabase Storage.
///
/// Files are organized as: `documents/{machine_id_or_general}/{document_type_id}/rev_{revision}_{filename}`
pub async fn upload_file(
    config: &AppConfig,
    bucket: &str,
    path: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<String, AppError> {
    let url = format!(
        "{}/storage/v1/object/{}/{}",
        config.supabase_url, bucket, path
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.supabase_service_role_key))
        .header("Content-Type", content_type)
        .body(data)
        .send()
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to upload file to storage");
            AppError::Internal("File upload failed".to_string())
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        tracing::error!(status = %status, body = %body, "Storage upload error");
        return Err(AppError::Internal("File upload to storage failed".to_string()));
    }

    Ok(format!("{}/{}", bucket, path))
}

/// Delete a file from Supabase Storage.
pub async fn delete_file(
    config: &AppConfig,
    bucket: &str,
    path: &str,
) -> Result<(), AppError> {
    let url = format!(
        "{}/storage/v1/object/{}/{}",
        config.supabase_url, bucket, path
    );

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", config.supabase_service_role_key))
        .send()
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete file from storage");
            AppError::Internal("File deletion failed".to_string())
        })?;

    if !response.status().is_success() {
        tracing::warn!(
            status = %response.status(),
            "Storage delete returned non-success (file may not exist)"
        );
    }

    Ok(())
}
