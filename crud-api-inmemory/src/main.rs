use axum::{
    routing::{get},
    Router,
};
use std::{net::SocketAddr, sync::{Arc, Mutex}};

mod models;
mod service;
use models::ItemStore;
use service::{create_item, get_items, get_item, update_item, delete_item};


#[tokio::main]
async fn main() {
    // Initialize the in-memory store
    let store = ItemStore {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/items", get(get_items).post(create_item))
        .route("/items/:id",get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}