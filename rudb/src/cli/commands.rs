use std::{collections::HashMap, iter::Peekable, str::SplitWhitespace};

use crate::{cli::errors::{ParseErr, ParseResult}, database::{Condition, Database, DatabaseKey, FieldType, Record, Schema, Table, Value}, errors::DbErr};
use crate::cli::tokens::*;



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
        string
        .eq_ignore_ascii_case("true")
        .then_some(Value::Bool(true))
        .or_else(|| 
            string
                .eq_ignore_ascii_case("false")
                .then_some(Value::Bool(false))
        )
        .ok_or_else(|| ParseErr::InvalidLiteral { literal: string.to_string(), typ: FieldType::Bool.to_string() })
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
    fn parse_from(map: &HashMap<String, String>, schema: &Schema, table: &str) -> Result<Self, ParseErr> {
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
            let field_type = schema.get_fields().get(field).ok_or(ParseErr::Unreachable)?;
            let parsed_val = Value::parse_from(val_str, field_type)?;
            record_map.insert(field.to_string(), parsed_val);
        }
        // Hm to chyba się dało collectem zrobic
        Ok(Record::from_map(record_map))
    }
}


impl Condition {
    fn parse_from(op_str: &str, value: Value) -> Result<Self, ParseErr> {
        match op_str {
            ">" => {
                Ok(Self::Greater(value))
            },
            "<" => {
                Ok(Self::LessThan(value))
            },
            "=" => {
                Ok(Self::Equals(value))
            },
            "!=" => {
                Ok(Self::NotEquals(value))
            },
            ">=" => {
                Ok(Self::GreaterEqual(value))
            },
            "<=" => {
                Ok(Self::LessEqual(value))
            },
            other => {
                Err(ParseErr::UnknownOperator(other.to_string()))
            }
        }
    }
}


/* Trait  */

pub trait Command<'a, K>
where 
    Self: Sized,
    K: DatabaseKey
{
    /// Execute command on a database. The output is printed to stdout.
    fn exec(&mut self, history: &[String]) -> Result<String, DbErr>;

    /// Parse command from a token iterator and a database
    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where
        I: Iterator<Item = &'b str>;
}


/* Create  */

const FIELD_NAME_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";
const COND_OPERATORS: &[&str] = &["=", ">", "<", "!=", "<=", ">="];

pub struct Create<'a, K: DatabaseKey> {
    database: &'a mut Database<K>,
    table: String,
    schema: Schema,
}

impl<'a, K: DatabaseKey> Command<'a, K> for Create<'a, K> {
    fn exec(&mut self, _history: &[String]) -> Result<String, DbErr> {
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
            
            if schema_map.insert(field_name.to_string(), field_type).is_some() {
                return Err(ParseErr::FieldExists(field_name.to_string()));
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
    fn exec(&mut self, _history: &[String]) -> Result<String, DbErr> {
        self.table.insert(&self.key.clone(), self.record.clone())?;
        Ok(String::from("Successfuly inserted."))
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
            
            if field_map.insert(field_name.to_string(), field_val.to_string()).is_some() {
                return Err(ParseErr::FieldExists(field_name.to_string()));
            }
            if !comma {
                break;
            }
        }
        expect_token(tokens, "<FIELD_VALUE>", "INTO")?;
        let table = database.get_table_mut(next_token(tokens, "INTO", "<TABLE_NAME>")?)?;
        let record = Record::parse_from(&field_map, table.get_schema(), table.get_name())?;
        let key = record.get_key::<K>(table.get_schema()).ok_or(ParseErr::Unreachable)?;

        Ok( AnyCommand::Insert(Insert::<'a, K> { table, key, record }))
    }
}


/* Delete */

pub struct Delete<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    key: K
}

impl<'a, K: DatabaseKey> Command<'a, K> for Delete<'a, K> {
    fn exec(&mut self, _history: &[String]) -> Result<String, DbErr> {
        self.table.delete(&self.key)?;
        Ok(String::from("Successfuly deleted."))
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
                    .ok_or(ParseErr::Unreachable)?
            )?
        )
        .ok_or(ParseErr::Unreachable)?;
    
        Ok(AnyCommand::Delete(Delete::<'a, K> { table, key }))
    }
}


/* Select */

pub struct Select<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    fields: Vec<String>,
    conditions: HashMap<String, Condition>
}

impl<'a, K: DatabaseKey> Select<'a, K> {
    fn parse_where<'b, I>(tokens: &mut Peekable<I>, table: &Table<K>) -> Result<HashMap<String, Condition>, ParseErr>
    where
        I: Iterator<Item = &'b str>
    {
        let mut tokens_empty = false;
        token_or_empty(tokens, "<TABLE_NAME>", "WHERE", &mut tokens_empty)?;
        if tokens_empty {
            return Ok(HashMap::new())
        }
        
        let mut condidtions = HashMap::<String, Condition>::new();
        loop {
            let mut found_sep: &str = "";
            let field_name = matches_charset(token_any_separator(tokens, "<FIELD_NAME>", COND_OPERATORS, &mut found_sep)?, FIELD_NAME_CHARSET)?;
            let mut comma = false;
            let field_type = table.get_field_type(field_name)?;
            let field_val = Value::parse_from(token_maybe_separator(tokens, "<FIELD_VALUE>", ",", &mut comma)?, field_type)?;
            let condition = Condition::parse_from(found_sep, field_val)?;
            
            if condidtions.insert(field_name.to_string(), condition).is_some() {
                return Err(ParseErr::FieldExists(field_name.to_string()));
            }
            if !comma {
                break;
            }
        }
        
        Ok(condidtions)
    }   
}

impl<'a, K: DatabaseKey> Command<'a, K> for Select<'a, K> {
    fn exec(&mut self, _history: &[String]) -> Result<String, DbErr> {
        self.table.select(&self.fields, &self.conditions)
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

        let conditions = Self::parse_where(tokens, table)?;

        Ok(AnyCommand::Select(Select::<'a, K> { table, fields, conditions }))
    }
}


/* ReadFrom */

pub struct ReadFrom<'a, K: DatabaseKey> {
    database: &'a mut Database<K>,
    lines: Vec<String>, /* Due to lifetimes, I can't store parsed commands here. */
}

impl<'a, K: DatabaseKey> Command<'a, K> for ReadFrom<'a, K> {
    /* For lifetime reasons, commands have to be executed as they are parsed and this causes an issue where if a later command
     * fails to parse then we don't get result from the previous commands */
    fn exec(&mut self, history: &[String]) -> Result<String, DbErr> {
        let mut command_results = Vec::<String>::new();

        for line in &self.lines {
            let mut tokens = token_stream::<SplitWhitespace>(line);
            let mut command = match AnyCommand::<K>::parse_from(&mut tokens, self.database) {
                Ok(c) => Ok(c),
                Err(perr) => Err(DbErr::Parse(perr.to_string())),
            }?;
            command_results.push(command.exec(history)?);
        }

        Ok(command_results.join("\n"))
    }

    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K>
    where
        I: Iterator<Item = &'b str>
    {
        let file_name =  
            tokens
                .next()
                .ok_or_else(|| ParseErr::MissingToken {
                    prev: "READ_FROM".to_string(),
                    missing: "<FILE_NAME>".to_string()
                })?;
        
        expect_empty(tokens, "<NEWLINE>")?;

        let lines = std::fs::read_to_string(file_name)?
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>();
        
        Ok(AnyCommand::ReadFrom(ReadFrom { database, lines }))
    }
}

/* SaveAs */

pub struct SaveAs {
    file: String, /* Due to lifetimes, I can't store parsed commands here. */
}

impl<'a, K: DatabaseKey> Command<'a, K> for SaveAs {
    /* For lifetime reasons, commands have to be executed as they are parsed and this causes an issue where if a later command
     * fails to parse then we don't get result from the previous commands */
    fn exec(&mut self, history: &[String]) -> Result<String, DbErr> {
        std::fs::write(&self.file, history.join("\n").as_bytes())?;

        Ok(format!("Successfuly saved history to {}", self.file))
    }

    fn parse_from<'b, I>(tokens: &mut Peekable<I>, _database: &'a mut Database<K>) -> ParseResult<'a, K>
    where
        I: Iterator<Item = &'b str>
    {
        let file_name =  
            tokens
                .next()
                .ok_or_else(|| ParseErr::MissingToken {
                    prev: "READ_FROM".to_string(),
                    missing: "<FILE_NAME>".to_string()
                })?;
        
        expect_empty(tokens, "<NEWLINE>")?;
        
        Ok(AnyCommand::SaveAs(SaveAs { file: file_name.to_string() }))
    }
}

/* Enum */

pub enum AnyCommand<'a, K: DatabaseKey> {
    Create(Create<'a, K>),
    Insert(Insert<'a, K>),
    Delete(Delete<'a, K>),
    Select(Select<'a, K>),
    ReadFrom(ReadFrom<'a, K>),
    SaveAs(SaveAs),
}

impl<'a, K: DatabaseKey> Command<'a, K> for AnyCommand<'a, K> {
    fn parse_from<'b, I>(tokens: &mut Peekable<I>, database: &'a mut Database<K>) -> ParseResult<'a, K> 
    where 
        I: Iterator<Item = &'b str>
{
        let mut command_name= tokens.next().ok_or(ParseErr::Empty)?.to_string();
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
            "select" => {
                Select::parse_from(tokens, database)
            },
            "read_from" => {
                ReadFrom::parse_from(tokens, database)
            },
            "save_as" => {
                SaveAs::parse_from(tokens, database)
            },
            _ => {
                Err(ParseErr::UnknownCommand(command_name))
            }
        }
    }
    
    fn exec(&mut self, history: &[String]) -> Result<String, DbErr> {
        match self {
            AnyCommand::Create(create) => create.exec(history),
            AnyCommand::Insert(insert) => insert.exec(history),
            AnyCommand::Delete(delete) => delete.exec(history),
            AnyCommand::Select(select) => select.exec(history),
            AnyCommand::ReadFrom(read_from) => read_from.exec(history),
            AnyCommand::SaveAs(save_as) => Command::<'_, K>::exec(save_as, history),
        }
    }
}
