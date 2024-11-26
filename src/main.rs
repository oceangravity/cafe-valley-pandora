use actix_web::{get, web, App, HttpServer, Responder};
use std::env;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    HttpServer::new(|| App::new().service(greet))
        .bind(addr)?
        .run()
        .await
}
