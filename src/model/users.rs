use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::model::db::DB; 
use bcrypt::{hash, DEFAULT_COST, verify}; 

pub async fn is_email_taken(email:String) -> Result<bool, String> {
    let db = DB.get().ok_or("Database not initialized")?;

    let sql = "SELECT * FROM user WHERE email = $email";
    let mut response = db.query(sql).bind(("email", email)).await.map_err(|e| e.to_string())?;

    // SELECT returns a list, so we deserialize into Vec<User>
    let users: Vec<User> = response.take(0).map_err(|e| e.to_string())?; 
    Ok(!users.is_empty())
}

pub async fn is_username_taken(username: String) -> Result<bool, String> {
    let db = DB.get().ok_or("Database not initialized")?;

    let sql = "SELECT * FROM user WHERE username = $username";
    let mut response = db.query(sql).bind(("username", username)).await.map_err(|e| e.to_string())?;

    let users: Vec<User> = response.take(0).map_err(|e| e.to_string())?;
    Ok(!users.is_empty())
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<Thing>, 
    pub email:String,
    pub username: String,
    pub joined_at: String,
    pub password: String 
}

// Function to create a new user in the local database. wirh secure HASHED password.
pub async fn create_new_user(username: String, email: String, raw_password: String) -> Result<User, String> {
    let db = DB.get().ok_or("Database not initialized")?;

    // Hash password
    let hashed_password = hash(raw_password, DEFAULT_COST).map_err(|e| e.to_string())?;
    
    let new_user = User {
        id: None, // SurrealDB generates ID
        username: username.clone(),
        email: email.clone(),
        joined_at: chrono::Utc::now().to_rfc3339(),
        password: hashed_password
    };

    // Create user in DB
    // We use "create" to ensure we get a return value. 
    // Table is "user". Content is the struct.
    let created: Option<User> = db.create("user").content(new_user).await.map_err(|e| e.to_string())?;
    
    created.ok_or("Failed to create user".to_string())
}

pub async fn login_user(username: String, password: String) -> Result<User, String> {
    leptos::logging::log!("LOGIN ATTEMPT: Starting for user '{}'", username);
    
    let db = DB.get().ok_or("Database not initialized")?;
    leptos::logging::log!("DB ACQUIRED");

    let sql = "SELECT * FROM user WHERE username = $username";
    leptos::logging::log!("QUERYING USER...");
    let mut response = db.query(sql).bind(("username", username)).await.map_err(|e| e.to_string())?;

    let users: Vec<User> = response.take(0).map_err(|e| e.to_string())?;
    leptos::logging::log!("QUERY COMPLETE. Users found: {}", users.len());
    
    if let Some(user) = users.first() {
        leptos::logging::log!("VERIFYING PASSWORD...");
        if verify(&password, &user.password).map_err(|e| e.to_string())? {
            leptos::logging::log!("LOGIN SUCCESS");
            return Ok(user.clone());
        } else {
             leptos::logging::log!("PASSWORD MISMATCH");
        }
    } else {
         leptos::logging::log!("USER NOT FOUND");
    }
    
    Err("Invalid username or password".to_string())
}
