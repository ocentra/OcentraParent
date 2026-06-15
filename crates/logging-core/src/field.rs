use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogFieldValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null(()),
}

pub type LogFields = BTreeMap<String, LogFieldValue>;

impl From<&str> for LogFieldValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for LogFieldValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for LogFieldValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for LogFieldValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
