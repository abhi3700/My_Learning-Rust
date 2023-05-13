use actix_web::{App, HttpServer};

mod services;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(services::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
