use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState, 
    domain::{Email, Password, AuthAPIError},
    utils::auth::generate_auth_cookie
};

pub async fn login(
    State(state): State<AppState>, 
    jar: CookieJar,
    Json(req): Json<LoginRequest>
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {

    let email = match Email::parse(req.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(req.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.write().await;


    if let Err(_e) = user_store.validate_user(&email, &password).await {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }
    if let Err(_e) = user_store.get_user(&email).await {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let _ = Json(LoginResponse {
        message: "User login successfully!".to_string(),
    });

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_e) => return (jar, Err(AuthAPIError::UnexpectedError),)
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
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