use leptos::*;

#[component]
pub fn ConfirmModal<F>(
    title: String,
    message: F,
    confirm_label: String,
    on_confirm: Callback<()>,
    on_cancel: Callback<()>,
) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    view! {
        <div class="modal-overlay" on:click=move |_| on_cancel.call(())>
            <div class="modal-content" on:click=move |ev| ev.stop_propagation()>
                <h3 style="margin-bottom: 0.75rem;">{title}</h3>
                <p style="margin-bottom: 1.25rem; color: var(--color-text-secondary); font-size: 0.875rem;">
                    {message}
                </p>
                <div style="display: flex; justify-content: flex-end; gap: 0.5rem;">
                    <button class="btn btn-outline" on:click=move |_| on_cancel.call(())>"Cancel"</button>
                    <button class="btn btn-danger" on:click=move |_| on_confirm.call(())>{confirm_label}</button>
                </div>
            </div>
        </div>
    }
}
