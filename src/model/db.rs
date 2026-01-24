use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb::engine::local::IndxDb;
use std::sync::OnceLock;

// Wrapper to allow Surreal<Db> in OnceLock (WASM is single-threaded, so this is safe-ish)
pub struct SafeSurreal(pub Surreal<Db>);
unsafe impl Sync for SafeSurreal {}
unsafe impl Send for SafeSurreal {}

impl std::ops::Deref for SafeSurreal {
    type Target = Surreal<Db>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub static DB: OnceLock<SafeSurreal> = OnceLock::new(); 

pub async fn init_db() {
    // This creates a folder inside the user's browser named "glassbox"
    match Surreal::new::<IndxDb>("glassbox").await {
        Ok(client) => {
            if let Err(e) = client.use_ns("user_private").use_db("history").await {
                leptos::logging::error!("Failed to select DB: {:?}", e);
                return;
            }
            let _ = DB.set(SafeSurreal(client));
            leptos::logging::log!("GlassBox Secure Vault: ONLINE");
        },
        Err(e) => {
            leptos::logging::error!("CRITICAL: Could not init Local DB: {:?}", e);
        }
    }
}

