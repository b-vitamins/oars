use serde::de::DeserializeOwned;
use serde_json::{self, Error as SerdeError, Value};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub trait Leavenable {
    fn leaven<L: DeserializeOwned>(self) -> Result<L, SerdeError>;
}

#[derive(Debug)]
pub enum Deflated {
    String(String),
    JsonValue(Value),
    ByteArray(Vec<u8>),
}

impl Deflated {
    // Attempts to unwrap the enum as a String.
    // Returns Some(String) if the enum is Deflated::String variant, or None otherwise.
    pub fn to_string(self) -> Option<String> {
        match self {
            Deflated::String(s) => Some(s),
            _ => None,
        }
    }

    // Attempts to unwrap the enum as a serde_json::Value.
    // Returns Some(Value) if the enum is Deflated::JsonValue variant, or None otherwise.
    pub fn to_json(self) -> Option<Value> {
        match self {
            Deflated::JsonValue(v) => Some(v),
            _ => None,
        }
    }

    // Attempts to unwrap the enum as a Vec<u8>.
    // Returns Some(Vec<u8>) if the enum is Deflated::ByteArray variant, or None otherwise.
    pub fn to_bytes(self) -> Option<Vec<u8>> {
        match self {
            Deflated::ByteArray(b) => Some(b),
            _ => None,
        }
    }
}

pub enum Deflation {
    ToString,
    ToJsonValue,
    ToByteArray,
}

pub trait Deflatable {
    fn deflate(&self, format: Deflation) -> Result<Deflated, SerdeError>;
}

impl Leavenable for &str {
    fn leaven<L: DeserializeOwned>(self) -> Result<L, SerdeError> {
        serde_json::from_str(self)
    }
}

impl Leavenable for String {
    fn leaven<L: DeserializeOwned>(self) -> Result<L, SerdeError> {
        serde_json::from_str(&self)
    }
}

impl Leavenable for &Value {
    fn leaven<L: DeserializeOwned>(self) -> Result<L, SerdeError> {
        serde_json::from_value(self.clone())
    }
}

impl Leavenable for PathBuf {
    fn leaven<L: DeserializeOwned>(self) -> Result<L, SerdeError> {
        let mut file = File::open(self).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents)
    }
}

impl Leavenable for Vec<u8> {
    fn leaven<L: DeserializeOwned>(self) -> Result<L, SerdeError> {
        serde_json::from_slice(&self).map_err(SerdeError::into)
    }
}

impl Leavenable for &[u8] {
    fn leaven<L: DeserializeOwned>(self) -> Result<L, SerdeError> {
        serde_json::from_slice(self).map_err(SerdeError::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_from_str() {
        let json_str = r#"{"name":"John","age":30}"#;
        let result: Result<Value, SerdeError> = json_str.leaven();
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["name"], "John");
        assert_eq!(value["age"], 30);
    }

    #[test]
    fn test_deserialize_from_string() {
        let json_string = String::from(r#"{"name":"Jane","age":25}"#);
        let result: Result<Value, SerdeError> = json_string.leaven();
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["name"], "Jane");
        assert_eq!(value["age"], 25);
    }

    #[test]
    fn test_deserialize_from_value() {
        let json_value: Value = serde_json::json!({
            "name": "Doe",
            "age": 40
        });
        let result: Result<Value, SerdeError> = json_value.leaven();
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["name"], "Doe");
        assert_eq!(value["age"], 40);
    }
}
