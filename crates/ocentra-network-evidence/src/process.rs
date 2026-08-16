use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;
use crate::process_support::{
    matched_app_inventory, matched_process_snapshot, non_empty_option,
    validate_process_correlation_input,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkProcessCorrelationState {
    ProcessAndAppAttributed,
    ProcessAttributed,
    ProcessCandidate,
    ProcessUnknown,
    AdapterUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkProcessCorrelationBasis {
    PidSnapshot,
    ProcessPathAppInventory,
    ProcessNameCandidate,
    MissingProcessEvidence,
    AdapterUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkProcessCorrelationUncertainty {
    ConfirmedByReplay,
    AppInventoryMatched,
    CandidateNeedsConfirmation,
    Unknown,
    AdapterUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkFlowProcessObservation {
    pub flow_ref: String,
    pub observed_pid: Option<u32>,
    pub observed_process_name: Option<String>,
    pub observed_executable_path: Option<String>,
    pub adapter_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkProcessSnapshot {
    pub pid: u32,
    pub process_name: String,
    pub executable_path: Option<String>,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAppInventoryEntry {
    pub app_id: String,
    pub display_name: String,
    pub executable_path: Option<String>,
    pub process_name: Option<String>,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkProcessAppCorrelationInput {
    pub flow: NetworkFlowProcessObservation,
    pub process_snapshots: Vec<NetworkProcessSnapshot>,
    pub app_inventory: Vec<NetworkAppInventoryEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkProcessAppCorrelation {
    pub state: NetworkProcessCorrelationState,
    pub basis: NetworkProcessCorrelationBasis,
    pub uncertainty: NetworkProcessCorrelationUncertainty,
    pub process_name: Option<String>,
    pub executable_path: Option<String>,
    pub app_id: Option<String>,
    pub app_display_name: Option<String>,
    pub evidence_refs: Vec<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub browser_url_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkProcessCorrelationError {
    EmptyFlowRef,
    EmptyProcessSnapshotRef,
    EmptyProcessSnapshotName,
    EmptyAppInventoryRef,
    EmptyAppInventoryId,
    EmptyAppInventoryDisplayName,
}

pub fn correlate_process_app_activity(
    input: NetworkProcessAppCorrelationInput,
) -> Result<NetworkProcessAppCorrelation, NetworkProcessCorrelationError> {
    validate_process_correlation_input(&input)?;

    if !input.flow.adapter_available {
        return Ok(adapter_unavailable_correlation(input.flow.flow_ref));
    }

    if let Some(snapshot) = matched_process_snapshot(&input) {
        if let Some(app) = matched_app_inventory(snapshot, &input.app_inventory) {
            return Ok(process_app_correlation(&input.flow, snapshot, app));
        }
        return Ok(process_only_correlation(&input.flow, snapshot));
    }

    if let Some(process_name) = non_empty_option(input.flow.observed_process_name.as_ref()) {
        return Ok(process_candidate_correlation(&input, process_name));
    }

    Ok(process_unknown_correlation(input.flow.flow_ref))
}

fn process_app_correlation(
    flow: &NetworkFlowProcessObservation,
    snapshot: &NetworkProcessSnapshot,
    app: &NetworkAppInventoryEntry,
) -> NetworkProcessAppCorrelation {
    NetworkProcessAppCorrelation {
        state: NetworkProcessCorrelationState::ProcessAndAppAttributed,
        basis: NetworkProcessCorrelationBasis::ProcessPathAppInventory,
        uncertainty: NetworkProcessCorrelationUncertainty::AppInventoryMatched,
        process_name: Some(snapshot.process_name.clone()),
        executable_path: snapshot.executable_path.clone(),
        app_id: Some(app.app_id.clone()),
        app_display_name: Some(app.display_name.clone()),
        evidence_refs: vec![
            flow.flow_ref.clone(),
            snapshot.source_ref.clone(),
            app.source_ref.clone(),
        ],
        evidence_grade: NetworkEvidenceGrade::C,
        exact_url_available: false,
        decrypted_payload_available: false,
        browser_url_claimed: false,
    }
}

fn process_only_correlation(
    flow: &NetworkFlowProcessObservation,
    snapshot: &NetworkProcessSnapshot,
) -> NetworkProcessAppCorrelation {
    NetworkProcessAppCorrelation {
        state: NetworkProcessCorrelationState::ProcessAttributed,
        basis: NetworkProcessCorrelationBasis::PidSnapshot,
        uncertainty: NetworkProcessCorrelationUncertainty::ConfirmedByReplay,
        process_name: Some(snapshot.process_name.clone()),
        executable_path: snapshot.executable_path.clone(),
        app_id: None,
        app_display_name: None,
        evidence_refs: vec![flow.flow_ref.clone(), snapshot.source_ref.clone()],
        evidence_grade: NetworkEvidenceGrade::C,
        exact_url_available: false,
        decrypted_payload_available: false,
        browser_url_claimed: false,
    }
}

fn process_candidate_correlation(
    input: &NetworkProcessAppCorrelationInput,
    process_name: &str,
) -> NetworkProcessAppCorrelation {
    let app = input
        .app_inventory
        .iter()
        .find(|entry| process_name_matches(entry.process_name.as_ref(), process_name));

    let mut evidence_refs = vec![input.flow.flow_ref.clone()];
    let (app_id, app_display_name) = match app {
        Some(entry) => {
            evidence_refs.push(entry.source_ref.clone());
            (Some(entry.app_id.clone()), Some(entry.display_name.clone()))
        }
        None => (None, None),
    };

    NetworkProcessAppCorrelation {
        state: NetworkProcessCorrelationState::ProcessCandidate,
        basis: NetworkProcessCorrelationBasis::ProcessNameCandidate,
        uncertainty: NetworkProcessCorrelationUncertainty::CandidateNeedsConfirmation,
        process_name: Some(process_name.to_owned()),
        executable_path: input.flow.observed_executable_path.clone(),
        app_id,
        app_display_name,
        evidence_refs,
        evidence_grade: NetworkEvidenceGrade::D,
        exact_url_available: false,
        decrypted_payload_available: false,
        browser_url_claimed: false,
    }
}

fn adapter_unavailable_correlation(flow_ref: String) -> NetworkProcessAppCorrelation {
    base_unattributed_correlation(
        NetworkProcessCorrelationState::AdapterUnavailable,
        NetworkProcessCorrelationBasis::AdapterUnavailable,
        NetworkProcessCorrelationUncertainty::AdapterUnavailable,
        flow_ref,
    )
}

fn process_unknown_correlation(flow_ref: String) -> NetworkProcessAppCorrelation {
    base_unattributed_correlation(
        NetworkProcessCorrelationState::ProcessUnknown,
        NetworkProcessCorrelationBasis::MissingProcessEvidence,
        NetworkProcessCorrelationUncertainty::Unknown,
        flow_ref,
    )
}

fn base_unattributed_correlation(
    state: NetworkProcessCorrelationState,
    basis: NetworkProcessCorrelationBasis,
    uncertainty: NetworkProcessCorrelationUncertainty,
    flow_ref: String,
) -> NetworkProcessAppCorrelation {
    NetworkProcessAppCorrelation {
        state,
        basis,
        uncertainty,
        process_name: None,
        executable_path: None,
        app_id: None,
        app_display_name: None,
        evidence_refs: vec![flow_ref],
        evidence_grade: NetworkEvidenceGrade::D,
        exact_url_available: false,
        decrypted_payload_available: false,
        browser_url_claimed: false,
    }
}

fn process_name_matches(candidate: Option<&String>, process_name: &str) -> bool {
    candidate
        .map(String::as_str)
        .map(str::trim)
        .is_some_and(|candidate| candidate.eq_ignore_ascii_case(process_name))
}
