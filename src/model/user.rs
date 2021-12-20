use serde::{Serialize, Deserialize};
use sqlx::{Any, AnyPool, Pool, Row};

pub type UserId = i32;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

pub enum UserError {
    UsernameAlreadyExists,
    EmailAlreadyExists,
    PhoneAlreadyExists,
    UserNotFound,
    InvalidPassword,
    DatabaseError,
}

pub enum UserLoginType {
    Username,
    Email,
    Phone,
}

impl User {
    pub async fn find_by_id(id: i32, pool: &Pool<Any>) -> Result<User, UserError> {
        let rec = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
            .bind(id)
            .fetch_optional(pool).await;
        match rec {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(UserError::UserNotFound),
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    pub async fn find_by_login_type(identifier: &str, login_type: UserLoginType, pool: &AnyPool) -> Result<User, UserError> {
        let rec = match login_type {
            UserLoginType::Username => {
                sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = ?")
            }
            UserLoginType::Email => {
                sqlx::query_as::<_, User>("SELECT * FROM user WHERE email = ?")
            }
            UserLoginType::Phone => {
                sqlx::query_as::<_, User>("SELECT * FROM user WHERE phone = ?")
            }
        }.bind(identifier).fetch_optional(pool).await;
        match rec {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(UserError::UserNotFound),
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    pub async fn create(username: &str, phone: &str, email: &str, password: &str, pool: &AnyPool) -> Result<User, UserError> {
        let rec = sqlx::query_as::<_, User>("INSERT INTO user (username, phone, email, password) VALUES (?, ?, ?, ?)")
            .bind(username)
            .bind(phone)
            .bind(email)
            .bind(password)
            .execute(pool).await;
        match rec {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    pub async fn update_password(id: i32, password: &str, pool: &AnyPool) -> Result<User, UserError> {
        let rec = sqlx::query_as::<_, User>("UPDATE user SET password = ? WHERE id = ?")
            .bind(password)
            .bind(id)
            .execute(pool).await;
        match rec {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    pub async fn update_phone(id: i32, phone: &str, pool: &AnyPool) -> Result<User, UserError> {
        let rec = sqlx::query_as::<_, User>("UPDATE user SET phone = ? WHERE id = ?")
            .bind(phone)
            .bind(id)
            .execute(pool).await;
        match rec {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::DatabaseError),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserDetail {
    pub id: i32,
    pub nickname: String,
    pub avatar: String,
    pub created_at: String,
    pub updated_at: String,
}

impl UserDetail {
    pub async fn get_by_id(id: i32, pool: &Pool<Any>) -> Result<UserDetail, UserError> {
        let rec = sqlx::query_as::<_, UserDetail>("SELECT * FROM user_detail WHERE id = ?")
            .bind(id)
            .fetch_optional(pool).await;
        match rec {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(UserError::UserNotFound),
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    pub async fn create(id: i32, nickname: &str, pool: &AnyPool) -> Result<UserDetail, UserError> {
        let rec = sqlx::query_as::<_, UserDetail>("INSERT INTO user_detail (id, nickname) VALUES (?, ?)")
            .bind(id)
            .bind(nickname)
            .execute(pool).await;
        match rec {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    pub async fn update_nickname(id: i32, nickname: &str, pool: &AnyPool) -> Result<UserDetail, UserError> {
        let rec = sqlx::query_as::<_, UserDetail>("UPDATE user_detail SET nickname = ? WHERE id = ?")
            .bind(nickname)
            .bind(id)
            .execute(pool).await;
        match rec {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    pub async fn update_avatar(id: i32, avatar: &str, pool: &AnyPool) -> Result<UserDetail, UserError> {
        let rec = sqlx::query_as::<_, UserDetail>("UPDATE user_detail SET avatar = ? WHERE id = ?")
            .bind(avatar)
            .bind(id)
            .execute(pool).await;
        match rec {
            Ok(user) => Ok(user),
            Err(_) => Err(UserError::DatabaseError),
        }
    }
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
    pub identifier: String,
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
    pub message: Option<String>,
}
