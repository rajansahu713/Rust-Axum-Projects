# Hello World - Axum Web Server

A simple Rust web server built with Axum framework demonstrating 5 different endpoints.

## Endpoints

### 1. Health Check
- **Method:** GET
- **Route:** `/health`
- **Description:** Returns server health status
- **Response:** 
  ```json
  {
    "status": "Server is healthy"
  }
  ```

### 2. Hello World
- **Method:** GET
- **Route:** `/hello`
- **Description:** Returns a simple hello world message in JSON format
- **Response:**
  ```json
  {
    "message": "Hello, World!"
  }
  ```

### 3. Greet User (Path Variable)
- **Method:** GET
- **Route:** `/greet/:name`
- **Description:** Takes a name from the URL path and returns a personalized greeting
- **Example:** `/greet/John`
- **Response:**
  ```json
  {
    "greeting": "Hello, John!"
  }
  ```

### 4. Greet with Query Parameter
- **Method:** GET
- **Route:** `/greet-query`
- **Query Parameters:** `name` (string)
- **Description:** Takes a name as a query parameter and returns it
- **Example:** `/greet-query?name=Alice`
- **Response:**
  ```json
  {
    "name": "Alice"
  }
  ```

### 5. Create User (POST)
- **Method:** POST
- **Route:** `/users`
- **Description:** Accepts user data in JSON format and returns a confirmation
- **Request Body:**
  ```json
  {
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
  }
  ```
- **Response:**
  ```json
  {
    "message": "User John Doe created successfully",
    "received_data": {
      "name": "John Doe",
      "age": 30,
      "email": "john@example.com"
    }
  }
  ```

## Running the Server

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## Testing the Endpoints

### Using curl:

```bash
# Health check
curl http://localhost:3000/health

# Hello world
curl http://localhost:3000/hello

# Greet user with path variable
curl http://localhost:3000/greet/John

# Greet with query parameter
curl "http://localhost:3000/greet-query?name=Alice"

# Create user (POST)
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","age":30,"email":"john@example.com"}'
```

## Dependencies

- **axum** - Web framework
- **tokio** - Async runtime
- **serde** - Serialization/deserialization
- **serde_json** - JSON support
