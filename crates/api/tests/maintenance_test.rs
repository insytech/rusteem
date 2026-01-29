mod common;

use axum::http::{Method, StatusCode};
use shared::MaintenancePlan;

#[tokio::test]
async fn get_upcoming_returns_200() {
    let app = common::test_app().await;
    let (status, body): (_, Option<Vec<MaintenancePlan>>) =
        common::send_json(&app, Method::GET, "/api/maintenance/upcoming?days=7", None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_some());
}

#[tokio::test]
async fn get_overdue_returns_200() {
    let app = common::test_app().await;
    let (status, body): (_, Option<Vec<MaintenancePlan>>) =
        common::send_json(&app, Method::GET, "/api/maintenance/overdue", None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_some());
}

#[tokio::test]
async fn create_maintenance_requires_auth() {
    let app = common::test_app().await;
    let status = common::send_no_auth(&app, Method::POST, "/api/maintenance").await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_and_complete_maintenance_plan() {
    let app = common::test_app().await;

    // Create plan
    let body = serde_json::json!({
        "description": "Integration Test Plan",
        "frequency_value": 7,
        "frequency_unit": "days",
        "next_due_at": "2026-02-01T00:00:00Z"
    });

    let (status, plan): (_, Option<MaintenancePlan>) =
        common::send_json(&app, Method::POST, "/api/maintenance", Some(body)).await;

    assert_eq!(status, StatusCode::CREATED);
    let plan = plan.expect("Should return created plan");
    assert_eq!(plan.description, "Integration Test Plan");
    assert_eq!(plan.frequency_value, 7);

    // Complete plan
    let complete_body = serde_json::json!({
        "performed_at": "2026-01-29T12:00:00Z"
    });
    let uri = format!("/api/maintenance/{}/complete", plan.id);
    let (status, updated): (_, Option<MaintenancePlan>) =
        common::send_json(&app, Method::POST, &uri, Some(complete_body)).await;

    assert_eq!(status, StatusCode::OK);
    let updated = updated.expect("Should return updated plan");
    assert!(updated.last_performed_at.is_some());
    assert!(updated.next_due_at.is_some());

    // Clean up: delete plan
    let uri = format!("/api/maintenance/{}", plan.id);
    let (status, _): (_, Option<serde_json::Value>) =
        common::send_json(&app, Method::DELETE, &uri, None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn create_maintenance_empty_description_fails() {
    let app = common::test_app().await;

    let body = serde_json::json!({
        "description": "  ",
        "frequency_value": 7,
        "frequency_unit": "days"
    });

    let (status, _): (_, Option<serde_json::Value>) =
        common::send_json(&app, Method::POST, "/api/maintenance", Some(body)).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn create_maintenance_zero_frequency_fails() {
    let app = common::test_app().await;

    let body = serde_json::json!({
        "description": "Bad Plan",
        "frequency_value": 0,
        "frequency_unit": "days"
    });

    let (status, _): (_, Option<serde_json::Value>) =
        common::send_json(&app, Method::POST, "/api/maintenance", Some(body)).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn delete_maintenance_not_found() {
    let app = common::test_app().await;
    let (status, _): (_, Option<serde_json::Value>) = common::send_json(
        &app,
        Method::DELETE,
        "/api/maintenance/00000000-0000-0000-0000-000000000000",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_machine_maintenance_returns_200() {
    let app = common::test_app().await;
    let (status, body): (_, Option<Vec<MaintenancePlan>>) = common::send_json(
        &app,
        Method::GET,
        "/api/machines/00000000-0000-0000-0000-000000000000/maintenance",
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let list = body.expect("Should return array");
    assert!(list.is_empty());
}
