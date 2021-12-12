use actix_web::{HttpResponse, web};
use crate::controller::user::register;

pub fn api_router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/")
                .service(web::resource("/").route(web::get().to(|| HttpResponse::Forbidden().body("Hacker!"))))
        )
        .service(web::resource("/register").route(web::post().to(register)))
        .service(
            web::scope("/user")
                .service(web::resource("/").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
        )
        .service(
            web::scope("/competition")
                .service(web::resource("/").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                .service(web::resource("/{id}").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                .service(web::resource("/{id}/quiz/{quiz_id}").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
        )
    ;
}

pub fn admin_api_router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/admin")
                .service(web::resource("/").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                .service(web::scope("/competition")
                    .service(web::resource("/").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                    .service(web::resource("/{id}").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                )
                .service(web::scope("/user")
                    .service(web::resource("/").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                    .service(web::resource("/{id}").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                )
                .service(web::scope("/quiz")
                    .service(web::resource("/").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                    .service(web::resource("/{id}").route(web::get().to(|| HttpResponse::Ok().body("Hello World!"))))
                )
        )
    ;
}