use actix_web::{App, HttpServer};
use router::{echo, hello , eiei};
mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(eiei)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}