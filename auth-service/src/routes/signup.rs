use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::domain::{Email, Password};

use crate::{app_state::AppState, domain::{AuthAPIError, User}};

    pub async fn signup(State(state): State<AppState>, Json(req): Json<SignupRequest>) -> impl IntoResponse {
        let email = Email::parse(req.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
        let password = Password::parse(req.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    
        let mut user_store = state.user_store.write().await;

        let user = User::new(email, password, req.requires_2fa);

        if user_store.get_user(&user.email).await.is_ok() {
            return Err(AuthAPIError::UserAlreadyExists);
        }
    
        if user_store.add_user(user).await.is_err() {
            return Err(AuthAPIError::UnexpectedError);
        }
    
        let response = Json(SignupResponse {
            message: "User created successfully!".to_string(),
        });
    
        Ok((StatusCode::CREATED, response))
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