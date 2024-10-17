use auth_service::Application;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::app_state::AppState;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = 
        Arc::new(
            RwLock::new(HashmapUserStore {
                users: HashMap::new(),
            }));
            
        let app_state = AppState::new(user_store);
        let app = Application::build( app_state, "127.0.0.1:0")
        .await
        .expect("Failed to build App");
        let address = format!("http://{}", app.address.clone());
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::builder().build().unwrap();

        TestApp { address, http_client }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
        .get(&format!("{}/", &self.address))
        .send()
        .await
        .expect("Failed to execute request.")
    }

    pub async fn login(&self) -> reqwest::Response {
        self.http_client
        .post(&format!("{}/login", &self.address))
        .send()
        .await
        .expect("Failed to execute request.")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
        .post(&format!("{}/logout", &self.address))
        .send()
        .await
        .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_token(&self, jwt_token: &str) -> reqwest::Response {
        self.http_client
        .post(&format!("{}/verify-token", &self.address))
        .send()
        .await
        .expect(&format!("Failed to execute request. {}", jwt_token))
    }

    pub async fn verify_2fa(&self, code: &str) -> reqwest::Response {
        self.http_client
        .post(&format!("{}/verify-2fa", &self.address))
        .send()
        .await
        .expect(&format!("Failed to execute request. {}", code))
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}