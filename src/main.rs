#[macro_use]
extern crate juniper;

// Packages
use actix_web::{get, web, App, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

// File imports
mod routes;
use crate::routes::ping::Rping;
mod schemas;
use crate::schemas::schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(|| App::new()
        .data(schema.clone())
        .service(web::resource("graphql").route(web::post().to_async(graphql)))
        .service(web::resource("graphiql").route(web::get().to(graphiql)))
        .service(Rping)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
