use leptos::prelude::*;
use crate::model::users::User;

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub user_id: Option<String>, // Serialized SurrealDB Thing
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
                 let u = storage.get_item("glassbox_username").unwrap_or(None);
                 let uid = storage.get_item("glassbox_userid").unwrap_or(None);
                 
                 if let (Some(username), Some(userid)) = (u, uid) {
                     set.set(Session { user_id: Some(userid), username: Some(username) });
                 }
             }
        }
        
        Self(set, get)
    }

    pub fn login(&self, user: User) {
        // Extract ID. If None, we can't track history, but allow login?
        // SurrealDB users should always have ID.
        let uid_str = user.id.map(|t| t.to_string());

        // Save to Signal
        (self.0).set(Session {
            user_id: uid_str.clone(),
            username: Some(user.username.clone()),
        });

        // Save to LocalStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Some(final_uid) = uid_str {
                     let _ = storage.set_item("glassbox_userid", &final_uid);
                }
                let _ = storage.set_item("glassbox_username", &user.username);
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
                let _ = storage.remove_item("glassbox_username");
                let _ = storage.remove_item("glassbox_userid");
            }
        }
        
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href("/");
        }
    }
}
