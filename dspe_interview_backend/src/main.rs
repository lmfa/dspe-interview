mod database;
mod routes;

mod handlers;
mod models;
mod dto;

use dotenvy::dotenv;
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL missing");

    let pool =
        database::create_pool(&database_url)
            .await
            .unwrap();

    let cors = CorsLayer::permissive();
    let app =
        routes::create_routes(pool)
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind(
            "127.0.0.1:8080"
        )
        .await
        .unwrap();

    println!("Listening on 8080");

    axum::serve(listener, app)
        .await
        .unwrap();
}