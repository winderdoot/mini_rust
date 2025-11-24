use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbErr {
    #[error(transparent)]
    Create(#[from] CreateErr),
    #[error(transparent)]
    Insert(#[from] InsertErr),
    #[error(transparent)]
    Delete(#[from] DeleteErr),
    #[error(transparent)]
    Select(#[from] SelectErr),
    #[error(transparent)]
    Record(#[from] RecordErr),
    #[error("Unreachable error 🦀")]
    Unreachable,
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
    #[error("Primary key '{0}' missing")]
    PrimaryKeyMissing(String),
    #[error("Table '{0}' not found")]
    TableNotFound(String),
    #[error("Invalid primary key type '{got}', expected '{expected}'")]
    InvalidKeyType { got: String, expected: String },
}

#[derive(Error, Debug)]
pub enum DeleteErr {
    #[error("Record with primary key '{key}' not found in table '{table}'")]
    InvalidKey { table: String, key: String },
    #[error("Table '{0}' not found")]
    TableNotFound(String),
}

#[derive(Error, Debug)]
pub enum SelectErr {
    #[error("Field '{field}' not found in table '{table}'")]
    InvalidField { table: String, field: String },
    #[error(transparent)]
    Record(#[from] RecordErr),
    #[error(transparent)]
    Condition(#[from] ConditionErr)
}

pub type DbResult<T> = std::result::Result<T, DbErr>;


#[derive(Error, Debug)]
pub enum RecordErr {
    #[error("Field '{0}' is inalid")]
    InvalidField(String),
    #[error(transparent)]
    Condition(#[from] ConditionErr)
}

#[derive(Error, Debug)]
pub enum ConditionErr {
    #[error("Mismatched operand types: left: '{left}', right: '{right}'")]
    MismatchedOperandTypes { left: String, right: String },
}
