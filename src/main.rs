use actix_web::{get, App, HttpServer};

mod routes;
use crate::routes::ping::Rping;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
HttpServer::new(|| App::new()
        .service(Rping)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
