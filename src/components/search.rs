use crate::api::search::search_videos;
use crate::cards::search_results::SearchResults;
use crate::model::session::SessionState;
use crate::model::video::Video;
use leptos::prelude::*;
use leptos_meta::{Script, Stylesheet};

#[component]
pub fn Search() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (videos, set_videos) = signal(Vec::<Video>::new());
    let (error_message, set_error_message) = signal(Option::<String>::None);

    let on_input = move |ev| {
        set_query.set(event_target_value(&ev));
    };

    let search_action = move || {
        let q = query.get();
        if q.is_empty() {
            return;
        }

        set_is_loading.set(true);
        set_error_message.set(None);
        set_videos.set(Vec::new());

        let q_for_search = q.clone();
        leptos::task::spawn_local(async move {
            // Updated to pass String, not &str, as per server function signature
            // Start search
            match search_videos(&q_for_search).await {
                Ok(results) => {
                    leptos::logging::log!("Found {} videos", results.len());
                    if results.is_empty() {
                        set_error_message.set(Some("No videos found.".to_string()));
                    } else {
                        set_videos.set(results);
                    }
                }
                Err(e) => {
                    let err_str = e.to_string();
                    leptos::logging::log!("Search error: {}", err_str);
                    set_error_message.set(Some(format!("Error: {}", err_str)));
                }
            }
            set_is_loading.set(false);
        });

        // Save history (fire and forget)
        let sess = use_context::<SessionState>();
        if let Some(s) = sess {
            let session_data = s.1.get();
            if let Some(uid_str) = session_data.user_id {
                let query_val = q.clone();
                leptos::task::spawn_local(async move {
                    if let Ok(thing) = surrealdb::sql::thing(&uid_str) {
                        let _ = crate::model::history::save_search(thing, query_val).await;
                    }
                });
            }
        }
    };

    let on_search_click = move |_| {
        search_action();
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            search_action();
        }
    };

    view! {
        <Stylesheet href="search.css"/>
        <Script src="search.js"/>

        <div class="search-wrapper" style="width: 100%; display: flex; flex-direction: column; align-items: center; gap: 20px;">
            <div class="search-container">
                <input
                    type="text"
                    class="search-input"
                    placeholder="Search..."
                    on:input=on_input
                    prop:value=query
                    on:keydown=on_keydown
                />
                <button type="button" class="search-button" on:click=on_search_click>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="11" cy="11" r="8"></circle>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                    </svg>
                </button>
            </div>

            // Show Loading Indicator
            {move || if is_loading.get() {
                view! { <div class="search-loading" style="color: white; padding: 10px;">"Searching..."</div> }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}

            // Show Error Message
            {move || match error_message.get() {
                Some(msg) => view! { <div class="search-error" style="color: #ff6b6b; padding: 10px; background: rgba(0,0,0,0.5); border-radius: 8px;">{msg}</div> }.into_any(),
                None => view! { <div/> }.into_any()
            }}

            // Render Results
            <SearchResults videos=videos />
        </div>
    }
}
