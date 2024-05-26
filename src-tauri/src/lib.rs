use anyhow::Result;
use persistence::sqlite_manager::setup_db;
use sea_orm::Database;
mod controllers;
mod models;
mod persistence;

use controllers::checkpoint::{
    delete_checkpoint, get_checkpoints, insert_checkpoint, update_checkpoint,
};
use controllers::configuration::{get_configuration, update_configuration};
use controllers::register::{
    delete_entry, get_entries, get_historical_entries, insert_entry, update_entry,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await?;

    // TODO(Otavio): use https://github.com/tauri-apps/tauri-plugin-sql
    setup_db(&connection).await?;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_entries,
            get_historical_entries,
            insert_entry,
            update_entry,
            delete_entry,
            get_checkpoints,
            insert_checkpoint,
            update_checkpoint,
            delete_checkpoint,
            get_configuration,
            update_configuration,
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
