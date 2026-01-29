use leptos::*;
use shared::dto::dashboard::{ActivityEntry, DashboardSummary, MachineAlert, StageCount};

use crate::api;

#[component]
pub fn HomePage() -> impl IntoView {
    let summary = create_resource(
        || (),
        |_| async move { api::fetch_dashboard_summary().await },
    );

    view! {
        <div>
            <div class="page-header">
                <div>
                    <h2>"Dashboard"</h2>
                    <p class="page-header-sub">"Equipment pipeline overview"</p>
                </div>
            </div>

            <Suspense fallback=move || view! { <DashboardSkeleton/> }>
                {move || summary.get().map(|result| match result {
                    Ok(data) => view! { <DashboardContent data=data/> }.into_view(),
                    Err(e) => view! {
                        <div class="error-message">{format!("Failed to load dashboard: {e}")}</div>
                    }.into_view(),
                })}
            </Suspense>
        </div>
    }
}

/// Skeleton placeholder while data loads
#[component]
fn DashboardSkeleton() -> impl IntoView {
    view! {
        <div class="card" style="margin-bottom: 1.25rem;">
            <div class="skeleton skeleton-text" style="width: 30%; margin-bottom: 1rem;"></div>
            <div style="display: flex; gap: 0;">
                <div class="skeleton" style="flex: 1; height: 64px; border-radius: 10px 0 0 10px;"></div>
                <div class="skeleton" style="flex: 1; height: 64px; border-radius: 0;"></div>
                <div class="skeleton" style="flex: 1; height: 64px; border-radius: 0;"></div>
                <div class="skeleton" style="flex: 1; height: 64px; border-radius: 0;"></div>
                <div class="skeleton" style="flex: 1; height: 64px; border-radius: 0;"></div>
                <div class="skeleton" style="flex: 1; height: 64px; border-radius: 0;"></div>
                <div class="skeleton" style="flex: 1; height: 64px; border-radius: 0 10px 10px 0;"></div>
            </div>
        </div>
        <div class="stats-grid">
            <div class="skeleton skeleton-block"></div>
            <div class="skeleton skeleton-block"></div>
            <div class="skeleton skeleton-block"></div>
            <div class="skeleton skeleton-block"></div>
        </div>
        <div class="dashboard-bottom-grid">
            <div class="skeleton skeleton-block" style="height: 200px;"></div>
            <div class="skeleton skeleton-block" style="height: 200px;"></div>
        </div>
    }
}

#[component]
fn DashboardContent(data: DashboardSummary) -> impl IntoView {
    view! {
        <PipelineBar stages=data.pipeline.clone()/>
        <KpiCards
            total_active=data.total_active
            total_in_progress=data.total_in_progress
            total_released=data.total_released
            total_overdue=data.total_overdue
            total_breaches=data.total_breaches
            released_this_month=data.released_this_month
        />
        <div class="dashboard-bottom-grid">
            <NeedsAttentionTable alerts=data.needs_attention.clone()/>
            <ActivityFeed entries=data.recent_activity.clone()/>
        </div>
    }
}

#[component]
fn PipelineBar(stages: Vec<StageCount>) -> impl IntoView {
    let total: i64 = stages.iter().map(|s| s.count).sum();
    let max_count = stages.iter().map(|s| s.count).max().unwrap_or(0);

    view! {
        <div class="card pipeline-section">
            <h3 class="section-title">"Pipeline"</h3>
            <div class="pipeline-track">
                {stages.into_iter().map(|stage| {
                    let has = stage.count > 0;
                    let is_large = max_count > 0 && stage.count == max_count && has;
                    let node_class = format!(
                        "pipeline-node{}{}",
                        if has { " has-count" } else { "" },
                        if is_large { " scale-lg" } else { "" }
                    );
                    view! {
                        <div class={node_class}>
                            <div class="pipeline-node-circle">
                                {stage.count}
                            </div>
                            <span class="pipeline-node-label">{stage.label}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="pipeline-total">
                "Total in pipeline: "<strong>{total}</strong>
            </div>
        </div>
    }
}

#[component]
fn KpiCards(
    total_active: i64,
    total_in_progress: i64,
    total_released: i64,
    total_overdue: i64,
    total_breaches: i64,
    released_this_month: i64,
) -> impl IntoView {
    view! {
        <div class="stats-grid">
            // Total Active
            <div class="stat-card kpi-card kpi-default">
                <div class="stat-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <rect x="2" y="6" width="20" height="12" rx="2"/>
                        <path d="M12 12h.01"/>
                        <path d="M17 12h.01"/>
                        <path d="M7 12h.01"/>
                    </svg>
                </div>
                <div class="stat-value">{total_active}</div>
                <div class="kpi-text">
                    <div class="stat-label">"Total Active"</div>
                    <div class="kpi-subtext">
                        {total_released}" released, "{total_in_progress}" in progress"
                    </div>
                </div>
            </div>
            // Overdue
            <div class={format!("stat-card kpi-card{}", if total_overdue > 0 { " kpi-danger" } else { " kpi-default" })}>
                <div class="stat-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="10"/>
                        <polyline points="12 6 12 12 16 14"/>
                    </svg>
                </div>
                <div class="stat-value">{total_overdue}</div>
                <div class="kpi-text">
                    <div class="stat-label">"Overdue"</div>
                    <div class="kpi-subtext">"Past any stage deadline"</div>
                </div>
            </div>
            // Breaches
            <div class={format!("stat-card kpi-card{}", if total_breaches > 0 { " kpi-warning" } else { " kpi-default" })}>
                <div class="stat-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/>
                        <line x1="12" y1="9" x2="12" y2="13"/>
                        <line x1="12" y1="17" x2="12.01" y2="17"/>
                    </svg>
                </div>
                <div class="stat-value">{total_breaches}</div>
                <div class="kpi-text">
                    <div class="stat-label">"Breaches"</div>
                    <div class="kpi-subtext">"Process breaches requiring attention"</div>
                </div>
            </div>
            // Released This Month
            <div class="stat-card kpi-card kpi-success">
                <div class="stat-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M22 11.08V12a10 10 0 11-5.93-9.14"/>
                        <polyline points="22 4 12 14.01 9 11.01"/>
                    </svg>
                </div>
                <div class="stat-value">{released_this_month}</div>
                <div class="kpi-text">
                    <div class="stat-label">"Released This Month"</div>
                    <div class="kpi-subtext">"Completed the full pipeline"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NeedsAttentionTable(alerts: Vec<MachineAlert>) -> impl IntoView {
    view! {
        <div class="card attention-section">
            <h3 class="section-title">"Needs Attention"</h3>
            {if alerts.is_empty() {
                view! {
                    <div class="empty-state">
                        <svg class="empty-state-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M22 11.08V12a10 10 0 11-5.93-9.14"/>
                            <polyline points="22 4 12 14.01 9 11.01"/>
                        </svg>
                        <p>"All clear — no machines require immediate attention."</p>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="table-container attention-table">
                        <table>
                            <thead>
                                <tr>
                                    <th>"Machine"</th>
                                    <th>"Area"</th>
                                    <th>"Stage"</th>
                                    <th>"Status"</th>
                                    <th>"Days"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {alerts.into_iter().map(|alert| {
                                    let badge_class = match alert.status.as_str() {
                                        "overdue" => "badge badge-overdue",
                                        "breach" => "badge badge-breach",
                                        _ => "badge badge-draft",
                                    };
                                    let days_class = if alert.days_overdue > 7 {
                                        "days-overdue days-overdue-high"
                                    } else if alert.days_overdue > 3 {
                                        "days-overdue days-overdue-mid"
                                    } else {
                                        "days-overdue days-overdue-low"
                                    };
                                    view! {
                                        <tr>
                                            <td><strong>{alert.machine_name}</strong></td>
                                            <td>{alert.area.unwrap_or_else(|| "—".to_string())}</td>
                                            <td>{alert.current_stage}</td>
                                            <td><span class={badge_class}>{&alert.status}</span></td>
                                            <td class={days_class}>{alert.days_overdue}</td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                }.into_view()
            }}
        </div>
    }
}

#[component]
fn ActivityFeed(entries: Vec<ActivityEntry>) -> impl IntoView {
    view! {
        <div class="card activity-section">
            <h3 class="section-title">"Recent Activity"</h3>
            {if entries.is_empty() {
                view! {
                    <div class="empty-state">
                        <svg class="empty-state-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="10"/>
                            <polyline points="12 6 12 12 16 14"/>
                        </svg>
                        <p>"No recent activity recorded."</p>
                    </div>
                }.into_view()
            } else {
                view! {
                    <ul class="activity-feed">
                        {entries.into_iter().map(|entry| {
                            let time_str = entry.timestamp.format("%b %d, %H:%M").to_string();
                            view! {
                                <li class="activity-item">
                                    <span class="activity-dot"></span>
                                    <div class="activity-content">
                                        <span class="activity-text">{entry.description}</span>
                                        <span class="activity-time">{time_str}</span>
                                    </div>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                }.into_view()
            }}
        </div>
    }
}
