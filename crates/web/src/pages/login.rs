use leptos::*;
use leptos_router::*;

use crate::auth;

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(Option::<String>::None);
    let (loading, set_loading) = create_signal(false);
    let (mode, set_mode) = create_signal(LoginMode::SignIn);

    let navigate = use_navigate();
    let navigate = store_value(navigate);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        let email_val = email.get();
        let password_val = password.get();
        let current_mode = mode.get();

        spawn_local(async move {
            match current_mode {
                LoginMode::SignIn => {
                    match auth::login(&email_val, &password_val).await {
                        Ok(_) => {
                            let nav = navigate.get_value();
                            nav("/", Default::default());
                        }
                        Err(e) => set_error.set(Some(e)),
                    }
                }
                LoginMode::SignUp => {
                    match auth::signup(&email_val, &password_val).await {
                        Ok(_) => {
                            set_error.set(None);
                            // Try to auto-login after signup
                            match auth::login(&email_val, &password_val).await {
                                Ok(_) => {
                                    let nav = navigate.get_value();
                                    nav("/", Default::default());
                                }
                                Err(_) => {
                                    set_mode.set(LoginMode::SignIn);
                                    set_error.set(Some("Account created. Please sign in.".to_string()));
                                }
                            }
                        }
                        Err(e) => set_error.set(Some(e)),
                    }
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="login-page">
            <div class="login-card">
                <div class="login-brand">
                    <h1>"RustEEM"</h1>
                    <small>"Equipment Management"</small>
                </div>

                <h2 class="login-title">
                    {move || match mode.get() {
                        LoginMode::SignIn => "Sign In",
                        LoginMode::SignUp => "Create Account",
                    }}
                </h2>

                <Show when=move || error.get().is_some()>
                    <div class="error-message">{move || error.get().unwrap_or_default()}</div>
                </Show>

                <form on:submit=on_submit>
                    <div class="form-group">
                        <label>"Email"</label>
                        <input
                            type="email"
                            required
                            placeholder="you@example.com"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label>"Password"</label>
                        <input
                            type="password"
                            required
                            placeholder="Your password"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>
                    <button
                        class="btn btn-primary login-submit"
                        type="submit"
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() {
                            "Please wait..."
                        } else {
                            match mode.get() {
                                LoginMode::SignIn => "Sign In",
                                LoginMode::SignUp => "Create Account",
                            }
                        }}
                    </button>
                </form>

                <div class="login-toggle">
                    {move || match mode.get() {
                        LoginMode::SignIn => view! {
                            <span>"Don't have an account? "</span>
                            <button class="btn-link" on:click=move |_| {
                                set_mode.set(LoginMode::SignUp);
                                set_error.set(None);
                            }>"Sign Up"</button>
                        }.into_view(),
                        LoginMode::SignUp => view! {
                            <span>"Already have an account? "</span>
                            <button class="btn-link" on:click=move |_| {
                                set_mode.set(LoginMode::SignIn);
                                set_error.set(None);
                            }>"Sign In"</button>
                        }.into_view(),
                    }}
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LoginMode {
    SignIn,
    SignUp,
}
