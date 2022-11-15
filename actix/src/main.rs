use std::time::Duration;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::rt::time::sleep;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/sleep")]
async fn sleep_hello() -> impl Responder {
    sleep(Duration::from_secs(5)).await;
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(sleep_hello)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}