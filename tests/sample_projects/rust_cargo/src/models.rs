use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

impl User {
    pub fn new(id: i32, username: &str, email: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Post {
    pub fn new(id: i32, user_id: i32, title: &str, content: &str) -> Self {
        Self {
            id,
            user_id,
            title: title.to_string(),
            content: content.to_string(),
            published: false,
        }
    }
}
