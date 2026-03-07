use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};
mod models;
mod db;
use db::{init_db};
mod service;
use service::{health, create_user, get_users};


#[tokio::main]
async fn main() {
    // Initialize database
    if let Err(e) = init_db() {
        eprintln!(" Failed to initialize database: {}", e);
        return;
    }

    let app = Router::new()
        .route("/", get(health))
        .route("/users", get(get_users).post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
