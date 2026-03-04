# CRUD API In-Memory

A simple REST API built with **Axum** framework that demonstrates CRUD operations using in-memory storage with thread-safe data structures.

## Features

- **Create** - Add new items to the store
- **Read** - Retrieve all items or a specific item by ID
- **Update** - Modify existing items
- **Delete** - Remove items from the store

- **Thread-Safe** - Uses `Arc<Mutex<>>` for concurrent access
- **Fast** - In-memory storage for quick operations

## Project Structure

```
crud-api-inmemory/
├── src/
│   ├── main.rs          # Main application, routes, and handlers
│   ├── models.rs        # Data structures (Item, CreateItemRequest, ItemStore)
│   └── lib.rs           # (optional)
├── Cargo.toml           # Dependencies
└── README.md            # This file
```

## Dependencies

- **axum** (0.7) - Web framework
- **tokio** (1) - Async runtime
- **serde** (1.0) - Serialization/deserialization
- **serde_json** (1.0) - JSON support
- **hyper** (1.0) - HTTP protocol
- **tower** (0.4) - Service tower

## API Endpoints

### 1. Health Check
**GET** `/`
```bash
curl http://localhost:3000/
# Response: Hello, World!
```

### 2. Create Item
**POST** `/items`
```bash
curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Laptop","description":"Dell XPS 13"}'
```

**Response** (201 Created):
```json
{
  "id": 1,
  "name": "Laptop",
  "description": "Dell XPS 13"
}
```

### 3. Get All Items
**GET** `/items`
```bash
curl http://localhost:3000/items
```

**Response**:
```json
[
  {"id": 1, "name": "Laptop", "description": "Dell XPS 13"},
  {"id": 2, "name": "Mouse", "description": "Logitech MX"}
]
```

### 4. Get Single Item
**GET** `/items/:id`
```bash
curl http://localhost:3000/items/1
```

**Response** (200 OK or 404 Not Found):
```json
{
  "id": 1,
  "name": "Laptop",
  "description": "Dell XPS 13"
}
```

### 5. Update Item
**PUT** `/items/:id`
```bash
curl -X PUT http://localhost:3000/items/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"MacBook Pro","description":"M3 Pro Max"}'
```

**Response** (200 OK or 404 Not Found):
```json
{
  "id": 1,
  "name": "MacBook Pro",
  "description": "M3 Pro Max"
}
```

### 6. Delete Item
**DELETE** `/items/:id`
```bash
curl -X DELETE http://localhost:3000/items/1
```

**Response**: `204 No Content` (or `404 Not Found`)



## Running the Server

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Cargo

### Build & Run

```bash
cd crud-api-inmemory
cargo run
```

Server will start at `http://127.0.0.1:3000`

Output:
```
Server running on http://127.0.0.1:3000
Listening on 127.0.0.1:3000
```



**Note**: Uses `Arc<Mutex<>>` for thread-safe access to data across async handlers.

## How It Works

1. **State Management**: `ItemStore` holds all items in a `Vec` protected by `Mutex`
2. **Thread Safety**: `Arc` allows multiple handlers to share the store safely
3. **Async Handlers**: All route handlers are async functions using Axum's `State` extractor
4. **JSON Serialization**: Request/response bodies are automatically serialized/deserialized with `serde_json`

## Example Workflow

```bash
# Create 3 items
curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Item 1","description":"First item"}'

curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Item 2","description":"Second item"}'

# Get all items
curl http://localhost:3000/items | jq


# Update item 1
curl -X PUT http://localhost:3000/items/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Item 1","description":"Modified"}'

# Delete item 2
curl -X DELETE http://localhost:3000/items/2


```

## Key Concepts

### Arc (Atomic Reference Counting)
- Allows multiple ownership of data
- Thread-safe reference counting
- Used to share `ItemStore` across async handlers

### Mutex (Mutual Exclusion)
- Protects mutable data from concurrent access
- `lock().unwrap()` acquires the lock
- Lock is automatically released when `MutexGuard` goes out of scope

### Axum State Extractor
- `State(store): State<ItemStore>` injects the shared state into handlers
- Automatically clones the `Arc` (cheap operation)
- Enables safe concurrent access







