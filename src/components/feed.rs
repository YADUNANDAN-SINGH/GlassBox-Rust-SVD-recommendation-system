use crate::model::db::DB;
use crate::model::video::Video;
use leptos::prelude::*;

#[component]
pub fn Feed() -> impl IntoView {
    // Use signals instead of Resource to avoid Send issues
    let videos = RwSignal::new(Vec::<Video>::new());
    let loading = RwSignal::new(true);

    // spawn_local doesn't require Send — perfect for WASM
    leptos::task::spawn_local(async move {
        leptos::logging::log!("FEED: Starting fetch...");

        // Wait for DB to be ready (retry up to 15 times, ~3 seconds)
        let mut db_ref = None;
        for attempt in 1..=15 {
            if let Some(db) = DB.get() {
                db_ref = Some(db);
                break;
            }
            leptos::logging::log!("FEED: DB not ready, attempt {}/15", attempt);
            // Use gloo_timers for async delay
            gloo_timers::future::TimeoutFuture::new(200).await;
        }

        match db_ref {
            Some(db) => {
                leptos::logging::log!("FEED: DB acquired, querying 'video' table...");
                let result: Result<Vec<Video>, _> = db.select("video").await;
                match result {
                    Ok(v) => {
                        leptos::logging::log!("FEED: Fetched {} videos", v.len());
                        for vid in &v {
                            leptos::logging::log!("  - {}", vid.title);
                        }
                        videos.set(v);
                    }
                    Err(e) => {
                        leptos::logging::error!("FEED: Query error: {}", e);
                    }
                }
            }
            None => {
                leptos::logging::error!("FEED: Database not available after 15 retries!");
            }
        }
        loading.set(false);
    });

    view! {
        <div class="feed-container" style="margin-top: 50px; padding: 20px;">
            <h2 style="color: white; border-bottom: 1px solid #333; padding-bottom: 10px;">
                "Your Library (Database)"
            </h2>

            {move || {
                let v = videos.get();
                let is_loading = loading.get();

                if is_loading {
                    view! { <p style="color: #888;">"Loading feed..."</p> }.into_any()
                } else if v.is_empty() {
                    view! {
                        <div style="text-align: center; margin-top: 30px; color: #666;">
                            <p>"Your database is empty."</p>
                            <p>"Search for a movie and click it to save!"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="movie-grid" style="display: grid; grid-template-columns: repeat(auto-fill, minmax(150px, 1fr)); gap: 20px; margin-top: 20px;">
                            {v.into_iter().map(|video| view! {
                                <div class="movie-card" style="background: rgba(255,255,255,0.05); padding: 10px; border-radius: 8px;">
                                    <img src={video.thumbnail_url} style="width: 100%; border-radius: 4px;" />
                                    <h4 style="color: white; font-size: 0.9rem; margin-top: 5px;">{video.title}</h4>
                                    <div style="font-size: 0.7rem; color: #aaa;">
                                        {video.genres.join(", ")}
                                    </div>
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
