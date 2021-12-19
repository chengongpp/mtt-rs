use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::error::Error;
use sqlx::{Any, Pool};
use crate::model::user::{User, UserRegister, UserLogin};

pub async fn register(
    req: HttpRequest,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {
    // TODO: captcha check

    Ok(HttpResponse::Ok().json(""))
}

pub async fn login(
    req: HttpRequest,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}

pub async fn modify(
    req: HttpRequest,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().json(""))
}

pub async fn view(req: HttpRequest, pool: web::Data<Pool<Any>>) -> Result<HttpResponse, Error> {
    unimplemented!();
    Ok(HttpResponse::Ok().json(""))
}
