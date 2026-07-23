use crate::{
    network_event_runtime_state::{
        ai_audit_state, evidence_grade, evidence_grade_contract, evidence_scope, policy_action,
        risk_budget_state,
    },
    NetworkObservation,
};
use ocentra_eventing::bus::reports::dead_letter::DeadLetter;
use ocentra_eventing::bus::reports::handler::PublishReport;
use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType,
    ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent, ids::SourceService,
    ids::SubscriberId, ids::TargetHandler,
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
use std::sync::{Arc, Mutex};

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

struct NetworkRuntimeSpine {
    bus: EventBus,
    handled_phases: Arc<Mutex<Vec<NetworkRuntimePhase>>>,
}

impl NetworkRuntimeSpine {
    async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        let handled_phases = Arc::new(Mutex::new(Vec::new()));
        for phase in NetworkRuntimePhase::ordered_chain() {
            subscribe_default_handler(&bus, *phase, Arc::clone(&handled_phases)).await?;
        }
        Ok(Self {
            bus,
            handled_phases,
        })
    }

    async fn publish_observation_chain(
        &self,
        observation: NetworkObservation,
        observed_at: &str,
    ) -> Result<NetworkRuntimeReport, EventingError> {
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
        Ok(NetworkRuntimeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
            handled_phases: self
                .handled_phases
                .lock()
                .map_err(|_poison| EventingError::InvalidValue {
                    field: "network_runtime_handler_trace",
                    value: "poisoned".to_string(),
                })?
                .clone(),
        })
    }
}

async fn subscribe_default_handler(
    bus: &EventBus,
    phase: NetworkRuntimePhase,
    handled_phases: Arc<Mutex<Vec<NetworkRuntimePhase>>>,
) -> Result<(), EventingError> {
    bus.subscribe::<NetworkRuntimeEventPayload, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(phase.subscriber_id())?,
            EventType::parse(phase.event_type())?,
            TargetHandler::parse(phase.target_handler())?,
        ),
        move |context| {
            let handled_phases = Arc::clone(&handled_phases);
            async move { record_handled_phase(&handled_phases, context.payload().phase) }
        },
    )
    .await
    .map(|_| ())
}

fn record_handled_phase(
    handled_phases: &Mutex<Vec<NetworkRuntimePhase>>,
    phase: NetworkRuntimePhase,
) -> Result<(), EventingError> {
    handled_phases
        .lock()
        .map_err(|_poison| EventingError::InvalidValue {
            field: "network_runtime_handler_trace",
            value: "poisoned".to_string(),
        })?
        .push(phase);
    Ok(())
}

fn network_event_metadata(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
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
