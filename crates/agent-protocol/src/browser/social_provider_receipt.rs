use ocentra_eventing::{
    bus::reports::dead_letter::DeadLetter, envelope::StoredEventEnvelope,
    request::EventResponseContract, request::RequestReport,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct BrowserRuntimeSocialProviderReceiptStatusReport {
    pub request_report: RequestReport<BrowserRuntimeSocialProviderReceiptStatusResponse>,
    pub stored_events: Vec<StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserRuntimeSocialProviderReceiptStatusResponse {
    pub receipt_boundary_row_count: usize,
    pub provider_dispatch_required_count: usize,
    pub manual_receipt_required_count: usize,
    pub provider_attempt_ref: Option<String>,
    pub provider_receipt_proof_ref: Option<String>,
    pub source_ref: String,
    pub evidence_ref: String,
    pub action_intent_id: Option<String>,
    pub receipt_boundary_state: String,
    pub receipt_runtime_state: String,
    pub provider_receipt_count: u8,
    pub provider_dispatch_count: u8,
    pub provider_webhook_count: u8,
    pub provider_credentials_count: u8,
    pub parent_notification_ui_delivery_count: u8,
    pub report_delivery_execution_count: u8,
    pub final_policy_execution_count: u8,
    pub connector_native_runtime_count: u8,
    pub enforcement_execution_count: u8,
}

impl EventResponseContract for BrowserRuntimeSocialProviderReceiptStatusResponse {}
