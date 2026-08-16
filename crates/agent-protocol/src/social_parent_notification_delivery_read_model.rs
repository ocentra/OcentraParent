use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RequestId, SchemaVersion};
use ocentra_eventing::request::{EventResponseContract, RequestEvent};
use serde::{Deserialize, Serialize};

use crate::constants;

pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION: &str =
    "social-parent-notification-delivery-read-model";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID: &str =
    "social-parent-notification-delivery-readiness-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_REPORT_WRITER_PROOF_REF: &str =
    "social-report-writer-delivery-proof-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_REPORT_READY: &str =
    "social-parent-notification-ready-high-risk-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_MANUAL_REQUIRED: &str =
    "social-parent-notification-manual-required-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_UNAVAILABLE: &str =
    "social-parent-notification-unavailable-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_WRITER_ROW_REF: &str =
    "social-report-writer-delivery-row-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_ROW_REPORT_READY: &str =
    "social-report-delivery-weekly-summary-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_ROW_MANUAL_REQUIRED: &str =
    "social-report-delivery-manual-required-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_ROW_UNAVAILABLE: &str =
    "social-report-delivery-unavailable-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_SOURCE_INTENT_REF: &str =
    "social-alert-report-high-risk-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_INTENT_REF: &str =
    "social-alert-report-high-risk-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_PARENT_VISIBLE_REPORT_STATUS_REF: &str =
    "social-parent-visible-report-status-high-risk-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_VISIBLE_REPORT_STATUS_REF: &str =
    "social-parent-visible-report-status-high-risk-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_PARENT_VISIBLE_MANUAL_REQUIRED_REF: &str =
    "social-parent-visible-report-status-manual-required-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_VISIBLE_MANUAL_REQUIRED_REF: &str =
    "social-parent-visible-report-status-manual-required-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_PARENT_REPORT_REF: &str =
    "social-parent-report-high-risk-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_REPORT_REF: &str =
    "social-parent-report-high-risk-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_REPORT_ARTIFACT_REF: &str =
    "social-report-artifact-high-risk-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_ARTIFACT_REF: &str =
    "social-report-artifact-high-risk-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_REPORT_RECEIPT_REF: &str =
    "social-report-receipt-high-risk-service";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_RECEIPT_REF: &str =
    "social-report-receipt-high-risk-service";
pub const SOCIAL_REPORT_WRITER_DELIVERY_STATE_REPORT_READY: &str = "report-delivery-ready";
pub const SOCIAL_REPORT_WRITER_DELIVERY_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_REPORT_WRITER_DELIVERY_STATE_UNAVAILABLE: &str = "unavailable";
pub const SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_RECORDED: &str =
    "parent-owned-report-receipt-recorded";
pub const SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_NOT_RECORDED: &str = "not-recorded";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_EVIDENCE_REF: &str = "evidence-social-route-gate";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_POLICY_REF: &str = "policy-ref-social-high-risk";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_AUDIT_REF: &str = "audit-ref-social-alert-report";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_MANUAL_UI_PROOF_REQUIRED: &str =
    "manual-parent-notification-ui-runtime-proof-required";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXTERNAL_RUNTIME_UNAVAILABLE: &str =
    "external-report-delivery-runtime-unavailable";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY: &str =
    "parent-report-status-ready";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE: &str = "unavailable";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXECUTION_REPORT_READY: &str =
    "parent-owned-report-ready";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_CAPABILITY_READY: &str =
    "service-backed-social-parent-notification-delivery-readiness";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PARENT_NOTIFICATION_UI: &str =
    "no-parent-notification-ui-delivery";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_EXTERNAL_RUNTIME: &str =
    "no-external-runtime-report-delivery";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_DELIVERY: &str =
    "no-provider-delivery";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_RECEIPT: &str =
    "no-provider-receipt-ingestion";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_FINAL_POLICY: &str =
    "no-final-policy-execution";
pub const SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_ENFORCEMENT: &str = "no-enforcement";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialParentNotificationDeliveryReadinessSnapshot {
    pub schema_version: String,
    pub readiness_id: String,
    pub generated_at: String,
    pub source_report_writer_proof_ref: String,
    pub rows: Vec<SocialParentNotificationDeliveryReadinessRow>,
    pub non_claims: Vec<String>,
    pub parent_report_status_ready_count: usize,
    pub manual_required_count: usize,
    pub unavailable_count: usize,
    pub parent_notification_ui_delivery_claimed: bool,
    pub external_runtime_report_delivery_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialParentNotificationDeliveryReadModelRequest {
    pub request_id: RequestId,
    pub requested_at: String,
}

impl DomainEvent for SocialParentNotificationDeliveryReadModelRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::browser::EVENT_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_REQUESTED)?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(
            constants::browser::IDEMPOTENCY_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_PREFIX,
        );
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for SocialParentNotificationDeliveryReadModelRequest {
    type Response = SocialParentNotificationDeliveryReadModelResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialParentNotificationDeliveryReadinessRow {
    pub notification_delivery_readiness_row_id: String,
    pub source_report_writer_delivery_row_ref: String,
    pub source_intent_ref: String,
    pub parent_visible_report_status_ref: Option<String>,
    pub parent_notification_ui_ref: Option<String>,
    pub parent_report_ref: Option<String>,
    pub report_artifact_ref: Option<String>,
    pub report_receipt_ref: Option<String>,
    pub source_evidence_refs: Vec<String>,
    pub source_policy_refs: Vec<String>,
    pub source_audit_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub notification_delivery_readiness_state: String,
    pub report_delivery_execution_state: String,
    pub parent_owned_report_artifact_written: bool,
    pub parent_owned_report_receipt_recorded: bool,
    pub parent_notification_ui_delivered: bool,
    pub external_runtime_report_delivery_claimed: bool,
    pub provider_delivery_attempted: bool,
    pub provider_receipt_ingested: bool,
    pub final_policy_decision_claimed: bool,
    pub enforcement_claimed: bool,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialParentNotificationDeliveryReadModelResponse {
    pub read_model: SocialParentNotificationDeliveryReadinessSnapshot,
}

impl EventResponseContract for SocialParentNotificationDeliveryReadModelResponse {}
