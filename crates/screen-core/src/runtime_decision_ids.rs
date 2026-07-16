use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

fn parse_screen_text_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    Ok(value)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ScreenRuntimeDecisionId(String);

impl ScreenRuntimeDecisionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        Ok(Self(parse_screen_text_id(
            value,
            "screen.runtime_decision_id",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ScreenRuntimeDecisionId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ScreenRuntimeDecisionId> for String {
    fn from(value: ScreenRuntimeDecisionId) -> Self {
        value.0
    }
}

impl std::fmt::Display for ScreenRuntimeDecisionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ScreenAggregateId(String);

impl ScreenAggregateId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        Ok(Self(parse_screen_text_id(value, "screen.aggregate_id")?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ScreenAggregateId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ScreenAggregateId> for String {
    fn from(value: ScreenAggregateId) -> Self {
        value.0
    }
}

impl std::fmt::Display for ScreenAggregateId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
