use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("CREATE: '{0}'")]
    Create(CreateErr),
    #[error("INSERT: '{0}'")]
    Insert(InsertErr),
    #[error("DELETE: '{0}'")]
    Delete(DeleteErr)
}

#[derive(Error, Debug)]
pub enum CreateErr {
    #[error("Table '{table}' already exists!")]
    AlreadyExists { table: String }
}

#[derive(Error, Debug)]
pub enum InsertErr {
    #[error("Field '{field}' not found in table '{table}'")]
    InvalidField { table: String, field: String },
    #[error("Field '{field}' present in table '{table}' is missing")]
    MissingField { table: String, field: String },
    #[error("Key '{key}' already used in table '{table}'")]
    KeyUsed { table: String, key: String },
    #[error("Table '{0}' not found")]
    TableNotFound(String),
}

#[derive(Error, Debug)]
pub enum DeleteErr {
    #[error("Key '{key}' not found in table '{table}'")]
    InvalidKey { table: String, key: String },
    #[error("Table '{0}' not found")]
    TableNotFound(String),
}

pub type DbResult<T> = std::result::Result<T, DbError>;


#[derive(Error, Debug)]
pub enum RecordError {
    #[error("Field '{0}' is inalid")]
    InvalidField(String),
    #[error("Field '{0}' is missing")]
    MissingField(String),
}