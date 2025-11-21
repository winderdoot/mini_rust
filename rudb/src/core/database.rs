use std::collections::{BTreeMap, HashMap};
use crate::{core::errors::*};


/* === DatabaseKey === */

pub trait DatabaseKey
where 
Self: Sized + ToString + Clone + Ord + PartialEq {
    fn get_type() -> FieldType;
}

impl DatabaseKey for i64 {
    fn get_type() -> FieldType {
        FieldType::Int
    }
}

impl DatabaseKey for String {
    fn get_type() -> FieldType {
        FieldType::String
    }
}

// pub enum AnyDbKey {
//     Int(i64),
//     String(String)
// }


/* === Schema ===  */

#[derive(Clone, PartialEq)]
pub enum FieldType {
    Bool,
    String,
    Int,
    Float
}

impl ToString for FieldType {
    fn to_string(&self) -> String {
        match self {
            FieldType::Bool => "bool".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Int => "int".to_string(),
            FieldType::Float => "float".to_string(),
        }
    }
}


/// Contains the column names and types of a table, including the primary key
#[derive(Clone)]
pub struct Schema {
    fields: HashMap<String, FieldType>,
    key: String
}

impl Schema {
    pub fn from_map(map: HashMap<String, FieldType>, key: &str) -> Option<Self> {
        if !map.contains_key(key) {
            return None;
        }
        Some(Schema { fields: map, key: key.to_string() })
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_key_type(&self) -> Option<&FieldType> {
        self.fields.get(&self.key)
    }

    pub fn get_fields(&self) -> &HashMap<String, FieldType> {
        &self.fields
    }
}

pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

impl Value {
    pub fn get_type(&self) -> FieldType {
        match self {
            Value::Bool(_) => FieldType::Bool,
            Value::String(_) => FieldType::String,
            Value::Int(_) => FieldType::Int,
            Value::Float(_) => FieldType::Float,
        }
    }
}

pub struct Record {
    fields: HashMap<String, Value>
}

impl Record {
    fn matches(&self, schema: &Schema) -> Result<(), RecordError> {
        for field in schema.fields.keys() {
            if !self.fields.contains_key(field) {
                return Err(RecordError::MissingField(field.to_string()));
            }
        }

        for field in self.fields.keys() {
            if !schema.fields.contains_key(field) {
                return Err(RecordError::InvalidField(field.to_string()));
            }
        }
        Ok(())
    }

    pub fn from_map(map: HashMap<String, Value>) -> Self {
        Self { fields: map }
    }
}


/* === Table === */

pub struct Table<K: DatabaseKey> {
    name: String,
    schema: Schema,
    records: BTreeMap<K, Record>, /* Record also contains a copy of the key */
}

impl<K: DatabaseKey> Table<K> {
    pub fn from_schema(name: &str, schema: &Schema) -> Self {
        Table::<K> {
            name: name.to_string(),
            schema: schema.clone(),
            records: BTreeMap::<K, Record>::new()
        }
    }

    pub fn get_schema(&self) -> &Schema {
        &self.schema
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn insert(&mut self, key: K, record: Record) -> Result<(), DbErr> {
        match self.records.insert(key.clone(), record) {
            Some(_) => {
                return Err(InsertErr::KeyUsed { table: self.name.to_string(), key: key.to_string() })?;
            },
            None => {
                return Ok(())
            },
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

    pub fn get_table_mut(&mut self, table: &str) -> Result<&mut Table<K>, DbErr> {
        Ok(self.tables.get_mut(table).ok_or_else(|| InsertErr::TableNotFound(table.to_string()))?)
    }

    pub fn add_table(&mut self, table: &str, schema: &Schema) -> Result<(), DbErr> {
        if self.tables.contains_key(table) {
            Err(CreateErr::AlreadyExists { table: table.to_string() })?;
        }
        let schema_key = schema
            .get_key_type()
            .ok_or_else(|| DbErr::Unreachable)?;
        schema_key
            .ne(&K::get_type())
            .then_some(())
            .map_or_else(|| Ok(()), |_| Err(InsertErr::InvalidKeyType { got: schema_key.to_string(), expected: K::get_type().to_string() }))?;
            
        self.tables.insert(table.to_string(), Table::from_schema(table, schema));

        Ok(())
    }

    pub fn delete_from(&mut self, table: &str, key: K) -> Result<(), DbErr> {
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