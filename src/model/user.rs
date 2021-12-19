use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDetail {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub phone: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub login_type: String,
    pub identity: String,
    pub password: String,
    pub captcha: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserAvatar {
    pub id: i32,
    pub avatar: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: Option<String>
}
