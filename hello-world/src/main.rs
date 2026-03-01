use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[derive(Serialize)]
struct GreetingResponse {
    greeting: String,
}

#[derive(Serialize)]
struct QueryResponse {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Payload {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize)]
struct PostResponse {
    message: String,
    received_data: Payload,
}

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

// 1. Health Check Endpoint
async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "Server is healthy".to_string(),
        }),
    )
}

// 2. Hello World in JSON Format
async fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}

// 3. Path Variable Endpoint
async fn greet_user(Path(name): Path<String>) -> Json<GreetingResponse> {
    Json(GreetingResponse {
        greeting: format!("Hello, {}!", name),
    })
}

// 4. Query Parameter Endpoint
async fn greet_with_query(
    Query(params): Query<QueryParams>,
) -> Json<QueryResponse> {
    Json(QueryResponse {
        name: params.name,
    })
}

// 5. POST Endpoint with Payload
async fn create_user(
    Json(payload): Json<Payload>,
) -> (StatusCode, Json<PostResponse>) {
    (
        StatusCode::CREATED,
        Json(PostResponse {
            message: format!("User {} created successfully", payload.name),
            received_data: payload,
        }),
    )
}

#[tokio::main]
async fn main() {
    // Build router with all endpoints
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/hello", get(hello_world))
        .route("/greet/:name", get(greet_user))
        .route("/greet-query", get(greet_with_query))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
