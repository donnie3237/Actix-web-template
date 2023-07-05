use actix_web::{web, HttpResponse, Responder , get ,post};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestBody {
    name: String,
    age: i32
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello worldsss!")
}

#[post("/echo")]
pub async fn echo(req_body: web::Json<RequestBody>) -> impl Responder {
    let name = &req_body.name;
    let age = &req_body.age;
    let response_body = format!("Hello, {}! and age {} years", name , age);

    HttpResponse::Ok().body(response_body)
}

#[get("/eiei")]
pub async fn eiei() -> impl Responder {
    HttpResponse::Ok().body("Hello eiei path!")
}