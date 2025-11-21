use std::{collections::HashMap, iter::Peekable, str::SplitWhitespace};

use crate::{cli::errors::{ParseErr, ParseResult}, database::{Database, DatabaseKey, FieldType, Record, Schema, Table, Value}, errors::DbErr};


/* Parsing helpers */

/* This is a bit dumb but I don't have time to think about how to better design parsing.
 * Actually I should have made a Token trait and implemented it for str&. You could chain methods
 * on the token trait and have readable syntax for what is supposed to follow the token and stuff.
 * Damn. 
 */

pub fn token_stream<'a>(string: &'a str) -> Peekable<SplitWhitespace<'a>> 
{
    string.split_whitespace().peekable()
}

pub fn matches_charset<'a>(token: &'a str, charset: &str) -> Result<&'a str, ParseErr> {
    match token.chars().find(|c| !charset.contains(*c)) {
        Some(c) => Err(ParseErr::FieldInvalidChar(c)),
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
        .ok_or(ParseErr::ExpectedToken(format!("{expect}{sep}")))
        .and_then(|tok| {
            if tok.ends_with(sep) {
                return Ok(tok.trim_end_matches(sep));
            }
            if iter.peek().is_some_and(|next| *next == sep) {
                iter.next();
                return Ok(tok);
            }
            Err(ParseErr::ExpectedToken(format!("{expect}{sep}")))
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
    fn parse_from(string: &str) -> Result<Self, ParseErr> {
        match string.to_ascii_lowercase().as_str() {
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            val => Err(ParseErr::InvalidType(val.to_string()))
        }
    }
}


impl Value {
    fn parse_from(string: &str, target: &FieldType) -> Result<Self, ParseErr> {
        match target {
            FieldType::Bool => {
                Ok(
                    string
                    .eq_ignore_ascii_case("true")
                    .then(|| Value::Bool(true))
                    .or_else(|| 
                        string
                            .eq_ignore_ascii_case("false")
                            .then(|| Value::Bool(false))
                    )
                    .ok_or_else(|| ParseErr::InvalidLiteral { literal: string.to_string(), typ: FieldType::Bool.to_string() })?
                )
            },
            FieldType::String => todo!(),
            FieldType::Int => todo!(),
            FieldType::Float => todo!(),
        }
    }
}

impl Record {
    fn parse_from<K: DatabaseKey>(map: &HashMap<String, String>, schema: &Schema, table: &str) -> Result<Self, ParseErr> {
        map
            .keys()
            .find(|field| !schema.get_fields().contains_key(*field))
            .map_or_else(|| Ok(()), |field| Err(ParseErr::InvalidField { field: field.clone(), table: table.to_string() }))?;
        schema
            .get_fields()
            .keys()
            .find(|field| !map.contains_key(*field))
            .map_or_else(|| Ok(()), |field| Err(ParseErr::MissingField { field: field.clone(), table: table.to_string() }))?;
        let res: HashMap<String, Result<Value, ParseErr>> = map
            .iter()
            .map(|(field, val)| (*field, Value::parse_from(val)))
            .collect();
        // res
        //     .iter()
        //     .map(|(field, res)| )
        //     .map_or_else(|| Ok(()), |(field, res)| (*res)?);


        todo!("Think of a way to check if any values failed to parse. Worst case scenario use loops")
    }
}

/* Trait  */

pub trait Command<'a, K>
where 
    Self: Sized,
    K: DatabaseKey
{
    /// Execute command on a database. The output is printed to stdout.
    fn exec(&mut self) -> Result<String, DbErr>;

    /// Parse command from a token iterator and a database
    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where
        I: Iterator<Item = &'b str>;
}


/* Create  */

const FIELD_NAME_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";

pub struct Create<'a, K: DatabaseKey> {
    database: &'a mut Database<K>,
    table: String,
    schema: Schema,
}

impl<'a, K: DatabaseKey> Command<'a, K> for Create<'a, K> {
    fn exec(&mut self) -> Result<String, DbErr> {
        self.database.add_table(&self.table, &self.schema)?;
        Ok(format!("Successfuly created '{}'", self.table))
    }
    
    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where
        I: Iterator<Item = &'b str> 
    {
        let table = next_token(tokens, "CREATE", "<TABLE NAME>")?;
        expect_token(tokens, table, "KEY")?;
        let key = next_token(tokens, "KEY", "<KEY_NAME>")?;
        expect_token(tokens, key, "FIELDS")?;

        let mut schema_map = HashMap::<String, FieldType>::new();
        loop {
            let field_name = matches_charset(token_separator(tokens, "<FIELD_NAME>", ":")?, FIELD_NAME_CHARSET)?;
            let mut comma = false;
            let field_type = FieldType::parse_from(token_maybe_separator(tokens, "<FIELD_TYPE>", ",", &mut comma)?)?;
            
            match schema_map.insert(field_name.to_string(), field_type) {
                Some(_) => return Err(ParseErr::FieldExists(field_name.to_string())),
                None => {},
            }
            if comma {
                continue;
            }            
            expect_empty(tokens, ",")?;
            break;
        }
        let schema = Schema::from_map(schema_map, key).ok_or_else(|| ParseErr::MissingPrimaryKey(key.to_string()))?;
        Ok(AnyCommand::Create(Create { database, table: table.to_string(), schema }))
    }
}


/* Insert */

pub struct Insert<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    record: Record,
}

impl<'a, K: DatabaseKey> Command<'a, K> for Insert<'a, K> {
    fn exec(&mut self) -> Result<String, DbErr> {
        todo!()
    }

    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where 
        I: Iterator<Item = &'b str>
    {
        let table = next_token(tokens, "INSERT", "<FIELD_NAME>")?;

        let mut field_map = HashMap::<String, String>::new();
        loop {
            let field_name = matches_charset(token_separator(tokens, "<FIELD_NAME>", "=")?, FIELD_NAME_CHARSET)?;
            let mut comma = false;
            let field_val = token_maybe_separator(tokens, "<FIELD_VALUE>", ",", &mut comma)?;
            
            match field_map.insert(field_name.to_string(), field_val.to_string()) {
                Some(_) => return Err(ParseErr::FieldExists(field_name.to_string())),
                None => {},
            }
            if !comma {
                break;
            }
        }
        expect_token(tokens, "<FIELD_VALUE>", "INTO")?;
        let table = database.get_table_mut(next_token(tokens, "INTO", "<TABLE_NAME>")?)?;
        let record = Record::parse_from::<K>(&field_map, table.get_schema(), table.get_name())?;
        Ok( AnyCommand::Insert(Insert::<'a, K> { table, record }))
    }
}

/* Enum */

pub enum AnyCommand<'a, K: DatabaseKey> {
    Create(Create<'a, K>),
    Insert(Insert<'a, K>)
}

impl<'a, K: DatabaseKey> Command<'a, K> for AnyCommand<'a, K> {
    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K> 
    where 
        I: Iterator<Item = &'b str>
    {
        let mut command_name= tokens
            .next()
            .ok_or(ParseErr::Empty)?
            .to_string();

        command_name.make_ascii_lowercase();

        match command_name.as_str() {
            "create" => {
                Create::parse_from(tokens, database)
            },
            _ => {
                Err(ParseErr::UnknownCommand(command_name))
            }
        }
    }
    
    fn exec(&mut self) -> Result<String, DbErr> {
        match self {
            AnyCommand::Create(create) => create.exec(),
            AnyCommand::Insert(insert) => todo!(),
        }
    }
}
