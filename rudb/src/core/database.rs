use std::collections::{BTreeMap, HashMap};
use crate::{cli::commands::Create, core::errors::*};


/* === DatabaseKey === */

pub trait DatabaseKey
where 
Self: Sized + ToString + Clone + Ord {
    fn to_field_type() -> FieldType;
}

impl DatabaseKey for i64 {
    fn to_field_type() -> FieldType {
        FieldType::Int
    }
}

impl DatabaseKey for String {
    fn to_field_type() -> FieldType {
        FieldType::String
    }
}

// pub enum AnyDbKey {
//     Int(i64),
//     String(String)
// }


/* === Schema ===  */

#[derive(Clone)]
pub enum FieldType {
    Bool,
    String,
    Int,
    Float
}


/// Contains the column names and types of a table, excluding the primary key, due to how we implement the database.
#[derive(Clone)]
pub struct Schema {
    columns: HashMap<String, FieldType>,
    key: String
}

impl Schema {
    pub fn from_map(map: HashMap<String, FieldType>, key: &str) -> Option<Self> {
        if !map.contains_key(key) {
            return None;
        }
        Some(Schema { columns: map, key: key.to_string() })
    }
}

pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

pub struct Record {
    fields: HashMap<String, Value>
}

impl Record {
    fn matches(&self, schema: &Schema) -> Result<(), RecordError> {
        for field in schema.columns.keys() {
            if !self.fields.contains_key(field) {
                return Err(RecordError::MissingField(field.to_string()));
            }
        }
        for field in self.fields.keys() {
            if !schema.columns.contains_key(field) {
                return Err(RecordError::InvalidField(field.to_string()));
            }
        }
        Ok(())
    }
}


/* === Table === */

pub struct Table<K: DatabaseKey> {
    schema: Schema,
    records: BTreeMap<K, Record>,
}

impl<K: DatabaseKey> Table<K> {
    pub fn from_schema(schema: &Schema) -> Self {
        Table::<K> {
            schema: schema.clone(),
            records: BTreeMap::<K, Record>::new()
        }
    }
}



/* === Database === */

pub struct Database<K: DatabaseKey> {
    tables: HashMap<String, Table<K>>,
}

impl<K: DatabaseKey> Database<K> {
    pub fn new() -> Database<K> {
        Database::<K> {
            tables: HashMap::<String, Table<K>>::new(),
        }
    }

    pub fn get_table_mut(&mut self, table: &str) -> Option<&mut Table<K>> {
        self.tables.get_mut(table)
    }

    pub fn add_table(&mut self, table: &str, schema: &Schema) -> Result<(), CreateErr> {
        if self.tables.contains_key(table) {
            return Err(CreateErr::AlreadyExists { table: table.to_string() });
        }
        self.tables.insert(table.to_string(), Table::from_schema(schema));

        Ok(())
    }

    pub fn insert_into(&mut self, table: &str, record: Record, key: K) -> Result<(), InsertErr> {
        let res = self.tables
            .get_mut(table)
            .ok_or(InsertErr::TableNotFound(table.to_string()))?;
        match record.matches(&res.schema) {
            Ok(_) => {},
            Err(RecordError::InvalidField(f)) => {
                return Err(InsertErr::InvalidField { table: table.to_string(), field: f });
            },
            Err(RecordError::MissingField(f)) => {
                return Err(InsertErr::MissingField { table: table.to_string(), field: f });
            }
        }
        match res.records.insert(key.clone(), record) {
            Some(_) => {
                return Err(InsertErr::KeyUsed { table: table.to_string(), key: key.to_string() });
            },
            None => {
                return Ok(())
            },
        }
    }

    pub fn delete_from(&mut self, table: &str, key: K) -> Result<(), DeleteErr> {
        let res = self.tables
            .get_mut(table)
            .ok_or(DeleteErr::TableNotFound(table.to_string()))?;
        res.records
            .remove(&key)
            .ok_or(DeleteErr::InvalidKey { table: table.to_string(), key: key.to_string() })?;
        Ok(())         
    }


}

pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}

impl AnyDatabase {
    pub fn key_type(&self) -> FieldType {
        match self {
            AnyDatabase::StringDatabase(_) => FieldType::String,
            AnyDatabase::IntDatabase(_) => FieldType::Int,
        }
    }
}



// Zyguła przyjmuje na inżynierki rustowe związane z interpreterami i kompilatorami
// 
//