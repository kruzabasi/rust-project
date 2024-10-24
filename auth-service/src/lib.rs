use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}, 
    routing::post, 
    serve::Serve,
    Json, Router
};

use tower_http::services::ServeDir;

use std::error::Error;
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};

use routes::{login, logout, signup, verify_2fa, verify_token};
use app_state::AppState;
use domain::AuthAPIError;

pub mod routes;
pub mod app_state;
pub mod services;
pub mod domain;
pub mod utils;
pub struct Application {
   server: Serve<Router, Router>,
   pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/verify-token", post(verify_token))
        .route("/verify-2fa", post(verify_2fa))
        .with_state(app_state);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }
    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("Server running at {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::MalformedCredentials => (StatusCode::UNPROCESSABLE_ENTITY, "Malformed credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}