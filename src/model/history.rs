use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::model::db::DB; 
use crate::model::video::Video;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchEntry {
    pub id: Option<Thing>,
    pub user: Thing,
    pub query: String,
    pub timestamp: String,
}

pub async fn save_search(user_id: Thing, query: String) -> Result<(), String> {
    let db = DB.get().ok_or("Database not initialized")?;
    let entry = SearchEntry {
        id: None,
        user: user_id,
        query,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    // Use "search_history" table
    let _: Option<SearchEntry> = db.create("search_history").content(entry).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionEntry {
    pub id: Option<Thing>,
    pub user: Thing,
    pub video_id: String,
    pub video_title: String,
    pub interaction_type: String, // "click", "view"
    pub timestamp: String,
}

pub async fn save_interaction(user_id: Thing, video: Video, interaction_type: String) -> Result<(), String> {
    let db = DB.get().ok_or("Database not initialized")?;
    
    let entry = InteractionEntry {
        id: None,
        user: user_id,
        video_id: video.video_id,
        video_title: video.title,
        interaction_type,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    // Use "interaction" table
    let _: Option<InteractionEntry> = db.create("interaction").content(entry).await.map_err(|e| e.to_string())?;
    Ok(())
}
