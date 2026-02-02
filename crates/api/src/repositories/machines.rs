use shared::dto::machines::{CreateMachineRequest, MachineDetail, MachineFilters, UpdateMachineRequest};
use shared::Machine;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_all_with_details(
    pool: &PgPool,
    filters: &MachineFilters,
    cursor_name: Option<&str>,
    cursor_id: Option<Uuid>,
    limit: i32,
) -> Result<(Vec<MachineDetail>, i64), sqlx::Error> {
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let rows = sqlx::query_as::<_, MachineDetail>(
        "SELECT
            m.*,
            mt.name AS machine_type_name,
            mfr.name AS manufacturer_name,
            loc.area AS location_area,
            loc.line AS location_line,
            p.name AS project_name
         FROM machines m
         LEFT JOIN machine_types mt ON m.machine_type_id = mt.id
         LEFT JOIN manufacturers mfr ON m.manufacturer_id = mfr.id
         LEFT JOIN locations loc ON m.location_id = loc.id
         LEFT JOIN projects p ON m.project_id = p.id
         WHERE ($1::bool IS NULL OR m.active = $1)
           AND ($2::text IS NULL OR m.area = $2)
           AND ($3::text IS NULL OR m.line = $3)
           AND ($4::text IS NULL OR m.station = $4)
           AND ($5::uuid IS NULL OR m.machine_type_id = $5)
           AND ($6::uuid IS NULL OR m.manufacturer_id = $6)
           AND ($7::uuid IS NULL OR m.location_id = $7)
           AND ($8::text IS NULL OR m.responsible = $8)
           AND ($9::text IS NULL OR m.name ILIKE $9)
           AND (
               $10::text IS NULL
               OR (m.name, m.id) > ($10, $11::uuid)
           )
         ORDER BY m.name ASC, m.id ASC
         LIMIT $12",
    )
    .bind(filters.active)
    .bind(&filters.area)
    .bind(&filters.line)
    .bind(&filters.station)
    .bind(filters.machine_type_id)
    .bind(filters.manufacturer_id)
    .bind(filters.location_id)
    .bind(&filters.responsible)
    .bind(&search_pattern)
    .bind(cursor_name)
    .bind(cursor_id)
    .bind(i64::from(limit))
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM machines m
         WHERE ($1::bool IS NULL OR m.active = $1)
           AND ($2::text IS NULL OR m.area = $2)
           AND ($3::text IS NULL OR m.line = $3)
           AND ($4::text IS NULL OR m.station = $4)
           AND ($5::uuid IS NULL OR m.machine_type_id = $5)
           AND ($6::uuid IS NULL OR m.manufacturer_id = $6)
           AND ($7::uuid IS NULL OR m.location_id = $7)
           AND ($8::text IS NULL OR m.responsible = $8)
           AND ($9::text IS NULL OR m.name ILIKE $9)",
    )
    .bind(filters.active)
    .bind(&filters.area)
    .bind(&filters.line)
    .bind(&filters.station)
    .bind(filters.machine_type_id)
    .bind(filters.manufacturer_id)
    .bind(filters.location_id)
    .bind(&filters.responsible)
    .bind(&search_pattern)
    .fetch_one(pool)
    .await?;

    Ok((rows, total.0))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>("SELECT * FROM machines WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_detail_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<MachineDetail>, sqlx::Error> {
    sqlx::query_as::<_, MachineDetail>(
        "SELECT
            m.*,
            mt.name AS machine_type_name,
            mfr.name AS manufacturer_name,
            loc.area AS location_area,
            loc.line AS location_line,
            p.name AS project_name
         FROM machines m
         LEFT JOIN machine_types mt ON m.machine_type_id = mt.id
         LEFT JOIN manufacturers mfr ON m.manufacturer_id = mfr.id
         LEFT JOIN locations loc ON m.location_id = loc.id
         LEFT JOIN projects p ON m.project_id = p.id
         WHERE m.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(pool: &PgPool, data: &CreateMachineRequest) -> Result<Machine, sqlx::Error> {
    sqlx::query_as::<_, Machine>(
        "INSERT INTO machines (name, asset_number, line, station, area, model, serial_number, machine_type_id, manufacturer_id, location_id, project_id, responsible)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
         RETURNING *",
    )
    .bind(&data.name)
    .bind(&data.asset_number)
    .bind(&data.line)
    .bind(&data.station)
    .bind(&data.area)
    .bind(&data.model)
    .bind(&data.serial_number)
    .bind(data.machine_type_id)
    .bind(data.manufacturer_id)
    .bind(data.location_id)
    .bind(data.project_id)
    .bind(&data.responsible)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    data: &UpdateMachineRequest,
) -> Result<Option<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>(
        "UPDATE machines SET
            name = COALESCE($2, name),
            asset_number = COALESCE($3, asset_number),
            line = COALESCE($4, line),
            station = COALESCE($5, station),
            area = COALESCE($6, area),
            active = COALESCE($7, active),
            model = COALESCE($8, model),
            serial_number = COALESCE($9, serial_number),
            machine_type_id = COALESCE($10, machine_type_id),
            manufacturer_id = COALESCE($11, manufacturer_id),
            location_id = COALESCE($12, location_id),
            project_id = COALESCE($13, project_id),
            responsible = COALESCE($14, responsible)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&data.name)
    .bind(&data.asset_number)
    .bind(&data.line)
    .bind(&data.station)
    .bind(&data.area)
    .bind(data.active)
    .bind(&data.model)
    .bind(&data.serial_number)
    .bind(data.machine_type_id)
    .bind(data.manufacturer_id)
    .bind(data.location_id)
    .bind(data.project_id)
    .bind(&data.responsible)
    .fetch_optional(pool)
    .await
}

pub async fn soft_delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE machines SET active = false WHERE id = $1 AND active = true")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_by_asset_number(
    pool: &PgPool,
    asset_number: &str,
) -> Result<Option<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>("SELECT * FROM machines WHERE asset_number = $1")
        .bind(asset_number)
        .fetch_optional(pool)
        .await
}

pub async fn duplicate(
    pool: &PgPool,
    source_id: Uuid,
    new_name: &str,
) -> Result<Option<Machine>, sqlx::Error> {
    sqlx::query_as::<_, Machine>(
        "INSERT INTO machines (name, line, station, area, model, serial_number, machine_type_id, manufacturer_id, location_id, project_id, responsible)
         SELECT $2, line, station, area, model, serial_number, machine_type_id, manufacturer_id, location_id, project_id, responsible
         FROM machines WHERE id = $1
         RETURNING *",
    )
    .bind(source_id)
    .bind(new_name)
    .fetch_optional(pool)
    .await
}
