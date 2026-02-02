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

use crate::handlers::{
    approvals, dashboard, documents, health, locations, machine_types, machines, maintenance,
    manufacturers, projects, purchase_rfqs, team_members,
};
use crate::middleware::cors::build_cors_layer;
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = build_cors_layer(&state.config);

    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/dashboard/summary", get(dashboard::summary))
        // Team Members
        .route(
            "/api/team-members",
            get(team_members::list).post(team_members::create),
        )
        .route(
            "/api/team-members/:id",
            get(team_members::get_by_id).put(team_members::update),
        )
        // Machines
        .route("/api/machines/stats", get(machines::get_stats))
        .route("/api/machines", get(machines::list).post(machines::create))
        .route(
            "/api/machines/:id",
            get(machines::get_by_id)
                .put(machines::update)
                .delete(machines::delete),
        )
        .route("/api/machines/:id/duplicate", post(machines::duplicate))
        .route("/api/machines/:id/pipeline", get(machines::get_pipeline_status))
        // Machine types
        .route(
            "/api/machine-types",
            get(machine_types::list).post(machine_types::create),
        )
        .route(
            "/api/machine-types/:id",
            get(machine_types::get_by_id).put(machine_types::update),
        )
        // Manufacturers
        .route(
            "/api/manufacturers",
            get(manufacturers::list).post(manufacturers::create),
        )
        .route(
            "/api/manufacturers/:id",
            get(manufacturers::get_by_id).put(manufacturers::update),
        )
        // Locations
        .route(
            "/api/locations",
            get(locations::list).post(locations::create),
        )
        .route(
            "/api/locations/:id",
            get(locations::get_by_id).put(locations::update),
        )
        // Projects
        .route(
            "/api/projects",
            get(projects::list).post(projects::create),
        )
        .route(
            "/api/projects/:id",
            get(projects::get_by_id).put(projects::update),
        )
        // Purchase RFQs (scoped to machine)
        .route(
            "/api/machines/:id/purchase-rfqs",
            get(purchase_rfqs::list_by_machine).post(purchase_rfqs::create),
        )
        .route(
            "/api/purchase-rfqs/:id",
            put(purchase_rfqs::update).delete(purchase_rfqs::delete),
        )
        // Documents
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
        // Maintenance
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
