use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use shared::dto::machines::MachineDetail;
use shared::PurchaseRfq;

use crate::api;
use crate::components::confirm_modal::ConfirmModal;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PipelineStage {
    stage: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PipelineStatus {
    machine_id: uuid::Uuid,
    current_stage: String,
    overall_status: String,
    release_level: i32,
    stages: Vec<PipelineStage>,
}

fn stage_label(stage: &str) -> &str {
    match stage {
        "scope_approval" => "Scope",
        "po_trail" => "PO Trail",
        "design" => "Design",
        "run_off" => "Run Off",
        "support_documents" => "Support Docs",
        "ramp_up" => "Ramp Up",
        "release" => "Release",
        _ => stage,
    }
}

fn stage_css(status: &str) -> &str {
    match status {
        "completed" => "pipeline-node has-count",
        "in_progress" => "pipeline-node has-count scale-lg",
        _ => "pipeline-node",
    }
}

#[component]
pub fn MachineDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params.with(|p| {
            p.get("id")
                .and_then(|id| id.parse::<uuid::Uuid>().ok())
        })
    };

    let machine = create_resource(
        id,
        move |maybe_id| async move {
            match maybe_id {
                Some(id) => api::get::<MachineDetail>(&format!("/machines/{id}")).await.ok(),
                None => None,
            }
        },
    );

    let rfqs = create_resource(
        id,
        move |maybe_id| async move {
            match maybe_id {
                Some(id) => api::get::<Vec<PurchaseRfq>>(&format!("/machines/{id}/purchase-rfqs")).await.unwrap_or_default(),
                None => vec![],
            }
        },
    );

    let pipeline = create_resource(
        id,
        move |maybe_id| async move {
            match maybe_id {
                Some(id) => api::get::<PipelineStatus>(&format!("/machines/{id}/pipeline")).await.ok(),
                None => None,
            }
        },
    );

    let (show_delete, set_show_delete) = create_signal(false);
    let navigate = use_navigate();
    let navigate = store_value(navigate);

    view! {
        <Suspense fallback=move || view! { <p class="loading">"Loading machine..."</p> }>
            {move || {
                let nav = navigate.get_value();
                machine.get().map(|maybe_m| match maybe_m {
                    None => view! {
                        <div class="error-message">"Machine not found."</div>
                    }.into_view(),
                    Some(m) => {
                        let machine_id = m.id;
                        let machine_name_for_modal = m.name.clone();
                        let edit_href = format!("/machines/{}/edit", m.id);
                        let nav_dup = nav.clone();
                        let nav_del = nav.clone();

                        view! {
                            // Header
                            <div class="page-header">
                                <div>
                                    <h2>{m.name.clone()}</h2>
                                    <div class="page-header-sub">
                                        {m.asset_number.clone().map(|a| format!("Asset: {a}")).unwrap_or_default()}
                                        " "
                                        <span class={if m.active { "badge badge-approved" } else { "badge badge-rejected" }}>
                                            {if m.active { "Active" } else { "Inactive" }}
                                        </span>
                                    </div>
                                </div>
                                <div style="display: flex; gap: 0.5rem;">
                                    <a href=edit_href class="btn btn-primary">"Edit"</a>
                                    <button class="btn btn-outline"
                                        on:click=move |_| {
                                            let nav_dup = nav_dup.clone();
                                            spawn_local(async move {
                                                match api::post::<shared::Machine, ()>(&format!("/machines/{machine_id}/duplicate"), &()).await {
                                                    Ok(dup) => {
                                                        nav_dup(&format!("/machines/{}", dup.id), Default::default());
                                                    }
                                                    Err(e) => {
                                                        web_sys::window()
                                                            .and_then(|w| w.alert_with_message(&format!("Duplicate failed: {e}")).ok());
                                                    }
                                                }
                                            });
                                        }
                                    >"Duplicate"</button>
                                    <button class="btn btn-danger" on:click=move |_| set_show_delete.set(true)>"Delete"</button>
                                </div>
                            </div>

                            // Info grid
                            <div class="detail-grid">
                                <div class="card">
                                    <div class="section-title">"Basic Information"</div>
                                    <InfoRow label="Name" value=m.name.clone() />
                                    <InfoRow label="Asset Number" value=m.asset_number.clone().unwrap_or_else(|| "-".to_string()) />
                                    <InfoRow label="Model" value=m.model.clone().unwrap_or_else(|| "-".to_string()) />
                                    <InfoRow label="Serial Number" value=m.serial_number.clone().unwrap_or_else(|| "-".to_string()) />
                                </div>
                                <div class="card">
                                    <div class="section-title">"Classification"</div>
                                    <InfoRow label="Type" value=m.machine_type_name.clone().unwrap_or_else(|| "-".to_string()) />
                                    <InfoRow label="Manufacturer" value=m.manufacturer_name.clone().unwrap_or_else(|| "-".to_string()) />
                                    <InfoRow label="Responsible" value=m.responsible.clone().unwrap_or_else(|| "-".to_string()) />
                                    <InfoRow label="Project" value=m.project_name.clone().unwrap_or_else(|| "-".to_string()) />
                                </div>
                            </div>

                            // Location card
                            <div class="card">
                                <div class="section-title">"Location"</div>
                                <div style="display: flex; gap: 2rem;">
                                    <div>
                                        <span style="color: var(--color-text-muted); font-size: 0.75rem;">"Area"</span>
                                        <div style="font-weight: 500;">{m.location_area.clone().or(m.area.clone()).unwrap_or_else(|| "-".to_string())}</div>
                                    </div>
                                    <div>
                                        <span style="color: var(--color-text-muted); font-size: 0.75rem;">"Line"</span>
                                        <div style="font-weight: 500;">{m.location_line.clone().or(m.line.clone()).unwrap_or_else(|| "-".to_string())}</div>
                                    </div>
                                </div>
                            </div>

                            // Pipeline visualization
                            {move || pipeline.get().map(|maybe_p| {
                                maybe_p.map(|p| {
                                    let stages_view: Vec<_> = p.stages.into_iter().map(|s| {
                                        let css = stage_css(&s.status).to_string();
                                        let label = stage_label(&s.stage).to_string();
                                        let status_char = match s.status.as_str() {
                                            "completed" => "✓",
                                            "in_progress" => "●",
                                            _ => "○",
                                        };
                                        view! {
                                            <div class=css>
                                                <div class="pipeline-node-circle">{status_char}</div>
                                                <div class="pipeline-node-label">{label}</div>
                                            </div>
                                        }
                                    }).collect();
                                    let release_level = p.release_level.to_string();
                                    view! {
                                        <div class="card pipeline-section">
                                            <div class="section-title">"Pipeline"</div>
                                            <div class="pipeline-track">
                                                {stages_view}
                                            </div>
                                            <div class="pipeline-total">
                                                "Release Level: " <strong>{release_level}</strong>
                                            </div>
                                        </div>
                                    }
                                })
                            })}

                            // Purchase RFQs
                            <div class="card">
                                <div class="section-title">"Purchase / RFQ"</div>
                                <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
                                    {move || rfqs.get().map(|rfq_list| {
                                        if rfq_list.is_empty() {
                                            view! { <p style="color: var(--color-text-muted); font-size: 0.8125rem;">"No purchase/RFQ records."</p> }.into_view()
                                        } else {
                                            view! {
                                                <div class="table-container">
                                                    <table>
                                                        <thead>
                                                            <tr>
                                                                <th>"RFQ #"</th>
                                                                <th>"Purchase Order"</th>
                                                                <th>"Tooling"</th>
                                                                <th>"Notes"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            {rfq_list.into_iter().map(|r| {
                                                                let rfq = r.rfq_number.clone().unwrap_or_else(|| "-".to_string());
                                                                let po = r.purchase_order.clone().unwrap_or_else(|| "-".to_string());
                                                                let tooling = if r.tooling_agreement {
                                                                    r.tooling_number.clone().unwrap_or_else(|| "Yes".to_string())
                                                                } else {
                                                                    "No".to_string()
                                                                };
                                                                let notes = r.notes.clone().unwrap_or_else(|| "-".to_string());
                                                                view! {
                                                                    <tr>
                                                                        <td class="font-mono">{rfq}</td>
                                                                        <td class="font-mono">{po}</td>
                                                                        <td>{tooling}</td>
                                                                        <td>{notes}</td>
                                                                    </tr>
                                                                }
                                                            }).collect_view()}
                                                        </tbody>
                                                    </table>
                                                </div>
                                            }.into_view()
                                        }
                                    })}
                                </Suspense>
                            </div>

                            // Delete modal
                            <Show when=move || show_delete.get()>
                                <ConfirmModal
                                    title="Delete Machine".to_string()
                                    message={
                                        let name = machine_name_for_modal.clone();
                                        move || format!("Are you sure you want to deactivate \"{}\"?", name)
                                    }
                                    confirm_label="Delete".to_string()
                                    on_confirm=Callback::new({
                                        let nav_del = nav_del.clone();
                                        move |_: ()| {
                                            let nav_del = nav_del.clone();
                                            spawn_local(async move {
                                                if let Err(e) = api::delete_req(&format!("/machines/{machine_id}")).await {
                                                    web_sys::window()
                                                        .and_then(|w| w.alert_with_message(&format!("Delete failed: {e}")).ok());
                                                } else {
                                                    nav_del("/machines", Default::default());
                                                }
                                            });
                                        }
                                    })
                                    on_cancel=Callback::new(move |_: ()| set_show_delete.set(false))
                                />
                            </Show>
                        }.into_view()
                    }
                })
            }}
        </Suspense>
    }
}

#[component]
fn InfoRow(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: space-between; padding: 0.375rem 0; border-bottom: 1px solid var(--color-border-light);">
            <span style="color: var(--color-text-muted); font-size: 0.8125rem;">{label}</span>
            <span style="font-size: 0.8125rem; font-weight: 500;">{value}</span>
        </div>
    }
}
