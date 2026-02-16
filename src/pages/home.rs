use crate::cards::main_card::MainCard;
use crate::components::feed::Feed;
use crate::components::search::Search;
use crate::model::feed_control::FeedTrigger;
use crate::model::session::SessionState;
use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn Home() -> impl IntoView {
    let session = use_context::<SessionState>().expect("SessionState not found");
    let get_session = session.1;

    // Create a signal to trigger feed updates
    let trigger = RwSignal::new(0);
    provide_context(FeedTrigger(trigger));

    view! {
        <Stylesheet href="home.css?v=2"/>
        // We can remove home.js if you don't have specific JS logic yet
        // <Script src="home.js"/>

        <div class="home-container">
            {move || match get_session.get().username {
                Some(name) => view! {
                    <MainCard>
                        <div style="text-align: center; margin-bottom: 30px;">
                            <h1>"Welcome Back, " <span style="color: #646cff;">{name}</span></h1>
                            <p style="opacity: 0.8;">"What would you like to watch today?"</p>
                        </div>

                        // This Component now handles Input AND Results
                        <Search />

                        // Scroll Tester (as you requested)
                        <Feed />
                    </MainCard>
                }.into_any(),

                None => view! {
                    <div class="landing-container">
                        <div class="glass-card landing-card">
                            <div class="landing-content">
                                <div class="profile-section">
                                    <div class="profile-image-container">
                                        <img src="user.png" alt="Yadunandan Singh" class="profile-image" />
                                    </div>
                                    <h2 class="profile-name">"Yadunandan Singh"</h2>
                                    <p class="profile-intro">
                                        "Hi, I'm 18 and I built this project to showcase my skills in full-stack development.
                                        GlassBox is a movie recommendation engine powered by Rust and WASM."
                                    </p>
                                    <a href="https://yadunandan-singh.pages.dev/" target="_blank" class="portfolio-link">
                                        "Visit My Portfolio"
                                    </a>
                                </div>
                                <div class="divider"></div>
                                <div class="login-prompt-section">
                                    <h1 class="brand-title">"GlassBox"</h1>
                                    <p class="login-text">"Please Login to continue"</p>
                                    <div class="button-group">
                                        <a href="/login" class="login-button">"Login"</a>
                                        <a href="/signup" class="signup-button">"Sign Up"</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
