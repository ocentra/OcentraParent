use ocentra_eventing::error::EventingError;
use serde::de::{Deserializer, Error as DeError};
use serde::{Deserialize, Serialize};

pub const APP_RUNTIME_DECISION_ID_PREFIX: &str = "app.runtime-decision-";
pub const APP_AGGREGATE_ID_PREFIX: &str = "app.aggregate.";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct AppRuntimeDecisionId(String);

impl AppRuntimeDecisionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_app_text_id(
            value,
            "app.runtime_decision_id",
            APP_RUNTIME_DECISION_ID_PREFIX,
        )
        .map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AppRuntimeDecisionId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for AppRuntimeDecisionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        parse_legacy_app_text_id(value, "app.runtime_decision_id")
            .map(Self)
            .map_err(D::Error::custom)
    }
}

impl From<AppRuntimeDecisionId> for String {
    fn from(value: AppRuntimeDecisionId) -> Self {
        value.0
    }
}

impl std::fmt::Display for AppRuntimeDecisionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct AppAggregateId(String);

impl AppAggregateId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_app_text_id(value, "app.aggregate_id", APP_AGGREGATE_ID_PREFIX).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AppAggregateId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for AppAggregateId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        parse_legacy_app_text_id(value, "app.aggregate_id")
            .map(Self)
            .map_err(D::Error::custom)
    }
}

impl From<AppAggregateId> for String {
    fn from(value: AppAggregateId) -> Self {
        value.0
    }
}

impl std::fmt::Display for AppAggregateId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

fn parse_app_text_id(
    value: impl Into<String>,
    field: &'static str,
    required_prefix: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    let Some(suffix) = value.strip_prefix(required_prefix) else {
        return Err(EventingError::InvalidValue { field, value });
    };
    if !is_opaque_identifier_suffix(suffix) {
        return Err(EventingError::InvalidValue { field, value });
    }
    Ok(value)
}

fn parse_legacy_app_text_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    (!value.trim().is_empty())
        .then_some(value)
        .ok_or(EventingError::EmptyValue { field })
}

fn is_opaque_identifier_suffix(value: &str) -> bool {
    let bytes = value.as_bytes();
    let Some(first) = bytes.first() else {
        return false;
    };
    let Some(last) = bytes.last() else {
        return false;
    };
    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return false;
    }
    if !last.is_ascii_lowercase() && !last.is_ascii_digit() {
        return false;
    }
    bytes
        .iter()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
}
