use actix_web::{HttpResponse, web};
use actix_web::error::Error;
use sqlx::{Any, Pool};
use model::user::{User, UserRegister, UserLogin};

pub async fn register(
    user: UserRegister,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {
    // TODO: captcha check
    let conn = pool.get().await?;
    let user = user.into_inner();
    let user = User::new(user.username, user.password);
    let user = user.register(&conn).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn login(
    user: UserLogin,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {
    let user = user.into_inner();
    let user = User::new(user.username, user.password);
    let conn = pool.get().await?;
    let user = user.login(&conn).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn modify(
    user: User,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {
    let user = user.into_inner();
    let conn = pool.get().await?;
    Ok(HttpResponse::Ok().json(user))
}

