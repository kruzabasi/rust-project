use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{Email, Password, AuthAPIError}};

pub async fn login(State(state): State<AppState>, Json(req): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(req.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(req.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.write().await;

    if let Err(_e) = user_store.validate_user(&email, &password).await {
        return Err(AuthAPIError::IncorrectCredentials);
    }
    if let Err(_e) = user_store.get_user(&email).await {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let response = Json(LoginResponse {
        message: "User login successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    pub message: String,
}