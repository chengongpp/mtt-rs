use actix_web::Responder;

async fn index() -> impl Responder {
    "Hello world!"
}