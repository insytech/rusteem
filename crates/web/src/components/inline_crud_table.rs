use leptos::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ColumnDef {
    pub key: &'static str,
    pub label: &'static str,
    pub required: bool,
}

#[derive(Clone)]
pub struct RowData {
    pub id: String,
    pub values: HashMap<String, String>,
    pub active: bool,
}

#[component]
pub fn InlineCrudTable(
    title: &'static str,
    columns: Vec<ColumnDef>,
    items: Vec<RowData>,
    on_create: Callback<HashMap<String, String>>,
    on_update: Callback<(String, HashMap<String, String>)>,
    on_toggle_active: Callback<(String, bool)>,
) -> impl IntoView {
    let (editing_id, set_editing_id) = create_signal(Option::<String>::None);
    let (adding, set_adding) = create_signal(false);
    let (edit_values, set_edit_values) = create_signal(HashMap::<String, String>::new());

    let columns = store_value(columns);

    let start_edit = move |row: &RowData| {
        set_editing_id.set(Some(row.id.clone()));
        set_edit_values.set(row.values.clone());
    };

    let cancel_edit = move || {
        set_editing_id.set(None);
        set_edit_values.set(HashMap::new());
    };

    let save_edit = move || {
        if let Some(id) = editing_id.get() {
            on_update.call((id, edit_values.get()));
            set_editing_id.set(None);
            set_edit_values.set(HashMap::new());
        }
    };

    let start_add = move || {
        set_adding.set(true);
        set_edit_values.set(HashMap::new());
    };

    let cancel_add = move || {
        set_adding.set(false);
        set_edit_values.set(HashMap::new());
    };

    let save_add = move || {
        on_create.call(edit_values.get());
        set_adding.set(false);
        set_edit_values.set(HashMap::new());
    };

    view! {
        <div>
            <div class="section-title">{title}</div>
            <div class="table-container" style="margin-bottom: 1rem;">
                <table>
                    <thead>
                        <tr>
                            {move || columns.get_value().iter().map(|c| {
                                let label = c.label;
                                view! { <th>{label}</th> }
                            }).collect_view()}
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {items.into_iter().map(|row| {
                            let row_id = row.id.clone();
                            let row_id_toggle = row.id.clone();
                            let row_active = row.active;
                            let row_for_edit = row.clone();
                            let row_values = row.values.clone();

                            view! {
                                <tr>
                                    {move || {
                                        let is_editing = editing_id.get().as_ref() == Some(&row_id);
                                        columns.get_value().iter().map(|col| {
                                            let key = col.key.to_string();
                                            let display_val = row_values.get(&key).cloned().unwrap_or_default();
                                            if is_editing {
                                                let key_for_input = key.clone();
                                                let current = edit_values.get().get(&key).cloned().unwrap_or_else(|| display_val.clone());
                                                view! {
                                                    <td>
                                                        <input
                                                            type="text"
                                                            class="crud-inline-input"
                                                            prop:value=current
                                                            on:input=move |ev| {
                                                                let val = event_target_value(&ev);
                                                                set_edit_values.update(|m| { m.insert(key_for_input.clone(), val); });
                                                            }
                                                        />
                                                    </td>
                                                }.into_view()
                                            } else {
                                                let val = if display_val.is_empty() { "-".to_string() } else { display_val };
                                                view! { <td>{val}</td> }.into_view()
                                            }
                                        }).collect_view()
                                    }}
                                    <td>
                                        <span class={if row_active { "badge badge-approved" } else { "badge badge-rejected" }}>
                                            {if row_active { "Active" } else { "Inactive" }}
                                        </span>
                                    </td>
                                    <td>
                                        <div class="actions-cell">
                                            {move || {
                                                let is_editing = editing_id.get().as_ref() == Some(&row_for_edit.id);
                                                if is_editing {
                                                    view! {
                                                        <button class="btn btn-outline btn-icon" title="Save" on:click=move |_| save_edit()>
                                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                                <polyline points="20 6 9 17 4 12"/>
                                                            </svg>
                                                        </button>
                                                        <button class="btn btn-outline btn-icon" title="Cancel" on:click=move |_| cancel_edit()>
                                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                                <line x1="18" y1="6" x2="6" y2="18"/>
                                                                <line x1="6" y1="6" x2="18" y2="18"/>
                                                            </svg>
                                                        </button>
                                                    }.into_view()
                                                } else {
                                                    let row_for_start = row_for_edit.clone();
                                                    let row_id_for_toggle = row_id_toggle.clone();
                                                    let current_active = row_active;
                                                    view! {
                                                        <button class="btn btn-outline btn-icon" title="Edit" on:click=move |_| start_edit(&row_for_start)>
                                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                                <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                                                                <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                                                            </svg>
                                                        </button>
                                                        <button
                                                            class="btn btn-outline btn-icon"
                                                            title={if current_active { "Deactivate" } else { "Activate" }}
                                                            on:click=move |_| on_toggle_active.call((row_id_for_toggle.clone(), !current_active))
                                                        >
                                                            {if current_active {
                                                                view! {
                                                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                                        <path d="M18.36 6.64A9 9 0 015.64 18.36 9 9 0 0118.36 6.64z"/>
                                                                        <line x1="1" y1="1" x2="23" y2="23"/>
                                                                    </svg>
                                                                }.into_view()
                                                            } else {
                                                                view! {
                                                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                                        <polyline points="20 6 9 17 4 12"/>
                                                                    </svg>
                                                                }.into_view()
                                                            }}
                                                        </button>
                                                    }.into_view()
                                                }
                                            }}
                                        </div>
                                    </td>
                                </tr>
                            }
                        }).collect_view()}
                        // Add new row
                        <Show when=move || adding.get()>
                            <tr>
                                {move || columns.get_value().iter().map(|col| {
                                    let key = col.key.to_string();
                                    let key_for_input = key.clone();
                                    let current = edit_values.get().get(&key).cloned().unwrap_or_default();
                                    view! {
                                        <td>
                                            <input
                                                type="text"
                                                class="crud-inline-input"
                                                placeholder=col.label
                                                prop:value=current
                                                on:input=move |ev| {
                                                    let val = event_target_value(&ev);
                                                    set_edit_values.update(|m| { m.insert(key_for_input.clone(), val); });
                                                }
                                            />
                                        </td>
                                    }
                                }).collect_view()}
                                <td></td>
                                <td>
                                    <div class="actions-cell">
                                        <button class="btn btn-outline btn-icon" title="Save" on:click=move |_| save_add()>
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                <polyline points="20 6 9 17 4 12"/>
                                            </svg>
                                        </button>
                                        <button class="btn btn-outline btn-icon" title="Cancel" on:click=move |_| cancel_add()>
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width:14px;height:14px;">
                                                <line x1="18" y1="6" x2="6" y2="18"/>
                                                <line x1="6" y1="6" x2="18" y2="18"/>
                                            </svg>
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        </Show>
                    </tbody>
                </table>
            </div>
            <Show when=move || !adding.get()>
                <button class="btn btn-outline btn-sm" on:click=move |_| start_add()>"+ Add New"</button>
            </Show>
        </div>
    }
}
