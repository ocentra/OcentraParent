use super::{ScreenAiAggregateId, ScreenAiPipelineDecisionId, ScreenAiPipelineEvaluationId};
use ocentra_eventing::error::EventingError;

impl ScreenAiPipelineEvaluationId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: "screen_ai.evaluation_id",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ScreenAiPipelineDecisionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: "screen_ai.decision_id",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ScreenAiAggregateId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: "screen_ai.aggregate_id",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<ScreenAiPipelineEvaluationId> for String {
    fn from(value: ScreenAiPipelineEvaluationId) -> Self {
        value.0
    }
}

impl From<ScreenAiPipelineDecisionId> for String {
    fn from(value: ScreenAiPipelineDecisionId) -> Self {
        value.0
    }
}

impl From<ScreenAiAggregateId> for String {
    fn from(value: ScreenAiAggregateId) -> Self {
        value.0
    }
}
