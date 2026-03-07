use axum::{
    extract::Json,
    http::StatusCode,
};

use rusqlite::{params};

// models is defined at crate root (src/models.rs), not inside service/
use crate::models::CreateUserRequest;
use crate::db::execute_query;
use crate::db::DB;



// Health check
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": " SQLite API is running",
        "status": "ok"
    }))
}

// Create a user
pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let db = DB.lock().unwrap();
    
    if let Some(conn) = db.as_ref() {
        match conn.execute(
            "INSERT INTO users (name, email) VALUES (?1, ?2)",
            params![&payload.name, &payload.email],
        ) {
            Ok(_) => {
                let response = serde_json::json!({
                    "success": true,
                    "message": "User created successfully",
                    "data": {
                        "name": payload.name,
                        "email": payload.email
                    }
                });
                return (StatusCode::CREATED, Json(response));
            }
            Err(e) => {
                let response = serde_json::json!({
                    "success": false,
                    "message": format!("Error: {}", e)
                });
                return (StatusCode::BAD_REQUEST, Json(response));
            }
        }
    }
    
    let response = serde_json::json!({
        "success": false,
        "message": "Database not initialized"
    });
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
}

// Get all users
pub async fn get_users() -> (StatusCode, Json<serde_json::Value>) {
    match execute_query("SELECT * FROM users") {
        Ok(users) => {
            let response = serde_json::json!({
                "success": true,
                "data": users,
                "count": users.len()
            });
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = serde_json::json!({
                "success": false,
                "message": format!("Error: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}
