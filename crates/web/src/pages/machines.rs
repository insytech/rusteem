use leptos::*;
use shared::dto::machines::CreateMachineRequest;
use shared::dto::pagination::PaginatedResponse;
use shared::Machine;

use crate::api;

#[component]
pub fn MachinesPage() -> impl IntoView {
    let (refresh_counter, set_refresh) = create_signal(0u32);
    let (machines, set_machines) = create_signal(Vec::<Machine>::new());
    let (next_cursor, set_next_cursor) = create_signal(Option::<String>::None);
    let (total, set_total) = create_signal(0i64);
    let (loading_more, set_loading_more) = create_signal(false);

    let initial_load = create_resource(
        move || refresh_counter.get(),
        move |_| async move {
            let result = api::get::<PaginatedResponse<Machine>>("/machines?active=true").await;
            if let Ok(ref page) = result {
                set_machines.set(page.items.clone());
                set_next_cursor.set(page.next_cursor.clone());
                set_total.set(page.total);
            }
            result
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
            spawn_local(async move {
                let url = format!("/machines?active=true&cursor={cursor}");
                match api::get::<PaginatedResponse<Machine>>(&url).await {
                    Ok(page) => {
                        set_machines.update(|list| list.extend(page.items));
                        set_next_cursor.set(page.next_cursor);
                        set_total.set(page.total);
                    }
                    Err(e) => {
                        web_sys::window()
                            .and_then(|w| w.alert_with_message(&format!("Load more failed: {e}")).ok());
                    }
                }
                set_loading_more.set(false);
            });
        }
    };

    let (show_form, set_show_form) = create_signal(false);

    view! {
        <div>
            <div class="page-header">
                <h2>"Machines"</h2>
                <button class="btn btn-primary" on:click=move |_| set_show_form.set(!show_form.get())>
                    {move || if show_form.get() { "Cancel" } else { "+ New Machine" }}
                </button>
            </div>

            <Show when=move || show_form.get()>
                <CreateMachineForm on_created=Callback::new(move |_: ()| {
                    set_show_form.set(false);
                    trigger_refresh();
                })/>
            </Show>

            <Suspense fallback=move || view! { <p class="loading">"Loading machines..."</p> }>
                {move || initial_load.get().map(|result| match result {
                    Ok(_) => view! {
                        <MachineTable
                            machines=machines.get()
                            on_deleted=Callback::new(move |_: ()| trigger_refresh())
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
        </div>
    }
}

#[component]
fn MachineTable(
    machines: Vec<Machine>,
    on_deleted: Callback<()>,
) -> impl IntoView {
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
                        <th>"Area"</th>
                        <th>"Line"</th>
                        <th>"Station"</th>
                        <th>"Status"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {machines.into_iter().map(|m| {
                        let id = m.id;
                        let name = m.name.clone();
                        let asset = m.asset_number.clone().unwrap_or_else(|| "-".to_string());
                        let area = m.area.clone().unwrap_or_else(|| "-".to_string());
                        let line = m.line.clone().unwrap_or_else(|| "-".to_string());
                        let station = m.station.clone().unwrap_or_else(|| "-".to_string());
                        let active = m.active;
                        view! {
                            <tr>
                                <td>{name}</td>
                                <td>{asset}</td>
                                <td>{area}</td>
                                <td>{line}</td>
                                <td>{station}</td>
                                <td>
                                    <span class={if active { "badge badge-approved" } else { "badge badge-rejected" }}>
                                        {if active { "Active" } else { "Inactive" }}
                                    </span>
                                </td>
                                <td>
                                    <button class="btn btn-danger btn-sm"
                                        on:click=move |_| {
                                            spawn_local(async move {
                                                if let Err(e) = api::delete_req(&format!("/machines/{id}")).await {
                                                    web_sys::window()
                                                        .and_then(|w| w.alert_with_message(&format!("Delete failed: {e}")).ok());
                                                } else {
                                                    on_deleted.call(());
                                                }
                                            });
                                        }
                                    >
                                        "Delete"
                                    </button>
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

#[component]
fn CreateMachineForm(on_created: Callback<()>) -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (asset_number, set_asset_number) = create_signal(String::new());
    let (area, set_area) = create_signal(String::new());
    let (line, set_line) = create_signal(String::new());
    let (station, set_station) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);
    let (submitting, set_submitting) = create_signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error.set(None);

        let req = CreateMachineRequest {
            name: name.get(),
            asset_number: Some(asset_number.get()).filter(|s| !s.is_empty()),
            area: Some(area.get()).filter(|s| !s.is_empty()),
            line: Some(line.get()).filter(|s| !s.is_empty()),
            station: Some(station.get()).filter(|s| !s.is_empty()),
        };

        spawn_local(async move {
            match api::post::<Machine, _>("/machines", &req).await {
                Ok(_) => on_created.call(()),
                Err(e) => set_error.set(Some(format!("Failed to create: {e}"))),
            }
            set_submitting.set(false);
        });
    };

    view! {
        <div class="card" style="margin-bottom: 1rem;">
            <h3 style="margin-bottom: 1rem;">"New Machine"</h3>

            <Show when=move || error.get().is_some()>
                <div class="error-message">{move || error.get().unwrap_or_default()}</div>
            </Show>

            <form on:submit=on_submit>
                <div class="form-group">
                    <label>"Name *"</label>
                    <input type="text" required
                        prop:value=name
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                    />
                </div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                    <div class="form-group">
                        <label>"Asset Number"</label>
                        <input type="text"
                            prop:value=asset_number
                            on:input=move |ev| set_asset_number.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label>"Area"</label>
                        <input type="text"
                            prop:value=area
                            on:input=move |ev| set_area.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label>"Line"</label>
                        <input type="text"
                            prop:value=line
                            on:input=move |ev| set_line.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label>"Station"</label>
                        <input type="text"
                            prop:value=station
                            on:input=move |ev| set_station.set(event_target_value(&ev))
                        />
                    </div>
                </div>
                <button class="btn btn-primary" type="submit" disabled=move || submitting.get()>
                    {move || if submitting.get() { "Creating..." } else { "Create Machine" }}
                </button>
            </form>
        </div>
    }
}
