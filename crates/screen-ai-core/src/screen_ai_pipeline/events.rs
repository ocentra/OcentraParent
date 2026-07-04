use super::{
    screen_ai_event_contract, screen_ai_idempotency_key, ScreenAiPipelineDecisionRecordedEvent,
    ScreenAiPipelineEvaluationRequestedEvent, SCREEN_AI_PIPELINE_DECISION_RECORDED_EVENT_TYPE,
    SCREEN_AI_PIPELINE_EVALUATION_REQUESTED_EVENT_TYPE,
};
use ocentra_eventing::{
    envelope::DomainEvent, envelope::EventContract, error::EventingError, ids::AggregateKey,
    ids::IdempotencyKey,
};

impl DomainEvent for ScreenAiPipelineEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        screen_ai_event_contract(SCREEN_AI_PIPELINE_EVALUATION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        screen_ai_idempotency_key(
            SCREEN_AI_PIPELINE_EVALUATION_REQUESTED_EVENT_TYPE,
            self.evaluation_id.as_str(),
        )
    }
}

impl DomainEvent for ScreenAiPipelineDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        screen_ai_event_contract(SCREEN_AI_PIPELINE_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        screen_ai_idempotency_key(
            SCREEN_AI_PIPELINE_DECISION_RECORDED_EVENT_TYPE,
            self.decision_id.as_str(),
        )
    }
}
