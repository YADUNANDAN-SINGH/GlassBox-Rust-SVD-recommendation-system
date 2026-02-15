// <------- This is the file where I am gonna write the main recommendation model engine for user feed ------->

use crate::model::video::Video;

const ALL_GENRES: &[&str] = &[
    "Action",
    "Adventure",
    "Anime",
    "Comedy",
    "Crime",
    "Drama",
    "Espionage",
    "Family",
    "Fantasy",
    "History",
    "Horror",
    "Music",
    "Mystery",
    "Romance",
    "Science-Fiction",
    "Supernatural",
    "Thriller",
    "War",
    "Western",
];

pub struct SVD;

impl SVD {
    // <------- vectorise the genres ------->
    pub fn vectorise(video: &Video) -> Vec<f64> {
        let mut vec = vec![0.0; ALL_GENRES.len()];

        for (i, genre_key) in ALL_GENRES.iter().enumerate() {
            if video
                .genres
                .iter()
                .any(|g| g.eq_ignore_ascii_case(genre_key))
            {
                vec[i] = 1.0;
            }
        }
        vec
    }

    // <------- Calculate the vector base on user watch history ------->
    pub fn user_vector(history: &[Video]) -> Vec<f64> {
        let mut user_vec = vec![0.0; ALL_GENRES.len()];

        if history.is_empty() {
            return user_vec;
        }

        for video in history {
            let vec_vdo = Self::vectorise(video);

            // We weigh the vector by the rating (if available) or default to 5.0
            // Since Video struct usually has rating, we use it. If 0.0, assume implied interest (e.g. 5.0)
            let weight = if video.rating > 0.0 {
                video.rating
            } else {
                5.0 // Default weight for interaction
            };

            for i in 0..vec_vdo.len() {
                user_vec[i] += vec_vdo[i] * weight;
            }
        }

        // Normalize the vector
        let count = history.len() as f64;
        for i in 0..user_vec.len() {
            user_vec[i] /= count;
        }

        user_vec
    }

    pub fn predict_match(user_vector: &[f64], video: &Video) -> f64 {
        if user_vector.len() != ALL_GENRES.len() {
            return 0.0;
        }

        let vid_vec = Self::vectorise(video);
        let mut score = 0.0;

        for i in 0..ALL_GENRES.len() {
            score += user_vector[i] * vid_vec[i];
        }

        score
    }

    pub fn get_top_genre(user_vector: &[f64]) -> String {
        if user_vector.len() != ALL_GENRES.len() {
            return "Trending".to_string();
        }

        let mut max_score = -1.0;
        let mut best_idx = 0;

        for (i, score) in user_vector.iter().enumerate() {
            if *score > max_score {
                max_score = *score;
                best_idx = i;
            }
        }

        if max_score <= 0.0 {
            "Trending".to_string() // Fallback
        } else {
            ALL_GENRES[best_idx].to_string()
        }
    }
}
