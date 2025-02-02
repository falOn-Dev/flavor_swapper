// src/handlers.rs
use crate::models::{Flavor, SlackCommand};
use axum::{
    extract::{Form, State},
    response::Json, Extension,
};
use serde_json::json;
use sqlx::query;
use std::sync::Arc;


// pub async fn swap_flavors(
//     Extension(db): Extension<Arc<sqlx::SqlitePool>>,
//     Form(command): Form<SlackCommand>,
// ) -> Json<serde_json::Value> {
//     let args: Vec<&str> = command.text.split_whitespace().collect();

//     if args.len() != 3 {
//         return Json(json!({
//             "text": "Invalid number of arguments. Please provide two flavors to swap."
//         }));
//     }

//     let swapped_flavor = args[0].to_lowercase();
//     let new_flavor = args[1].to_lowercase();

//     println!("Swapping {} with {}", swapped_flavor, new_flavor);


//     let mut availiable_flavors = state.availiable_flavors.lock().unwrap();
//     let mut currently_serving = state.currently_serving.lock().unwrap();

//     let new_flavor_index = availiable_flavors.iter().position(|flavor| flavor.search_terms.contains(&new_flavor));
//     let swapped_flavor_index = currently_serving.iter().position(|flavor| flavor.search_terms.contains(&swapped_flavor));

//     match (new_flavor_index, swapped_flavor_index) {
//         (Some(new_flavor_index), Some(swapped_flavor_index)) => {
//             let new_flavor = availiable_flavors.remove(new_flavor_index);
//             let swapped_flavor = currently_serving.remove(swapped_flavor_index);

//             availiable_flavors.push(swapped_flavor.clone());
//             currently_serving.push(new_flavor.clone());
            
//             Json(json!({
//                 "text": format!("Replaced {} with {}", swapped_flavor.name, new_flavor.name),
//                 "response_type": "in_channel"
//             }))
//         },
//         (None, None) => {
//             Json(json!({
//                 "text": format!("Neither {} nor {} found", new_flavor, swapped_flavor),
//                 "response_type": "in_channel"
//             }))
//         },
//         // Specify the error message if the new flavor is not found
//         (None, _) => {
//             Json(json!({
//                 "text": format!("Flavor {} not found in availiable flavors", new_flavor),
//                 "response_type": "in_channel"
//             }))
//         },
//         // Specify the error message if the swapped flavor is not found
//         (_, None) => {
//             Json(json!({
//                 "text": format!("Flavor {} not found in currently serving flavors", swapped_flavor),
//                 "response_type": "in_channel"
//             }))
//         },
//     }
// }



pub async fn list_flavors(
    Extension(db): Extension<sqlx::SqlitePool>,
    Form(command): Form<SlackCommand>,
) -> Json<serde_json::Value> {
    println!("Listing flavors");

    let store = command.text.to_lowercase().trim().to_owned();

    println!("Listing flavors for {}", store);

    let rows = query!(
        r#"
            SELECT f.name
            FROM flavors f
            INNER JOIN store_flavors sf ON f.id = sf.flavor_id
            INNER JOIN stores s ON s.id = sf.store_id
            WHERE s.name = ? 
        "#,
        store
    ).fetch_all(&db).await.unwrap();

    println!("Found {} flavors", rows.len());
    println!("{:?}", rows);

    let formatted_flavors = rows.iter().fold(String::new(), |acc, flavor| {
        format!("{}\n\t- {}", acc, flavor.name)
    });

    println!("Formatted flavors: {}", formatted_flavors);

    Json(json!({
        "text": format!("Currently serving flavors at {}:{}", store, formatted_flavors),
        "response_type": "in_channel"
    }))
}