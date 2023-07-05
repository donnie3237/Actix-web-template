use actix_web::{web, HttpResponse, Responder , get ,post};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestBody {
    name: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello worldsss!")
}

#[post("/echo")]
pub async fn echo(req_body: web::Json<RequestBody>) -> impl Responder {
    let name = &req_body.name;
    let response_body = format!("Hello, {}!", name);

    HttpResponse::Ok().body(response_body)
}

#[get("/eiei")]
pub async fn eiei() -> impl Responder {
    HttpResponse::Ok().body("Hello eiei path!")
}