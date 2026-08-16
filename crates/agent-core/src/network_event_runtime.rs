use crate::{
    network_event_runtime_state::{
        ai_audit_state, evidence_grade, evidence_grade_contract, evidence_scope, policy_action,
        risk_budget_state,
    },
    NetworkObservation,
};
use ocentra_eventing::bus::reports::dead_letter::DeadLetter;
use ocentra_eventing::bus::reports::handler::{HandlerOutcome, PublishReport};
use ocentra_eventing::{
    bus::DispatchMode, bus::EventBus, envelope::StoredEventEnvelope, error::EventingError,
    ids::EventId, journal::production_file::ProductionFileEventJournal,
};
use ocentra_network_core::network_runtime::NetworkRuntimeDecision;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkInterventionState, NetworkRuntimeClaimBoundary,
    NetworkRuntimeEventPayload as ProtocolNetworkRuntimeEventPayload, NetworkRuntimePhase,
};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

pub mod broker_delivery;
mod decision;
mod decision_input;
mod durable;
#[path = "network_event_runtime/helpers.rs"]
mod helpers;
#[path = "network_event_runtime/identity.rs"]
mod identity;
mod identity_payload;
#[path = "network_event_runtime/metadata.rs"]
mod metadata;
pub mod queue;
mod refs;
pub mod remote_delivery_cross_process_custody_readiness;
pub mod remote_delivery_cross_process_custody_readiness_types;
pub mod remote_delivery_cross_process_replay;
pub mod remote_delivery_cross_process_replay_types;
pub mod remote_delivery_delete_export_propagation;
pub mod remote_delivery_delete_export_propagation_types;
pub mod remote_delivery_dispatch_readiness;
pub mod remote_delivery_dispatch_readiness_types;
pub mod remote_delivery_durable_envelope;
pub mod remote_delivery_durable_envelope_types;
pub mod remote_delivery_event_chain_journal;
pub mod remote_delivery_event_chain_journal_types;
mod remote_delivery_event_chain_store;
pub mod remote_delivery_external_cross_process_transport;
pub mod remote_delivery_external_cross_process_transport_types;
pub mod remote_delivery_fixture_transport;
pub mod remote_delivery_fixture_transport_types;
pub mod remote_delivery_no_enforcement_invariant;
pub mod remote_delivery_no_enforcement_invariant_types;
pub mod remote_delivery_outbox_handoff;
pub mod remote_delivery_outbox_handoff_types;
pub mod remote_delivery_provider_child_readiness;
pub mod remote_delivery_provider_child_readiness_types;
pub mod remote_delivery_receipt_ledger;
pub mod remote_delivery_receipt_ledger_types;
pub mod remote_delivery_status;
pub mod remote_delivery_transport_dispatch_state;
pub mod remote_delivery_transport_dispatch_state_types;
pub mod review;

use refs::NetworkRuntimeChainRefs;

pub type NetworkRuntimeEventPayload = ProtocolNetworkRuntimeEventPayload;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeJournalPath(PathBuf);

impl NetworkRuntimeJournalPath {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeReport {
    pub publish_reports: Vec<PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
    pub handled_phases: Vec<NetworkRuntimePhase>,
    pub journal_state: NetworkRuntimeJournalState,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NetworkRuntimeJournalState {
    Durable,
    #[default]
    InMemoryManualRequired,
    UnavailableManualRequired,
}

impl NetworkRuntimeJournalState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Durable => "durable",
            Self::InMemoryManualRequired => "in-memory-manual-required",
            Self::UnavailableManualRequired => "unavailable-manual-required",
        }
    }
}

impl NetworkRuntimeReport {
    pub fn manual_required(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<NetworkRuntimeEventPayload>()
                .map(|envelope| {
                    envelope.payload.intervention_state == NetworkInterventionState::ManualRequired
                })
                .unwrap_or(false)
        })
    }
}

fn network_runtime_event_payload_from_observation(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    decision: NetworkRuntimeDecision,
) -> NetworkRuntimeEventPayload {
    network_runtime_event_payload_from_observation_with_source(
        phase,
        observation,
        observed_at,
        decision,
        None,
    )
}

fn network_runtime_event_payload_from_observation_with_source(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    decision: NetworkRuntimeDecision,
    source_event_id: Option<&str>,
) -> NetworkRuntimeEventPayload {
    let chain_refs = NetworkRuntimeChainRefs::for_phase(
        phase,
        observation,
        observed_at,
        &decision,
        source_event_id,
    );
    let risk_budget_state = risk_budget_state(observation);
    NetworkRuntimeEventPayload {
        phase,
        capability_status: observation.status,
        domain_attribution_status: observation.domain_attribution_status(),
        process_attribution_status: observation.process_attribution_status(),
        protocol: observation.protocol,
        tcp_state: observation.tcp_state,
        local_ip: observation.local_ip.clone(),
        local_port: observation.local_port,
        destination_ip: observation.destination_ip.clone(),
        destination_port: observation.destination_port,
        destination_domain: observation.destination_domain.clone(),
        process_id: observation.pid,
        process_name: observation.process_name.clone(),
        associated_pid_count: observation.associated_pid_count,
        evidence_scope: evidence_scope(observation),
        evidence_grade: evidence_grade(observation),
        evidence_grade_contract: evidence_grade_contract(observation),
        ai_audit_state: ai_audit_state(phase, decision.ai_handoff_state),
        risk_budget_state,
        intervention_state: helpers::intervention_state_from_budget(&risk_budget_state),
        policy_action: policy_action(observation),
        claim_boundary: NetworkRuntimeClaimBoundary::metadata_only(),
        previous_phase_ref: chain_refs.previous_phase_ref,
        evidence_ref: chain_refs.evidence_ref,
        ai_request_ref: chain_refs.ai_request_ref,
        ai_analysis_ref: chain_refs.ai_analysis_ref,
        policy_evaluation_ref: chain_refs.policy_evaluation_ref,
        policy_decision_ref: chain_refs.policy_decision_ref,
        adapter_capability_ref: chain_refs.adapter_capability_ref,
        enforcement_command_ref: chain_refs.enforcement_command_ref,
        enforcement_result_ref: chain_refs.enforcement_result_ref,
        audit_entry_ref: chain_refs.audit_entry_ref,
        observed_at: observed_at.to_string(),
    }
}

pub(super) fn should_publish_phase_for_runtime_decision(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    decision: &NetworkRuntimeDecision,
) -> bool {
    decision::should_publish_phase_for_runtime_decision(phase, observation, decision)
}

pub(super) fn network_correlation_id(
    observation: &NetworkObservation,
    observed_at: &str,
) -> String {
    helpers::network_correlation_id(observation, observed_at)
}

pub(super) fn network_aggregate_key(payload: &NetworkRuntimeEventPayload) -> String {
    helpers::network_aggregate_key(payload)
}

pub(super) fn network_runtime_decision_from_observation(
    observation: &NetworkObservation,
) -> NetworkRuntimeDecision {
    decision::network_runtime_decision_from_observation(observation)
}

pub fn network_runtime_event_id_for_source_event(
    phase: NetworkRuntimePhase,
    source_event_id: &str,
) -> Result<EventId, EventingError> {
    helpers::network_event_id(phase, source_event_id)
}

pub fn network_runtime_event_ids_for_source_event(
    source_event_id: &EventId,
    observation: &NetworkObservation,
) -> Result<Vec<EventId>, EventingError> {
    decision::network_runtime_event_ids_for_source_event(source_event_id, observation)
}

#[derive(Clone)]
pub struct NetworkRuntimeSpine {
    bus: EventBus,
    chain_lock: Arc<AsyncMutex<()>>,
    durable_journal: Arc<ProductionFileEventJournal>,
}

impl NetworkRuntimeSpine {
    pub async fn publish_observation_chain_for_source_event(
        &self,
        source_event_id: &str,
        observation: NetworkObservation,
        observed_at: &str,
    ) -> Result<NetworkRuntimeReport, EventingError> {
        self.publish_observation_chain_inner(source_event_id, observation, observed_at)
            .await
    }

    async fn publish_observation_chain_inner(
        &self,
        source_event_id: &str,
        observation: NetworkObservation,
        observed_at: &str,
    ) -> Result<NetworkRuntimeReport, EventingError> {
        let _chain_guard = self.chain_lock.lock().await;
        let runtime_decision = network_runtime_decision_from_observation(&observation);
        let planned_events = NetworkRuntimePhase::ordered_chain()
            .iter()
            .copied()
            .filter(|phase| {
                should_publish_phase_for_runtime_decision(*phase, &observation, &runtime_decision)
            })
            .map(|phase| {
                let payload = network_runtime_event_payload_from_observation_with_source(
                    phase,
                    &observation,
                    observed_at,
                    runtime_decision,
                    Some(source_event_id),
                );
                let metadata = metadata::network_event_metadata_for_source_event(
                    phase,
                    &observation,
                    observed_at,
                    phase.target_handler(),
                    source_event_id,
                )?;
                Ok((payload, metadata))
            })
            .collect::<Result<Vec<_>, EventingError>>()?;
        let mut reports = Vec::new();
        for (payload, metadata) in planned_events {
            let report = self
                .bus
                .publish_with_mode_and_before_dispatch_receipt_validator(
                    payload,
                    metadata,
                    DispatchMode::Sequential,
                    durable::require_verified_v3_synchronization_receipt,
                )
                .await?;
            reports.push(report);
        }
        let current_event_ids: BTreeSet<_> = reports
            .iter()
            .map(|report| report.event_id.clone())
            .collect();
        let stored_events = retained_current_events(self.bus.journal().await, &current_event_ids);
        if stored_events.len() != reports.len() {
            return Err(EventingError::InvalidValue {
                field: "network_runtime_journal",
                value: "current_publish_event_missing_from_retained_journal".to_string(),
            });
        }
        let dead_letters: Vec<_> = self
            .bus
            .dead_letters()
            .await
            .into_iter()
            .filter(|dead_letter| current_event_ids.contains(&dead_letter.envelope.event_id))
            .collect();
        let handled_phases = handled_phases_for_reports(&reports);
        Ok(NetworkRuntimeReport {
            publish_reports: reports,
            stored_events,
            dead_letters,
            handled_phases,
            journal_state: NetworkRuntimeJournalState::Durable,
        })
    }
}

fn handled_phases_for_reports(reports: &[PublishReport]) -> Vec<NetworkRuntimePhase> {
    reports
        .iter()
        .filter(|report| {
            report
                .handler_reports
                .iter()
                .any(|handler| handler.outcome == HandlerOutcome::Handled)
        })
        .filter_map(|report| {
            NetworkRuntimePhase::ordered_chain()
                .iter()
                .copied()
                .find(|phase| phase.event_type() == report.event_type.as_str())
        })
        .collect()
}

fn network_event_metadata(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    target_handler: &str,
) -> Result<ocentra_eventing::envelope::EventMetadata, EventingError> {
    metadata::network_event_metadata_for_fallback(phase, observation, observed_at, target_handler)
}

fn retained_current_events(
    events: Vec<StoredEventEnvelope>,
    current_event_ids: &BTreeSet<EventId>,
) -> Vec<StoredEventEnvelope> {
    let mut retained = Vec::new();
    let mut seen = BTreeSet::new();
    for event in events.into_iter().rev() {
        if current_event_ids.contains(&event.event_id) && seen.insert(event.event_id.clone()) {
            retained.push(event);
        }
    }
    retained.reverse();
    retained
}
