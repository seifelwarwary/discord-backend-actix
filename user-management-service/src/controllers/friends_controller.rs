use actix_web::{get, post, HttpResponse, Responder};
use tracing::event;
use macros_lib::authenticate;

#[get("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    event!(tracing::Level::INFO, "Echoing back the request body");
    HttpResponse::Ok().body(req_body)
}

