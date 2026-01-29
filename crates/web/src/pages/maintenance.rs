use leptos::*;
use shared::MaintenancePlan;

use crate::api;

#[component]
pub fn MaintenancePage() -> impl IntoView {
    let overdue = create_resource(
        || (),
        |_| async move { api::get::<Vec<MaintenancePlan>>("/maintenance/overdue").await },
    );

    let upcoming = create_resource(
        || (),
        |_| async move { api::get::<Vec<MaintenancePlan>>("/maintenance/upcoming?days=7").await },
    );

    view! {
        <div>
            <div class="page-header">
                <h2>"Maintenance"</h2>
            </div>

            <h3 style="margin-bottom: 0.75rem; color: var(--color-danger);">"Overdue"</h3>
            <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
                {move || overdue.get().map(|result| match result {
                    Ok(list) => view! { <PlanList plans=list/> }.into_view(),
                    Err(e) => view! {
                        <div class="error-message">{format!("Error: {e}")}</div>
                    }.into_view(),
                })}
            </Suspense>

            <h3 style="margin: 1.5rem 0 0.75rem;">"Upcoming (Next 7 Days)"</h3>
            <Suspense fallback=move || view! { <p class="loading">"Loading..."</p> }>
                {move || upcoming.get().map(|result| match result {
                    Ok(list) => view! { <PlanList plans=list/> }.into_view(),
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
