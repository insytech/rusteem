use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct StageCountRow {
    pub stage: String,
    pub count: i64,
}

#[derive(sqlx::FromRow)]
pub struct KpiRow {
    pub total_active: i64,
    pub total_released: i64,
    pub total_in_progress: i64,
    pub total_overdue: i64,
    pub total_breaches: i64,
    pub released_this_month: i64,
}

#[derive(sqlx::FromRow)]
pub struct AlertRow {
    pub machine_id: uuid::Uuid,
    pub machine_name: String,
    pub area: Option<String>,
    pub current_stage: String,
    pub status: String,
    pub days_overdue: i64,
}

#[derive(sqlx::FromRow)]
pub struct ActivityRow {
    pub id: uuid::Uuid,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn get_pipeline_counts(pool: &PgPool) -> Result<Vec<StageCountRow>, sqlx::Error> {
    sqlx::query_as::<_, StageCountRow>(
        "SELECT
            s.stage::text AS stage,
            COALESCE(c.cnt, 0) AS count
         FROM unnest(enum_range(NULL::stage_name)) AS s(stage)
         LEFT JOIN (
             SELECT current_stage::text AS stage, COUNT(*) AS cnt
             FROM machine_pipeline_status
             WHERE overall_status = 'in_progress'
             GROUP BY current_stage
         ) c ON c.stage = s.stage::text
         ORDER BY s.stage::text"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_kpis(pool: &PgPool) -> Result<KpiRow, sqlx::Error> {
    sqlx::query_as::<_, KpiRow>(
        "SELECT
            (SELECT COUNT(*) FROM machines WHERE active = true) AS total_active,
            (SELECT COUNT(*) FROM machine_pipeline_status WHERE overall_status = 'released') AS total_released,
            (SELECT COUNT(*) FROM machine_pipeline_status WHERE overall_status = 'in_progress') AS total_in_progress,
            (SELECT COUNT(DISTINCT machine_id) FROM machine_approval_stages WHERE status = 'overdue') AS total_overdue,
            (SELECT COUNT(DISTINCT machine_id) FROM machine_approval_stages WHERE status = 'breach') AS total_breaches,
            (SELECT COUNT(*) FROM machine_pipeline_status
             WHERE overall_status = 'released'
               AND updated_at >= date_trunc('month', now())
            ) AS released_this_month"
    )
    .fetch_one(pool)
    .await
}

pub async fn get_needs_attention(pool: &PgPool) -> Result<Vec<AlertRow>, sqlx::Error> {
    sqlx::query_as::<_, AlertRow>(
        "SELECT
            m.id AS machine_id,
            m.name AS machine_name,
            m.area,
            ps.current_stage::text AS current_stage,
            mas.status::text AS status,
            COALESCE(EXTRACT(DAY FROM now() - mas.due_date)::bigint, 0) AS days_overdue
         FROM machine_approval_stages mas
         JOIN machines m ON m.id = mas.machine_id
         JOIN machine_pipeline_status ps ON ps.machine_id = m.id
         WHERE mas.status IN ('overdue', 'breach')
         ORDER BY days_overdue DESC
         LIMIT 10"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_recent_activity(pool: &PgPool) -> Result<Vec<ActivityRow>, sqlx::Error> {
    sqlx::query_as::<_, ActivityRow>(
        "SELECT
            ah.id,
            CONCAT(m.name, ' — ', ah.action,
                CASE WHEN ah.notes IS NOT NULL THEN CONCAT(': ', ah.notes) ELSE '' END
            ) AS description,
            ah.created_at AS timestamp
         FROM approval_history ah
         LEFT JOIN documents d ON d.id = ah.document_id
         LEFT JOIN machines m ON m.id = d.machine_id
         ORDER BY ah.created_at DESC
         LIMIT 10"
    )
    .fetch_all(pool)
    .await
}
