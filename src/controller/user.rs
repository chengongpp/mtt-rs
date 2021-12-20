use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::error::Error;
use sqlx::{Any, Pool};
use crate::model::user::{User, UserId, UserRegister, UserLogin, UserError};

pub async fn register(
    req: HttpRequest,
    data: web::Data<UserRegister>,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {
    // TODO: captcha check
    if data.username.is_empty() || data.password.is_empty() {
        return Ok(HttpResponse::BadRequest().finish());
    }


    Ok(HttpResponse::Ok().json(""))
}

pub async fn login(
    req: HttpRequest,
    data: web::Data<UserLogin>,
    pool: web::Data<Pool<Any>>,
) -> Result<HttpResponse, Error> {
    // TODO captcha check
    let login_type = match data.login_type.as_str() {
        "username" => LoginType::Username,
        "email" => LoginType::Email,
        "phone" => LoginType::Phone,
        _ => LoginType::Username,
    };
    let user = User::find_by_login_type(data.identifier.as_str(), login_type, pool.get_ref()).await;
    let ret = match user {
        Ok(u) => {
            match u.password == data.password {
                true => { HttpResponse::Ok().json("") }
                false => { HttpResponse::Unauthorized().finish() }
            }
        }
        Err(e) => {
            match e {
                UserError::UserNotFound => { HttpResponse::NotFound().finish() }
                _ => { HttpResponse::InternalServerError().finish() }
            }
        }
    };
    Ok(ret)
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
