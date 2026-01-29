mod common;

use axum::http::{Method, StatusCode};
use shared::Document;

#[tokio::test]
async fn list_documents_returns_200() {
    let app = common::test_app().await;
    let (status, body): (_, Option<Vec<Document>>) =
        common::send_json(&app, Method::GET, "/api/documents", None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_some());
}

#[tokio::test]
async fn get_document_not_found_returns_404() {
    let app = common::test_app().await;
    let (status, _): (_, Option<serde_json::Value>) = common::send_json(
        &app,
        Method::GET,
        "/api/documents/00000000-0000-0000-0000-000000000000",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_document_requires_auth() {
    let app = common::test_app().await;
    let status = common::send_no_auth(&app, Method::POST, "/api/documents").await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn update_status_invalid_transition_fails() {
    let app = common::test_app().await;

    // Try to update status of a non-existent document
    let body = serde_json::json!({ "status": "approved" });
    let (status, _): (_, Option<serde_json::Value>) = common::send_json(
        &app,
        Method::PATCH,
        "/api/documents/00000000-0000-0000-0000-000000000000/status",
        Some(body),
    )
    .await;

    // Should be 404 since document doesn't exist
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_document_not_found() {
    let app = common::test_app().await;
    let (status, _): (_, Option<serde_json::Value>) = common::send_json(
        &app,
        Method::DELETE,
        "/api/documents/00000000-0000-0000-0000-000000000000",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}
