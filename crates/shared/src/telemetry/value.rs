use nalgebra::Isometry3;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

/// Conversion implementations for common types
impl From<f64> for Value {
    fn from(v: f64) -> Self { Value::Float(v) }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self { Value::Int(v) }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self { Value::Bool(v) }
}

impl From<String> for Value {
    fn from(v: String) -> Self { Value::String(v) }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self { Value::String(v.to_string()) }
}