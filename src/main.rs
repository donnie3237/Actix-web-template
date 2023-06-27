use actix_web::{get, post, put , web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct RequestBody {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: web::Json<RequestBody>) -> impl Responder {
    let name = &req_body.name; // Access the name field from the request body
    let response_body = format!("hello {}" , name);

    HttpResponse::Ok().body(response_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}