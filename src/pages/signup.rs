use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_meta::Script;
use crate::model::users::{create_new_user, is_email_taken, is_username_taken};
use leptos_router::components::A;

#[component]
pub fn Signup() -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let (status_msg, set_status) = signal(String::new()); 
    let (email_error, set_email_error) = signal(Option::<String>::None);
    let (username_error, set_username_error) = signal(Option::<String>::None); 

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default(); 

        set_status.set("Creating account...".to_string());

        let name: String = username.get();
        let mail: String = email.get();
        let pass: String = password.get();

        leptos::task::spawn_local(async move {
            match create_new_user(name, mail, pass).await {
                Ok(_) => {
                    set_status.set("SUCCESS: Account created! You can now log in.".to_string());
                    // Clear the form
                    set_username.set("".to_string());
                    set_email.set("".to_string());
                    set_password.set("".to_string());
                },
                Err(e) => {
                    set_status.set(format!("ERROR: {}", e));
                }
            }
        });
    };

    let check_email = move |ev| {
        let val = event_target_value(&ev);
        if val.is_empty() { return; }
        
        leptos::task::spawn_local(async move {
            match is_email_taken(val).await {
                Ok(true) => set_email_error.set(Some("This email already exists. ".to_string())),
                Ok(false) => set_email_error.set(None),
                Err(_) => {} 
            }
        });
    };

    let check_username = move |ev| {
        let val = event_target_value(&ev);
        if val.is_empty() { return; }

        leptos::task::spawn_local(async move {
            match is_username_taken(val).await {
                Ok(true) => set_username_error.set(Some("This username already exists. ".to_string())),
                Ok(false) => set_username_error.set(None),
                Err(_) => {}
            }
        });
    };

    view! {
        <Stylesheet href="signup.css"/>
        <Script src="signup.js"/>
        <div class="signup-container">
            <div class="glass-card">
                <form class="signup-form" on:submit=on_submit>
                    <h1 class="form-title">"Create Account"</h1>
                    
                    // Status Message Display
                    {move || if !status_msg.get().is_empty() {
                        view! { <p class="status-msg">{status_msg.get()}</p> }.into_any()
                    } else {
                        ().into_any()
                    }}

                    <div class="input-group">
                        <label class="input-label" for="email">"Email Address"</label>
                        <input 
                            id="email" 
                            class="form-input" 
                            placeholder="name@example.com" 
                            type="email"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            on:blur=check_email
                        />
                        {move || email_error.get().map(|err| view! {
                            <p class="error-msg" style="color: red; font-size: 0.9rem; margin-top: 5px;">
                                {err} 
                                <A href="/login"><span class="text-blue-500 hover:underline">"Would you like to login?"</span></A>
                            </p>
                        })}
                    </div>

                    <div class="input-group">
                        <label class="input-label" for="username">"Username"</label>
                        <input 
                            id="username" 
                            class="form-input" 
                            placeholder="Choose a unique username" 
                            type="text"
                            prop:value=username
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            on:blur=check_username
                        />
                        {move || username_error.get().map(|err| view! {
                            <p class="error-msg" style="color: red; font-size: 0.9rem; margin-top: 5px;">
                                {err}
                                <A href="/login"><span class="text-blue-500 hover:underline">"Would you like to login?"</span></A>
                            </p>
                        })}
                    </div>

                    <div class="input-group">
                        <label class="input-label" for="password">"Password"</label>
                        <input 
                            id="password" 
                            class="form-input" 
                            placeholder="Create a strong password" 
                            type="password"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                        <div class="password-strength">
                            <div class="strength-bar">
                                <div id="strength-fill"></div>
                            </div>
                            <p id="strength-text"></p>
                        </div>
                    </div>

                    <button type="submit" class="submit-btn">"Sign Up"</button>
                </form>
            </div>
        </div>
    }
}
