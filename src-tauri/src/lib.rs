mod consts;
mod controllers;
mod models;
mod persistence;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_std::task;
use consts::database::{DATABASE_FOLDER, DATABASE_URL};
use controllers::checkpoint::{
    delete_checkpoint, get_checkpoints, insert_checkpoint, update_checkpoint,
};
use controllers::configuration::{get_configuration, update_configuration};
use controllers::register::{
    delete_entry, get_entries, get_historical_entries, insert_entry, update_entry,
};
use log::{error, info};
use persistence::sqlite_manager::setup_db;
use sea_orm::Database;
use tauri::Manager;

#[derive(Default)]
struct Store {
    state: Arc<Mutex<HashMap<String, String>>>,
}

fn setup(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the database");
    let result = task::block_on(async {
        use anyhow::Ok;
        if async_std::fs::metadata(DATABASE_FOLDER.clone())
            .await
            .is_ok()
        {
            info!("Creating database folder {}", DATABASE_FOLDER.clone());
            async_std::fs::create_dir_all(DATABASE_FOLDER.clone()).await?;
        }
        let connection = Database::connect(DATABASE_URL.clone()).await?;
        setup_db(&connection).await?;
        Ok(connection)
    });
    info!("Database connection result: {:?}", &result);
    info!("Database setup completed");
    if let Err(err) = result {
        error!("Error setting up the database: {:?}", err);
        app.state::<Store>()
            .state
            .lock()
            .unwrap()
            .insert("STATE".into(), "Error".into());
        app.state::<Store>()
            .state
            .lock()
            .unwrap()
            .insert("MESSAGE".into(), err.to_string());
        return Ok(());
    }
    app.state::<Store>()
        .state
        .lock()
        .unwrap()
        .insert("STATE".into(), "Completed".into());
    info!("Setup completed");
    Ok(())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Payload {
    message: Option<String>,
    state: String,
}

#[tauri::command]
fn get_setup_state(storage: tauri::State<Store>) -> Payload {
    info!("Getting setup state");
    let state = storage
        .state
        .lock()
        .unwrap()
        .get("STATE")
        .unwrap_or(&String::from("Loading"))
        .clone();
    info!("State: {:?}", state);
    let message = storage.state.lock().unwrap().get("MESSAGE").cloned();
    info!("Message: {:?}", message);
    let payload = Payload {
        message,
        state: state.clone(),
    };
    info!("Setup state: {:?}", payload);
    payload
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_log::Builder::default().build())
        .manage(Store::default())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            get_setup_state,
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

    #[async_std::test]
    async fn test_database_connection() {
        let connection = Database::connect(DATABASE_URL.clone()).await;
        assert!(connection.is_ok());
    }
}
