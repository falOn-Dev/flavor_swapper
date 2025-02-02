// src/main.rs
use axum::{
    routing::post,
    Router,
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr, sync::{Arc, Mutex}};
use tokio::net::TcpListener;

mod handlers;
mod models;

use handlers::swap_flavors;
use models::{AppState, Flavor};

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let state = Arc::new(AppState {
        availiable_flavors: Arc::new(Mutex::new(vec![
            Flavor::new("Old Bay Caramel", vec!["obc", "oldbaycaramel"]),
            Flavor::new("Salty Caramel", vec!["salty", "saltcaramel", "saltycaramel"]),
            Flavor::new("Vanilla", vec!["vanilla"]),
            Flavor::new("Telltale Chocolate", vec!["telltale", "telltalechocolate", "darkchocolate"]),
        ])),
        currently_serving: Arc::new(Mutex::new(vec![
            Flavor::new("Maryland Mud", vec!["mud", "marylandmud", "muddy"]),
            Flavor::new("Mint Mountain", vec!["mint", "mintmountain"]),
            Flavor::new("Malty Vanilla Chip", vec!["malty", "maltyvanilla", "vanillachip", "vanillachocolatechip"]),
        ])),
    });

    let app = Router::new()
        .route("/swap-flavors", post(swap_flavors))
        .route("/list-flavors", post(handlers::list_flavors))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
