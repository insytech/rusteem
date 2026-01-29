use leptos::*;
use shared::dto::pagination::PaginatedResponse;
use shared::MaintenancePlan;

use crate::api;

#[component]
pub fn MaintenancePage() -> impl IntoView {
    // Overdue state
    let (overdue_plans, set_overdue_plans) = create_signal(Vec::<MaintenancePlan>::new());
    let (overdue_cursor, set_overdue_cursor) = create_signal(Option::<String>::None);
    let (overdue_total, set_overdue_total) = create_signal(0i64);
    let (overdue_loading, set_overdue_loading) = create_signal(false);

    let overdue = create_resource(
        || (),
        move |_| async move {
            let result = api::get::<PaginatedResponse<MaintenancePlan>>("/maintenance/overdue").await;
            if let Ok(ref page) = result {
                set_overdue_plans.set(page.items.clone());
                set_overdue_cursor.set(page.next_cursor.clone());
                set_overdue_total.set(page.total);
            }
            result
        },
    );

    let load_more_overdue = move |_| {
        if let Some(cursor) = overdue_cursor.get() {
            set_overdue_loading.set(true);
            spawn_local(async move {
                let url = format!("/maintenance/overdue?cursor={cursor}");
                match api::get::<PaginatedResponse<MaintenancePlan>>(&url).await {
                    Ok(page) => {
                        set_overdue_plans.update(|list| list.extend(page.items));
                        set_overdue_cursor.set(page.next_cursor);
                        set_overdue_total.set(page.total);
                    }
                    Err(e) => {
                        web_sys::window()
                            .and_then(|w| w.alert_with_message(&format!("Load more failed: {e}")).ok());
                    }
                }
                set_overdue_loading.set(false);
            });
        }
    };

    // Upcoming state
    let (upcoming_plans, set_upcoming_plans) = create_signal(Vec::<MaintenancePlan>::new());
    let (upcoming_cursor, set_upcoming_cursor) = create_signal(Option::<String>::None);
    let (upcoming_total, set_upcoming_total) = create_signal(0i64);
    let (upcoming_loading, set_upcoming_loading) = create_signal(false);

    let upcoming = create_resource(
        || (),
        move |_| async move {
            let result = api::get::<PaginatedResponse<MaintenancePlan>>("/maintenance/upcoming?days=7").await;
            if let Ok(ref page) = result {
                set_upcoming_plans.set(page.items.clone());
                set_upcoming_cursor.set(page.next_cursor.clone());
                set_upcoming_total.set(page.total);
            }
            result
        },
    );

    let load_more_upcoming = move |_| {
        if let Some(cursor) = upcoming_cursor.get() {
            set_upcoming_loading.set(true);
            spawn_local(async move {
                let url = format!("/maintenance/upcoming?days=7&cursor={cursor}");
                match api::get::<PaginatedResponse<MaintenancePlan>>(&url).await {
                    Ok(page) => {
                        set_upcoming_plans.update(|list| list.extend(page.items));
                        set_upcoming_cursor.set(page.next_cursor);
                        set_upcoming_total.set(page.total);
                    }
                    Err(e) => {
                        web_sys::window()
                            .and_then(|w| w.alert_with_message(&format!("Load more failed: {e}")).ok());
                    }
                }
                set_upcoming_loading.set(false);
            });
        }
    };

    view! {
        <div>
            <div class="page-header">
                <h2>"Maintenance"</h2>
            </div>

            <h3 style="margin-bottom: 0.75rem; color: var(--color-danger);">"Overdue"</h3>
            <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
                {move || overdue.get().map(|result| match result {
                    Ok(_) => view! {
                        <PlanList plans=overdue_plans.get()/>
                        <Show when=move || overdue_cursor.get().is_some()>
                            <div style="text-align: center; margin: 1rem 0;">
                                <button
                                    class="btn btn-primary"
                                    on:click=load_more_overdue
                                    disabled=move || overdue_loading.get()
                                >
                                    {move || if overdue_loading.get() { "Loading..." } else { "Load More" }}
                                </button>
                                <p style="margin-top: 0.5rem; color: var(--color-muted);">
                                    {move || format!("Showing {} of {}", overdue_plans.get().len(), overdue_total.get())}
                                </p>
                            </div>
                        </Show>
                    }.into_view(),
                    Err(e) => view! {
                        <div class="error-message">{format!("Error: {e}")}</div>
                    }.into_view(),
                })}
            </Suspense>

            <h3 style="margin: 1.5rem 0 0.75rem;">"Upcoming (Next 7 Days)"</h3>
            <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
                {move || upcoming.get().map(|result| match result {
                    Ok(_) => view! {
                        <PlanList plans=upcoming_plans.get()/>
                        <Show when=move || upcoming_cursor.get().is_some()>
                            <div style="text-align: center; margin: 1rem 0;">
                                <button
                                    class="btn btn-primary"
                                    on:click=load_more_upcoming
                                    disabled=move || upcoming_loading.get()
                                >
                                    {move || if upcoming_loading.get() { "Loading..." } else { "Load More" }}
                                </button>
                                <p style="margin-top: 0.5rem; color: var(--color-muted);">
                                    {move || format!("Showing {} of {}", upcoming_plans.get().len(), upcoming_total.get())}
                                </p>
                            </div>
                        </Show>
                    }.into_view(),
                    Err(e) => view! {
                        <div class="error-message">{format!("Error: {e}")}</div>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn PlanList(plans: Vec<MaintenancePlan>) -> impl IntoView {
    if plans.is_empty() {
        return view! {
            <div class="card">
                <p class="empty-state">"No plans found."</p>
            </div>
        }
        .into_view();
    }

    view! {
        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th>"Description"</th>
                        <th>"Frequency"</th>
                        <th>"Next Due"</th>
                        <th>"Last Performed"</th>
                    </tr>
                </thead>
                <tbody>
                    {plans.into_iter().map(|p| {
                        let desc = p.description.clone();
                        let freq = format!("{} {:?}", p.frequency_value, p.frequency_unit).to_lowercase();
                        let next = p.next_due_at
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| "-".to_string());
                        let last = p.last_performed_at
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| "Never".to_string());
                        view! {
                            <tr>
                                <td>{desc}</td>
                                <td>{freq}</td>
                                <td>{next}</td>
                                <td>{last}</td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
    .into_view()
}
