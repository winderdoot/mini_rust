use std::{collections::{BTreeMap, HashMap}, fmt::{Debug, Display}};
use crate::{core::errors::*};


/* === DatabaseKey === */

pub trait DatabaseKey: Debug
where 
Self: Sized + ToString + Clone + Ord + PartialEq + Display {
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



/* === Schema ===  */

#[derive(Clone, PartialEq, Debug)]
pub enum FieldType {
    Bool,
    String,
    Int,
    Float
}

/* I need this for easier FieldType parsing. Calling to_string() is more convenient
 * than using something like format!("{}", field_type)  */
#[allow(clippy::to_string_trait_impl)]
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
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
                    cond.holds_for(val).map_err(RecordErr::Condition)
                })
                .collect::<Result<Vec<bool>, RecordErr>>()?
                .iter()
                .all(|b| *b)
        )
    }

    fn filter(&self, fields: &[String], conditions: &HashMap<String, Condition>) -> Result<Option<Vec<&Value>>, RecordErr> {
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

#[derive(Debug)]
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

#[derive(Debug)]
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
                Err(InsertErr::KeyUsed { table: self.name.to_string(), key: key.to_string() })?
            },
            None => {
                Ok(())
            },
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<(), DbErr> {
        self.records
            .remove(key)
            .ok_or(DeleteErr::InvalidKey { table: self.name.to_string(), key: key.to_string() })?;
        Ok(())   
    }

    fn map_value_vec(record_vec: &Vec<&Value>, fields: &[String]) -> String {
        record_vec
            .iter()
            .enumerate()
            .map(|(i, val)| format!("{}: {}", fields.get(i).unwrap_or(&"_".to_string()), *val))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn select(&mut self, fields: &[String], conditions: &HashMap<String, Condition>) -> Result<String, DbErr> {
        fields
            .iter()
            .find(|field| !self.schema.fields.contains_key(*field))
            .map_or_else(|| Ok(()), |bad_field| Err(SelectErr::InvalidField { table: self.name.clone(), field: bad_field.clone() }))?;

        Ok (
            self.records
                .values()
                .map(|record| record.filter(fields, conditions))
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

#[derive(Debug)]
pub struct Database<K: DatabaseKey> {
    tables: HashMap<String, Table<K>>,
}

impl<K: DatabaseKey> Default for Database<K> {
    fn default() -> Self {
        Self::new()
    }
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
            .ok_or(DbErr::Unreachable)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_value_enum_comparison() {
        let int_a = Value::Int(10);
        let int_b = Value::Int(5);
        let str_val = Value::String("I hate c++".to_string());

        assert_eq!(int_a.gt(&int_b).unwrap(), true);
        
        assert_eq!(int_b.lt(&int_a).unwrap(), true);

        assert_eq!(int_a.eq(&int_a).unwrap(), true);
        assert_eq!(int_a.eq(&int_b).unwrap(), false);

        let result = int_a.eq(&str_val);
        assert_matches!(result, Err(ConditionErr::MismatchedOperandTypes { .. }));
    }

    #[test]
    fn test_schema_validation() {
        let mut fields = HashMap::new();
        fields.insert("cow_name".to_string(), FieldType::String);
        fields.insert("was_milked".to_string(), FieldType::Bool);

        let schema_some = Schema::from_map(fields.clone(), "cow_name");
        assert_matches!(schema_some, Some(_));
        let s = schema_some.unwrap();
        assert_eq!(s.get_key(), "cow_name");

        let schema_none = Schema::from_map(fields, "definitely_not_cow_name");
        assert_matches!(schema_none, None);
    }

    fn create_pope_schema() -> Schema {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), FieldType::Int);
        fields.insert("name".to_string(), FieldType::String);
        fields.insert("likes_kremówki".to_string(), FieldType::Bool);
        Schema::from_map(fields, "id").unwrap()
    }

    #[test]
    fn test_table_insert_unique() {
        let schema = create_pope_schema();
        let mut table = Table::<i64>::from_schema("Pope_table", &schema);

        let mut r1_map = HashMap::new();
        r1_map.insert("id".to_string(), Value::Int(1));
        r1_map.insert("name".to_string(), Value::String("John Paul II".to_string()));
        r1_map.insert("likes_kremówki".to_string(), Value::Bool(true));
        let record1 = Record::from_map(r1_map);
        let res1 = table.insert(&1, record1);

        assert_matches!(res1, Ok(_));

        let mut r2_map = HashMap::new();
        r2_map.insert("id".to_string(), Value::Int(1));
        r2_map.insert("name".to_string(), Value::String("Leon XIV".to_string()));
        r2_map.insert("likes_kremówki".to_string(), Value::Bool(false));
        let record2 = Record::from_map(r2_map);
        let res2 = table.insert(&1, record2);

        assert_matches!(res2, Err(DbErr::Insert(InsertErr::KeyUsed { .. })));
    }

    #[test]
    fn test_select_where() {
        let schema = create_pope_schema();
        let mut table = Table::<i64>::from_schema("Pope_table", &schema);

        let mut r1 = HashMap::new();
        r1.insert("id".to_string(), Value::Int(1));
        r1.insert("name".to_string(), Value::String("John Paul II".to_string()));
        r1.insert("likes_kremówki".to_string(), Value::Bool(true));
        table.insert(&1, Record::from_map(r1)).unwrap();

        let mut r2 = HashMap::new();
        r2.insert("id".to_string(), Value::Int(2));
        r2.insert("name".to_string(), Value::String("Francis".to_string()));
        r2.insert("likes_kremówki".to_string(), Value::Bool(false));
        table.insert(&2, Record::from_map(r2)).unwrap();

        let mut conditions = HashMap::new();
        conditions.insert("likes_kremówki".to_string(), Condition::Equals(Value::Bool(true)));
        
        let result = table.select(&["name".to_string(), "likes_kremówki".to_string()], &conditions).unwrap();

        assert_eq!(result, "name: \"John Paul II\", likes_kremówki: true");
    }

    #[test]
    fn test_database_key_type_mismatch() {
        let mut db = Database::<String>::new();

        let mut fields = HashMap::new();
        fields.insert("id".to_string(), FieldType::Int);
        fields.insert("some_column_idk_what_to_name_it_really".to_string(), FieldType::String);
        let bad_schema = Schema::from_map(fields, "id").unwrap();

        let result = db.add_table("funny_table", &bad_schema);

        assert_matches!(result, Err(DbErr::Insert(InsertErr::InvalidKeyType { .. })));
    }
}


// Zyguła przyjmuje na inżynierki rustowe związane z interpreterami i kompilatorami
// 
//