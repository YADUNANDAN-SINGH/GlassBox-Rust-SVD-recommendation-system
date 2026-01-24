use leptos::prelude::*;
use crate::model::video::Video;
use leptos_meta::Stylesheet;

#[component]
pub fn SearchResults(videos: ReadSignal<Vec<Video>>) -> impl IntoView {
    view! {
        <Stylesheet href="search_results.css"/>
        <div class="search-results-container">
            <For
                each=move || videos.get()
                key=|video| video.video_id.clone()
                children=move |video| {
                    view! {
                        <div class="search-result-card">
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
    }
}
