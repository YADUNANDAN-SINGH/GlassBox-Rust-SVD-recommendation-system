use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use chrono::{DateTime, Utc};
use crate::model::db::DB;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub id: Option<RecordId>,

    // TVMaze ID is a number (e.g., 169), but we store as String for consistency
    pub video_id: String,       
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    
    // Discovery Signals
    pub rating: f64,            // 1.0 - 10.0
    pub genres: Vec<String>,    // ["Drama", "Sci-Fi"] -> Perfect for AI
    
    // We can fetch "Cast" or "Crew" later for more graph connections
    pub channel_name: String,   // We'll use "Network" here (e.g., "HBO")
    
    pub related_ids: Vec<String>, 
    pub saved_at: DateTime<Utc>,
}

// --- DATABASE SAVE FUNCTION (Unchanged Logic) ---
pub async fn save_video(video: Video) -> Result<Video, String> {
    let db = DB.get().ok_or("Database not loaded")?;
    let record_id = RecordId::from(("video", video.video_id.as_str()));
    
    let result: Result<Option<Video>, _> = db
        .update(record_id)
        .content(video)
        .await;

    match result {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Err("Save failed".to_string()),
        Err(e) => Err(e.to_string()),
    }
}