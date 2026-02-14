use leptos::prelude::*;
use crate::model::video::Video;
use crate::components::movie_modal::MovieModal;
use leptos_meta::Stylesheet;

#[component]
pub fn SearchResults(videos: ReadSignal<Vec<Video>>) -> impl IntoView {
    // State to track which video is selected for modal
    let (selected_video, set_selected_video) = signal(Option::<Video>::None);

    // Handler to close modal
    let close_modal = move || {
        set_selected_video.set(None);
    };

    view! {
        <Stylesheet href="search_results.css"/>
        <div class="search-results-container">
            <For
                each=move || videos.get()
                key=|video| video.video_id.clone()
                children=move |video| {
                    let video_clone = video.clone();
                    view! {
                        <div 
                            class="search-result-card"
                            on:click=move |_| {
                                leptos::logging::log!("CLICKED video: {}", video_clone.title);
                                set_selected_video.set(Some(video_clone.clone()));

                                // Save interaction
                                let sess = use_context::<crate::model::session::SessionState>();
                                if let Some(s) = sess {
                                    let session_data = s.1.get();
                                    if let Some(uid_str) = session_data.user_id {
                                        let v_for_save = video_clone.clone();
                                        leptos::task::spawn_local(async move {
                                            if let Ok(thing) = surrealdb::sql::thing(&uid_str) {
                                                 let _ = crate::model::history::save_interaction(thing, v_for_save, "click".to_string()).await;
                                            }
                                        });
                                    }
                                }
                            }
                        >
                            // Thumbnail Section (Left)
                            <div class="result-thumbnail-container">
                                <img 
                                    src=video.thumbnail_url 
                                    alt=format!("Thumbnail for {}", video.title) 
                                    class="result-thumbnail"
                                    // Fallback to placeholder if broken? (Optional improvement)
                                />
                            </div>

                            // Info Section (Right)
                            <div class="result-info-container">
                                // Title Pill
                                <div class="result-title-bar">
                                    {video.title}
                                </div>

                                // Description Box
                                <div class="result-description-box">
                                    {if video.description.is_empty() {
                                        "No description available.".to_string()
                                    } else {
                                        video.description
                                    }}
                                </div>
                            </div>
                        </div>
                    }
                }
            />
        </div>

        // Modal Component
        <MovieModal video=selected_video on_close=close_modal />
    }
}
