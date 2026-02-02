use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::layout::Layout;
use crate::pages::{
    approvals::ApprovalsPage, documents::DocumentsPage, home::HomePage,
    machines::{MachineDetailPage, MachinesPage}, maintenance::MaintenancePage,
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
                    <Route path="/machines/:id" view=MachineDetailPage/>
                    <Route path="/machines/:id/edit" view=MachineEditPage/>
                    <Route path="/documents" view=DocumentsPage/>
                    <Route path="/approvals" view=ApprovalsPage/>
                    <Route path="/maintenance" view=MaintenancePage/>
                </Routes>
            </Layout>
        </Router>
    }
}

#[component]
fn MachineEditPage() -> impl IntoView {
    use leptos_router::*;
    use shared::dto::machines::MachineDetail;

    let params = use_params_map();
    let navigate = use_navigate();
    let navigate = store_value(navigate);

    let id = move || {
        params.with(|p| {
            p.get("id")
                .and_then(|id| id.parse::<uuid::Uuid>().ok())
        })
    };

    let machine = create_resource(
        id,
        move |maybe_id| async move {
            match maybe_id {
                Some(id) => crate::api::get::<MachineDetail>(&format!("/machines/{id}")).await.ok(),
                None => None,
            }
        },
    );

    view! {
        <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
            {move || machine.get().map(|maybe_m| match maybe_m {
                None => view! {
                    <div class="error-message">"Machine not found."</div>
                }.into_view(),
                Some(m) => {
                    let nav = navigate.get_value();
                    let machine_id = m.id;
                    view! {
                        <div class="page-header">
                            <h2>"Edit Machine"</h2>
                            <a href=format!("/machines/{machine_id}") class="btn btn-outline">"Cancel"</a>
                        </div>
                        <crate::pages::machines::form::MachineForm
                            machine=Some(m)
                            on_saved=Callback::new(move |_: ()| {
                                nav(&format!("/machines/{machine_id}"), Default::default());
                            })
                        />
                    }.into_view()
                }
            })}
        </Suspense>
    }
}
