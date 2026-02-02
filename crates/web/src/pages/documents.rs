use leptos::*;
use shared::dto::pagination::PaginatedResponse;
use shared::Document;

use crate::api;
use crate::components::toast::use_toast;

#[component]
pub fn DocumentsPage() -> impl IntoView {
    let toast = use_toast();
    let (refresh_counter, set_refresh) = create_signal(0u32);
    let (documents, set_documents) = create_signal(Vec::<Document>::new());
    let (next_cursor, set_next_cursor) = create_signal(Option::<String>::None);
    let (total, set_total) = create_signal(0i64);
    let (loading_more, set_loading_more) = create_signal(false);

    let initial_load = create_resource(
        move || refresh_counter.get(),
        move |_| async move {
            let result = api::get::<PaginatedResponse<Document>>("/documents").await;
            if let Ok(ref page) = result {
                set_documents.set(page.items.clone());
                set_next_cursor.set(page.next_cursor.clone());
                set_total.set(page.total);
            }
            result
        },
    );

    let trigger_refresh = move || {
        set_documents.set(vec![]);
        set_next_cursor.set(None);
        set_refresh.update(|c| *c += 1);
    };

    let load_more = move |_| {
        if let Some(cursor) = next_cursor.get() {
            set_loading_more.set(true);
            let toast = toast;
            spawn_local(async move {
                let url = format!("/documents?cursor={cursor}");
                match api::get::<PaginatedResponse<Document>>(&url).await {
                    Ok(page) => {
                        set_documents.update(|list| list.extend(page.items));
                        set_next_cursor.set(page.next_cursor);
                        set_total.set(page.total);
                    }
                    Err(e) => {
                        toast.error(&format!("Load more failed: {e}"));
                    }
                }
                set_loading_more.set(false);
            });
        }
    };

    view! {
        <div>
            <div class="page-header">
                <h2>"Documents"</h2>
            </div>

            <Suspense fallback=move || view! { <p class="loading">"Loading documents..."</p> }>
                {move || initial_load.get().map(|result| match result {
                    Ok(_) => view! {
                        <DocumentTable documents=documents.get() on_deleted=Callback::new(move |_: ()| trigger_refresh())/>
                        <Show when=move || next_cursor.get().is_some()>
                            <div style="text-align: center; margin: 1rem 0;">
                                <button
                                    class="btn btn-primary"
                                    on:click=load_more
                                    disabled=move || loading_more.get()
                                >
                                    {move || if loading_more.get() { "Loading..." } else { "Load More" }}
                                </button>
                                <p style="margin-top: 0.5rem; color: var(--color-muted);">
                                    {move || format!("Showing {} of {}", documents.get().len(), total.get())}
                                </p>
                            </div>
                        </Show>
                    }.into_view(),
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
    let toast = use_toast();

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
                        let toast_del = toast.clone();
                        view! {
                            <tr>
                                <td>{title}</td>
                                <td>{"Rev "}{revision}</td>
                                <td><span class={badge_class}>{&status}</span></td>
                                <td>{updated}</td>
                                <td>
                                    <button class="btn btn-danger btn-sm"
                                        on:click=move |_| {
                                            let toast = toast_del.clone();
                                            spawn_local(async move {
                                                if let Err(e) = api::delete_req(&format!("/documents/{id}")).await {
                                                    toast.error(&format!("Delete failed: {e}"));
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
