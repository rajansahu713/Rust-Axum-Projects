# CRUD API with SQLite

A simple REST API built with Rust and Axum framework that provides CRUD operations for users stored in a SQLite database.


## Project Structure

```
crud-api-sqlite/
├── Cargo.toml          # Project dependencies and metadata
├── app.db             # SQLite database file (auto-created)
├── src/
│   ├── main.rs        # Application entry point and server setup
│   ├── models.rs      # Data structures and request/response types
│   ├── db.rs          # Database initialization and query functions
│   └── service.rs     # HTTP handlers for API endpoints
└── README.md          # This file
```

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/rajansahu713/Rust-Axum-Projects.git
   cd Rust-Axum-Projects/crud-api-sqlite
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:3000`

## API Endpoints

### Health Check
- **GET** `/`
- Returns API status information

**Response:**
```json
{
  "message": "SQLite API is running",
  "status": "ok"
}
```

### Create User
- **POST** `/users`
- Creates a new user in the database

**Request Body:**
```json
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Success Response (201):**
```json
{
  "success": true,
  "message": "User created successfully",
  "data": {
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

**Error Response (400):**
```json
{
  "success": false,
  "message": "Error: UNIQUE constraint failed: users.email"
}
```

### Get All Users
- **GET** `/users`
- Retrieves all users from the database

**Success Response (200):**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "John Doe",
      "email": "john@example.com"
    },
    {
      "id": 2,
      "name": "Jane Smith",
      "email": "jane@example.com"
    }
  ],
  "count": 2
}
```

## Database Schema

The application automatically creates a `users` table with the following structure:

```sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);
```


### Database

- The SQLite database file `app.db` is created automatically in the project root
- The database persists between server restarts
- You can inspect the database using any SQLite browser or CLI tool

### Code Organization

- **`main.rs`**: Server setup, routing, and database initialization
- **`models.rs`**: Request/response data structures
- **`db.rs`**: Database connection management and query execution
- **`service.rs`**: HTTP request handlers and business logic

## Error Handling

The API includes comprehensive error handling for:
- Database connection issues
- Invalid request data
- Constraint violations (e.g., duplicate emails)
- Internal server errors

All errors return appropriate HTTP status codes and descriptive JSON error messages.
