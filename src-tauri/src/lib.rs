use anyhow::Result;
use persistence::sqlite_manager::setup_db;
use sea_orm::Database;
mod controllers;
mod models;
mod persistence;

use controllers::register::{delete_entry, get_entries, insert_entry};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await?;
    setup_db(&connection).await?;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_entries,
            insert_entry,
            delete_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let connection = Database::connect("sqlite://data.db?mode=rwc").await;
        assert!(connection.is_ok());
    }
}
