use serde::{Deserialize, Serialize};
use surrealdb::Response;
use crate::model::db::DB; // <------- Import the connection we made in db.rs
use bcrypt::{hash, DEFAULT_COST, verify}; // <------- This helps us to store hashed password NOT real passwrod.



pub async fn is_email_taken(email:String) -> Result<bool, String> {
    let db = DB.get().ok_or("Database not included")?;

    let sql:&str = "SELECT * FROM user WHERE email = $email";
    let mut response:Response = db.query(sql).bind(("email", email)).await.map_err(|e| e.to_string())?;

    // SELECT returns a list, so we deserialize into Vec<User>
    let users: Vec<User> = response.take(0).map_err(|e| e.to_string())?; 
    Ok(!users.is_empty())
}

pub async fn is_username_taken(username: String) -> Result<bool, String> {
    let db = DB.get().ok_or("Database not included")?;

    let sql:&str = "SELECT * FROM user WHERE username = $username";
    let mut response:Response = db.query(sql).bind(("username", username)).await.map_err(|e| e.to_string())?;

    let users: Vec<User> = response.take(0).map_err(|e| e.to_string())?;
    Ok(!users.is_empty())
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<surrealdb::RecordId>, // <------- SurrealDB will generate a unique ID (like 'user:kb73...').
    pub email:String,
    pub username: String,
    pub joined_at: String,
    pub password: String // <------- stores the hash not the real password. 
}

// Function to create a new user in the local database. wirh secure HASHED password.
pub async fn create_new_user(username: String, email: String, raw_password: String) -> Result<User, String> {
    let db = DB.get().ok_or("Database not loaded yet!")?;

    // 1. SECURITY: Hash the password before doing anything else
    // 'DEFAULT_COST' determines how hard it is to crack (usually 12)
    let hashed_password = match hash(raw_password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Err("Failed to encrypt password".to_string()),
    };

    // Check if user already exists to prevent duplicates (Server-side validation)
    if is_email_taken(email.clone()).await? {
        return Err("Email already exists".to_string());
    }
    if is_username_taken(username.clone()).await? {
        return Err("Username already exists".to_string());
    }

    // 2. Insert into DB (Saving the Hash, NOT the password)
    // We use a query parameter block to keep it clean
    let sql = "
        CREATE user CONTENT {
            username: $username,
            email: $email,
            password: $password,
            joined_at: $joined_at
        }
    ";

    let mut response = db
        .query(sql)
        .bind(("username", username))
        .bind(("email", email)) 
        .bind(("password", hashed_password)) 
        .bind(("joined_at", "2024-01-01T00:00:00Z")) // Safe hardcoded time for now or use formatted string
        .await
        .map_err(|e| e.to_string())?;

    let user: Option<User> = response.take(0).map_err(|e| e.to_string())?;
    user.ok_or("Failed to retrieve created user".to_string())
}

pub async fn login_user(username: String, password: String) -> Result<User, String> {
    leptos::logging::log!("LOGIN ATTEMPT: Starting for user '{}'", username);
    let db = DB.get().ok_or("Database not loaded")?;

    // 1. Find the user by username
    let sql = "SELECT * FROM user WHERE username = $username";
    let mut response: Response = db.query(sql)
        .bind(("username", username.clone()))
        .await
        .map_err(|e| e.to_string())?;

    // Get the first user found (if any)
    let users: Vec<User> = response.take(0).map_err(|e| e.to_string())?;
    
    // 2. Check if user exists
    if let Some(user) = users.first() {
        leptos::logging::log!("LOGIN: User found: {}", user.username);
        // 3. Verify the password against the stored hash
        let valid = verify(&password, &user.password).unwrap_or(false);
        if valid {
            leptos::logging::log!("LOGIN: Password verification SUCCESS");
            Ok(user.clone())
        } else {
            leptos::logging::log!("LOGIN: Password verification FAILED");
            Err("Invalid password".to_string())
        }
    } else {
        leptos::logging::log!("LOGIN: User '{}' NOT FOUND", username);
        Err("User not found".to_string())
    }
}
