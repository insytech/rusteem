use leptos::*;

#[component]
pub fn MaintenancePage() -> impl IntoView {
    view! {
        <div>
            <div class="page-header">
                <h2>"Maintenance"</h2>
            </div>
            <div class="card">
                <p class="empty-state">"Maintenance planning coming soon."</p>
            </div>
        </div>
    }
}
