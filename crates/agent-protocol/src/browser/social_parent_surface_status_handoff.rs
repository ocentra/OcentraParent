use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RequestId, SchemaVersion};
use ocentra_eventing::request::{EventResponseContract, RequestEvent};
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialProviderStatusHandoffReadModel {
    pub handoff_id: String,
    pub rows: Vec<SocialProviderStatusHandoffRow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialProviderStatusHandoffRequest {
    pub request_id: RequestId,
    pub requested_at: String,
}

impl DomainEvent for SocialProviderStatusHandoffRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED,
            )?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(
            constants::browser::IDEMPOTENCY_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_PREFIX,
        );
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for SocialProviderStatusHandoffRequest {
    type Response = SocialProviderStatusHandoffResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialProviderStatusHandoffRow {
    pub handoff_row_id: String,
    pub source_intent_ref: String,
    pub notification_status_ref: String,
    pub audit_ref: String,
    pub manual_proof_requirement: String,
    pub unavailable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialProviderStatusHandoffResponse {
    pub read_model: SocialProviderStatusHandoffReadModel,
}

impl EventResponseContract for SocialProviderStatusHandoffResponse {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialPreferenceStatusHandoffReadModel {
    pub handoff_id: String,
    pub rows: Vec<SocialPreferenceStatusHandoffRow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialPreferenceStatusHandoffRequest {
    pub request_id: RequestId,
    pub requested_at: String,
}

impl DomainEvent for SocialPreferenceStatusHandoffRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_REQUESTED,
            )?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(
            constants::browser::IDEMPOTENCY_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_PREFIX,
        );
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for SocialPreferenceStatusHandoffRequest {
    type Response = SocialPreferenceStatusHandoffResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialPreferenceStatusHandoffRow {
    pub handoff_row_id: String,
    pub source_preference_status_ref: String,
    pub audit_ref: String,
    pub manual_proof_requirement: String,
    pub preference_disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialPreferenceStatusHandoffResponse {
    pub read_model: SocialPreferenceStatusHandoffReadModel,
}

impl EventResponseContract for SocialPreferenceStatusHandoffResponse {}
