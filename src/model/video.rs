use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)] // Added PartialEq for signals
pub struct Video {
    pub id: Option<Thing>, // SurrealDB native ID type

    // TVMaze ID is a number (e.g., 169), but we store as String for consistency
    pub video_id: String,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,

    // Discovery Signals
    pub rating: f64,         // 1.0 - 10.0
    pub genres: Vec<String>, // ["Drama", "Sci-Fi"] -> Perfect for AI

    // We can fetch "Cast" or "Crew" later for more graph connections
    pub channel_name: String, // We'll use "Network" here (e.g., "HBO")

    pub related_ids: Vec<String>,
    pub saved_at: DateTime<Utc>,
}

// --- DATABASE SAVE FUNCTION ---
pub async fn save_video(video: Video) -> Result<Video, String> {
    leptos::logging::log!("SAVE_VIDEO: Attempting to save video: {}", video.title);

    let db = crate::model::db::DB.get().ok_or_else(|| {
        leptos::logging::error!("SAVE_VIDEO: Database not loaded!");
        "Database not loaded".to_string()
    })?;

    leptos::logging::log!(
        "SAVE_VIDEO: DB acquired, saving with id: {}",
        video.video_id
    );

    // Create a new record or update existing
    // We use the video_id as the key. Table is "video".
    // 2.0 syntax: ("video", id)
    let id_str = video.video_id.clone();

    // We need to clone video because .content() consumes it
    let content = video.clone();

    // Use .upsert to create OR update (update only works on existing records)
    let result: Result<Option<Video>, _> = db.upsert(("video", id_str)).content(content).await;

    match result {
        Ok(Some(v)) => {
            leptos::logging::log!("SAVE_VIDEO: Successfully saved: {}", v.title);
            Ok(v)
        }
        Ok(None) => {
            leptos::logging::error!("SAVE_VIDEO: Save returned None");
            Err("Save failed".to_string())
        }
        Err(e) => {
            leptos::logging::error!("SAVE_VIDEO: Error: {}", e);
            Err(e.to_string())
        }
    }
}
