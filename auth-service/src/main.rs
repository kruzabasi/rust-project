use auth_service::{
    Application, 
    app_state::AppState,
    utils::constants::prod,
    services::{
        hashset_banned_token_store::HashsetBannedTokenStore,
        hashmap_user_store::HashmapUserStore
    }};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = 
        Arc::new(
            RwLock::new(HashmapUserStore {
                users: HashMap::new(),
    }));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let app_state = AppState::new(user_store, banned_token_store);
    let app = Application::build(app_state, prod::APP_ADDRESS)
    .await
    .expect("Failed to build App");

    app.run().await.expect("Failed to run App");
}