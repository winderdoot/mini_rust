use std::collections::{BTreeMap, HashMap};
use crate::core::errors::{DbError, DbResult, RecordError};


/* === DatabaseKey === */

pub trait DatabaseKey
where 
Self: Sized,
Self: Clone {
    fn default_key() -> Self;
    fn next(&self) -> DbResult<Self>;
}

impl DatabaseKey for i64 {
    fn default_key() -> Self {
        0
    }
    
    /// Returns the next possible database key
    fn next(&self) -> DbResult<Self> {
        match self.checked_add(1) {
            Some(key) => Ok(key),
            None => Err(DbError::KeyGenerationError),
        }
    }
}

impl DatabaseKey for String {
    fn default_key() -> Self {
        String::from("0")
    }
    
    fn next(&self) -> DbResult<Self> {
        todo!("Think of a way to implement this!")
    }
}


/* === Record ===  */

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
    fn matches(&self, format: &Record) -> Result<(), RecordError> {
        for field in format.fields.keys() {
            if !self.fields.contains_key(field) {
                return Err(RecordError::MissingField(field.to_string()));
            }
        }
        for field in self.fields.keys() {
            if !format.fields.contains_key(field) {
                return Err(RecordError::InvalidField(field.to_string()));
            }
        }
        Ok(())
    }
}


/* === Table === */

pub struct Table<K: DatabaseKey> {
    curr_key: K,
    format: Record,
    records: BTreeMap<K, Record>,
}

impl<K: DatabaseKey> Table<K> {
    pub fn from_format(format: Record) -> Self {
        Table::<K> {
            curr_key: K::default_key(),
            format,
            records: BTreeMap::<K, Record>::new()
        }
    }

    pub fn next_key(&mut self) -> DbResult<K> {
        let prev = self.curr_key.clone();
        self.curr_key = self.curr_key.next()?;
        Ok(prev)
    }

    pub fn insert(&mut self, record: Record) -> Result<(), RecordError> {
        record.matches(&self.format)?;
        self.records.insert(key, value)
        todo!();
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

    pub fn add_table(&mut self, name: &str, format: Record) {
        todo!();
    }

    pub fn insert_into(&mut self, table: &str, record: Record) -> DbResult<()> {
        match self.tables.get_mut(table) {
            None => Err(DbError::TableNotFound(table.to_string())),
            Some(table) => {
                table.insert(record)?;
                Ok(())
            },
        }
    }
}

pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}



// Zyguła przyjmuje na inżynierki rustowe związane z interpreterami i kompilatorami
// 
//