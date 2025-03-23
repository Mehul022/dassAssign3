use std::collections::HashMap;
use std::fs;
use serde_json;
use crate::models::Database;

const DB_FILE: &str = "database.json";

pub fn load_database() -> Database {
    if let Ok(data) = fs::read_to_string(DB_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Database::default())
    } else {
        Database::default()
    }
}

pub fn save_database(db: &Database) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(db).unwrap();
    fs::write(DB_FILE, data)
}