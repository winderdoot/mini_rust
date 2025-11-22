use std::{collections::HashMap, iter::Peekable};

use crate::{cli::errors::{ParseErr, ParseResult}, database::{Database, DatabaseKey, FieldType, Record, Schema, Table, Value}, errors::DbErr};


/* Parsing helpers */

/* This is a bit dumb but I don't have time to think about how to better design parsing.
 * Actually I should have made a Token trait and implemented it for str&. You could chain methods
 * on the token trait and have readable syntax for what is supposed to follow the token and stuff.
 * Damn. 
 */

fn split_and_keep<'a>(text: &'a str, sep: &'a str) -> Vec<&'a str> {
    let mut parts = Vec::new();
    let mut last_end = 0;

    for (start, matched) in text.match_indices(sep) {
        if start > last_end {
            parts.push(&text[last_end..start]);
        }
        parts.push(matched);
        last_end = start + matched.len();
    }

    if last_end < text.len() {
        parts.push(&text[last_end..]);
    }

    parts.into_iter().filter(|s| !s.is_empty()).collect()
}

pub fn token_stream<'a, I: Iterator<Item = &'a str>>(string: &'a str) -> Peekable<impl Iterator<Item = &'a str>> 
{
    string
        .split_whitespace()
        .flat_map(|tok| split_and_keep(tok, ","))
        .flat_map(|tok| split_and_keep(tok, ":"))
        .flat_map(|tok| split_and_keep(tok, "="))
        .peekable()

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
    fn parse_bool(string: &str) -> Result<Self, ParseErr> {
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
    }

    // At this point I had no energy left to use method chaining
    fn parse_string(string: &str) -> Result<Self, ParseErr> {
        if string.len() < 2 {
            return Err(ParseErr::InvalidLiteral { literal: string.to_string(), typ: FieldType::String.to_string() });
        }
        if !string.starts_with("\"") || !string.ends_with("\"") {
            return Err(ParseErr::InvalidLiteral { literal: string.to_string(), typ: FieldType::String.to_string() }); 
        }
        let inner = &string[1..string.len() - 1];
        if inner.contains("\"") {
            return Err(ParseErr::InvalidLiteral { literal: string.to_string(), typ: FieldType::String.to_string() }); 
        }
        Ok(Value::String(inner.to_string()))
    }

    fn parse_from(string: &str, target: &FieldType) -> Result<Self, ParseErr> {
        match target {
            FieldType::Bool => {
                Self::parse_bool(string)
            },
            FieldType::String => { 
                Self::parse_string(string)
            },
            FieldType::Int => {
                match string.parse::<i64>() {
                    Ok(v) => Ok(Value::Int(v)),
                    Err(_) => Err(ParseErr::InvalidLiteral { literal: string.to_string(), typ: FieldType::Int.to_string() }),
                }
            },
            FieldType::Float => {
                match string.parse::<f64>() {
                    Ok(v) => Ok(Value::Float(v)),
                    Err(_) => Err(ParseErr::InvalidLiteral { literal: string.to_string(), typ: FieldType::Float.to_string() }),
                }
            }
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
        let mut record_map: HashMap<String, Value> = HashMap::new();
        for (field, val_str) in map {
            let field_type = schema.get_fields().get(field).ok_or_else(|| ParseErr::Unreachable)?;
            let parsed_val = Value::parse_from(val_str, field_type)?;
            record_map.insert(field.to_string(), parsed_val);
        }
        // Hm to chyba się dało collectem zrobic
        Ok(Record::from_map(record_map))
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
        Ok(format!("Successfuly created '{}'.", self.table))
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
    key: K,
    record: Record,
}

impl<'a, K: DatabaseKey> Command<'a, K> for Insert<'a, K> {
    fn exec(&mut self) -> Result<String, DbErr> {
        self.table.insert(&self.key.clone(), self.record.clone())?;
        Ok(format!("Successfuly inserted."))
    }

    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where 
        I: Iterator<Item = &'b str>
    {
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
        let key = record.get_key::<K>(table.get_schema()).ok_or_else(|| ParseErr::Unreachable)?;

        Ok( AnyCommand::Insert(Insert::<'a, K> { table, key, record }))
    }
}


/* Delete */

pub struct Delete<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    key: K
}

impl<'a, K: DatabaseKey> Command<'a, K> for Delete<'a, K> {
    fn exec(&mut self) -> Result<String, DbErr> {
        self.table.delete(&self.key)?;
        Ok(format!("Successfuly deleted."))
    }

    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where
        I: Iterator<Item = &'b str> 
    {
        let key_val_str = next_token(tokens, "DELETE", "<KEY_VALUE>")?;
        expect_token(tokens, "<KEY_VALUE>", "FROM")?;
        let table = database.get_table_mut(next_token(tokens, "FROM", "<TABLE_NAME>")?)?;
        let key = K::from_value(
            &Value::parse_from(
                key_val_str, 
                table
                    .get_schema()
                    .get_key_type()
                    .ok_or_else(|| ParseErr::Unreachable)?
            )?
        )
        .ok_or_else(|| ParseErr::Unreachable)?;
    
        Ok(AnyCommand::Delete(Delete::<'a, K> { table, key }))
    }
}


/* Select */

pub struct Select<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    fields: Vec<String>,
}

impl<'a, K: DatabaseKey> Command<'a, K> for Select<'a, K> {
    fn exec(&mut self) -> Result<String, DbErr> {

        todo!()
    }

    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where
        I: Iterator<Item = &'b str> 
    {
        let mut fields= Vec::new();
        loop {
            let mut comma = false;
            let field_name = matches_charset(token_maybe_separator(tokens, "<FIELD_NAME>", ",", &mut comma)?, FIELD_NAME_CHARSET)?;
            
            fields.push(field_name.to_string());
            if !comma {
                break;
            }
        }
        expect_token(tokens, "<FIELD_NAME>", "FROM")?;
        let table = database.get_table_mut(next_token(tokens, "FROM", "<TABLE_NAME>")?)?;

        Ok(AnyCommand::Select(Select::<'a, K> { table, fields }))
    }
}

/* Enum */

pub enum AnyCommand<'a, K: DatabaseKey> {
    Create(Create<'a, K>),
    Insert(Insert<'a, K>),
    Delete(Delete<'a, K>),
    Select(Select<'a, K>),
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
            "insert" => {
                Insert::parse_from(tokens, database)
            },
            "delete" => {
                Delete::parse_from(tokens, database)
            },
            _ => {
                Err(ParseErr::UnknownCommand(command_name))
            }
        }
    }
    
    fn exec(&mut self) -> Result<String, DbErr> {
        match self {
            AnyCommand::Create(create) => create.exec(),
            AnyCommand::Insert(insert) => insert.exec(),
            AnyCommand::Delete(delete) => delete.exec(),
            AnyCommand::Select(select) => select.exec(),
        }
    }
}
