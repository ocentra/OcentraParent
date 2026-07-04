use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdentity {
    pub device_id: String,
    pub hostname: String,
    pub platform: String,
    pub service_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogSource {
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "dev-server")]
    DevServer,
    #[serde(rename = "local-api")]
    LocalApi,
    #[serde(rename = "portal")]
    Portal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogFieldValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null(()),
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogFields(BTreeMap<String, LogFieldValue>);

impl LogFields {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &LogFieldValue)> {
        self.0.iter()
    }

    pub fn get(&self, key: &str) -> Option<&LogFieldValue> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, value: LogFieldValue) -> Option<LogFieldValue> {
        self.0.insert(key, value)
    }

    pub fn into_inner(self) -> BTreeMap<String, LogFieldValue> {
        self.0
    }
}

impl From<BTreeMap<String, LogFieldValue>> for LogFields {
    fn from(value: BTreeMap<String, LogFieldValue>) -> Self {
        Self(value)
    }
}

impl std::iter::FromIterator<(String, LogFieldValue)> for LogFields {
    fn from_iter<T: IntoIterator<Item = (String, LogFieldValue)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentLogEntry {
    pub schema_version: u16,
    pub id: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: String,
    pub fields: LogFields,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentLogSnapshot {
    pub schema_version: u16,
    pub agent: AgentIdentity,
    pub entries: Vec<AgentLogEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevLogEntry {
    pub schema_version: u16,
    pub id: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: String,
    pub fields: LogFields,
}
