use thiserror::Error;
use crate::cli::commands::AnyCommand;

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
    #[error("Column with name '{0}' repeats. Consider choosing another column name")]
    ColumnExists(String),
    #[error("Column name cannot contain '{0}'")]
    ColumnInvalidChar(char),
    #[error("Unreachable error")]
    Unreachable

}

pub type ParseResult<'a> = Result<AnyCommand<'a>, ParseErr>;