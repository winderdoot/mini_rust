use std::{collections::HashMap, iter::Peekable, string::ParseError};

use crate::{cli::errors::{self, ParseErr, ParseResult}, database::{AnyDatabase, DatabaseKey, FieldType, Schema, Table}, errors::DbResult};


/* Parsing helpers */

/* This is a bit dumb but I don't have time to think about how to better design parsing.
 * Actually I should have made a Token trait and implemented it for str&. You could chain methods
 * on the token trait and have readable syntax for what is supposed to follow the token and stuff.
 * Damn. 
 */

pub fn matches_charset<'a>(token: &'a str, charset: &str) -> Result<&'a str, ParseErr> {
    match token.chars().find(|c| !charset.contains(*c)) {
        Some(c) => Err(ParseErr::ColumnInvalidChar(c)),
        None => Ok(token),
    }
}

/// Advances the iterator, expecting a certain token
pub fn next_token<'a, I>(iter: &mut I, prev: &str, expect: &str) -> Result<I::Item, ParseErr>
where 
    I: Iterator<Item = &'a str> 
{
    iter
        .next()
        .ok_or(ParseErr::MissingToken { prev: prev.to_string(), missing: expect.to_string() })
}


/// Advances the iterator and checks if next token matches a value
pub fn expect_token<'a, I>(iter: &mut I, prev: &str, expect: &str) -> Result<I::Item, ParseErr>
where 
    I: Iterator<Item = &'a str>
{
    iter
        .next()
        .ok_or_else(|| ParseErr::MissingToken { prev: prev.to_string(), missing: expect.to_string() })
        .and_then(|tok| {
            if tok.eq_ignore_ascii_case(expect) {
                return Ok(tok);
            }
            Err(ParseErr::MissingToken { prev: prev.to_string(), missing: expect.to_string() })
        })
}

/// Advances the iterator, expecting a token and a trailing separator, that could also be the next token afterwads.
/// In the latter case, consumes the separator token as well.
pub fn token_separator<'a, I>(iter: &mut Peekable<I>, expect: &str, sep: &str) -> Result<I::Item, ParseErr>
where
    I: Iterator<Item = &'a str>
{
    iter
        .next()
        .ok_or(ParseErr::ExpectedToken(expect.to_string()))
        .and_then(|tok| {
            if tok.ends_with(sep) {
                return Ok(tok.trim_end_matches(sep));
            }
            if iter.peek().is_some_and(|next| *next == sep) {
                iter.next();
                return Ok(tok);
            }
            Err(ParseErr::MissingToken { prev: tok.to_string(), missing: sep.to_string() })
        })
}

pub fn token_maybe_separator<'a, I>(iter: &mut Peekable<I>, expect: &str, sep: &str, found_sep: &mut bool) -> Result<I::Item, ParseErr>
where
    I: Iterator<Item = &'a str>
{
    iter
        .next()
        .ok_or(ParseErr::ExpectedToken(expect.to_string()))
        .and_then(|tok| {
            if tok.ends_with(sep) {
                *found_sep = true;
                return Ok(tok.trim_end_matches(sep));
            }
            if iter.peek().is_some_and(|next| *next == sep) {
                iter.next();
                *found_sep = true;
                return Ok(tok);
            }
            Ok(tok)
        })
}

pub fn expect_empty<'a, I>(iter: &mut Peekable<I>, expect: &str) -> Result<(), ParseErr>
where
    I: Iterator<Item = &'a str>
{
    match iter.next() {
        Some(tok) => Err(ParseErr::WrongToken { expected: expect.to_string(), got: tok.to_string() }),
        None => Ok(()),
    }
}


impl FieldType {
    fn parse_from(value: &str) -> Result<Self, ParseErr> {
        match value.to_ascii_lowercase().as_str() {
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            val => Err(ParseErr::InvalidType(val.to_string()))
        }
    }
}

/* Trait  */

pub trait Command 
where 
Self: Sized {
    /// Execute command on a database. The output is printed to stdout.
    fn exec(&mut self);
}


/* Create  */

const COLUMN_NAME_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";

pub struct Create<'a> {
    database: &'a mut AnyDatabase,
    table: String,
    schema: Schema,
}

impl<'a> Create<'a> {
    /// I am really not impressed with myself
    pub fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut AnyDatabase) -> ParseResult<'a>
    where 
        I: Iterator<Item = &'b str>
    {
        let table = next_token(tokens, "CREATE", "<TABLE NAME>")?;
        expect_token(tokens, table, "KEY")?;
        let key = next_token(tokens, "KEY", "<KEY_NAME>")?;
        expect_token(tokens, key, "FIELDS")?;

        let mut schema_map = HashMap::<String, FieldType>::from([(key.to_string(), database.key_type())]);
        loop {
            let field_name = matches_charset(token_separator(tokens, "<COLUMN_NAME>", ":")?, COLUMN_NAME_CHARSET)?;
            let mut comma = false;
            let field_type = FieldType::parse_from(token_maybe_separator(tokens, "<COLUMN_TYPE>", ",", &mut comma)?)?;
            
            match schema_map.insert(field_name.to_string(), field_type) {
                Some(_) => return Err(ParseErr::ColumnExists(field_name.to_string())),
                None => {},
            }
            if comma {
                continue;
            }            
            expect_empty(tokens, ",")?;
            break;
        }
        let schema = Schema::from_map(schema_map, key).ok_or_else(|| ParseErr::Unreachable)?;
        Ok(AnyCommand::Create(Create { database, table: table.to_string(), schema }))
    }
}

impl<'a> Command for Create<'a> {
    fn exec(&mut self) {
        match self.database {
            AnyDatabase::StringDatabase(database) => {
                match database.add_table(&self.table, &self.schema) {
                    Ok(_) => println!("Table '{}' succcessfuly added.", self.table),
                    Err(err) => println!("Database error: {}", err)
                }
            },
            AnyDatabase::IntDatabase(database) => {
                match database.add_table(&self.table, &self.schema) {
                    Ok(_) => println!("Table '{}' succcessfuly added.", self.table),
                    Err(err) => println!("Database error: {}", err)
                }
            }
        }
    }
}


/* Insert */

pub struct Insert<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    key: String,
    schema: Schema,
}

impl<'a, K: DatabaseKey> Insert<'a, K> {
    pub fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut AnyDatabase) -> ParseResult<'a>
    where 
        I: Iterator<Item = &'b str>
    {
        let table = next_token(tokens, "INSERT", "<COLUMN NAME>")?;

        let mut column_map = HashMap::<String, String>::new();
        loop {
            let field_name = matches_charset(token_separator(tokens, "<COLUMN_NAME>", "=")?, COLUMN_NAME_CHARSET)?;
            let mut comma = false;
            let field_val = token_maybe_separator(tokens, "<COLUMN_VALUE>", ",", &mut comma)?;
            
            match column_map.insert(field_name.to_string(), field_val.to_string()) {
                Some(_) => return Err(ParseErr::ColumnExists(field_name.to_string())),
                None => {},
            }
            if !comma {
                break;
            }
        }
        expect_token(tokens, "<COLUMN_VALUE>", "INTO")?;
        let table_name = next_token(tokens, "INTO", "<TABLE_NAME>")?;
        match database {
            AnyDatabase::StringDatabase(database) => database.get_table_mut(table),
            AnyDatabase::IntDatabase(database) => todo!(),
        }

        todo!();
    }
}

// impl<'a> Command for Create<'a> {
//     fn exec(&mut self) {
//         match self.database {
//             AnyDatabase::StringDatabase(database) => {
//                 match database.add_table(&self.table, &self.schema) {
//                     Ok(_) => println!("Table '{}' succcessfuly added.", self.table),
//                     Err(err) => println!("Database error: {}", err)
//                 }
//             },
//             AnyDatabase::IntDatabase(database) => {
//                 match database.add_table(&self.table, &self.schema) {
//                     Ok(_) => println!("Table '{}' succcessfuly added.", self.table),
//                     Err(err) => println!("Database error: {}", err)
//                 }
//             }
//         }
//     }
// }

/* Enum  */

pub enum AnyCommand<'a> {
    Create(Create<'a>),
}

impl<'a> AnyCommand<'a> {
    pub fn parse_from(str: &str, database: &'a mut AnyDatabase) -> ParseResult<'a> {
        let mut tokens = str.split_whitespace().peekable();
        let mut command_name= tokens
            .next()
            .ok_or(ParseErr::Empty)?
            .to_string();

        command_name.make_ascii_lowercase();

        match command_name.as_str() {
            "create" => {
                Create::parse_from(&mut tokens, database)
            },
            _ => {
                Err(ParseErr::UnknownCommand(command_name))
            }
        }
    }
}

impl<'a> Command for AnyCommand<'a> {
    fn exec(&mut self) {
        match self {
            AnyCommand::Create(create) => create.exec(),
        }
    }
}