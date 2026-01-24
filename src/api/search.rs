use crate::model::video::Video;
use serde::Deserialize;
use reqwest::Client;
use chrono::Utc;

// --- TVMAZE JSON STRUCTURES ---
#[derive(Deserialize, Debug)]
struct TvMazeSearchItem {
    show: TvMazeShow,
}

#[derive(Deserialize, Debug)]
struct TvMazeShow {
    id: u64,
    name: String,
    summary: Option<String>, // HTML string
    image: Option<TvMazeImage>,
    network: Option<TvMazeNetwork>,
    rating: Option<TvMazeRating>,
    genres: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct TvMazeImage {
    medium: String, // The poster URL
}

#[derive(Deserialize, Debug)]
struct TvMazeNetwork {
    name: String,
}

#[derive(Deserialize, Debug)]
struct TvMazeRating {
    average: Option<f64>,
}

// --- PUBLIC FUNCTION ---
// No #[server] needed! This works directly in the browser.
pub async fn search_videos(query: &str) -> Result<Vec<Video>, String> {
    let url = format!("https://api.tvmaze.com/search/shows?q={}", query);
    
    let client = Client::new();
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("API Error: {}", resp.status()));
    }

    let results: Vec<TvMazeSearchItem> = resp.json().await.map_err(|e| e.to_string())?;

    // Convert to GlassBox "Video" Model
    let videos = results.into_iter().map(|item| {
        let s = item.show;
        
        // Clean up the summary (Remove <p> tags)
        let raw_desc = s.summary.unwrap_or("No description".to_string());
        let clean_desc = raw_desc.replace("<p>", "").replace("</p>", "").replace("<b>", "").replace("</b>", "");

        Video {
            id: None,
            video_id: s.id.to_string(),
            title: s.name,
            description: clean_desc,
            thumbnail_url: s.image.map(|i| i.medium).unwrap_or("https://via.placeholder.com/210x295?text=No+Image".to_string()),
            channel_name: s.network.map(|n| n.name).unwrap_or("Web Series".to_string()),
            rating: s.rating.and_then(|r| r.average).unwrap_or(0.0),
            genres: s.genres, // CRITICAL FOR YOUR AI
            related_ids: vec![], // We will fill this later
            saved_at: Utc::now(),
        }
    }).collect();

    Ok(videos)
}