use shared::dto::dashboard::{
    ActivityEntry, DashboardSummary, MachineAlert, StageCount,
};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::repositories::dashboard as repo;

const STAGE_LABELS: &[(&str, &str)] = &[
    ("scope_approval", "Scope Approval"),
    ("po_trail", "PO Trail"),
    ("design", "Design"),
    ("run_off", "Run Off"),
    ("support_documents", "Support Docs"),
    ("ramp_up", "Ramp Up"),
    ("release", "Release"),
];

fn stage_label(stage: &str) -> String {
    STAGE_LABELS
        .iter()
        .find(|(k, _)| *k == stage)
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| stage.replace('_', " "))
}

pub async fn get_summary(pool: &PgPool) -> Result<DashboardSummary, AppError> {
    let pipeline_rows = repo::get_pipeline_counts(pool).await?;
    let kpis = repo::get_kpis(pool).await?;
    let alert_rows = repo::get_needs_attention(pool).await?;
    let activity_rows = repo::get_recent_activity(pool).await?;

    // Map pipeline rows, preserving the defined stage order
    let pipeline: Vec<StageCount> = STAGE_LABELS
        .iter()
        .map(|(key, label)| {
            let count = pipeline_rows
                .iter()
                .find(|r| r.stage == *key)
                .map(|r| r.count)
                .unwrap_or(0);
            StageCount {
                stage: key.to_string(),
                label: label.to_string(),
                count,
            }
        })
        .collect();

    let needs_attention: Vec<MachineAlert> = alert_rows
        .into_iter()
        .map(|r| MachineAlert {
            machine_id: r.machine_id,
            machine_name: r.machine_name,
            area: r.area,
            current_stage: stage_label(&r.current_stage),
            status: r.status,
            days_overdue: r.days_overdue,
            responsible: None,
        })
        .collect();

    let recent_activity: Vec<ActivityEntry> = activity_rows
        .into_iter()
        .map(|r| ActivityEntry {
            id: r.id,
            description: r.description,
            timestamp: r.timestamp,
        })
        .collect();

    Ok(DashboardSummary {
        pipeline,
        total_active: kpis.total_active,
        total_released: kpis.total_released,
        total_in_progress: kpis.total_in_progress,
        total_overdue: kpis.total_overdue,
        total_breaches: kpis.total_breaches,
        released_this_month: kpis.released_this_month,
        needs_attention,
        recent_activity,
    })
}
