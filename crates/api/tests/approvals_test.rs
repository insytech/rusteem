mod common;

use axum::http::{Method, StatusCode};
use shared::ApprovalHistory;

#[tokio::test]
async fn get_pending_approvals_requires_auth() {
    let app = common::test_app().await;
    let status = common::send_no_auth(&app, Method::GET, "/api/approvals/pending").await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_pending_approvals_returns_200() {
    let app = common::test_app().await;
    let (status, _): (_, Option<serde_json::Value>) =
        common::send_json(&app, Method::GET, "/api/approvals/pending", None).await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn get_document_history_returns_200() {
    let app = common::test_app().await;

    // Even for a non-existent document, history returns empty array (not 404)
    let (status, body): (_, Option<Vec<ApprovalHistory>>) = common::send_json(
        &app,
        Method::GET,
        "/api/documents/00000000-0000-0000-0000-000000000000/history",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let list = body.expect("Should return array");
    assert!(list.is_empty());
}

#[tokio::test]
async fn initiate_workflow_requires_auth() {
    let app = common::test_app().await;
    let status = common::send_no_auth(
        &app,
        Method::POST,
        "/api/documents/00000000-0000-0000-0000-000000000000/workflows/00000000-0000-0000-0000-000000000001/initiate",
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn initiate_workflow_document_not_found() {
    let app = common::test_app().await;
    let (status, _): (_, Option<serde_json::Value>) = common::send_json(
        &app,
        Method::POST,
        "/api/documents/00000000-0000-0000-0000-000000000000/workflows/00000000-0000-0000-0000-000000000001/initiate",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn decide_approval_not_found() {
    let app = common::test_app().await;
    let body = serde_json::json!({
        "decision": "approved",
        "comments": "Looks good"
    });

    let (status, _): (_, Option<serde_json::Value>) = common::send_json(
        &app,
        Method::POST,
        "/api/approvals/00000000-0000-0000-0000-000000000000/decide",
        Some(body),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}
