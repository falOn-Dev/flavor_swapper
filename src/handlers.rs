// src/handlers.rs
use crate::models::{AppState, Flavor, SlackCommand};
use axum::{
    extract::{Form, State},
    response::Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn swap_flavors(
    State(state): State<Arc<AppState>>,
    Form(command): Form<SlackCommand>,
) -> Json<serde_json::Value> {
    let args: Vec<&str> = command.text.split_whitespace().collect();

    if args.len() != 2 {
        return Json(json!({
            "text": "Invalid number of arguments. Please provide two flavors to swap."
        }));
    }

    let swapped_flavor = args[0].to_lowercase();
    let new_flavor = args[1].to_lowercase();

    println!("Swapping {} with {}", swapped_flavor, new_flavor);


    let mut availiable_flavors = state.availiable_flavors.lock().unwrap();
    let mut currently_serving = state.currently_serving.lock().unwrap();

    let new_flavor_index = availiable_flavors.iter().position(|flavor| flavor.search_terms.contains(&new_flavor));
    let swapped_flavor_index = currently_serving.iter().position(|flavor| flavor.search_terms.contains(&swapped_flavor));

    match (new_flavor_index, swapped_flavor_index) {
        (Some(new_flavor_index), Some(swapped_flavor_index)) => {
            let new_flavor = availiable_flavors.remove(new_flavor_index);
            let swapped_flavor = currently_serving.remove(swapped_flavor_index);

            availiable_flavors.push(swapped_flavor.clone());
            currently_serving.push(new_flavor.clone());
            
            Json(json!({
                "text": format!("Replaced {} with {}", swapped_flavor.name, new_flavor.name),
                "response_type": "in_channel"
            }))
        },
        (None, None) => {
            Json(json!({
                "text": format!("Neither {} nor {} found", new_flavor, swapped_flavor),
                "response_type": "in_channel"
            }))
        },
        // Specify the error message if the new flavor is not found
        (None, _) => {
            Json(json!({
                "text": format!("Flavor {} not found in availiable flavors", new_flavor),
                "response_type": "in_channel"
            }))
        },
        // Specify the error message if the swapped flavor is not found
        (_, None) => {
            Json(json!({
                "text": format!("Flavor {} not found in currently serving flavors", swapped_flavor),
                "response_type": "in_channel"
            }))
        },
    }
}


pub async fn list_flavors(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let currently_serving = state.currently_serving.lock().unwrap();

    // Format our currently serving flavors into a nice message like so
    // Currently Serving:
    // (tab) - flavor 1
    // (tab) - flavor 2
    // (tab) - flavor 3

    let formatted_flavors = currently_serving.iter().fold(String::new(), |acc, flavor| {
        format!("{}\n\t- {}", acc, flavor.name)
    });

    Json(json!({
        "text": format!("Currently Serving:{}", formatted_flavors),
        "response_type": "in_channel"
    }))
}