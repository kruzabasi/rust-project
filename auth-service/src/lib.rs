use axum::{Router, serve::Serve};

use std::error::Error;
use tokio::net::TcpListener;

pub struct Application {
   server: Serve<Router, Router>,
   pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new();
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