use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RequestId, SchemaVersion};
use ocentra_eventing::request::{EventResponseContract, RequestEvent};
use serde::{Deserialize, Serialize};

use crate::constants;

pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_SCHEMA_VERSION: &str =
    "social-alert-report-parent-surface-read-model";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_INTENT_ID: &str =
    "social-alert-report-parent-surface-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_HANDOFF_ID: &str =
    "social-provider-status-handoff-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_HANDOFF_ID: &str =
    "social-preference-status-handoff-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_HIGH_RISK: &str =
    "social-parent-surface-provider-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_MANUAL: &str =
    "social-parent-surface-manual-action-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_UNAVAILABLE: &str =
    "social-parent-surface-unavailable-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_HIGH_RISK_REF: &str =
    "social-provider-status-handoff-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_MANUAL_REF: &str =
    "social-provider-status-handoff-manual-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_UNAVAILABLE_REF: &str =
    "social-provider-status-handoff-unavailable-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_HIGH_RISK_REF: &str =
    "social-preference-status-handoff-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_MANUAL_REF: &str =
    "social-preference-status-handoff-manual-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_UNAVAILABLE_REF: &str =
    "social-preference-status-handoff-unavailable-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_HIGH_RISK_REF: &str =
    "social-alert-report-intent-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_MANUAL_REF: &str =
    "social-alert-report-intent-manual-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_UNAVAILABLE_REF: &str =
    "social-alert-report-intent-unavailable-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_HIGH_RISK_REF: &str =
    "social-notification-status-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_MANUAL_REF: &str =
    "social-notification-status-manual-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_UNAVAILABLE_REF: &str =
    "social-notification-status-unavailable-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_HIGH_RISK_REF: &str =
    "social-preference-status-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_MANUAL_REF: &str =
    "social-preference-status-manual-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_UNAVAILABLE_REF: &str =
    "social-preference-status-unavailable-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_HIGH_RISK_REF: &str =
    "audit-ref-social-parent-surface-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_MANUAL_REF: &str =
    "audit-ref-social-parent-surface-manual-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_UNAVAILABLE_REF: &str =
    "audit-ref-social-parent-surface-unavailable-service";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_HIGH_RISK_PROOF_REF: &str =
    "manual-parent-surface-high-risk-runtime-proof-required";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_ACTION_PROOF_REF: &str =
    "manual-parent-surface-runtime-proof-required";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_UNAVAILABLE_PROOF_REF: &str =
    "manual-parent-surface-unavailable-runtime-proof-required";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_MINIMAL_BOUNDARY: &str =
    "parent-surface-status-ref-only";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL: &str = "manual-action-required";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_UNAVAILABLE: &str = "unavailable-visible";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_HISTORY_VISIBLE: &str = "history-row-visible";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_HISTORY_UNAVAILABLE: &str = "unavailable-row-visible";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_SETUP: &str = "preference-setup-required";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_DISABLED: &str =
    "preference-disabled-visible";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_NOTIFICATION_UI: &str =
    "no-parent-notification-ui-rendered";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PREFERENCE_UI: &str =
    "no-parent-notification-preference-ui-rendered";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_FREQUENCY_UI: &str =
    "no-parent-frequency-control-ui-rendered";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_HISTORY_UI: &str =
    "no-parent-notification-history-ui-rendered";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_DELIVERY: &str =
    "no-provider-delivery-execution";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_RECEIPT: &str =
    "no-provider-receipt-ingestion";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_CREDENTIALS: &str =
    "no-provider-credentials";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CLOUD_ROUTING: &str = "no-cloud-routing";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CHILD_DELIVERY: &str = "no-child-delivery";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_QUIET_HOURS: &str =
    "no-quiet-hours-timer-runtime";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_RETRY_WORKER: &str =
    "no-retry-worker-runtime";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_DURABLE_OUTBOX: &str =
    "no-production-durable-outbox-storage";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_ADAPTER_DISPATCH: &str =
    "no-adapter-dispatch";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_REPORT_DELIVERY: &str =
    "no-report-delivery-execution";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_FINAL_POLICY: &str =
    "no-final-policy-execution";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CONNECTOR_NATIVE: &str =
    "no-connector-native-runtime";
pub const SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_ENFORCEMENT: &str = "no-enforcement";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportParentSurfaceReadModelSnapshot {
    pub schema_version: String,
    pub intent_id: String,
    pub generated_at: String,
    pub source_provider_status_handoff_id: String,
    pub source_preference_status_handoff_id: String,
    pub rows: Vec<SocialAlertReportParentSurfaceReadModelRow>,
    pub manual_action_required_count: usize,
    pub unavailable_visible_count: usize,
    pub history_visible_count: usize,
    pub preference_setup_required_count: usize,
    pub parent_surface_non_claims: Vec<String>,
    pub parent_notification_ui_rendered: bool,
    pub parent_notification_preference_ui_rendered: bool,
    pub parent_frequency_control_ui_rendered: bool,
    pub parent_notification_history_ui_rendered: bool,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub provider_credentials_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub child_delivery_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub retry_execution_runtime_claimed: bool,
    pub production_durable_outbox_storage_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub report_delivery_execution_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub connector_native_runtime_claimed: bool,
    pub enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialAlertReportParentSurfaceReadModelRequest {
    pub request_id: RequestId,
    pub requested_at: String,
}

impl DomainEvent for SocialAlertReportParentSurfaceReadModelRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::browser::EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED)?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(
            constants::browser::IDEMPOTENCY_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_PREFIX,
        );
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for SocialAlertReportParentSurfaceReadModelRequest {
    type Response = SocialAlertReportParentSurfaceReadModelResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportParentSurfaceReadModelRow {
    pub surface_row_id: String,
    pub source_provider_handoff_row_id: String,
    pub source_preference_handoff_row_id: String,
    pub source_intent_ref: String,
    pub parent_surface_status: String,
    pub history_visibility: String,
    pub preference_visibility: String,
    pub notification_status_ref: String,
    pub source_preference_status_ref: String,
    pub drill_in_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub minimal_surface_payload_boundary: String,
    pub sensitive_detail_included: bool,
    pub parent_notification_ui_rendered: bool,
    pub parent_notification_preference_ui_rendered: bool,
    pub parent_frequency_control_ui_rendered: bool,
    pub parent_notification_history_ui_rendered: bool,
    pub provider_delivery_claimed: bool,
    pub provider_receipt_claimed: bool,
    pub parent_preference_mutation_claimed: bool,
    pub child_delivery_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub report_delivery_execution_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialAlertReportParentSurfaceReadModelResponse {
    pub read_model: SocialAlertReportParentSurfaceReadModelSnapshot,
}

impl EventResponseContract for SocialAlertReportParentSurfaceReadModelResponse {}
