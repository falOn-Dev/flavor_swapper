use axum::{
    extract::{Form, State},
    response::Json,
    routing::post,
    Router,
    handler::Handler
};

use dotenvy::dotenv;
use  serde::Deserialize;
use std::{env, net::SocketAddr, sync::{Arc, Mutex}, vec};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // Load .env file

    let state = Arc::new(AppState {
        availiable_flavors: Arc::new(Mutex::new(vec!["chocolate", "vanilla", "strawberry"].iter().map(|name| Flavor { name: name.to_string() }).collect())),
        currently_serving: Arc::new(Mutex::new(vec!["salty"].iter().map(|name| Flavor { name: name.to_string() }).collect()))
    });

    let app = 
        Router::new()
            .route("/swap-flavors", post(swap_flavors))
            .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
    
}

// Slack's request format
#[derive(Debug, Deserialize)]
struct SlackCommand {
    user_name: String,
    command: String,
    text: String,
    response_url: String,
}


#[derive(Debug, Clone)]
struct Flavor {
    name: String
}

struct AppState {
    availiable_flavors: Arc<Mutex<Vec<Flavor>>>,
    currently_serving: Arc<Mutex<Vec<Flavor>>>
}


async fn swap_flavors(
    State(state): State<Arc<AppState>>,
    Form(command): Form<SlackCommand>,

) -> Json<serde_json::Value> {
    let args: Vec<&str> = command.text.split(" ").collect();

    if args.len() != 2 {
        return Json(serde_json::json!({
            "text": "Invalid number of arguments. Please provide two flavors to swap."
        }));
    }

    let new_flavor = args[0];
    let swapped_flavor = args[1];

    let mut availiable_flavors = state.availiable_flavors.lock().unwrap();
    let mut currently_serving = state.currently_serving.lock().unwrap();

    println!("New flavor: {}", new_flavor);
    println!("Swapped flavor: {}", swapped_flavor);

    let new_flavor_index = availiable_flavors.iter().position(|flavor| flavor.name == new_flavor);
    let swapped_flavor_index = currently_serving.iter().position(|flavor| flavor.name == swapped_flavor);

    println!("New flavor index: {:?}", new_flavor_index);
    println!("Swapped flavor index: {:?}", swapped_flavor_index);

    match (new_flavor_index, swapped_flavor_index) {
        (Some(new_flavor_index), Some(swapped_flavor_index)) => {
            let new_flavor = availiable_flavors.remove(new_flavor_index);
            let swapped_flavor = currently_serving.remove(swapped_flavor_index);

            availiable_flavors.push(swapped_flavor.clone());
            currently_serving.push(new_flavor.clone());

            println!("New availiable flavors: {:?}", availiable_flavors);
            println!("New currently serving flavors: {:?}", currently_serving);
            
            return Json(serde_json::json!({
                "text": format!("Swapped {} with {}", new_flavor.name, swapped_flavor.name)
            }));


        },
        _ => {
            return Json(serde_json::json!({
                "text": "Invalid flavors provided. Please provide valid flavors."
            }));
        }
    }
}

// Handle Slack command
async fn handle_slack_command(Form(command): Form<SlackCommand>) -> Json<serde_json::Value> {
    let response_text = format!(
        "👋 Hello, {}! You used `{}` with args: `{}`",
        command.user_name, command.command, command.text
    );

    Json(serde_json::json!({ "text": response_text }))
}