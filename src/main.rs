// src/main.rs
use axum::{
    routing::post, Extension, Router
};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, net::SocketAddr, sync::{Arc, Mutex}};
use tokio::net::TcpListener;

mod handlers;
mod models;

use handlers::{list_flavors};
use models::{Flavor};

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    let db = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let db = Arc::new(db);

    let app = Router::new()
        .route("/list-flavors", post(list_flavors))
        .layer(Extension(db));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
