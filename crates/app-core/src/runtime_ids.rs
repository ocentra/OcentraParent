use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

pub const APP_RUNTIME_DECISION_ID_PREFIX: &str = "app.runtime-decision-";
pub const APP_AGGREGATE_ID_PREFIX: &str = "app.aggregate.";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
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
    if suffix.trim().is_empty() {
        return Err(EventingError::InvalidValue { field, value });
    }
    Ok(value)
}
