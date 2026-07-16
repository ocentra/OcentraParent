use ocentra_eventing::{
    bus::reports::dead_letter::DeadLetter, envelope::StoredEventEnvelope,
    request::EventResponseContract, request::RequestReport,
};
use serde::{Deserialize, Serialize};

use crate::{
    NetworkInterventionState, NetworkRiskBudgetState, NetworkRuntimeClaimBoundary,
    NetworkRuntimeEvidenceGrade,
};

#[derive(Clone, Debug)]
pub struct NetworkRuntimeReviewReport {
    pub request_report: RequestReport<NetworkRuntimeReviewResponse>,
    pub stored_events: Vec<StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuntimeReviewResponse {
    pub evidence_grade: NetworkRuntimeEvidenceGrade,
    pub risk_budget_state: NetworkRiskBudgetState,
    pub intervention_state: NetworkInterventionState,
    pub review_required: bool,
    pub claim_boundary: NetworkRuntimeClaimBoundary,
}

impl EventResponseContract for NetworkRuntimeReviewResponse {}
