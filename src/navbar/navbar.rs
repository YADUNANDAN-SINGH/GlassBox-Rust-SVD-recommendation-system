use leptos::prelude::*;
use leptos_router::components::A;
use leptos_meta::Stylesheet;
use leptos_meta::Script;

use crate::model::session::SessionState;

#[component]
pub fn Navbar() -> impl IntoView {
    // Signal for mobile menu toggle
    let (is_open, set_is_open) = signal(false);

    let toggle_menu = move |_| set_is_open.update(|open| *open = !*open);

    // Get the session state
    let session = use_context::<SessionState>().expect("SessionState not found");
    let (_set_session, get_session) = (session.0, session.1);

    view! {
        <Stylesheet href="navbar.css"/>
        <Script src="navbar.js"/>
        
        <nav class="navbar" id="main-navbar">
            <div class="nav-container">
                <a href="/" class="nav-logo">
                    "GlassBox"
                </a>

                // Hamburger Menu Icon
                <div class="menu-icon" on:click=toggle_menu>
                    <div class={move || if is_open.get() { "bar open" } else { "bar" }}></div>
                    <div class={move || if is_open.get() { "bar open" } else { "bar" }}></div>
                    <div class={move || if is_open.get() { "bar open" } else { "bar" }}></div>
                </div>

                // Links
                <ul class={move || if is_open.get() { "nav-menu active" } else { "nav-menu" }}>
                    <li class="nav-item" on:click=move |_| set_is_open.set(false)>
                        <A href="/" attr:class="nav-link">"Home"</A>
                    </li>
                    {move || match get_session.get().username {
                        Some(_name) => view! {

                            <li class="nav-item" on:click=move |_| session.logout()>
                                <span class="nav-link" style="cursor: pointer">"Logout"</span>
                            </li>
                        }.into_any(),
                        None => view! {
                            <li class="nav-item"><A href="/login" attr:class="nav-link">"Login"</A></li>
                            <li class="nav-item"><A href="/signup" attr:class="nav-link">"Sign Up"</A></li>
                        }.into_any()
                    }}
                </ul>
            </div>
        </nav>
    }
}
