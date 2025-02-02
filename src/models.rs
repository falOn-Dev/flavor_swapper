// src/models.rs
use std::{sync::{Arc, Mutex}, vec};

use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct SlackCommand {
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Flavor {
    pub name: String,
    pub search_terms: Vec<String>,
}

impl Flavor {
    pub fn new(name: &str, search_terms: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            search_terms: search_terms.into_iter().map(|term| term.to_string()).collect(),
        }
    }
}
