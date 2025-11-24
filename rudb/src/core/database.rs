use std::{collections::{BTreeMap, HashMap}, fmt::Display};
use crate::{core::errors::*};


/* === DatabaseKey === */

pub trait DatabaseKey
where 
Self: Sized + ToString + Clone + Ord + PartialEq {
    fn get_type() -> FieldType;
    fn from_value(val: &Value) -> Option<Self>;
}

impl DatabaseKey for i64 {
    fn get_type() -> FieldType {
        FieldType::Int
    }
    
    fn from_value(val: &Value) -> Option<Self> {
        match val {
            Value::Int(k) => Some(*k),
            _ => None
        }
    }
}

impl DatabaseKey for String {
    fn get_type() -> FieldType {
        FieldType::String
    }
    
    fn from_value(val: &Value) -> Option<Self> {
        match val {
            Value::String(k) => Some(k.clone()),
            _ => None
        }
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

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "\"{}\"", *v),
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
        }
    }
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

    /* This is horrible */
    pub fn eq(&self, right_op: &Value) -> Result<bool, ConditionErr> {
        match (self, right_op) {
            (Value::Bool(left), Value::Bool(right)) => {
                Ok(left.eq(right))
            },
            (Value::Int(left), Value::Int(right)) => {
                Ok(left.eq(right))  
            },
            (Value::String(left), Value::String(right)) => {
                Ok(left.eq(right))  
            },
            (Value::Float(left), Value::Float(right)) => {
                Ok(left.eq(right))  
            },
            _ => {
                Err(ConditionErr::MismatchedOperandTypes { left: self.get_type().to_string(), right: right_op.get_type().to_string() })
            }
        }
    }

    /* This is horrible */
    pub fn gt(&self, right_op: &Value) -> Result<bool, ConditionErr> {
        match (self, right_op) {
            (Value::Bool(left), Value::Bool(right)) => {
                Ok(left.gt(right))
            },
            (Value::Int(left), Value::Int(right)) => {
                Ok(left.gt(right))  
            },
            (Value::String(left), Value::String(right)) => {
                Ok(left.gt(right))  
            },
            (Value::Float(left), Value::Float(right)) => {
                Ok(left.gt(right))  
            },
            _ => {
                Err(ConditionErr::MismatchedOperandTypes { left: self.get_type().to_string(), right: right_op.get_type().to_string() })
            }
        }
    }

    /* This is horrible */
    pub fn lt(&self, right_op: &Value) -> Result<bool, ConditionErr> {
        match (self, right_op) {
            (Value::Bool(left), Value::Bool(right)) => {
                Ok(left.lt(right))
            },
            (Value::Int(left), Value::Int(right)) => {
                Ok(left.lt(right))  
            },
            (Value::String(left), Value::String(right)) => {
                Ok(left.lt(right))  
            },
            (Value::Float(left), Value::Float(right)) => {
                Ok(left.lt(right))  
            },
            _ => {
                Err(ConditionErr::MismatchedOperandTypes { left: self.get_type().to_string(), right: right_op.get_type().to_string() })
            }
        }
    }

    /* This is horrible */
    pub fn ge(&self, right_op: &Value) -> Result<bool, ConditionErr> {
        match (self, right_op) {
            (Value::Bool(left), Value::Bool(right)) => {
                Ok(left.ge(right))
            },
            (Value::Int(left), Value::Int(right)) => {
                Ok(left.ge(right))  
            },
            (Value::String(left), Value::String(right)) => {
                Ok(left.ge(right))  
            },
            (Value::Float(left), Value::Float(right)) => {
                Ok(left.ge(right))  
            },
            _ => {
                Err(ConditionErr::MismatchedOperandTypes { left: self.get_type().to_string(), right: right_op.get_type().to_string() })
            }
        }
    }

    /* This is horrible */
    pub fn le(&self, right_op: &Value) -> Result<bool, ConditionErr> {
        match (self, right_op) {
            (Value::Bool(left), Value::Bool(right)) => {
                Ok(left.le(right))
            },
            (Value::Int(left), Value::Int(right)) => {
                Ok(left.le(right))  
            },
            (Value::String(left), Value::String(right)) => {
                Ok(left.le(right))  
            },
            (Value::Float(left), Value::Float(right)) => {
                Ok(left.le(right))  
            },
            _ => {
                Err(ConditionErr::MismatchedOperandTypes { left: self.get_type().to_string(), right: right_op.get_type().to_string() })
            }
        }
    }
}

#[derive(Clone)]
pub struct Record {
    fields: HashMap<String, Value>
}

impl Record {
    fn passes_conditions(&self, conditions: &HashMap<String, Condition>) -> Result<bool, RecordErr> {
        Ok(
            conditions
                .iter()
                .map(|(field, cond)| {
                    let val = self.fields.get(field).ok_or_else(|| RecordErr::InvalidField(field.to_string()))?;
                    cond.holds_for(val).or_else(| err| Err(RecordErr::Condition(err)))
                })
                .collect::<Result<Vec<bool>, RecordErr>>()?
                .iter()
                .all(|b| *b)
        )
    }

    fn filter(&self, fields: &Vec<String>, conditions: &HashMap<String, Condition>) -> Result<Option<Vec<&Value>>, RecordErr> {
        if !self.passes_conditions(conditions)? {
            return Ok(None);
        }
        
        let vec = 
            fields
            .iter()
            .map(|field| 
                self.fields
                    .get(field)
                    .ok_or_else(|| RecordErr::InvalidField(field.clone()))
            )
            .collect::<Result<Vec<&Value>, RecordErr>>()?; // Woooow ale potężny collect. Najpotężniejszy collect
        
        Ok(Some(vec))
    }

    pub fn from_map(map: HashMap<String, Value>) -> Self {
        Self { fields: map }
    }

    pub fn get_key<K: DatabaseKey>(&self, schema: &Schema) -> Option<K> {
        let key_val = self.fields.get(schema.get_key())?;
        K::from_value(key_val)
    }
}


/* Operator */

pub enum Condition {
    Equals(Value),
    NotEquals(Value),
    LessThan(Value),
    LessEqual(Value),
    Greater(Value),
    GreaterEqual(Value),
}

impl Condition {
    pub fn holds_for(&self, left_operand: &Value) -> Result<bool, ConditionErr> {
        match self {
            Condition::Equals(right) => {
                left_operand.eq(right)
            },
            Condition::NotEquals(right) => {
                left_operand.eq(right).map(|b| !b)
            },
            Condition::LessThan(right) => {
                left_operand.le(right)
            },
            Condition::LessEqual(right) => {
                left_operand.le(right)
            },
            Condition::Greater(right) => {
                left_operand.gt(right)
            },
            Condition::GreaterEqual(right) => {
                left_operand.ge(right)
            },
        }
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

    pub fn get_field_type(&self, field: &str) -> Result<&FieldType, DbErr> {
        Ok(self.schema.fields.get(field).ok_or_else(|| InsertErr::InvalidField { table: self.name.clone(), field: field.to_string() })?)
    }

    pub fn insert(&mut self, key: &K, record: Record) -> Result<(), DbErr> {
        if self.records.contains_key(key) {
            return Err(InsertErr::KeyUsed { table: self.name.to_string(), key: key.to_string() })?;
        }
        match self.records.insert(key.clone(), record) {
            Some(_) => {
                return Err(InsertErr::KeyUsed { table: self.name.to_string(), key: key.to_string() })?;
            },
            None => {
                return Ok(())
            },
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<(), DbErr> {
        self.records
            .remove(key)
            .ok_or(DeleteErr::InvalidKey { table: self.name.to_string(), key: key.to_string() })?;
        Ok(())   
    }

    fn map_value_vec(record_vec: &Vec<&Value>, fields: &Vec<String>) -> String {
        record_vec
            .iter()
            .enumerate()
            .map(|(i, val)| format!("{}: {}", fields.get(i).unwrap_or(&"_".to_string()), *val))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn select(&mut self, fields: &Vec<String>, conditions: &HashMap<String, Condition>) -> Result<String, DbErr> {
        fields
            .iter()
            .find(|field| self.schema.fields.get(*field).is_none())
            .map_or_else(|| Ok(()), |bad_field| Err(SelectErr::InvalidField { table: self.name.clone(), field: bad_field.clone() }))?;

        Ok (
            self.records
                .iter()
                .map(|(_, record)| record.filter(fields, conditions))
                .collect::<Result<Vec<Option<Vec<&Value>>>, RecordErr>>()?
                .iter()
                .flatten()
                .map(|record_vec| Self::map_value_vec(record_vec, fields))
                .collect::<Vec<String>>()
                .join("\n")
        )
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