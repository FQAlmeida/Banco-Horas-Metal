use crate::models::configuration::Entity as Configuration;
use crate::models::register::Entity as Register;
use anyhow::Result;
use sea_orm::{sea_query::SqliteQueryBuilder, ConnectionTrait, DbBackend, DbConn, Schema};

pub async fn setup_db(db: &DbConn) -> Result<()> {
    let schema = Schema::new(DbBackend::Sqlite);

    let stmt_register_table = schema
        .create_table_from_entity(Register)
        .if_not_exists()
        .to_owned();

    stmt_register_table.build(SqliteQueryBuilder);

    db.execute(db.get_database_backend().build(&stmt_register_table))
        .await?;

    let stmt_config_table = schema
        .create_table_from_entity(Configuration)
        .if_not_exists()
        .to_owned();

    stmt_config_table.build(SqliteQueryBuilder);

    db.execute(db.get_database_backend().build(&stmt_config_table))
        .await?;

    Ok(())
}
