use rusqlite::{Connection, Result as RusqliteResult};
use std::sync::Mutex;

// Database connection pool (simplified)
pub static DB: Mutex<Option<Connection>> = Mutex::new(None);

// Initialize SQLite database
pub fn init_db() -> RusqliteResult<()> {
    let conn = Connection::open("app.db")?;
    
    // Create users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    let mut db = DB.lock().unwrap();
    *db = Some(conn);
    
    println!(" Database initialized successfully!");
    Ok(())
}


// Execute a SQL query and return results
pub fn execute_query(query: &str) -> RusqliteResult<Vec<serde_json::Value>> {
    
    let db = DB.lock().unwrap();
    let conn = db.as_ref().ok_or(rusqlite::Error::QueryReturnedNoRows)?;
    let mut stmt = conn.prepare(query)?;


    // Collect column names first to avoid borrow conflict
    let columns: Vec<String> = stmt.column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();

    let rows = stmt.query_map([], |row| {
        let mut obj = serde_json::Map::new();


        for (i, col_name) in columns.iter().enumerate() {
            
            
            // Try to get the value with proper type handling
            let value = match row.get_ref(i) {
                Ok(rusqlite::types::ValueRef::Null) => serde_json::Value::Null,
                Ok(rusqlite::types::ValueRef::Integer(int_val)) => {
                    serde_json::Value::Number(int_val.into())
                }
                Ok(rusqlite::types::ValueRef::Real(float_val)) => {
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(float_val)
                            .unwrap_or_else(|| serde_json::Number::from(0))
                    )
                }
                Ok(rusqlite::types::ValueRef::Text(text_bytes)) => {
                    let text = String::from_utf8_lossy(text_bytes).to_string();
                    serde_json::Value::String(text)
                }
                Ok(rusqlite::types::ValueRef::Blob(_)) => {
                    serde_json::Value::String("[BLOB]".to_string())
                }
                Err(_) => serde_json::Value::Null,
            };

            obj.insert(col_name.clone(), value);
        }
        Ok(serde_json::Value::Object(obj))
    })?;

    let mut results = Vec::new();
    for row_result in rows {
        if let Ok(row) = row_result {
            results.push(row);
        }
    }

    Ok(results)
}
