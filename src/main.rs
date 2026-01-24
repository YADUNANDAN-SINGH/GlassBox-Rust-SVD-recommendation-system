use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes};
use leptos_router::path;
use leptos_meta::provide_meta_context;

use navbar::navbar::Navbar;
use pages::login::Login;
use pages::home::Home;
use pages::signup::Signup;
use model::db::init_db;
use model::session::SessionState;

mod pages;
mod navbar;
mod model;
mod cards;
mod API;
mod components;

// Re-export server functions so they can be called from client
// #[cfg(not(target_arch = "wasm32"))]
// pub use api::search::search_videos;

// #[cfg(not(target_arch = "wasm32"))]
// pub use api::youtube::fetch_video;
mod components;

fn main(){
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    // Initialize DB
    leptos::task::spawn_local(async {
        init_db().await;
    });

    let session = SessionState::new();
    provide_context(session);

    view! {
        <Router>
            // Navbar is outside <Routes>, so it stays visible on every page
            <Navbar />

            <main>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/login") view=Login />
                    <Route path=path!("/signup") view=Signup />
                </Routes>
            </main>
        </Router>
    }
}
