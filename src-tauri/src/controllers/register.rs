use crate::models::register;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DbConn, DeleteResult, EntityTrait, ModelTrait,
    QueryFilter, QueryOrder, Set, TryIntoModel,
};
use serde::{Deserialize, Serialize};

use crate::models::register::Entity as Register;

#[tauri::command]
pub async fn get_entries(entered_at: DateTime<Utc>) -> Result<Vec<register::Model>, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let registers = get_registers_after_date(&connection.unwrap(), entered_at).await;
    if let Err(c) = registers {
        return Err(c.to_string());
    }
    Ok(registers.unwrap())
}

#[tauri::command]
pub async fn insert_entry(entry: RegisterData) -> Result<register::Model, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let result = insert_register(&connection.unwrap(), entry).await;
    if let Err(c) = result {
        return Err(c.to_string());
    }
    Ok(result.unwrap())
}

#[tauri::command]
pub async fn update_entry(id: u32, entry: RegisterData) -> Result<register::Model, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let result = update_register(&connection.unwrap(), id, entry).await;
    if let Err(c) = result {
        return Err(c.to_string());
    }
    Ok(result.unwrap())
}

#[tauri::command]
pub async fn delete_entry(id: u32) -> Result<u64, String> {
    let connection = Database::connect("sqlite://data.db?mode=rwc").await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let result = delete_register(&connection.unwrap(), id).await;
    if let Err(c) = result {
        return Err(c.to_string());
    }
    Ok(result.unwrap().rows_affected)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegisterData {
    pub started_at: DateTime<Utc>,
    pub exited_at: DateTime<Utc>,
}

async fn insert_register(
    connection: &DbConn,
    register_data: RegisterData,
) -> Result<register::Model> {
    let register = register::ActiveModel {
        started_at: Set(register_data.started_at),
        exited_at: Set(register_data.exited_at),
        ..Default::default()
    };
    let result = register.save(connection).await?;
    let model = result.try_into_model()?;
    Ok(model)
}

async fn update_register(
    connection: &DbConn,
    register_id: u32,
    register_data: RegisterData,
) -> Result<register::Model> {
    let register = register::ActiveModel {
        register: Set(register_id),
        started_at: Set(register_data.started_at),
        exited_at: Set(register_data.exited_at),
        ..Default::default()
    };
    let result = register.update(connection).await?;
    let model = result.try_into_model()?;
    Ok(model)
}

async fn delete_register(connection: &DbConn, register_id: u32) -> Result<DeleteResult> {
    let register = Register::find_by_id(register_id).one(connection).await?;
    if register.is_none() {
        return Err(anyhow::anyhow!("Register not found"));
    }
    let result = register.unwrap().delete(connection).await?;
    Ok(result)
}

async fn get_registers_after_date(
    connection: &DbConn,
    date: DateTime<Utc>,
) -> Result<Vec<register::Model>> {
    let registers = Register::find()
        .filter(register::Column::StartedAt.gte(date))
        .order_by_asc(register::Column::StartedAt)
        .all(connection)
        .await?;
    Ok(registers)
}

#[cfg(test)]
mod tests {
    use crate::persistence::sqlite_manager::setup_db;

    use super::*;

    #[tokio::test]
    async fn test_insert_register() -> Result<()> {
        let connection = Database::connect("sqlite://data.db?mode=rwc")
            .await
            .unwrap();

        setup_db(&connection).await?;

        let register_data = RegisterData {
            started_at: Utc::now(),
            exited_at: Utc::now(),
        };

        let result = insert_register(&connection, register_data).await;
        assert!(result.is_ok());
        let cleanup_result = delete_register(&connection, result.unwrap().register).await;
        assert!(cleanup_result.is_ok());

        Ok(())
    }
}
