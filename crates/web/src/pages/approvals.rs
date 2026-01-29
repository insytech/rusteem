use leptos::*;
use shared::dto::approvals::PendingApproval;

use crate::api;

#[component]
pub fn ApprovalsPage() -> impl IntoView {
    let pending = create_resource(
        || (),
        |_| async move { api::get::<Vec<PendingApproval>>("/approvals/pending").await },
    );

    view! {
        <div>
            <div class="page-header">
                <h2>"Pending Approvals"</h2>
            </div>

            <Suspense fallback=move || view! { <p class="loading">"Loading approvals..."</p> }>
                {move || pending.get().map(|result| match result {
                    Ok(list) => {
                        if list.is_empty() {
                            view! {
                                <div class="card">
                                    <p class="empty-state">"No pending approvals."</p>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="table-container">
                                    <table>
                                        <thead>
                                            <tr>
                                                <th>"Document"</th>
                                                <th>"Step"</th>
                                                <th>"Role"</th>
                                                <th>"Requested"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {list.into_iter().map(|a| {
                                                let title = a.document_title.clone();
                                                let role = a.role_name.clone();
                                                let created = a.created_at.format("%Y-%m-%d %H:%M").to_string();
                                                view! {
                                                    <tr>
                                                        <td>{title}</td>
                                                        <td>{"Step "}{a.step_order}</td>
                                                        <td>{role}</td>
                                                        <td>{created}</td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }.into_view()
                        }
                    },
                    Err(e) => view! {
                        <div class="error-message">{format!("Failed to load approvals: {e}")}</div>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}
