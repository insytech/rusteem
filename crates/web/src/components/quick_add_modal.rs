use leptos::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FieldDef {
    pub label: &'static str,
    pub name: &'static str,
    pub required: bool,
}

#[component]
pub fn QuickAddModal(
    title: String,
    fields: Vec<FieldDef>,
    on_submit: Callback<HashMap<String, String>>,
    on_close: Callback<()>,
) -> impl IntoView {
    let field_signals: Vec<(FieldDef, (ReadSignal<String>, WriteSignal<String>))> = fields
        .into_iter()
        .map(|f| {
            let sig = create_signal(String::new());
            (f, sig)
        })
        .collect();

    let (error, set_error) = create_signal(Option::<String>::None);
    let (submitting, set_submitting) = create_signal(false);

    let field_signals = store_value(field_signals);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);

        let mut values = HashMap::new();
        let sigs = field_signals.get_value();
        for (def, (reader, _)) in &sigs {
            let val = reader.get();
            if def.required && val.trim().is_empty() {
                set_error.set(Some(format!("{} is required", def.label)));
                return;
            }
            if !val.trim().is_empty() {
                values.insert(def.name.to_string(), val);
            }
        }

        set_submitting.set(true);
        on_submit.call(values);
    };

    view! {
        <div class="modal-overlay" on:click=move |_| on_close.call(())>
            <div class="modal-content" on:click=move |ev| ev.stop_propagation()>
                <h3 style="margin-bottom: 0.75rem;">{title}</h3>

                <Show when=move || error.get().is_some()>
                    <div class="error-message">{move || error.get().unwrap_or_default()}</div>
                </Show>

                <form on:submit=handle_submit>
                    {move || {
                        let sigs = field_signals.get_value();
                        sigs.into_iter().map(|(def, (reader, writer))| {
                            let label = def.label;
                            let req = def.required;
                            view! {
                                <div class="form-group">
                                    <label>{label} {if req { " *" } else { "" }}</label>
                                    <input
                                        type="text"
                                        prop:value=reader
                                        on:input=move |ev| writer.set(event_target_value(&ev))
                                    />
                                </div>
                            }
                        }).collect_view()
                    }}
                    <div style="display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 0.75rem;">
                        <button class="btn btn-outline" type="button" on:click=move |_| on_close.call(())>"Cancel"</button>
                        <button class="btn btn-primary" type="submit" disabled=move || submitting.get()>
                            {move || if submitting.get() { "Adding..." } else { "Add" }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
