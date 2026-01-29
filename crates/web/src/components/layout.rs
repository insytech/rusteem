use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="app-layout">
            <Sidebar/>
            <main class="main-content">
                {children()}
            </main>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <div class="sidebar-brand">
                <h1>"RustEEM"</h1>
                <small>"Equipment & Document Management"</small>
            </div>
            <nav>
                <A href="/">"Dashboard"</A>
                <A href="/machines">"Machines"</A>
                <A href="/documents">"Documents"</A>
                <A href="/approvals">"Approvals"</A>
                <A href="/maintenance">"Maintenance"</A>
            </nav>
        </aside>
    }
}
