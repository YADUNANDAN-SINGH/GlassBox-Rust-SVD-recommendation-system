use crate::components::movie_modal::MovieModal;
use crate::model::video::Video;
use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn SearchResults(videos: ReadSignal<Vec<Video>>) -> impl IntoView {
    // State to track which video is selected for modal
    let (selected_video, set_selected_video) = signal(Option::<Video>::None);

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
                                leptos::logging::log!("SEARCH_RESULTS: Setting selected_video to Some(...)");
                                set_selected_video.set(Some(video_clone.clone()));

                                // Save interaction
                                let sess = use_context::<crate::model::session::SessionState>();
                                let feed_trigger = use_context::<crate::model::feed_control::FeedTrigger>();

                                if let Some(s) = sess {
                                    let session_data = s.1.get();
                                    if let Some(uid_str) = session_data.user_id {
                                        let v_for_save = video_clone.clone();
                                        let v_for_library = video_clone.clone(); // Clone for library save
                                        leptos::task::spawn_local(async move {
                                            // 1. Save interaction
                                            if let Ok(thing) = surrealdb::sql::thing(&uid_str) {
                                                 let _ = crate::model::history::save_interaction(thing, v_for_save, "click".to_string()).await;
                                            }

                                            // 2. Save to Library (Feed)
                                            match crate::model::video::save_video(v_for_library).await {
                                                Ok(v) => {
                                                    leptos::logging::log!("SEARCH_RESULTS: Video saved to library: {}", v.title);
                                                    // Trigger feed update
                                                    if let Some(trigger) = feed_trigger {
                                                        leptos::logging::log!("SEARCH_RESULTS: Incrementing feed trigger...");
                                                        trigger.0.update(|c| *c += 1);
                                                    }
                                                },
                                                Err(e) => leptos::logging::error!("SEARCH_RESULTS: Failed to save video: {}", e)
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

        // Modal Component — now uses WriteSignal directly, no closure
        <MovieModal video=selected_video set_video=set_selected_video />
    }
}
