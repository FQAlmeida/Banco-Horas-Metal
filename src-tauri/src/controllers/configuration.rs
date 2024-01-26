use crate::models::configuration;
use anyhow::Result;
use chrono::NaiveTime;
use sea_orm::{ActiveModelTrait, DbConn, DeleteResult, EntityTrait, ModelTrait, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::models::configuration::Entity as Configuration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConfigurationData {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub price_hour: f64,
}

async fn insert_configuration(
    connection: &DbConn,
    configuration_data: ConfigurationData,
) -> Result<configuration::Model> {
    let configuration = configuration::ActiveModel {
        start_time: Set(configuration_data.start_time),
        end_time: Set(configuration_data.end_time),
        price_hour: Set(configuration_data.price_hour),
        ..Default::default()
    };
    let result = configuration.save(connection).await?;
    let model = result.try_into_model()?;
    Ok(model)
}

async fn delete_configuration(connection: &DbConn, config_id: u32) -> Result<DeleteResult> {
    let configuration = Configuration::find_by_id(config_id).one(connection).await?;
    if configuration.is_none() {
        return Err(anyhow::anyhow!("Configuration not found"));
    }
    let result = configuration.unwrap().delete(connection).await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    use crate::persistence::sqlite_manager::setup_db;

    use super::*;
    #[tokio::test]
    async fn test_insert_configuration() -> Result<()> {
        let connection = Database::connect("sqlite://data.db?mode=rwc")
            .await
            .unwrap();

        setup_db(&connection).await?;

        let configuration_data = ConfigurationData {
            start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            price_hour: 10.0,
        };

        let result = insert_configuration(&connection, configuration_data).await;
        assert!(result.is_ok());
        let cleanup_result = delete_configuration(&connection, result.unwrap().id).await;
        assert!(cleanup_result.is_ok());

        Ok(())
    }
}
