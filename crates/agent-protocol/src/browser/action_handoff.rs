use ocentra_eventing::{
    bus::reports::dead_letter::DeadLetter, envelope::StoredEventEnvelope,
    request::EventResponseContract, request::RequestReport,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct BrowserRuntimeActionIntentHandoffReport {
    pub request_report: RequestReport<BrowserRuntimeActionIntentHandoffResponse>,
    pub stored_events: Vec<StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserRuntimeActionIntentHandoffResponse {
    pub candidate_count: usize,
    pub policy_preview_id: Option<String>,
    pub action_intent_id: Option<String>,
    pub source_event_ref: Option<String>,
    pub outbox_ref: Option<String>,
    pub handoff_ref: Option<String>,
    pub source_ref: String,
    pub evidence_ref: String,
    pub dry_run_only: bool,
    pub policy_authority_only: bool,
    pub dispatch_attempt_count: u8,
    pub adapter_execution_count: u8,
    pub browser_mutation_count: u8,
    pub child_intervention_execution_count: u8,
    pub enforcement_execution_count: u8,
}

impl EventResponseContract for BrowserRuntimeActionIntentHandoffResponse {}
