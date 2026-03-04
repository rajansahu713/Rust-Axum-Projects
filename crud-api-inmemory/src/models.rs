use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct ItemStore {
    pub items: Arc<Mutex<Vec<Item>>>,
}