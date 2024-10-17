use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User, services::UserStoreError};

pub async fn signup(State(state): State<AppState>, Json(req): Json<SignupRequest>) -> impl IntoResponse {
    let user = User::new(req.email, req.password, true);
    let mut user_store = state.user_store.write().await;
    let ok_response = Json(SignupResponse { message: "User created successfully!".to_string() });

 
    match user_store.add_user(user) {
        Ok(_) => ((), (StatusCode::CREATED, ok_response)),
        Err(UserStoreError::UserAlreadyExists) => ((), (StatusCode::CONFLICT, Json(SignupResponse { message: "User already exists!".to_string() }))),
        Err(_) => ((), (StatusCode::INTERNAL_SERVER_ERROR, Json(SignupResponse { message: "Unexpected error occurred!".to_string() })))
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}