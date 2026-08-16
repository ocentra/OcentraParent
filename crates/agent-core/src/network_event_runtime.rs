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
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType,
    ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent, ids::SourceService,
    ids::SubscriberId, ids::TargetHandler,
    journal::{policy::JournalPolicy, policy::JournalSelector, production_file::ProductionFileEventJournal},
    replay::ReplayFilter,
};
use ocentra_network_core::network_runtime::{
    evaluate_network_runtime, NetworkAdapterState, NetworkAiHandoffState,
    NetworkCapturePermissionState, NetworkObservationIntent, NetworkParserState,
    NetworkPolicyHandoffState, NetworkRuntimeDecision, NetworkRuntimeInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkInterventionState, NetworkRuntimeClaimBoundary,
    NetworkRuntimeEventPayload as ProtocolNetworkRuntimeEventPayload, NetworkRuntimePhase,
};
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

pub mod broker_delivery;
#[path = "network_event_runtime/helpers.rs"]
mod helpers;
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
    let chain_refs = NetworkRuntimeChainRefs::for_phase(phase, observation, observed_at, &decision);
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
    helpers::should_publish_phase(phase, observation)
        && match phase {
            NetworkRuntimePhase::AiAnalysisRequested | NetworkRuntimePhase::AiAnalysisCompleted => {
                decision.ai_handoff_state == NetworkAiHandoffState::Required
            }
            NetworkRuntimePhase::PolicyEvaluationRequested
            | NetworkRuntimePhase::PolicyDecisionCompleted => {
                decision.policy_handoff_state == NetworkPolicyHandoffState::Publish
                    || decision.ai_handoff_state == NetworkAiHandoffState::Required
            }
            _ => true,
        }
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
    evaluate_network_runtime(NetworkRuntimeInput {
        adapter_state: if observation.status
            == ocentra_parent_agent_protocol::ActivityCaptureCapabilityStatus::Available
        {
            NetworkAdapterState::Available
        } else {
            NetworkAdapterState::Missing
        },
        capture_permission_state: if observation.status
            == ocentra_parent_agent_protocol::ActivityCaptureCapabilityStatus::Available
        {
            NetworkCapturePermissionState::Granted
        } else {
            NetworkCapturePermissionState::Missing
        },
        parser_state: NetworkParserState::Valid,
        observation_intent: if evidence_grade(observation)
            == ocentra_parent_agent_protocol::network_flow::NetworkRuntimeEvidenceGrade::AdapterUnavailable
        {
            NetworkObservationIntent::TelemetryObservationOnly
        } else if observation.destination_domain.is_some() {
            NetworkObservationIntent::FlowRequiresPolicy
        } else {
            NetworkObservationIntent::UnknownRouteRequiresAi
        },
    })
}

pub async fn publish_network_runtime_chain_for_observation(
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<NetworkRuntimeReport, EventingError> {
    let spine = NetworkRuntimeSpine::with_default_handlers().await?;
    spine
        .publish_observation_chain(observation, observed_at)
        .await
}

#[derive(Clone)]
pub struct NetworkRuntimeSpine {
    bus: EventBus,
    chain_lock: Arc<AsyncMutex<()>>,
}

impl NetworkRuntimeSpine {
    pub async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        for phase in NetworkRuntimePhase::ordered_chain() {
            subscribe_default_handler(&bus, *phase).await?;
        }
        Ok(Self {
            bus,
            chain_lock: Arc::new(AsyncMutex::new(())),
        })
    }

    /// Construct the network-owned production chain and recover its durable
    /// journal before any capture caller can publish an observation.
    pub async fn with_durable_handlers(path: impl AsRef<Path>) -> Result<Self, EventingError> {
        let journal = Arc::new(ProductionFileEventJournal::new(path.as_ref().to_path_buf()));
        journal.recover().await?;
        let _ = journal.replay_projection(ReplayFilter::all()).await?;
        let bus = EventBus::with_journal(
            JournalPolicy::after_dispatch(JournalSelector::All),
            journal.clone().shared(),
        );
        for phase in NetworkRuntimePhase::ordered_chain() {
            subscribe_default_handler(&bus, *phase).await?;
        }
        Ok(Self {
            bus,
            chain_lock: Arc::new(AsyncMutex::new(())),
        })
    }

    pub async fn publish_observation_chain(
        &self,
        observation: NetworkObservation,
        observed_at: &str,
    ) -> Result<NetworkRuntimeReport, EventingError> {
        let _chain_guard = self.chain_lock.lock().await;
        let runtime_decision = network_runtime_decision_from_observation(&observation);
        let mut reports = Vec::new();
        for phase in NetworkRuntimePhase::ordered_chain()
            .iter()
            .copied()
            .filter(|phase| {
                should_publish_phase_for_runtime_decision(*phase, &observation, &runtime_decision)
            })
        {
            let payload = network_runtime_event_payload_from_observation(
                phase,
                &observation,
                observed_at,
                runtime_decision,
            );
            let metadata =
                network_event_metadata(phase, &observation, observed_at, phase.target_handler())?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        let current_event_ids: BTreeSet<_> = reports
            .iter()
            .map(|report| report.event_id.clone())
            .collect();
        let stored_events: Vec<_> = self
            .bus
            .journal()
            .await
            .into_iter()
            .filter(|event| current_event_ids.contains(&event.event_id))
            .collect();
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
            journal_state: if self.bus.has_production_durable_journal() {
                NetworkRuntimeJournalState::Durable
            } else {
                NetworkRuntimeJournalState::InMemoryManualRequired
            },
        })
    }

    pub fn journal_state(&self) -> NetworkRuntimeJournalState {
        if self.bus.has_production_durable_journal() {
            NetworkRuntimeJournalState::Durable
        } else {
            NetworkRuntimeJournalState::InMemoryManualRequired
        }
    }
}

async fn subscribe_default_handler(
    bus: &EventBus,
    phase: NetworkRuntimePhase,
) -> Result<(), EventingError> {
    bus.subscribe::<NetworkRuntimeEventPayload, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(phase.subscriber_id())?,
            EventType::parse(phase.event_type())?,
            TargetHandler::parse(phase.target_handler())?,
        ),
        |_| async { Ok(()) },
    )
    .await
    .map(|_| ())
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
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        helpers::network_event_id(phase, observation, observed_at)?,
        CorrelationId::parse(helpers::network_correlation_id(observation, observed_at))?,
        network_event_source(phase, observation)?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn network_event_source(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        helpers::event_custody(observation),
        phase.runtime_role()?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::network_flow::RUNTIME_COMPONENT_NETWORK_SPINE)?,
        RuntimeInstanceId::parse(constants::network_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
