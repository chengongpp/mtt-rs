pub struct User {
    pub id: i32,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

pub struct UserDetail {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub phone: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    pub role: String,
    pub status: String,
}

pub struct UserRegister {
    pub username: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

pub struct UserLogin {
    pub login_type: String,
    pub identity: String,
    pub password: String,
    pub captcha: Option<String>,
}

pub struct UserUpdate {
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct UserAvatar {
    pub id: i32,
    pub avatar: String,
}