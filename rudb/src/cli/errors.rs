use thiserror::Error;
use crate::{cli::commands::AnyCommand, database::DatabaseKey, errors::DbErr};

#[derive(Error, Debug)]
pub enum ParseErr {
    #[error("Empty line detected")]
    Empty,
    #[error("Unknown command '{0}'")]
    UnknownCommand(String),
    #[error("After '{prev}' expecting {missing}")]
    MissingToken { prev: String, missing: String },
    #[error("Expecting '{0}'")]
    ExpectedToken(String),
    #[error("Expected '{expected}' got '{got}'")]
    WrongToken { expected: String, got: String },
    #[error("'{0}' is not a valid field type")]
    InvalidType(String),
    #[error("'{literal}' is not a valid '{typ}' literal")]
    InvalidLiteral { literal: String, typ: String },
    #[error("Field with name '{0}' repeats. Consider choosing another column name")]
    FieldExists(String),
    #[error("Field name cannot contain '{0}'")]
    FieldInvalidChar(char),
    #[error("Field '{field}' doesn't exist in table '{table}'")]
    InvalidField { field: String, table: String },
    #[error("Field '{field}' from' {table}' is missing")]
    MissingField { field: String, table: String },
    #[error("Missing primary key '{0}' from FIELDS definiton")]
    MissingPrimaryKey(String),
    #[error("Unknown operator '{0}'")]
    UnknownOperator(String),
    #[error("Unreachable error 🥶")]
    Unreachable,
    #[error(transparent)]
    Database(#[from] DbErr)
}

pub type ParseResult<'a, K: DatabaseKey> = Result<AnyCommand<'a, K>, ParseErr>;