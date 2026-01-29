use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::layout::Layout;
use crate::pages::{
    approvals::ApprovalsPage, documents::DocumentsPage, home::HomePage,
    machines::MachinesPage, maintenance::MaintenancePage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="RustEEM"/>
        <Router>
            <Layout>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/machines" view=MachinesPage/>
                    <Route path="/documents" view=DocumentsPage/>
                    <Route path="/approvals" view=ApprovalsPage/>
                    <Route path="/maintenance" view=MaintenancePage/>
                </Routes>
            </Layout>
        </Router>
    }
}
