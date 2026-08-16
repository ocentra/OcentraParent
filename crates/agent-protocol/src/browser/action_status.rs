use ocentra_eventing::{
    bus::reports::dead_letter::DeadLetter, envelope::StoredEventEnvelope,
    request::EventResponseContract, request::RequestReport,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct BrowserRuntimeActionIntentStatusReport {
    pub request_report: RequestReport<BrowserRuntimeActionIntentStatusResponse>,
    pub stored_events: Vec<StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserRuntimeActionIntentStatusResponse {
    pub candidate_count: usize,
    pub policy_preview_id: Option<String>,
    pub action_intent_id: Option<String>,
    pub source_ref: String,
    pub evidence_ref: String,
    pub dry_run_only: bool,
    pub policy_authority_only: bool,
    pub dispatch_attempt_count: u8,
    pub adapter_execution_count: u8,
    pub child_intervention_execution_count: u8,
    pub enforcement_execution_count: u8,
}

impl EventResponseContract for BrowserRuntimeActionIntentStatusResponse {}
