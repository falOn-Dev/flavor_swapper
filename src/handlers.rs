// src/handlers.rs
use crate::models::{Flavor, SlackCommand};
use axum::{
    extract::{Form, State},
    response::Json, Extension,
};
use serde_json::json;
use sqlx::query;
use std::sync::Arc;


pub async fn swap_flavors(
    Extension(db): Extension<Arc<sqlx::SqlitePool>>,
    Form(command): Form<SlackCommand>,
) -> Json<serde_json::Value> {
    let args: Vec<&str> = command.text.split_whitespace().collect();

    if args.len() != 3 {
        return Json(json!({
            "text": "Invalid number of arguments. Please provide a store, and two flavors to swap."
        }));
    }

    let store = args[0].to_lowercase();
    let swapped_flavor = args[1].to_lowercase();
    let new_flavor = args[2].to_lowercase();

    let store_id = query!(
        "SELECT id FROM stores WHERE name = ?",
        store
    ).fetch_optional(&*db).await.unwrap();

    if store_id.is_none() {
        return Json(json!({
            "text": format!("Store '{}' not found", store),
            "response_type": "ephemeral"
        }));
    }

    let store_id = store_id.unwrap().id;

    let swap_flavor_id = query!(
        "SELECT flavor_id FROM flavor_search_terms WHERE search_term = ?",
        new_flavor
    ).fetch_optional(&*db).await.unwrap();
    
    if swap_flavor_id.is_none() {
        return Json(json!({
            "text": format!("Flavor '{}' not found", new_flavor),
            "response_type": "ephemeral"
        }));
    }

    let swap_flavor_id = swap_flavor_id.unwrap().flavor_id;

    let new_flavor_id = query!(
        "SELECT flavor_id FROM flavor_search_terms WHERE search_term = ?",
        swapped_flavor
    ).fetch_optional(&*db).await.unwrap();

    if new_flavor_id.is_none() {
        return Json(json!({
            "text": format!("Flavor '{}' not found", swapped_flavor),
            "response_type": "ephemeral"
        }));
    }

    let new_flavor_id = new_flavor_id.unwrap().flavor_id;

    println!("Store ID: {}", store_id);
    println!("New Flavor ID: {}", new_flavor_id);
    println!("Swap Flavor ID: {}", swap_flavor_id);

    // Check if the new flavor is currently served at the store
    let is_new_flavor_served = query!(
        "SELECT 1 as valid_identifier FROM store_flavors WHERE store_id = ? AND flavor_id = ?",
        store_id,
        new_flavor_id
    ).fetch_optional(&*db).await.unwrap().is_some();

    if !is_new_flavor_served {
        return Json(json!({
            "text": format!("Flavor '{}' is not currently served at store '{}'", swapped_flavor, store),
            "response_type": "ephemeral"
        }));
    }

    // Check if the swap flavor is already served at the store
    let is_swap_flavor_served = query!(
        "SELECT 1 as valid_identifier FROM store_flavors WHERE store_id = ? AND flavor_id = ?",
        store_id,
        swap_flavor_id
    ).fetch_optional(&*db).await.unwrap().is_some();

    if is_swap_flavor_served {
        return Json(json!({
            "text": format!("Flavor '{}' is already served at store '{}'", new_flavor, store),
            "response_type": "ephemeral"
        }));
    }

    // Remove the old flavor
    let _ = query!(
        "DELETE FROM store_flavors WHERE store_id = ? AND flavor_id = ?",
        store_id,
        new_flavor_id
    ).execute(&*db).await;

    // Add the new flavor
    let _ = query!(
        "INSERT INTO store_flavors (store_id, flavor_id) VALUES (?, ?)",
        store_id,
        swap_flavor_id
    ).execute(&*db).await;

    return Json(json!({
        "text": "Swapped flavors",
        "response_type": "in_channel"
    }));

}

pub async fn test() -> Json<serde_json::Value> {
    return Json(json!({
        "text": "Hello, world!",
        "response_type": "in_channel"
    }));
}


pub async fn list_flavors(
    Extension(db): Extension<Arc<sqlx::SqlitePool>>,
    Form(command): Form<SlackCommand>,
) -> Json<serde_json::Value> {



    if command.text.is_empty() {
        return Json(json!({
            "text": "Please provide a store name.",
            "response_type": "ephemeral",
        }));
    }

    let store = command.text.to_lowercase().trim().to_owned();

    println!("Listing flavors for {}", store);

    let rows = match query!(
        r#"
        SELECT f.name
        FROM flavors f
        INNER JOIN store_flavors sf ON f.id = sf.flavor_id
        INNER JOIN stores s ON s.id = sf.store_id
        WHERE s.name = ? 
    "#,
    store
    ).fetch_all(&*db).await {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error querying database: {}", e);
            return Json(json!({
                "text": "Failed to fetch flavors.",
                "response_type": "ephemeral",
            }));
        }
    };

    if rows.is_empty() {
        return Json(json!({
            "text": "No flavors found for this store.",
            "response_type": "ephemeral",
        }));
    }

    println!("Found {} flavors", rows.len());
    println!("{:?}", rows);

    let formatted_flavors = rows.iter().enumerate().fold(String::new(), |acc, (i, flavor)| {
        format!("{}\n- {}", acc, flavor.name)
        
    });
    

    println!("Formatted flavors: {}", formatted_flavors);

    Json(json!({
        "text": format!("Currently serving flavors at {}:{}", store, formatted_flavors),
        "response_type": "in_channel"
    }))
}