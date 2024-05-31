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
pub fn run() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    let connection = runtime
        .block_on(runtime.spawn(async {
            use anyhow::Ok;
            tokio::fs::create_dir_all("databases").await?;
            let _ = tokio::fs::File::create("databases/data.db").await?;
            tokio::fs::remove_file("databases/data.db").await?;
            let db = Database::connect("sqlite://databases/data.db?mode=rwc").await?;
            Ok(db)
        }))
        .unwrap()
        .unwrap();

    runtime
        .block_on(runtime.spawn(async move {
            use anyhow::Ok;
            setup_db(&connection).await?;
            Ok(())
        }))
        .unwrap()
        .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
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
