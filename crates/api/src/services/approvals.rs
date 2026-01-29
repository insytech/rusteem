use chrono::DateTime;
use shared::dto::approvals::{PendingApproval, SubmitDecisionRequest};
use shared::dto::pagination::{PaginatedResponse, PaginationParams};
use shared::{Approval, ApprovalDecision, ApprovalHistory, DocumentStatus};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::repositories::approvals as repo;
use crate::repositories::documents as doc_repo;
use crate::repositories::pagination::{clamp_limit, decode_cursor, encode_cursor};

/// Initiate an approval workflow for a document.
/// Creates one pending Approval per step in the workflow.
pub async fn initiate_workflow(
    pool: &PgPool,
    document_id: Uuid,
    workflow_id: Uuid,
    initiator_id: Uuid,
) -> Result<Vec<Approval>, AppError> {
    // Verify document exists and is in "pending" status
    let document = doc_repo::find_by_id(pool, document_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Document {document_id} not found")))?;

    if document.status != DocumentStatus::Pending {
        return Err(AppError::Validation(
            "Document must be in 'pending' status to initiate approval workflow".to_string(),
        ));
    }

    // Check no existing approvals for this document+workflow
    let existing = repo::get_approvals_for_document(pool, document_id).await?;
    let has_active = existing
        .iter()
        .any(|a| a.workflow_id == Some(workflow_id) && a.decision == ApprovalDecision::Pending);
    if has_active {
        return Err(AppError::Validation(
            "Workflow already initiated for this document".to_string(),
        ));
    }

    let steps = repo::get_workflow_steps(pool, workflow_id).await?;
    if steps.is_empty() {
        return Err(AppError::Validation(
            "Workflow has no steps defined".to_string(),
        ));
    }

    let mut approvals = Vec::with_capacity(steps.len());
    for step in &steps {
        let approval =
            repo::create_approval(pool, document_id, workflow_id, step.id).await?;
        approvals.push(approval);
    }

    // Record in history
    repo::insert_history(
        pool,
        document_id,
        initiator_id,
        "workflow_initiated",
        Some(&format!("Workflow {workflow_id} initiated")),
    )
    .await?;

    Ok(approvals)
}

/// Submit a decision (approve/reject) for an approval step.
pub async fn submit_decision(
    pool: &PgPool,
    approval_id: Uuid,
    user_id: Uuid,
    request: SubmitDecisionRequest,
) -> Result<Approval, AppError> {
    let approval = repo::find_approval_by_id(pool, approval_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Approval {approval_id} not found")))?;

    if approval.decision != ApprovalDecision::Pending {
        return Err(AppError::Validation(
            "This approval step has already been decided".to_string(),
        ));
    }

    if request.decision == ApprovalDecision::Pending {
        return Err(AppError::Validation(
            "Cannot set decision to 'pending'".to_string(),
        ));
    }

    let updated = repo::update_decision(
        pool,
        approval_id,
        user_id,
        &request.decision,
        request.comments.as_deref(),
    )
    .await?
    .ok_or_else(|| AppError::Internal("Failed to update approval".to_string()))?;

    let action = match request.decision {
        ApprovalDecision::Approved => "approved",
        ApprovalDecision::Rejected => "rejected",
        ApprovalDecision::Pending => unreachable!(),
    };

    repo::insert_history(
        pool,
        approval.document_id,
        user_id,
        action,
        request.comments.as_deref(),
    )
    .await?;

    // Check if workflow is complete
    if let Some(workflow_id) = approval.workflow_id {
        if request.decision == ApprovalDecision::Rejected {
            // Any rejection → document back to draft
            doc_repo::update_status(pool, approval.document_id, &DocumentStatus::Rejected)
                .await?;
            repo::insert_history(
                pool,
                approval.document_id,
                user_id,
                "document_rejected",
                Some("Approval step rejected; document status changed to rejected"),
            )
            .await?;
        } else if repo::all_required_approved(pool, approval.document_id, workflow_id).await? {
            // All required steps approved → document approved
            doc_repo::update_status(pool, approval.document_id, &DocumentStatus::Approved)
                .await?;
            repo::insert_history(
                pool,
                approval.document_id,
                user_id,
                "document_approved",
                Some("All required steps approved; document status changed to approved"),
            )
            .await?;
        }
    }

    Ok(updated)
}

/// Get pending approvals for a given role.
pub async fn get_pending_for_role(
    pool: &PgPool,
    role: &str,
    pagination: &PaginationParams,
) -> Result<PaginatedResponse<PendingApproval>, AppError> {
    let limit = clamp_limit(pagination.limit);
    let (cursor_ts, cursor_id) = match &pagination.cursor {
        Some(c) => {
            let (ts_str, id) = decode_cursor(c)?;
            let ts = DateTime::parse_from_rfc3339(&ts_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|_| AppError::Validation("Invalid cursor timestamp".to_string()))?;
            (Some(ts), Some(id))
        }
        None => (None, None),
    };

    let (mut rows, total) =
        repo::get_pending_for_user(pool, role, cursor_ts, cursor_id, limit + 1).await?;

    let has_more = rows.len() > limit as usize;
    if has_more {
        rows.pop();
    }

    let next_cursor = if has_more {
        rows.last()
            .map(|p| encode_cursor(&p.created_at.to_rfc3339(), p.approval_id))
    } else {
        None
    };

    Ok(PaginatedResponse {
        items: rows,
        next_cursor,
        total,
    })
}

/// Get approval history for a document.
pub async fn get_document_history(
    pool: &PgPool,
    document_id: Uuid,
) -> Result<Vec<ApprovalHistory>, AppError> {
    repo::get_history_for_document(pool, document_id)
        .await
        .map_err(AppError::from)
}

/// Get all approvals for a document.
pub async fn get_document_approvals(
    pool: &PgPool,
    document_id: Uuid,
) -> Result<Vec<Approval>, AppError> {
    repo::get_approvals_for_document(pool, document_id)
        .await
        .map_err(AppError::from)
}
