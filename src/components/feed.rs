use crate::api::search::search_videos;
use crate::model::db::DB;
use crate::model::svd::SVD;
use crate::model::video::Video;
use leptos::prelude::*;

#[component]
pub fn Feed() -> impl IntoView {
    // Use signals instead of Resource to avoid Send issues
    let videos = RwSignal::new(Vec::<Video>::new());
    let loading = RwSignal::new(true);
    let genre_title = RwSignal::new("Your Library".to_string()); // Dynamic title

    let feed_trigger = use_context::<crate::model::feed_control::FeedTrigger>();

    // spawn_local doesn't require Send — perfect for WASM
    Effect::new(move |_| {
        leptos::logging::log!("FEED: Effect triggered!");

        // Subscribe to trigger if available
        if let Some(t) = feed_trigger {
            t.0.track();
            leptos::logging::log!("FEED: Tracking trigger signal value: {}", t.0.get());
        } else {
            leptos::logging::error!("FEED: FeedTrigger context NOT FOUND!");
        }

        loading.set(true); // <--- Add this line!

        leptos::task::spawn_local(async move {
            leptos::logging::log!("FEED: Starting recommendation engine...");

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
                    leptos::logging::log!("FEED: DB acquired, fetching 'library'...");
                    let result: Result<Vec<Video>, _> = db.select("video").await;

                    match result {
                        Ok(library) => {
                            if library.is_empty() {
                                leptos::logging::log!(
                                    "FEED: Library is empty. No recommendations."
                                );
                                loading.set(false);
                                return;
                            }

                            // 1. Calculate User Vector
                            let user_vec = SVD::user_vector(&library);

                            // 2. Get Top Genre
                            let top_genre = SVD::get_top_genre(&user_vec);
                            leptos::logging::log!("FEED: Top Genre determined: {}", top_genre);
                            genre_title.set(format!("Recommended for you ({})", top_genre));

                            // 3. Fetch Candidates (Search API)
                            leptos::logging::log!(
                                "FEED: Fetching candidates for genre: {}",
                                top_genre
                            );
                            match search_videos(&top_genre).await {
                                Ok(mut candidates) => {
                                    // 4. Filter out movies already in library
                                    candidates.retain(|c| {
                                        !library.iter().any(|l| l.video_id == c.video_id)
                                    });

                                    // 5. Rank Candidates (SVD)
                                    candidates.sort_by(|a, b| {
                                        let score_a = SVD::predict_match(&user_vec, a);
                                        let score_b = SVD::predict_match(&user_vec, b);
                                        score_b
                                            .partial_cmp(&score_a)
                                            .unwrap_or(std::cmp::Ordering::Equal)
                                    });

                                    leptos::logging::log!(
                                        "FEED: Ranked {} candidates",
                                        candidates.len()
                                    );
                                    videos.set(candidates);
                                }
                                Err(e) => {
                                    leptos::logging::error!("FEED: API Error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            leptos::logging::error!("FEED: Database Error: {}", e);
                        }
                    }
                }
                None => {
                    leptos::logging::error!("FEED: Database not available after 15 retries!");
                }
            }
            loading.set(false);
        });
    });

    view! {
        <div class="feed-container" style="margin-top: 50px; padding: 20px;">
            <h2 style="color: white; border-bottom: 1px solid #333; padding-bottom: 10px;">
                {move || genre_title.get()}
            </h2>

            {move || {
                let v = videos.get();
                let is_loading = loading.get();

                if is_loading {
                    view! { <p style="color: #888;">"Analyzing your taste..."</p> }.into_any()
                } else if v.is_empty() {
                    view! {
                        <div style="text-align: center; margin-top: 30px; color: #666;">
                            <p>"Not enough data for recommendations."</p>
                            <p>"Search and click movies to build your profile!"</p>
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
