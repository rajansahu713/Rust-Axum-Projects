use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::models::{Item, CreateItemRequest, ItemStore};

// CREATE: Add a new item (POST /items)
pub async fn create_item(
    State(store): State<ItemStore>,
    Json(payload): Json<CreateItemRequest>,
) -> (StatusCode, Json<Item>) {
    let mut items = store.items.lock().unwrap();

    // find the length of the items and assign the id to the new item
    let current_len: usize = items.len();
    let new_id = 1 + current_len as u32;

    
    let item = Item {
        id: new_id,
        name: payload.name,
        description: payload.description,
    };
    
    items.push(item.clone());
    (StatusCode::CREATED, Json(item))
}

// READ: Get all items (GET /items)
pub async fn get_items(State(store): State<ItemStore>) -> Json<Vec<Item>> {
    let items = store.items.lock().unwrap();
    Json(items.clone())
}

// READ: Get a single item by id (GET /items/:id)
pub async fn get_item(
    State(store): State<ItemStore>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Option<Item>>) {
    let items = store.items.lock().unwrap();

    println!("Received request for item with id: {}, {:?}", id, items);
    if let Some(item) = items.iter().find(|i| i.id == id).cloned() {
        (StatusCode::OK, Json(Some(item)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

// UPDATE: Modify an existing item (PUT /items/:id)
pub async fn update_item(
    State(store): State<ItemStore>,
    Path(id): Path<u32>,
    Json(payload): Json<CreateItemRequest>,
) -> (StatusCode, Json<Option<Item>>) {
    let mut items = store.items.lock().unwrap();
    
    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        item.name = payload.name;
        item.description = payload.description;
        return (StatusCode::OK, Json(Some(item.clone())));
    }
    
    (StatusCode::NOT_FOUND, Json(None))
}

// DELETE: Remove an item (DELETE /items/:id)
pub async fn delete_item(
    State(store): State<ItemStore>,
    Path(id): Path<u32>,
) -> StatusCode {
    let mut items = store.items.lock().unwrap();
    let initial_len = items.len();
    items.retain(|i| i.id != id);
    
    if items.len() < initial_len {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}