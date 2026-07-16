use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RequestId, SchemaVersion};
use ocentra_eventing::request::{EventResponseContract, RequestEvent};
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialReportWriterDeliveryReadModel {
    pub generated_at: String,
    pub proof_ref: String,
    pub rows: Vec<SocialReportWriterDeliveryReadModelRow>,
    pub non_claims: Vec<String>,
    pub external_runtime_report_delivery_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialReportWriterDeliveryReadModelRequest {
    pub request_id: RequestId,
    pub requested_at: String,
}

impl DomainEvent for SocialReportWriterDeliveryReadModelRequest {
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

impl RequestEvent for SocialReportWriterDeliveryReadModelRequest {
    type Response = SocialReportWriterDeliveryReadModelResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialReportWriterDeliveryReadModelRow {
    pub row_id: String,
    pub source_intent_ref: String,
    pub parent_visible_report_status_ref: Option<String>,
    pub parent_report_ref: Option<String>,
    pub report_artifact_ref: Option<String>,
    pub report_receipt_ref: Option<String>,
    pub source_evidence_refs: Vec<String>,
    pub source_policy_refs: Vec<String>,
    pub source_audit_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub delivery_state: String,
    pub receipt_state: String,
    pub parent_owned_report_artifact_written: bool,
    pub parent_owned_report_receipt_recorded: bool,
    pub external_runtime_report_delivery_claimed: bool,
    pub provider_delivery_attempted: bool,
    pub provider_receipt_ingested: bool,
    pub final_policy_decision_claimed: bool,
    pub enforcement_claimed: bool,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialReportWriterDeliveryReadModelResponse {
    pub read_model: SocialReportWriterDeliveryReadModel,
}

impl EventResponseContract for SocialReportWriterDeliveryReadModelResponse {}
