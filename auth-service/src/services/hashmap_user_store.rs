use std::collections::HashMap;
use crate::domain::{User, data_stores::{UserStoreError, UserStore}};


#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        self.add_user(user)
    }
    async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.get_user(email)
    }
    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        self.validate_user(email, password)
    }
}

pub struct HashmapUserStore {
   pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
             return Err(UserStoreError::UserAlreadyExists);
         }
         self.users.insert(user.email.clone(), user);
         Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let waiting_user = self.get_user(&email);
        match waiting_user {
            Ok(waiting_user) => {
                if waiting_user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            Err(_) => Err(UserStoreError::UserNotFound),
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore { users: HashMap::new() };
        let new_user: User = User::new("test@example.com".to_string(), "password123".to_string(), true);
        assert!(store.add_user(new_user.clone()).is_ok());
        assert!(store.add_user(new_user).is_err());
        assert_eq!(store.get_user("test@example.com").unwrap().email, "test@example.com");
        assert_eq!(store.get_user("invalid_email").unwrap_err(), UserStoreError::UserNotFound);
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore { users: HashMap::new() };
        let user = User::new("test@example.com".to_string(), "password123".to_string(), true);
        store.add_user(user).unwrap();
        assert_eq!(store.get_user("test@example.com").unwrap().email, "test@example.com");
        assert_eq!(store.get_user("invalid_email").unwrap_err(), UserStoreError::UserNotFound);

    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore { users: HashMap::new() };
        let user = User::new("test@example.com".to_string(), "password123".to_string(), true);
        store.add_user(user).unwrap();
        assert!(store.validate_user("test@example.com", "password123").is_ok());
        assert_eq!(store.validate_user("test@example.com", "invalid_password").unwrap_err(), UserStoreError::InvalidCredentials);
        assert_eq!(store.validate_user("invalid_email", "password123").unwrap_err(), UserStoreError::UserNotFound);
    }
}