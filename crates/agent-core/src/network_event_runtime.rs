use crate::{
    network_event_runtime_state::{
        ai_audit_state, evidence_grade, evidence_scope, intervention_state, risk_budget_state,
        NetworkInterventionState, NetworkRuntimeClaimBoundary,
    },
    NetworkObservation,
};
use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventCustody,
    ids::EventId, ids::EventType, ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, NetworkRuntimePhase,
};

pub(crate) mod broker_delivery;
pub(crate) mod queue;
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
#[cfg(test)]
pub(crate) mod remote_delivery_event_chain_journal;
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
#[cfg(test)]
pub(crate) mod review;

use refs::NetworkRuntimeChainRefs;

pub(crate) type NetworkRuntimeEventPayload =
    ocentra_parent_agent_protocol::NetworkRuntimeEventPayload;

#[derive(Clone, Debug)]
pub struct NetworkRuntimeReport {
    pub publish_reports: Vec<ocentra_eventing::bus::reports::PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
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
) -> NetworkRuntimeEventPayload {
    let chain_refs = NetworkRuntimeChainRefs::for_phase(phase, observation, observed_at);
    NetworkRuntimeEventPayload {
        phase,
        capability_status: observation.status.clone(),
        domain_attribution_status: observation.domain_attribution_status(),
        process_attribution_status: observation.process_attribution_status(),
        protocol: observation.protocol.clone(),
        tcp_state: observation.tcp_state.clone(),
        local_ip: observation.local_ip.clone(),
        local_port: observation.local_port,
        destination_ip: observation.destination_ip.clone(),
        destination_port: observation.destination_port,
        destination_domain: observation.destination_domain.clone(),
        process_id: observation.pid,
        process_name: observation.process_name.clone(),
        evidence_scope: evidence_scope(observation),
        evidence_grade: evidence_grade(observation),
        ai_audit_state: ai_audit_state(phase),
        risk_budget_state: risk_budget_state(observation),
        intervention_state: intervention_state(observation),
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
}

impl NetworkRuntimeSpine {
    async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        for phase in NetworkRuntimePhase::ordered_chain() {
            bus.subscribe::<NetworkRuntimeEventPayload, _, _>(
                EventSubscriber::new(
                    SubscriberId::parse(phase.subscriber_id())?,
                    EventType::parse(phase.event_type())?,
                    TargetHandler::parse(phase.target_handler())?,
                ),
                |_| async { Ok(()) },
            )
            .await?;
        }
        Ok(Self { bus })
    }

    async fn publish_observation_chain(
        &self,
        observation: NetworkObservation,
        observed_at: &str,
    ) -> Result<NetworkRuntimeReport, EventingError> {
        let mut reports = Vec::new();
        for phase in NetworkRuntimePhase::ordered_chain()
            .iter()
            .copied()
            .filter(|phase| should_publish_phase(*phase, &observation))
        {
            let payload =
                network_runtime_event_payload_from_observation(phase, &observation, observed_at);
            let metadata =
                network_event_metadata(phase, &observation, observed_at, phase.target_handler())?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        Ok(NetworkRuntimeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }
}

fn network_event_metadata(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(network_correlation_id(observation, observed_at))?,
        network_event_source(phase, observation)?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn should_publish_phase(phase: NetworkRuntimePhase, observation: &NetworkObservation) -> bool {
    match intervention_state(observation) {
        NetworkInterventionState::DryRunOnly => true,
        NetworkInterventionState::ManualRequired | NetworkInterventionState::Unavailable => {
            !matches!(
                phase,
                NetworkRuntimePhase::EnforcementCommandIssued
                    | NetworkRuntimePhase::EnforcementResultObserved
            )
        }
    }
}

fn network_event_source(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        event_custody(observation),
        phase.runtime_role(),
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::network_flow::RUNTIME_COMPONENT_NETWORK_SPINE)?,
        RuntimeInstanceId::parse(constants::network_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}

fn event_custody(observation: &NetworkObservation) -> EventCustody {
    let value = if observation.status == ActivityCaptureCapabilityStatus::Available {
        constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE
    } else {
        constants::eventing_source::CUSTODY_UNAVAILABLE
    };
    match EventCustody::parse(value) {
        Ok(custody) => custody,
        Err(_) => std::process::abort(),
    }
}

fn network_aggregate_key(payload: &NetworkRuntimeEventPayload) -> String {
    let mut value = String::from(constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX);
    if let Some(domain) = &payload.destination_domain {
        value.push_str(domain);
        return value;
    }
    if let Some(ip) = &payload.destination_ip {
        value.push_str(ip);
        if let Some(port) = payload.destination_port {
            value.push(constants::delimiter::HYPHEN);
            value.push_str(&port.to_string());
        }
        return value;
    }
    value.push_str(payload.capability_status.as_protocol_str());
    value
}

fn network_correlation_id(observation: &NetworkObservation, observed_at: &str) -> String {
    let mut value = String::from(constants::network_flow::CORRELATION_NETWORK_RUNTIME_PREFIX);
    value.push_str(observation.status.as_protocol_str());
    value.push(constants::delimiter::HYPHEN);
    value.push_str(observed_at);
    value
}
