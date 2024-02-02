use chrono::{DateTime, NaiveTime, Utc};
use sea_orm;
use sea_orm::entity::prelude::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Checkpoints")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub checkpoint: DateTime<Utc>,
    pub price_hour: f64,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
