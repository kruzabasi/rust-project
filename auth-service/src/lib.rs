use axum::{routing::post, serve::Serve, Router};
use tower_http::services::ServeDir;

use std::error::Error;
use tokio::net::TcpListener;

use routes::{login, logout, signup, verify_2fa, verify_token};

pub mod routes;
pub struct Application {
   server: Serve<Router, Router>,
   pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/verify-token", post(verify_token))
        .route("/verify-2fa", post(verify_2fa));

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