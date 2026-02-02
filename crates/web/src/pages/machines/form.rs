use leptos::*;
use shared::dto::machines::{CreateMachineRequest, MachineDetail, UpdateMachineRequest};
use shared::dto::purchase_rfqs::CreatePurchaseRfqRequest;
use shared::{Location, Machine, MachineType, Manufacturer, Project, PurchaseRfq};

use crate::api;

#[component]
pub fn MachineForm(
    machine: Option<MachineDetail>,
    on_saved: Callback<()>,
) -> impl IntoView {
    let is_edit = machine.is_some();
    let machine_id = machine.as_ref().map(|m| m.id);

    let (name, set_name) = create_signal(machine.as_ref().map(|m| m.name.clone()).unwrap_or_default());
    let (asset_number, set_asset_number) = create_signal(machine.as_ref().and_then(|m| m.asset_number.clone()).unwrap_or_default());
    let (model, set_model) = create_signal(machine.as_ref().and_then(|m| m.model.clone()).unwrap_or_default());
    let (serial_number, set_serial_number) = create_signal(machine.as_ref().and_then(|m| m.serial_number.clone()).unwrap_or_default());
    let (machine_type_id, set_machine_type_id) = create_signal(machine.as_ref().and_then(|m| m.machine_type_id).map(|id| id.to_string()).unwrap_or_default());
    let (manufacturer_id, set_manufacturer_id) = create_signal(machine.as_ref().and_then(|m| m.manufacturer_id).map(|id| id.to_string()).unwrap_or_default());
    let (location_id, set_location_id) = create_signal(machine.as_ref().and_then(|m| m.location_id).map(|id| id.to_string()).unwrap_or_default());
    let (project_id, set_project_id) = create_signal(machine.as_ref().and_then(|m| m.project_id).map(|id| id.to_string()).unwrap_or_default());
    let (responsible, set_responsible) = create_signal(machine.as_ref().and_then(|m| m.responsible.clone()).unwrap_or_default());
    let (error, set_error) = create_signal(Option::<String>::None);
    let (submitting, set_submitting) = create_signal(false);

    // Reference data
    let machine_types = create_resource(|| (), |_| async {
        api::get::<Vec<MachineType>>("/machine-types").await.unwrap_or_default()
    });
    let manufacturers = create_resource(|| (), |_| async {
        api::get::<Vec<Manufacturer>>("/manufacturers").await.unwrap_or_default()
    });
    let locations = create_resource(|| (), |_| async {
        api::get::<Vec<Location>>("/locations").await.unwrap_or_default()
    });
    let projects = create_resource(|| (), |_| async {
        api::get::<Vec<Project>>("/projects").await.unwrap_or_default()
    });

    // Purchase RFQs (edit mode only)
    let rfqs = create_resource(
        move || machine_id,
        move |mid| async move {
            match mid {
                Some(id) => api::get::<Vec<PurchaseRfq>>(&format!("/machines/{id}/purchase-rfqs")).await.unwrap_or_default(),
                None => vec![],
            }
        },
    );

    let parse_uuid = |s: &str| -> Option<uuid::Uuid> {
        if s.is_empty() { None } else { s.parse().ok() }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error.set(None);

        let name_val = name.get();
        let asset_val = Some(asset_number.get()).filter(|s| !s.is_empty());
        let model_val = Some(model.get()).filter(|s| !s.is_empty());
        let serial_val = Some(serial_number.get()).filter(|s| !s.is_empty());
        let type_val = parse_uuid(&machine_type_id.get());
        let mfr_val = parse_uuid(&manufacturer_id.get());
        let loc_val = parse_uuid(&location_id.get());
        let proj_val = parse_uuid(&project_id.get());
        let resp_val = Some(responsible.get()).filter(|s| !s.is_empty());

        if is_edit {
            let id = machine_id.unwrap();
            let req = UpdateMachineRequest {
                name: Some(name_val),
                asset_number: asset_val,
                line: None,
                station: None,
                area: None,
                active: None,
                model: model_val,
                serial_number: serial_val,
                machine_type_id: type_val,
                manufacturer_id: mfr_val,
                location_id: loc_val,
                project_id: proj_val,
                responsible: resp_val,
            };
            spawn_local(async move {
                match api::put::<Machine, _>(&format!("/machines/{id}"), &req).await {
                    Ok(_) => on_saved.call(()),
                    Err(e) => set_error.set(Some(format!("Failed to update: {e}"))),
                }
                set_submitting.set(false);
            });
        } else {
            let req = CreateMachineRequest {
                name: name_val,
                asset_number: asset_val,
                line: None,
                station: None,
                area: None,
                model: model_val,
                serial_number: serial_val,
                machine_type_id: type_val,
                manufacturer_id: mfr_val,
                location_id: loc_val,
                project_id: proj_val,
                responsible: resp_val,
            };
            spawn_local(async move {
                match api::post::<Machine, _>("/machines", &req).await {
                    Ok(_) => on_saved.call(()),
                    Err(e) => set_error.set(Some(format!("Failed to create: {e}"))),
                }
                set_submitting.set(false);
            });
        }
    };

    // Add RFQ handler
    let (rfq_number, set_rfq_number) = create_signal(String::new());
    let (rfq_po, set_rfq_po) = create_signal(String::new());

    view! {
        <div class="card" style="margin-bottom: 1rem;">
            <h3 style="margin-bottom: 1rem;">{if is_edit { "Edit Machine" } else { "New Machine" }}</h3>

            <Show when=move || error.get().is_some()>
                <div class="error-message">{move || error.get().unwrap_or_default()}</div>
            </Show>

            <form on:submit=on_submit>
                // Section 1: Basic Info
                <div class="section-title">"Basic Information"</div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                    <div class="form-group">
                        <label>"Name *"</label>
                        <input type="text" required
                            prop:value=name
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label>"Asset Number"</label>
                        <input type="text"
                            prop:value=asset_number
                            on:input=move |ev| set_asset_number.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label>"Model"</label>
                        <input type="text"
                            prop:value=model
                            on:input=move |ev| set_model.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label>"Serial Number"</label>
                        <input type="text"
                            prop:value=serial_number
                            on:input=move |ev| set_serial_number.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                // Section 2: Classification
                <div class="section-title" style="margin-top: 1rem;">"Classification"</div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                    <div class="form-group">
                        <label>"Machine Type"</label>
                        <select
                            prop:value=machine_type_id
                            on:change=move |ev| set_machine_type_id.set(event_target_value(&ev))
                        >
                            <option value="">"— Select —"</option>
                            {move || machine_types.get().map(|types| {
                                types.into_iter().map(|t| {
                                    let id = t.id.to_string();
                                    let name = t.name;
                                    let selected = machine_type_id.get() == id;
                                    view! { <option value=id selected=selected>{name}</option> }
                                }).collect_view()
                            })}
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"Manufacturer"</label>
                        <select
                            prop:value=manufacturer_id
                            on:change=move |ev| set_manufacturer_id.set(event_target_value(&ev))
                        >
                            <option value="">"— Select —"</option>
                            {move || manufacturers.get().map(|mfrs| {
                                mfrs.into_iter().map(|m| {
                                    let id = m.id.to_string();
                                    let name = m.name;
                                    let selected = manufacturer_id.get() == id;
                                    view! { <option value=id selected=selected>{name}</option> }
                                }).collect_view()
                            })}
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"Location"</label>
                        <select
                            prop:value=location_id
                            on:change=move |ev| set_location_id.set(event_target_value(&ev))
                        >
                            <option value="">"— Select —"</option>
                            {move || locations.get().map(|locs| {
                                locs.into_iter().map(|l| {
                                    let id = l.id.to_string();
                                    let label = format!("{} — {}", l.area, l.line);
                                    let selected = location_id.get() == id;
                                    view! { <option value=id selected=selected>{label}</option> }
                                }).collect_view()
                            })}
                        </select>
                    </div>
                    <div class="form-group">
                        <label>"Project"</label>
                        <select
                            prop:value=project_id
                            on:change=move |ev| set_project_id.set(event_target_value(&ev))
                        >
                            <option value="">"— Select —"</option>
                            {move || projects.get().map(|projs| {
                                projs.into_iter().map(|p| {
                                    let id = p.id.to_string();
                                    let label = match p.code {
                                        Some(ref code) => format!("{} ({})", p.name, code),
                                        None => p.name.clone(),
                                    };
                                    let selected = project_id.get() == id;
                                    view! { <option value=id selected=selected>{label}</option> }
                                }).collect_view()
                            })}
                        </select>
                    </div>
                </div>

                // Section 3: Responsibility
                <div class="section-title" style="margin-top: 1rem;">"Responsibility"</div>
                <div class="form-group">
                    <label>"Responsible Person"</label>
                    <input type="text"
                        prop:value=responsible
                        on:input=move |ev| set_responsible.set(event_target_value(&ev))
                    />
                </div>

                <button class="btn btn-primary" type="submit" disabled=move || submitting.get() style="margin-top: 1rem;">
                    {move || if submitting.get() { "Saving..." } else if is_edit { "Update Machine" } else { "Create Machine" }}
                </button>
            </form>

            // Section 4: Purchase RFQs (edit mode only)
            <Show when=move || is_edit>
                <div class="section-title" style="margin-top: 1.5rem;">"Purchase / RFQ"</div>
                <Suspense fallback=move || view! { <p class="loading">"Loading RFQs..."</p> }>
                    {move || rfqs.get().map(|rfq_list| {
                        if rfq_list.is_empty() {
                            view! { <p style="color: var(--color-text-muted); font-size: 0.8125rem;">"No purchase/RFQ records yet."</p> }.into_view()
                        } else {
                            view! {
                                <div class="table-container" style="margin-bottom: 0.75rem;">
                                    <table>
                                        <thead>
                                            <tr>
                                                <th>"RFQ #"</th>
                                                <th>"PO #"</th>
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
                // Quick add RFQ
                <div style="display: flex; gap: 0.5rem; align-items: flex-end;">
                    <div class="form-group" style="margin-bottom: 0;">
                        <label>"RFQ #"</label>
                        <input type="text" prop:value=rfq_number on:input=move |ev| set_rfq_number.set(event_target_value(&ev)) />
                    </div>
                    <div class="form-group" style="margin-bottom: 0;">
                        <label>"PO #"</label>
                        <input type="text" prop:value=rfq_po on:input=move |ev| set_rfq_po.set(event_target_value(&ev)) />
                    </div>
                    <button class="btn btn-outline" type="button" style="margin-bottom: 0;"
                        on:click=move |_| {
                            let mid = machine_id.unwrap();
                            let rfq_req = CreatePurchaseRfqRequest {
                                rfq_number: Some(rfq_number.get()).filter(|s| !s.is_empty()),
                                purchase_order: Some(rfq_po.get()).filter(|s| !s.is_empty()),
                                tooling_agreement: None,
                                tooling_number: None,
                                notes: None,
                            };
                            spawn_local(async move {
                                match api::post::<PurchaseRfq, _>(&format!("/machines/{mid}/purchase-rfqs"), &rfq_req).await {
                                    Ok(_) => {
                                        set_rfq_number.set(String::new());
                                        set_rfq_po.set(String::new());
                                        rfqs.refetch();
                                    }
                                    Err(e) => {
                                        web_sys::window()
                                            .and_then(|w| w.alert_with_message(&format!("Add RFQ failed: {e}")).ok());
                                    }
                                }
                            });
                        }
                    >"+ Add RFQ"</button>
                </div>
            </Show>
        </div>
    }
}
