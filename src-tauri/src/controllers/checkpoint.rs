use crate::models::checkpoint;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, Database, DbConn, DeleteResult, EntityTrait, ModelTrait, QueryOrder, Set,
    TryIntoModel,
};
use serde::{Deserialize, Serialize};

use crate::models::checkpoint::Entity as Checkpoint;

#[tauri::command]
pub async fn get_checkpoints() -> Result<Vec<checkpoint::Model>, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let registers = _get_checkpoints(&connection.unwrap()).await;
    if let Err(c) = registers {
        return Err(c.to_string());
    }
    Ok(registers.unwrap())
}

#[tauri::command]
pub async fn insert_checkpoint(
    checkpoint_data: CheckpointData,
) -> Result<checkpoint::Model, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let result = _insert_checkpoint(&connection.unwrap(), checkpoint_data).await;
    if let Err(c) = result {
        return Err(c.to_string());
    }
    Ok(result.unwrap())
}

#[tauri::command]
pub async fn update_checkpoint(
    id: u32,
    new_checkpoint_data: CheckpointData,
) -> Result<checkpoint::Model, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let result = _update_checkpoint(&connection.unwrap(), id, new_checkpoint_data).await;
    if let Err(c) = result {
        return Err(c.to_string());
    }
    Ok(result.unwrap())
}

#[tauri::command]
pub async fn delete_checkpoint(checkpoint_id: u32) -> Result<u64, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let result = _delete_checkpoint(&connection.unwrap(), checkpoint_id).await;
    if let Err(c) = result {
        return Err(c.to_string());
    }
    Ok(result.unwrap().rows_affected)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CheckpointData {
    pub checkpoint: DateTime<Utc>,
}

async fn _insert_checkpoint(
    connection: &DbConn,
    checkpoint_data: CheckpointData,
) -> Result<checkpoint::Model> {
    let register = checkpoint::ActiveModel {
        checkpoint: Set(checkpoint_data.checkpoint),
        ..Default::default()
    };
    let result = register.save(connection).await?;
    let model = result.try_into_model()?;
    Ok(model)
}

async fn _update_checkpoint(
    connection: &DbConn,
    checkpoint_id: u32,
    checkpoint_data: CheckpointData,
) -> Result<checkpoint::Model> {
    let checkpoint = checkpoint::ActiveModel {
        id: Set(checkpoint_id),
        checkpoint: Set(checkpoint_data.checkpoint),
        ..Default::default()
    };
    let result = checkpoint.update(connection).await?;
    let model = result.try_into_model()?;
    Ok(model)
}

async fn _delete_checkpoint(connection: &DbConn, checkpoint_id: u32) -> Result<DeleteResult> {
    let checkpoint = Checkpoint::find_by_id(checkpoint_id)
        .one(connection)
        .await?;
    if checkpoint.is_none() {
        return Err(anyhow::anyhow!("Checkpoint not found"));
    }
    let result = checkpoint.unwrap().delete(connection).await?;
    Ok(result)
}

async fn _get_checkpoints(connection: &DbConn) -> Result<Vec<checkpoint::Model>> {
    let checkpoints = Checkpoint::find()
        .order_by_desc(checkpoint::Column::Checkpoint)
        .all(connection)
        .await?;
    Ok(checkpoints)
}

#[cfg(test)]
mod tests {
    use crate::persistence::sqlite_manager::setup_db;

    use super::*;

    #[tokio::test]
    async fn test_insert_checkpoint() -> Result<()> {
        let connection = Database::connect("sqlite://data.db?mode=rwc")
            .await
            .unwrap();

        setup_db(&connection).await?;

        let checkpoint_data = CheckpointData {
            checkpoint: Utc::now(),
        };

        let result = _insert_checkpoint(&connection, checkpoint_data).await;
        assert!(result.is_ok());
        let cleanup_result = _delete_checkpoint(&connection, result.unwrap().id).await;
        assert!(cleanup_result.is_ok());

        Ok(())
    }
}
