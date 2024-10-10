use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn signup(Json(_req): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}