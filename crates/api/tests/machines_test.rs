mod common;

use axum::http::{Method, StatusCode};
use shared::Machine;

#[tokio::test]
async fn list_machines_returns_200() {
    let app = common::test_app().await;
    let (status, body): (_, Option<Vec<Machine>>) =
        common::send_json(&app, Method::GET, "/api/machines", None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_some());
}

#[tokio::test]
async fn get_machine_not_found_returns_404() {
    let app = common::test_app().await;
    let (status, _): (_, Option<serde_json::Value>) = common::send_json(
        &app,
        Method::GET,
        "/api/machines/00000000-0000-0000-0000-000000000000",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_machine_requires_auth() {
    let app = common::test_app().await;
    let status = common::send_no_auth(&app, Method::POST, "/api/machines").await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_and_delete_machine() {
    let app = common::test_app().await;

    // Create
    let body = serde_json::json!({
        "name": "Test Machine Integration",
        "asset_number": "INT-TEST-001",
        "area": "Testing",
        "line": "L1",
        "station": "S1"
    });

    let (status, machine): (_, Option<Machine>) =
        common::send_json(&app, Method::POST, "/api/machines", Some(body)).await;

    assert_eq!(status, StatusCode::CREATED);
    let machine = machine.expect("Should return created machine");
    assert_eq!(machine.name, "Test Machine Integration");
    assert!(machine.active);

    // Soft delete
    let uri = format!("/api/machines/{}", machine.id);
    let (status, _): (_, Option<serde_json::Value>) =
        common::send_json(&app, Method::DELETE, &uri, None).await;

    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn create_machine_empty_name_fails() {
    let app = common::test_app().await;

    let body = serde_json::json!({
        "name": "  ",
    });

    let (status, _): (_, Option<serde_json::Value>) =
        common::send_json(&app, Method::POST, "/api/machines", Some(body)).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn health_check_returns_ok() {
    let app = common::test_app().await;
    let (status, body): (_, Option<serde_json::Value>) =
        common::send_json(&app, Method::GET, "/health", None).await;

    assert_eq!(status, StatusCode::OK);
    let body = body.expect("Should return health JSON");
    assert_eq!(body["status"], "ok");
    assert_eq!(body["database"], "connected");
}
