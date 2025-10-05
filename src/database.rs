use crate::config::DwellingConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Unable to connect to database: {0}")]
    NoConnection(String),
}

pub async fn init_database(config: &DwellingConfig) -> Result<Pool<Postgres>, DatabaseError> {
    let connection_string = format!(
        "postgres://{user}:{password}@{host}:{port}/{db}",
        user = &config.db_user,
        password = &config.db_password,
        host = &config.db_host,
        port = &config.db_port,
        db = &config.db_name
    );
    PgPoolOptions::new()
        .connect(&connection_string)
        .await
        .map_err(|err| DatabaseError::NoConnection(err.to_string()))
}
