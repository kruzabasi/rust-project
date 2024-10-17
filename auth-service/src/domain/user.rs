use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}