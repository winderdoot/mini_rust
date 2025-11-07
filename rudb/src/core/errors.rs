use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Field '{field}' not found in table '{field}'")]
    InvalidField { table: String, field: String },
    #[error("Key '{key}' not found in table '{table}'")]
    InvalidKey { table: String, key: String },
    #[error("Table '{0}' not found")]
    TableNotFound(String),
    #[error("Failed to generate new unique key")]
    KeyGenerationError,
}

pub type DbResult<T> = std::result::Result<T, DbError>;