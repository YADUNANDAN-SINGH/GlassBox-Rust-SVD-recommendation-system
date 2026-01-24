use leptos::prelude::*;
use leptos_meta::{Stylesheet, Script};
use crate::cards::main_card::MainCard;
use crate::components::search::Search; // We will create this next
use crate::model::session::SessionState;

#[component]
pub fn Home() -> impl IntoView {
    let session = use_context::<SessionState>().expect("SessionState not found");
    let get_session = session.1;

    view! {
        <Stylesheet href="home.css"/>
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
                        <div style="margin-top: 50px; height: 50vh; border-top: 1px solid rgba(255,255,255,0.1); display: flex; align-items: center; justify-content: center; opacity: 0.5;">
                            <p>"↓ Scroll for Feed (Coming Soon) ↓"</p>
                        </div>
                    </MainCard>
                }.into_any(),
                
                None => view! { 
                    <div style="text-align: center; padding-top: 20vh; color: white;">
                        <h1>"GlassBox"</h1>
                        <p>"Please Login to continue"</p>
                    </div> 
                }.into_any()
            }}
        </div>
    }
}