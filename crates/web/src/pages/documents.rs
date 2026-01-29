use leptos::*;
use shared::Document;

use crate::api;

#[component]
pub fn DocumentsPage() -> impl IntoView {
    let (refresh_counter, set_refresh) = create_signal(0u32);

    let documents = create_resource(
        move || refresh_counter.get(),
        |_| async move { api::get::<Vec<Document>>("/documents").await },
    );

    let trigger_refresh = move || set_refresh.update(|c| *c += 1);

    view! {
        <div>
            <div class="page-header">
                <h2>"Documents"</h2>
            </div>

            <Suspense fallback=move || view! { <p class="loading">"Loading documents..."</p> }>
                {move || documents.get().map(|result| match result {
                    Ok(list) => view! { <DocumentTable documents=list on_deleted=Callback::new(move |_: ()| trigger_refresh())/> }.into_view(),
                    Err(e) => view! {
                        <div class="error-message">{format!("Failed to load documents: {e}")}</div>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn DocumentTable(
    documents: Vec<Document>,
    on_deleted: Callback<()>,
) -> impl IntoView {
    if documents.is_empty() {
        return view! {
            <div class="card">
                <p class="empty-state">"No documents found."</p>
            </div>
        }
        .into_view();
    }

    view! {
        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th>"Title"</th>
                        <th>"Revision"</th>
                        <th>"Status"</th>
                        <th>"Updated"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {documents.into_iter().map(|d| {
                        let id = d.id;
                        let title = d.title.clone();
                        let revision = d.revision;
                        let status = format!("{:?}", d.status).to_lowercase();
                        let badge_class = format!("badge badge-{status}");
                        let updated = d.updated_at.format("%Y-%m-%d %H:%M").to_string();
                        view! {
                            <tr>
                                <td>{title}</td>
                                <td>{"Rev "}{revision}</td>
                                <td><span class={badge_class}>{&status}</span></td>
                                <td>{updated}</td>
                                <td>
                                    <button class="btn btn-danger btn-sm"
                                        on:click=move |_| {
                                            spawn_local(async move {
                                                if let Err(e) = api::delete_req(&format!("/documents/{id}")).await {
                                                    web_sys::window()
                                                        .and_then(|w| w.alert_with_message(&format!("Delete failed: {e}")).ok());
                                                } else {
                                                    on_deleted.call(());
                                                }
                                            });
                                        }
                                    >"Delete"</button>
                                </td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
    .into_view()
}
