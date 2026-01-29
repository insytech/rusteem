pub mod config;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod middleware;
pub mod repositories;
pub mod services;
pub mod state;

use axum::routing::{get, patch, post, put};
use axum::Router;
use tower_http::trace::TraceLayer;

use crate::handlers::{approvals, dashboard, documents, health, machines, maintenance};
use crate::middleware::cors::build_cors_layer;
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = build_cors_layer(&state.config);

    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/dashboard/summary", get(dashboard::summary))
        .route("/api/machines", get(machines::list).post(machines::create))
        .route(
            "/api/machines/:id",
            get(machines::get_by_id)
                .put(machines::update)
                .delete(machines::delete),
        )
        .route(
            "/api/documents",
            get(documents::list).post(documents::create),
        )
        .route(
            "/api/documents/:id",
            get(documents::get_by_id)
                .put(documents::update_metadata)
                .delete(documents::delete),
        )
        .route(
            "/api/documents/:id/status",
            patch(documents::update_status),
        )
        .route(
            "/api/documents/:id/approvals",
            get(approvals::get_document_approvals),
        )
        .route(
            "/api/documents/:id/history",
            get(approvals::get_document_history),
        )
        .route(
            "/api/documents/:id/workflows/:workflow_id/initiate",
            post(approvals::initiate_workflow),
        )
        .route(
            "/api/approvals/pending",
            get(approvals::get_pending),
        )
        .route(
            "/api/approvals/:id/decide",
            post(approvals::submit_decision),
        )
        .route(
            "/api/machines/:id/maintenance",
            get(maintenance::get_by_machine),
        )
        .route(
            "/api/maintenance/upcoming",
            get(maintenance::get_upcoming),
        )
        .route(
            "/api/maintenance/overdue",
            get(maintenance::get_overdue),
        )
        .route("/api/maintenance", post(maintenance::create))
        .route(
            "/api/maintenance/:id",
            put(maintenance::update).delete(maintenance::delete),
        )
        .route(
            "/api/maintenance/:id/complete",
            post(maintenance::complete),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
