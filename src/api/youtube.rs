use crate::model::video::Video;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use leptos::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PipedResponse {
    title: String,
    description: String,
    uploader: String, 
    views: Option<u64>, 
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String, 
    tags: Option<Vec<String>>,
    #[serde(rename = "relatedStreams")]
    related_streams: Vec<PipedRelated>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PipedRelated {
    url: String, 
}

const PIPED_INSTANCES: &[&str] = &[
    "https://pipedapi.kavin.rocks",
    "https://pipedapi.tokhmi.xyz",
    "https://pipedapi.moomoo.me",
    "https://api-piped.mha.fi",
];

#[server(FetchVideo, "/api")]
pub async fn fetch_video(video_id: String) -> Result<Video, ServerFnError> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ServerFnError::new(format!("Client build: {}", e)))?;
    
    let mut last_error = String::new();
    
    for base_url in PIPED_INSTANCES {
        let url = format!("{}/streams/{}", base_url, video_id);
        println!("SERVER: Trying {} for video ID: {}", base_url, video_id);
        
        match try_fetch(&client, &url, &video_id).await {
            Ok(video) => {
                println!("SERVER: Success with {}", base_url);
                return Ok(video);
            }
            Err(e) => {
                last_error = e;
                println!("SERVER: Failed with {}: {}", base_url, last_error);
                continue;
            }
        }
    }
    
    Err(ServerFnError::new(format!("All instances failed for video {}. Last error: {}", video_id, last_error)))
}

async fn try_fetch(client: &Client, url: &str, video_id: &str) -> Result<Video, String> {
    let resp = client.get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| format!("Network: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Status: {}", resp.status()));
    }

    let data: PipedResponse = resp.json().await
        .map_err(|e| format!("Parse: {}", e))?;

    let related_ids: Vec<String> = data.related_streams
        .iter()
        .filter_map(|r| {
            r.url
                .split("watch?v=")
                .nth(1)
                .or_else(|| r.url.split("v=").nth(1))
                .map(|s| s.split('&').next().unwrap_or(s).to_string())
        })
        .collect();

    Ok(Video {
        id: None,
        video_id: video_id.to_string(),
        title: data.title,
        description: data.description,
        thumbnail_url: data.thumbnail_url,
        channel_name: data.uploader,
        view_count: data.views.unwrap_or(0),
        category: "Unknown".to_string(),
        tags: data.tags.unwrap_or_default(),
        related_ids,
        saved_at: chrono::Utc::now().to_rfc3339(),
    })
}