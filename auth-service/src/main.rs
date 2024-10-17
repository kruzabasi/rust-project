use auth_service::Application;
use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;

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
    let app_state = AppState::new(user_store);
    let app = Application::build(app_state, "0.0.0.0:3000")
    .await
    .expect("Failed to build App");

    app.run().await.expect("Failed to run App");
}