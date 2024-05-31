mod consts;
mod controllers;
mod models;
mod persistence;

use std::collections::HashMap;
use std::sync::Mutex;

use consts::database::{DATABASE_FOLDER, DATABASE_PATH, DATABASE_URL};
use controllers::checkpoint::{
    delete_checkpoint, get_checkpoints, insert_checkpoint, update_checkpoint,
};
use controllers::configuration::{get_configuration, update_configuration};
use controllers::register::{
    delete_entry, get_entries, get_historical_entries, insert_entry, update_entry,
};
use persistence::sqlite_manager::setup_db;
use sea_orm::Database;
use tauri::Manager;

#[derive(Default)]
struct Store(Mutex<HashMap<String, String>>);

fn setup(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    let result = runtime
        .block_on(runtime.spawn(async {
            use anyhow::Ok;
            tokio::fs::create_dir_all(DATABASE_FOLDER).await?;
            let _ = tokio::fs::File::create(DATABASE_PATH.clone()).await?;
            tokio::fs::remove_file(DATABASE_PATH.clone()).await?;
            let connection = Database::connect(DATABASE_URL.clone()).await?;
            setup_db(&connection).await?;
            Ok(())
        }))
        .expect("Error while joining the setup task");
    if let Err(err) = result {
        app.state::<Store>()
            .0
            .lock()
            .unwrap()
            .insert("STATE".into(), "Error".into());
        app.state::<Store>()
            .0
            .lock()
            .unwrap()
            .insert("MESSAGE".into(), err.to_string());
        return Ok(());
    }
    app.state::<Store>()
        .0
        .lock()
        .unwrap()
        .insert("STATE".into(), "Completed".into());
    Ok(())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Payload {
    message: Option<String>,
    state: String,
}

#[tauri::command]
fn get_setup_state(app: tauri::AppHandle) -> Payload {
    Payload {
        state: app
            .state::<Store>()
            .0
            .lock()
            .unwrap()
            .get("STATE")
            .unwrap_or(&String::from("Loading"))
            .clone(),
        message: app
            .state::<Store>()
            .0
            .lock()
            .unwrap()
            .get("MESSAGE")
            .cloned(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
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

    #[tokio::test]
    async fn test_database_connection() {
        let connection = Database::connect(DATABASE_URL.clone()).await;
        assert!(connection.is_ok());
    }
}
