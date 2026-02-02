use leptos::*;
use shared::dto::machines::{MachineDetail, MachineStats};
use shared::dto::pagination::PaginatedResponse;
use shared::{Location, MachineType, Manufacturer, TeamMember};

use crate::api;
use crate::components::confirm_modal::ConfirmModal;
use crate::components::toast::use_toast;

#[component]
pub fn MachinesPage() -> impl IntoView {
    let toast = use_toast();
    let (refresh_counter, set_refresh) = create_signal(0u32);
    let (machines, set_machines) = create_signal(Vec::<MachineDetail>::new());
    let (next_cursor, set_next_cursor) = create_signal(Option::<String>::None);
    let (total, set_total) = create_signal(0i64);
    let (loading_more, set_loading_more) = create_signal(false);

    // Filter state
    let (search, set_search) = create_signal(String::new());
    let (filter_type, set_filter_type) = create_signal(String::new());
    let (filter_manufacturer, set_filter_manufacturer) = create_signal(String::new());
    let (filter_location, set_filter_location) = create_signal(String::new());
    let (filter_responsible, set_filter_responsible) = create_signal(String::new());

    // Reference data for dropdowns
    let machine_types = create_resource(|| (), |_| async {
        api::get::<Vec<MachineType>>("/machine-types").await.unwrap_or_default()
    });
    let manufacturers = create_resource(|| (), |_| async {
        api::get::<Vec<Manufacturer>>("/manufacturers").await.unwrap_or_default()
    });
    let locations = create_resource(|| (), |_| async {
        api::get::<Vec<Location>>("/locations").await.unwrap_or_default()
    });
    let team_members = create_resource(|| (), |_| async {
        api::get::<Vec<TeamMember>>("/team-members").await.unwrap_or_default()
    });

    // Server-side stats
    let stats = create_resource(
        move || refresh_counter.get(),
        |_| async { api::get::<MachineStats>("/machines/stats").await.ok() },
    );

    let build_query = move || -> String {
        let mut params = vec!["active=true".to_string()];
        let s = search.get();
        if !s.is_empty() {
            params.push(format!("search={s}"));
        }
        let t = filter_type.get();
        if !t.is_empty() {
            params.push(format!("machine_type_id={t}"));
        }
        let mfr = filter_manufacturer.get();
        if !mfr.is_empty() {
            params.push(format!("manufacturer_id={mfr}"));
        }
        let loc = filter_location.get();
        if !loc.is_empty() {
            params.push(format!("location_id={loc}"));
        }
        let resp = filter_responsible.get();
        if !resp.is_empty() {
            params.push(format!("responsible_id={resp}"));
        }
        format!("/machines?{}", params.join("&"))
    };

    let initial_load = create_resource(
        move || (refresh_counter.get(), search.get(), filter_type.get(), filter_manufacturer.get(), filter_location.get(), filter_responsible.get()),
        move |_| {
            let url = build_query();
            async move {
                let result = api::get::<PaginatedResponse<MachineDetail>>(&url).await;
                if let Ok(ref page) = result {
                    set_machines.set(page.items.clone());
                    set_next_cursor.set(page.next_cursor.clone());
                    set_total.set(page.total);
                }
                result
            }
        },
    );

    let trigger_refresh = move || {
        set_machines.set(vec![]);
        set_next_cursor.set(None);
        set_refresh.update(|c| *c += 1);
    };

    let load_more = move |_| {
        if let Some(cursor) = next_cursor.get() {
            set_loading_more.set(true);
            let url = format!("{}&cursor={cursor}", build_query());
            spawn_local(async move {
                match api::get::<PaginatedResponse<MachineDetail>>(&url).await {
                    Ok(page) => {
                        set_machines.update(|list| list.extend(page.items));
                        set_next_cursor.set(page.next_cursor);
                        set_total.set(page.total);
                    }
                    Err(e) => {
                        toast.error(&format!("Load more failed: {e}"));
                    }
                }
                set_loading_more.set(false);
            });
        }
    };

    let (show_form, set_show_form) = create_signal(false);

    // Delete confirmation modal state
    let (delete_target, set_delete_target) = create_signal(Option::<(uuid::Uuid, String)>::None);

    let on_confirm_delete = move || {
        if let Some((id, _)) = delete_target.get() {
            spawn_local(async move {
                if let Err(e) = api::delete_req(&format!("/machines/{id}")).await {
                    toast.error(&format!("Delete failed: {e}"));
                } else {
                    trigger_refresh();
                }
                set_delete_target.set(None);
            });
        }
    };

    view! {
        <div>
            <div class="page-header">
                <h2>"Machines"</h2>
                <button class="btn btn-primary" on:click=move |_| set_show_form.set(!show_form.get())>
                    {move || if show_form.get() { "Cancel" } else { "+ New Machine" }}
                </button>
            </div>

            // Stat cards (server-side)
            <div class="stats-grid">
                {move || stats.get().map(|maybe_s: Option<MachineStats>| {
                    let s = maybe_s.unwrap_or(MachineStats { total: 0, active: 0, by_type: vec![], by_area: vec![] });
                    view! {
                        <div class="stat-card">
                            <div class="stat-value">{s.total}</div>
                            <div class="stat-label">"Total Machines"</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-value">{s.active}</div>
                            <div class="stat-label">"Active"</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-value">{s.by_area.len()}</div>
                            <div class="stat-label">"Areas"</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-value">{s.by_type.len()}</div>
                            <div class="stat-label">"Types"</div>
                        </div>
                    }
                })}
            </div>

            // Filter bar
            <div class="filter-bar">
                <input
                    type="text"
                    placeholder="Search machines..."
                    class="filter-search"
                    prop:value=search
                    on:input=move |ev| set_search.set(event_target_value(&ev))
                />
                <select
                    class="filter-select"
                    on:change=move |ev| set_filter_type.set(event_target_value(&ev))
                >
                    <option value="">"All Types"</option>
                    {move || machine_types.get().map(|types| {
                        if types.is_empty() {
                            view! { <option value="" disabled>"No types yet"</option> }.into_view()
                        } else {
                            types.into_iter().map(|t| {
                                let id = t.id.to_string();
                                let name = t.name;
                                view! { <option value=id>{name}</option> }
                            }).collect_view()
                        }
                    })}
                </select>
                <select
                    class="filter-select"
                    on:change=move |ev| set_filter_manufacturer.set(event_target_value(&ev))
                >
                    <option value="">"All Manufacturers"</option>
                    {move || manufacturers.get().map(|mfrs| {
                        if mfrs.is_empty() {
                            view! { <option value="" disabled>"No manufacturers yet"</option> }.into_view()
                        } else {
                            mfrs.into_iter().map(|m| {
                                let id = m.id.to_string();
                                let name = m.name;
                                view! { <option value=id>{name}</option> }
                            }).collect_view()
                        }
                    })}
                </select>
                <select
                    class="filter-select"
                    on:change=move |ev| set_filter_location.set(event_target_value(&ev))
                >
                    <option value="">"All Locations"</option>
                    {move || locations.get().map(|locs| {
                        if locs.is_empty() {
                            view! { <option value="" disabled>"No locations yet"</option> }.into_view()
                        } else {
                            locs.into_iter().map(|l| {
                                let id = l.id.to_string();
                                let label = format!("{} — {}", l.area, l.line);
                                view! { <option value=id>{label}</option> }
                            }).collect_view()
                        }
                    })}
                </select>
                <select
                    class="filter-select"
                    on:change=move |ev| set_filter_responsible.set(event_target_value(&ev))
                >
                    <option value="">"All Responsible"</option>
                    {move || team_members.get().map(|members| {
                        if members.is_empty() {
                            view! { <option value="" disabled>"No team members yet"</option> }.into_view()
                        } else {
                            members.into_iter().map(|t| {
                                let id = t.id.to_string();
                                let name = t.name;
                                view! { <option value=id>{name}</option> }
                            }).collect_view()
                        }
                    })}
                </select>
            </div>

            <Show when=move || show_form.get()>
                <super::form::MachineForm
                    machine=None
                    on_saved=Callback::new(move |_: ()| {
                        set_show_form.set(false);
                        trigger_refresh();
                    })
                />
            </Show>

            <Suspense fallback=move || view! { <p class="loading">"Loading machines..."</p> }>
                {move || initial_load.get().map(|result| match result {
                    Ok(_) => view! {
                        <MachineTable
                            machines=machines.get()
                            on_delete=Callback::new(move |(id, name): (uuid::Uuid, String)| {
                                set_delete_target.set(Some((id, name)));
                            })
                            on_refresh=Callback::new(move |_: ()| trigger_refresh())
                        />
                        <Show when=move || next_cursor.get().is_some()>
                            <div style="text-align: center; margin: 1rem 0;">
                                <button
                                    class="btn btn-primary"
                                    on:click=load_more
                                    disabled=move || loading_more.get()
                                >
                                    {move || if loading_more.get() { "Loading..." } else { "Load More" }}
                                </button>
                                <p style="margin-top: 0.5rem; color: var(--color-muted);">
                                    {move || format!("Showing {} of {}", machines.get().len(), total.get())}
                                </p>
                            </div>
                        </Show>
                    }.into_view(),
                    Err(e) => view! {
                        <div class="error-message">{format!("Failed to load machines: {e}")}</div>
                    }.into_view(),
                })}
            </Suspense>

            // Delete confirmation modal
            <Show when=move || delete_target.get().is_some()>
                <ConfirmModal
                    title="Delete Machine".to_string()
                    message=move || {
                        delete_target.get()
                            .map(|(_, name)| format!("Are you sure you want to deactivate \"{}\"? This action can be reversed.", name))
                            .unwrap_or_default()
                    }
                    confirm_label="Delete".to_string()
                    on_confirm=Callback::new(move |_: ()| on_confirm_delete())
                    on_cancel=Callback::new(move |_: ()| set_delete_target.set(None))
                />
            </Show>
        </div>
    }
}

#[component]
fn MachineTable(
    machines: Vec<MachineDetail>,
    on_delete: Callback<(uuid::Uuid, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let toast = use_toast();

    if machines.is_empty() {
        return view! {
            <div class="card">
                <p class="empty-state">"No machines found. Create one to get started."</p>
            </div>
        }
        .into_view();
    }

    view! {
        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Asset #"</th>
                        <th>"Type"</th>
                        <th>"Location"</th>
                        <th>"Responsible"</th>
                        <th>"Status"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {machines.into_iter().map(|m| {
                        let id = m.id;
                        let name = m.name.clone();
                        let name_for_delete = m.name.clone();
                        let asset = m.asset_number.clone().unwrap_or_else(|| "-".to_string());
                        let type_name = m.machine_type_name.clone().unwrap_or_else(|| "-".to_string());
                        let location = match (&m.location_area, &m.location_line) {
                            (Some(area), Some(line)) => format!("{area} — {line}"),
                            _ => m.area.clone().unwrap_or_else(|| "-".to_string()),
                        };
                        let responsible = m.responsible_name.clone()
                            .unwrap_or_else(|| m.responsible.clone().unwrap_or_else(|| "-".to_string()));
                        let active = m.active;
                        let detail_href = format!("/machines/{id}");
                        let edit_href = format!("/machines/{id}/edit");
                        let toast_dup = toast;
                        view! {
                            <tr>
                                <td><a href=detail_href.clone() style="color: var(--color-text); font-weight: 500;">{name}</a></td>
                                <td class="font-mono">{asset}</td>
                                <td>{type_name}</td>
                                <td class="machine-location">{location}</td>
                                <td>{responsible}</td>
                                <td>
                                    <span class={if active { "badge badge-approved" } else { "badge badge-rejected" }}>
                                        {if active { "Active" } else { "Inactive" }}
                                    </span>
                                </td>
                                <td>
                                    <div class="actions-cell">
                                        // View icon (eye)
                                        <a href=detail_href class="btn btn-outline btn-icon" title="View">
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                                                <circle cx="12" cy="12" r="3"/>
                                            </svg>
                                        </a>
                                        // Edit icon (pencil)
                                        <a href=edit_href class="btn btn-outline btn-icon" title="Edit">
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                                                <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                                            </svg>
                                        </a>
                                        // Duplicate icon (copy)
                                        <button class="btn btn-outline btn-icon" title="Duplicate"
                                            on:click=move |_| {
                                                let on_refresh = on_refresh.clone();
                                                let toast = toast_dup;
                                                spawn_local(async move {
                                                    match api::post::<shared::Machine, ()>(&format!("/machines/{id}/duplicate"), &()).await {
                                                        Ok(_) => on_refresh.call(()),
                                                        Err(e) => {
                                                            toast.error(&format!("Duplicate failed: {e}"));
                                                        }
                                                    }
                                                });
                                            }
                                        >
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                            </svg>
                                        </button>
                                        // Delete icon (trash)
                                        <button class="btn btn-danger btn-icon" title="Delete"
                                            on:click={
                                                let name_for_delete = name_for_delete.clone();
                                                move |_| {
                                                    on_delete.call((id, name_for_delete.clone()));
                                                }
                                            }
                                        >
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                <polyline points="3 6 5 6 21 6"/>
                                                <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
                                            </svg>
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
    .into_view()
}
