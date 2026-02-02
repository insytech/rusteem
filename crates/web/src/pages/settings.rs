use leptos::*;
use std::collections::HashMap;

use shared::{Location, MachineType, Manufacturer, Project, TeamMember};

use crate::api;
use crate::components::inline_crud_table::{ColumnDef, InlineCrudTable, RowData};

#[component]
pub fn SettingsPage() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("team_members");
    let (refresh, set_refresh) = create_signal(0u32);
    let trigger_refresh = move || set_refresh.update(|c| *c += 1);

    let team_members = create_resource(
        move || refresh.get(),
        |_| async { api::get::<Vec<TeamMember>>("/team-members").await.unwrap_or_default() },
    );
    let machine_types = create_resource(
        move || refresh.get(),
        |_| async { api::get::<Vec<MachineType>>("/machine-types").await.unwrap_or_default() },
    );
    let manufacturers = create_resource(
        move || refresh.get(),
        |_| async { api::get::<Vec<Manufacturer>>("/manufacturers").await.unwrap_or_default() },
    );
    let locations = create_resource(
        move || refresh.get(),
        |_| async { api::get::<Vec<Location>>("/locations").await.unwrap_or_default() },
    );
    let projects = create_resource(
        move || refresh.get(),
        |_| async { api::get::<Vec<Project>>("/projects").await.unwrap_or_default() },
    );

    view! {
        <div>
            <div class="page-header">
                <h2>"Settings"</h2>
            </div>

            // Tab bar
            <div class="tabs">
                <button
                    class=move || if active_tab.get() == "team_members" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("team_members")
                >"Team Members"</button>
                <button
                    class=move || if active_tab.get() == "machine_types" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("machine_types")
                >"Machine Types"</button>
                <button
                    class=move || if active_tab.get() == "manufacturers" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("manufacturers")
                >"Manufacturers"</button>
                <button
                    class=move || if active_tab.get() == "locations" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("locations")
                >"Locations"</button>
                <button
                    class=move || if active_tab.get() == "projects" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("projects")
                >"Projects"</button>
            </div>

            <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
                // Team Members tab
                <Show when=move || active_tab.get() == "team_members">
                    {move || team_members.get().map(|items| {
                        let rows: Vec<RowData> = items.iter().map(|t| {
                            let mut values = HashMap::new();
                            values.insert("name".to_string(), t.name.clone());
                            values.insert("email".to_string(), t.email.clone());
                            values.insert("role".to_string(), t.role.clone().unwrap_or_default());
                            values.insert("department".to_string(), t.department.clone().unwrap_or_default());
                            RowData { id: t.id.to_string(), values, active: t.active }
                        }).collect();

                        view! {
                            <InlineCrudTable
                                title="Team Members"
                                columns=vec![
                                    ColumnDef { key: "name", label: "Name", required: true },
                                    ColumnDef { key: "email", label: "Email", required: true },
                                    ColumnDef { key: "role", label: "Role", required: false },
                                    ColumnDef { key: "department", label: "Department", required: false },
                                ]
                                items=rows
                                on_create=Callback::new(move |vals: HashMap<String, String>| {
                                    spawn_local(async move {
                                        let _ = api::post::<TeamMember, _>("/team-members", &serde_json::json!({
                                            "name": vals.get("name").cloned().unwrap_or_default(),
                                            "email": vals.get("email").cloned().unwrap_or_default(),
                                            "role": vals.get("role").cloned(),
                                            "department": vals.get("department").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_update=Callback::new(move |(id, vals): (String, HashMap<String, String>)| {
                                    spawn_local(async move {
                                        let _ = api::put::<TeamMember, _>(&format!("/team-members/{id}"), &serde_json::json!({
                                            "name": vals.get("name").cloned(),
                                            "email": vals.get("email").cloned(),
                                            "role": vals.get("role").cloned(),
                                            "department": vals.get("department").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_toggle_active=Callback::new(move |(id, active): (String, bool)| {
                                    spawn_local(async move {
                                        let _ = api::put::<TeamMember, _>(&format!("/team-members/{id}"), &serde_json::json!({
                                            "active": active,
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                            />
                        }
                    })}
                </Show>

                // Machine Types tab
                <Show when=move || active_tab.get() == "machine_types">
                    {move || machine_types.get().map(|items| {
                        let rows: Vec<RowData> = items.iter().map(|t| {
                            let mut values = HashMap::new();
                            values.insert("name".to_string(), t.name.clone());
                            values.insert("description".to_string(), t.description.clone().unwrap_or_default());
                            RowData { id: t.id.to_string(), values, active: t.active }
                        }).collect();

                        view! {
                            <InlineCrudTable
                                title="Machine Types"
                                columns=vec![
                                    ColumnDef { key: "name", label: "Name", required: true },
                                    ColumnDef { key: "description", label: "Description", required: false },
                                ]
                                items=rows
                                on_create=Callback::new(move |vals: HashMap<String, String>| {
                                    spawn_local(async move {
                                        let _ = api::post::<MachineType, _>("/machine-types", &serde_json::json!({
                                            "name": vals.get("name").cloned().unwrap_or_default(),
                                            "description": vals.get("description").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_update=Callback::new(move |(id, vals): (String, HashMap<String, String>)| {
                                    spawn_local(async move {
                                        let _ = api::put::<MachineType, _>(&format!("/machine-types/{id}"), &serde_json::json!({
                                            "name": vals.get("name").cloned(),
                                            "description": vals.get("description").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_toggle_active=Callback::new(move |(id, active): (String, bool)| {
                                    spawn_local(async move {
                                        let _ = api::put::<MachineType, _>(&format!("/machine-types/{id}"), &serde_json::json!({
                                            "active": active,
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                            />
                        }
                    })}
                </Show>

                // Manufacturers tab
                <Show when=move || active_tab.get() == "manufacturers">
                    {move || manufacturers.get().map(|items| {
                        let rows: Vec<RowData> = items.iter().map(|m| {
                            let mut values = HashMap::new();
                            values.insert("name".to_string(), m.name.clone());
                            values.insert("website".to_string(), m.website.clone().unwrap_or_default());
                            values.insert("contact_email".to_string(), m.contact_email.clone().unwrap_or_default());
                            RowData { id: m.id.to_string(), values, active: m.active }
                        }).collect();

                        view! {
                            <InlineCrudTable
                                title="Manufacturers"
                                columns=vec![
                                    ColumnDef { key: "name", label: "Name", required: true },
                                    ColumnDef { key: "website", label: "Website", required: false },
                                    ColumnDef { key: "contact_email", label: "Contact Email", required: false },
                                ]
                                items=rows
                                on_create=Callback::new(move |vals: HashMap<String, String>| {
                                    spawn_local(async move {
                                        let _ = api::post::<Manufacturer, _>("/manufacturers", &serde_json::json!({
                                            "name": vals.get("name").cloned().unwrap_or_default(),
                                            "website": vals.get("website").cloned(),
                                            "contact_email": vals.get("contact_email").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_update=Callback::new(move |(id, vals): (String, HashMap<String, String>)| {
                                    spawn_local(async move {
                                        let _ = api::put::<Manufacturer, _>(&format!("/manufacturers/{id}"), &serde_json::json!({
                                            "name": vals.get("name").cloned(),
                                            "website": vals.get("website").cloned(),
                                            "contact_email": vals.get("contact_email").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_toggle_active=Callback::new(move |(id, active): (String, bool)| {
                                    spawn_local(async move {
                                        let _ = api::put::<Manufacturer, _>(&format!("/manufacturers/{id}"), &serde_json::json!({
                                            "active": active,
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                            />
                        }
                    })}
                </Show>

                // Locations tab
                <Show when=move || active_tab.get() == "locations">
                    {move || locations.get().map(|items| {
                        let rows: Vec<RowData> = items.iter().map(|l| {
                            let mut values = HashMap::new();
                            values.insert("area".to_string(), l.area.clone());
                            values.insert("line".to_string(), l.line.clone());
                            RowData { id: l.id.to_string(), values, active: l.active }
                        }).collect();

                        view! {
                            <InlineCrudTable
                                title="Locations"
                                columns=vec![
                                    ColumnDef { key: "area", label: "Area", required: true },
                                    ColumnDef { key: "line", label: "Line", required: true },
                                ]
                                items=rows
                                on_create=Callback::new(move |vals: HashMap<String, String>| {
                                    spawn_local(async move {
                                        let _ = api::post::<Location, _>("/locations", &serde_json::json!({
                                            "area": vals.get("area").cloned().unwrap_or_default(),
                                            "line": vals.get("line").cloned().unwrap_or_default(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_update=Callback::new(move |(id, vals): (String, HashMap<String, String>)| {
                                    spawn_local(async move {
                                        let _ = api::put::<Location, _>(&format!("/locations/{id}"), &serde_json::json!({
                                            "area": vals.get("area").cloned(),
                                            "line": vals.get("line").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_toggle_active=Callback::new(move |(id, active): (String, bool)| {
                                    spawn_local(async move {
                                        let _ = api::put::<Location, _>(&format!("/locations/{id}"), &serde_json::json!({
                                            "active": active,
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                            />
                        }
                    })}
                </Show>

                // Projects tab
                <Show when=move || active_tab.get() == "projects">
                    {move || projects.get().map(|items| {
                        let rows: Vec<RowData> = items.iter().map(|p| {
                            let mut values = HashMap::new();
                            values.insert("name".to_string(), p.name.clone());
                            values.insert("code".to_string(), p.code.clone().unwrap_or_default());
                            values.insert("description".to_string(), p.description.clone().unwrap_or_default());
                            RowData { id: p.id.to_string(), values, active: p.active }
                        }).collect();

                        view! {
                            <InlineCrudTable
                                title="Projects"
                                columns=vec![
                                    ColumnDef { key: "name", label: "Name", required: true },
                                    ColumnDef { key: "code", label: "Code", required: false },
                                    ColumnDef { key: "description", label: "Description", required: false },
                                ]
                                items=rows
                                on_create=Callback::new(move |vals: HashMap<String, String>| {
                                    spawn_local(async move {
                                        let _ = api::post::<Project, _>("/projects", &serde_json::json!({
                                            "name": vals.get("name").cloned().unwrap_or_default(),
                                            "code": vals.get("code").cloned(),
                                            "description": vals.get("description").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_update=Callback::new(move |(id, vals): (String, HashMap<String, String>)| {
                                    spawn_local(async move {
                                        let _ = api::put::<Project, _>(&format!("/projects/{id}"), &serde_json::json!({
                                            "name": vals.get("name").cloned(),
                                            "code": vals.get("code").cloned(),
                                            "description": vals.get("description").cloned(),
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                                on_toggle_active=Callback::new(move |(id, active): (String, bool)| {
                                    spawn_local(async move {
                                        let _ = api::put::<Project, _>(&format!("/projects/{id}"), &serde_json::json!({
                                            "active": active,
                                        })).await;
                                        trigger_refresh();
                                    });
                                })
                            />
                        }
                    })}
                </Show>
            </Suspense>
        </div>
    }
}
