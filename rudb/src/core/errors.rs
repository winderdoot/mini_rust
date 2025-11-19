use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Field '{field}' not found in table '{table}'")]
    InvalidField { table: String, field: String },
    #[error("Field '{field}' present in table '{table}' is missing")]
    MissingField { table: String, field: String },
    #[error("Key '{key}' not found in table '{table}'")]
    InvalidKey { table: String, key: String },
    #[error("Table '{0}' not found")]
    TableNotFound(String),
    #[error("Failed to generate new unique key")]
    KeyGenerationError,
}

pub type DbResult<T> = std::result::Result<T, DbError>;

#[derive(Error, Debug)]
pub enum RecordError {
    #[error("Field '{0}' is inalid")]
    InvalidField(String),
    #[error("Field '{0}' is missing")]
    MissingField(String),
}