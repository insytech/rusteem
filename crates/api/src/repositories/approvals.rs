use chrono::{DateTime, Utc};
use shared::dto::approvals::PendingApproval;
use shared::{Approval, ApprovalDecision, ApprovalHistory, ApprovalStep};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_workflow_steps(
    pool: &PgPool,
    workflow_id: Uuid,
) -> Result<Vec<ApprovalStep>, sqlx::Error> {
    sqlx::query_as::<_, ApprovalStep>(
        "SELECT * FROM approval_steps WHERE workflow_id = $1 ORDER BY step_order",
    )
    .bind(workflow_id)
    .fetch_all(pool)
    .await
}

pub async fn get_approvals_for_document(
    pool: &PgPool,
    document_id: Uuid,
) -> Result<Vec<Approval>, sqlx::Error> {
    sqlx::query_as::<_, Approval>(
        "SELECT * FROM approvals WHERE document_id = $1 ORDER BY created_at",
    )
    .bind(document_id)
    .fetch_all(pool)
    .await
}

pub async fn find_approval_by_id(
    pool: &PgPool,
    approval_id: Uuid,
) -> Result<Option<Approval>, sqlx::Error> {
    sqlx::query_as::<_, Approval>("SELECT * FROM approvals WHERE id = $1")
        .bind(approval_id)
        .fetch_optional(pool)
        .await
}

pub async fn create_approval(
    pool: &PgPool,
    document_id: Uuid,
    workflow_id: Uuid,
    step_id: Uuid,
) -> Result<Approval, sqlx::Error> {
    sqlx::query_as::<_, Approval>(
        "INSERT INTO approvals (document_id, workflow_id, step_id, decision)
         VALUES ($1, $2, $3, 'pending')
         RETURNING *",
    )
    .bind(document_id)
    .bind(workflow_id)
    .bind(step_id)
    .fetch_one(pool)
    .await
}

pub async fn update_decision(
    pool: &PgPool,
    approval_id: Uuid,
    approver_id: Uuid,
    decision: &ApprovalDecision,
    comments: Option<&str>,
) -> Result<Option<Approval>, sqlx::Error> {
    let decision_str = match decision {
        ApprovalDecision::Pending => "pending",
        ApprovalDecision::Approved => "approved",
        ApprovalDecision::Rejected => "rejected",
    };

    sqlx::query_as::<_, Approval>(
        "UPDATE approvals
         SET approver_id = $2, decision = $3, comments = $4, decided_at = now()
         WHERE id = $1
         RETURNING *",
    )
    .bind(approval_id)
    .bind(approver_id)
    .bind(decision_str)
    .bind(comments)
    .fetch_optional(pool)
    .await
}

pub async fn insert_history(
    pool: &PgPool,
    document_id: Uuid,
    user_id: Uuid,
    action: &str,
    notes: Option<&str>,
) -> Result<ApprovalHistory, sqlx::Error> {
    sqlx::query_as::<_, ApprovalHistory>(
        "INSERT INTO approval_history (document_id, user_id, action, notes)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(document_id)
    .bind(user_id)
    .bind(action)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn get_history_for_document(
    pool: &PgPool,
    document_id: Uuid,
) -> Result<Vec<ApprovalHistory>, sqlx::Error> {
    sqlx::query_as::<_, ApprovalHistory>(
        "SELECT * FROM approval_history WHERE document_id = $1 ORDER BY created_at",
    )
    .bind(document_id)
    .fetch_all(pool)
    .await
}

pub async fn get_pending_for_user(
    pool: &PgPool,
    role: &str,
    cursor_created_at: Option<DateTime<Utc>>,
    cursor_id: Option<Uuid>,
    limit: i32,
) -> Result<(Vec<PendingApproval>, i64), sqlx::Error> {
    let rows = sqlx::query_as::<_, PendingApproval>(
        "SELECT a.id AS approval_id, a.document_id, d.title AS document_title,
                s.step_order, s.role_name, a.created_at
         FROM approvals a
         JOIN approval_steps s ON a.step_id = s.id
         JOIN documents d ON a.document_id = d.id
         WHERE a.decision = 'pending' AND s.role_name = $1
           AND (
               $2::timestamptz IS NULL
               OR (a.created_at, a.id) > ($2, $3::uuid)
           )
         ORDER BY a.created_at ASC, a.id ASC
         LIMIT $4",
    )
    .bind(role)
    .bind(cursor_created_at)
    .bind(cursor_id)
    .bind(i64::from(limit))
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)
         FROM approvals a
         JOIN approval_steps s ON a.step_id = s.id
         WHERE a.decision = 'pending' AND s.role_name = $1",
    )
    .bind(role)
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0))
}

/// Check if all required steps for a document+workflow are approved.
pub async fn all_required_approved(
    pool: &PgPool,
    document_id: Uuid,
    workflow_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM approvals a
         JOIN approval_steps s ON a.step_id = s.id
         WHERE a.document_id = $1
           AND a.workflow_id = $2
           AND s.is_required = true
           AND a.decision != 'approved'",
    )
    .bind(document_id)
    .bind(workflow_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0 == 0)
}

/// Check if any step for a document+workflow is rejected.
pub async fn any_rejected(
    pool: &PgPool,
    document_id: Uuid,
    workflow_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM approvals
         WHERE document_id = $1 AND workflow_id = $2 AND decision = 'rejected'",
    )
    .bind(document_id)
    .bind(workflow_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0 > 0)
}
