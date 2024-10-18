

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self, String> {
        if password.len() >= 8 {
            Ok(Self(password))
        } else {
            Err("Invalid password".to_string())
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}