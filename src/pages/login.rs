use crate::model::users::login_user;
use crate::model::session::SessionState;
use leptos_router::hooks::use_navigate;
use leptos::prelude::*;
use leptos_meta::{Stylesheet, Script};

#[component]
pub fn Login() -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error_msg, set_error) = signal(Option::<String>::None);

    let session = use_context::<SessionState>().expect("Session missing");
    let navigate = use_navigate();

    let (is_loading, set_is_loading) = signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default(); 
        set_is_loading.set(true);
        set_error.set(None);
        
        // Get values from signals
        let u_val = username.get();
        let p_val = password.get();
        
        leptos::logging::log!("LOGIN PAGE: Submitting for user '{}'", u_val);
        
        let u_for_login = u_val.clone();
        let u_for_session = u_val.clone(); 

        // Clone environment for async block
        let sess = session.clone();
        let nav = navigate.clone();
        let setter = set_error.clone();
        let loader = set_is_loading.clone();

        leptos::task::spawn_local(async move {
            match login_user(u_for_login, p_val).await {
                Ok(user) => {
                    leptos::logging::log!("LOGIN PAGE: Success, redirecting...");
                    // 1. Update Session with full User object
                    sess.login(user);
                    // 2. Redirect to Home
                    nav("/", Default::default());
                },
                Err(e) => {
                    leptos::logging::log!("LOGIN PAGE: Error: {}", e);
                    setter.set(Some(e));
                }
            }
            loader.set(false);
        });
    };

    view! {
        <Stylesheet href="login.css"/>
        <Script src="login.js"/>
        <div class="login-container">
            <div class="glass-card">
                <form class="login-form" on:submit=on_submit>
                    <h1 class="form-title">"Welcome Back"</h1>
                    
                    // Show Error if any
                    {move || error_msg.get().map(|e| view! {
                        <p style="color: red; text-align: center;">{e}</p>
                    })}

                    <div class="input-group">
                        <label class="input-label" for="username">"Username"</label>
                        <input 
                            id="username" 
                            class="form-input" 
                            type="text"
                            prop:value=username
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="input-group">
                        <label class="input-label" for="password">"Password"</label>
                        <input 
                            id="password" 
                            class="form-input" 
                            type="password" 
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="form-footer">
                        <a href="/forgot-password" class="forgot-link">"Forgot Password?"</a>
                    </div>

                    <button type="submit" class="submit-btn" disabled=move || is_loading.get()>
                        {move || if is_loading.get() { "Logging in..." } else { "Login" }}
                    </button>
                </form>
            </div>
        </div>
    }
}