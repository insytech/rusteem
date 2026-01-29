use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <div class="page-header">
                <h2>"Dashboard"</h2>
            </div>
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-value">"-"</div>
                    <div class="stat-label">"Active Machines"</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"-"</div>
                    <div class="stat-label">"Pending Approvals"</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"-"</div>
                    <div class="stat-label">"Overdue Maintenance"</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"-"</div>
                    <div class="stat-label">"Documents"</div>
                </div>
            </div>
            <div class="card">
                <p>"Welcome to RustEEM. Use the sidebar to navigate between modules."</p>
            </div>
        </div>
    }
}
