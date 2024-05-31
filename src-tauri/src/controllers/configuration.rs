use crate::{consts::database::DATABASE_URL, models::configuration};
use anyhow::Result;
use chrono::NaiveTime;
use sea_orm::{ActiveModelTrait, Database, DbConn, EntityTrait, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::models::configuration::Entity as Configuration;

#[tauri::command(async)]
pub async fn get_configuration() -> Result<configuration::Model, String> {
    let connection = Database::connect(DATABASE_URL.clone()).await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let configuration = _get_configuration(&connection.unwrap()).await;
    if let Err(c) = configuration {
        return Err(c.to_string());
    }
    Ok(configuration.unwrap())
}

#[tauri::command(async)]
pub async fn update_configuration(
    id: u32,
    configuration: ConfigurationData,
) -> Result<configuration::Model, String> {
    let connection = Database::connect(DATABASE_URL.clone()).await;
    if let Err(c) = connection {
        return Err(c.to_string());
    }
    let configuration = _update_configuration(&connection.unwrap(), id, configuration).await;
    if let Err(c) = configuration {
        return Err(c.to_string());
    }
    Ok(configuration.unwrap())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationData {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub price_hour: f64,
    pub state: String,
}

async fn insert_configuration(
    connection: &DbConn,
    configuration_data: ConfigurationData,
) -> Result<configuration::Model> {
    let configuration = configuration::ActiveModel {
        start_time: Set(configuration_data.start_time),
        end_time: Set(configuration_data.end_time),
        price_hour: Set(configuration_data.price_hour),
        state: Set(configuration_data.state),
        ..Default::default()
    };
    let result = configuration.save(connection).await?;
    let model = result.try_into_model()?;
    Ok(model)
}

async fn _get_configuration(connection: &DbConn) -> Result<configuration::Model> {
    let configuration = Configuration::find().one(connection).await?;
    if configuration.is_none() {
        let default_configuration = ConfigurationData {
            start_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            price_hour: 23.04,
            state: String::from("SP"),
        };
        let config = insert_configuration(connection, default_configuration).await?;
        return Ok(config);
    }
    let model = configuration.unwrap().try_into_model()?;
    Ok(model)
}
async fn _update_configuration(
    connection: &DbConn,
    id: u32,
    configuration: ConfigurationData,
) -> Result<configuration::Model> {
    let configuration = configuration::ActiveModel {
        id: Set(id),
        price_hour: Set(configuration.price_hour),
        start_time: Set(configuration.start_time),
        end_time: Set(configuration.end_time),
        state: Set(configuration.state),
        ..Default::default()
    };
    let result = configuration.update(connection).await?;
    let model = result.try_into_model()?;
    Ok(model)
}

#[cfg(test)]
mod tests {
    use sea_orm::{Database, DeleteResult, ModelTrait};

    use super::*;
    use crate::persistence::sqlite_manager::setup_db;

    async fn delete_configuration(connection: &DbConn, config_id: u32) -> Result<DeleteResult> {
        let configuration = Configuration::find_by_id(config_id).one(connection).await?;
        if configuration.is_none() {
            return Err(anyhow::anyhow!("Configuration not found"));
        }
        let result = configuration.unwrap().delete(connection).await?;
        Ok(result)
    }

    #[tokio::test]
    async fn test_insert_configuration() -> Result<()> {
        let connection = Database::connect(DATABASE_URL.clone())
            .await
            .unwrap();

        setup_db(&connection).await?;

        let configuration_data = ConfigurationData {
            start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            price_hour: 10.0,
            state: String::from("SP"),
        };

        let result = insert_configuration(&connection, configuration_data).await;
        assert!(result.is_ok());
        let cleanup_result = delete_configuration(&connection, result.unwrap().id).await;
        assert!(cleanup_result.is_ok());

        Ok(())
    }
}
