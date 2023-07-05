use actix_web::{App, HttpServer};
use router::{echo, hello};
use mongodb::{Client, options::ClientOptions};
mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}