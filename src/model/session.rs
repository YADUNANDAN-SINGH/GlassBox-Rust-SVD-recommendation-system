use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub user_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Copy, Clone)]
pub struct SessionState(pub WriteSignal<Session>, pub ReadSignal<Session>);

impl SessionState {
    pub fn new() -> Self {
        let (get, set) = signal(Session {
            user_id: None,
            username: None,
        });

        // Try to load from LocalStorage on startup
        if let Some(window) = web_sys::window() {
             if let Ok(Some(storage)) = window.local_storage() {
                 if let Ok(Some(u)) = storage.get_item("glassbox_user") {
                     set.set(Session { user_id: Some("saved".to_string()), username: Some(u) });
                 }
             }
        }
        
        Self(set, get)
    }

    pub fn login(&self, username: String) {
        // Save to Signal
        (self.0).set(Session {
            user_id: Some("logged-in".to_string()),
            username: Some(username.clone()),
        });

        // Save to LocalStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("glassbox_user", &username);
            }
        }
    }

    pub fn logout(&self) {
        // Clear Signal
        (self.0).set(Session {
            user_id: None,
            username: None,
        });

        // Clear LocalStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("glassbox_user");
            }
        }
        
        // Optional: Redirect to Home
        // Note: usage of use_navigate might require being in a component context or using <A>
        // For simplicity in a helper struct, we might skip navigation or use window location
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href("/");
        }
    }
}
