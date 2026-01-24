use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_meta::Script;

#[component]
pub fn MainCard(children: Children) -> impl IntoView {
    view! {
        <Stylesheet href="maincard.css"/>
        <Script src="maincard.js"/>
        <div class="main-card-container">
            <div class="glass-card">
                <div class="glass-card-content">
                    {children()}
                </div>
            </div>
        </div>
    }
}
