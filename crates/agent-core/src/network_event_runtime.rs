use ocentra_eventing::bus::subscriber::EventSubscriber;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::bus::reports::{DeadLetter, PublishReport};
use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::envelope::{DomainEvent, EventContract, EventMetadata, EventSource};
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityNetworkProtocol, ActivityNetworkTcpState, ActivityProcessAttributionStatus,
};
use serde::{Deserialize, Serialize};

use crate::{
    network_event_runtime_phase::NetworkRuntimePhase,
    network_event_runtime_state::{
        ai_audit_state, evidence_grade, evidence_scope, intervention_state, risk_budget_state,
        NetworkAiAuditState, NetworkEvidenceGrade, NetworkEvidenceScope, NetworkInterventionState,
        NetworkRiskBudgetState, NetworkRuntimeClaimBoundary,
    },
    NetworkObservation,
};

pub(crate) mod broker_delivery;
pub(crate) mod queue;
pub(crate) mod refs;
pub(crate) mod remote_delivery_cross_process_custody_readiness;
pub(crate) mod remote_delivery_cross_process_custody_readiness_types;
pub(crate) mod remote_delivery_cross_process_replay;
pub(crate) mod remote_delivery_cross_process_replay_types;
pub(crate) mod remote_delivery_delete_export_propagation;
pub(crate) mod remote_delivery_delete_export_propagation_types;
pub(crate) mod remote_delivery_dispatch_readiness;
pub(crate) mod remote_delivery_dispatch_readiness_types;
pub(crate) mod remote_delivery_durable_envelope;
pub(crate) mod remote_delivery_durable_envelope_types;
#[cfg(test)]
pub(crate) mod remote_delivery_event_chain_journal;
pub(crate) mod remote_delivery_event_chain_journal_types;
pub(crate) mod remote_delivery_event_chain_store;
pub(crate) mod remote_delivery_external_cross_process_transport;
pub(crate) mod remote_delivery_external_cross_process_transport_types;
pub(crate) mod remote_delivery_fixture_transport;
pub(crate) mod remote_delivery_fixture_transport_types;
pub(crate) mod remote_delivery_no_enforcement_invariant;
pub(crate) mod remote_delivery_no_enforcement_invariant_types;
pub(crate) mod remote_delivery_outbox_handoff;
pub(crate) mod remote_delivery_outbox_handoff_types;
pub(crate) mod remote_delivery_provider_child_readiness;
pub(crate) mod remote_delivery_provider_child_readiness_types;
pub(crate) mod remote_delivery_receipt_ledger;
pub(crate) mod remote_delivery_receipt_ledger_types;
pub(crate) mod remote_delivery_status;
pub(crate) mod remote_delivery_transport_dispatch_state;
pub(crate) mod remote_delivery_transport_dispatch_state_types;
#[cfg(test)]
pub(crate) mod review;

use refs::NetworkRuntimeChainRefs;

pub type NetworkRuntimeRemoteDeliveryBlockedDispatchRecord =
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryBlockedDispatchRecord;
pub type NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError =
    remote_delivery_cross_process_custody_readiness_types::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError;
pub type NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord =
    remote_delivery_cross_process_custody_readiness_types::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord;
pub type NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport =
    remote_delivery_cross_process_custody_readiness_types::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport;
pub type NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState =
    remote_delivery_cross_process_custody_readiness_types::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState;
pub type NetworkRuntimeRemoteDeliveryCrossProcessReplayError =
    remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayError;
pub type NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord =
    remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord;
pub type NetworkRuntimeRemoteDeliveryCrossProcessReplayReport =
    remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayReport;
pub type NetworkRuntimeRemoteDeliveryCrossProcessReplayState =
    remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayState;
pub type NetworkRuntimeRemoteDeliveryDeleteExportPropagationError =
    remote_delivery_delete_export_propagation_types::NetworkRuntimeRemoteDeliveryDeleteExportPropagationError;
pub type NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord =
    remote_delivery_delete_export_propagation_types::NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord;
pub type NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport =
    remote_delivery_delete_export_propagation_types::NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport;
pub type NetworkRuntimeRemoteDeliveryDeleteExportPropagationState =
    remote_delivery_delete_export_propagation_types::NetworkRuntimeRemoteDeliveryDeleteExportPropagationState;
pub type NetworkRuntimeRemoteDeliveryDispatchGate =
    remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchGate;
pub type NetworkRuntimeRemoteDeliveryDispatchReadinessError =
    remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchReadinessError;
pub type NetworkRuntimeRemoteDeliveryDispatchReadinessReport =
    remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchReadinessReport;
pub type NetworkRuntimeRemoteDeliveryDispatchReadinessState =
    remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchReadinessState;
pub type NetworkRuntimeRemoteDeliveryDurableEnvelopeError =
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeError;
pub type NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord =
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord;
pub type NetworkRuntimeRemoteDeliveryDurableEnvelopeReport =
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeReport;
pub type NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError =
    remote_delivery_external_cross_process_transport_types::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError;
pub type NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord =
    remote_delivery_external_cross_process_transport_types::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord;
pub type NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport =
    remote_delivery_external_cross_process_transport_types::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport;
pub type NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState =
    remote_delivery_external_cross_process_transport_types::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState;
pub type NetworkRuntimeRemoteDeliveryFixtureTransportError =
    remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportError;
pub type NetworkRuntimeRemoteDeliveryFixtureTransportRecord =
    remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportRecord;
pub type NetworkRuntimeRemoteDeliveryFixtureTransportReport =
    remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportReport;
pub type NetworkRuntimeRemoteDeliveryFixtureTransportState =
    remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportState;
pub type NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError =
    remote_delivery_no_enforcement_invariant_types::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError;
pub type NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport =
    remote_delivery_no_enforcement_invariant_types::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport;
pub type NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState =
    remote_delivery_no_enforcement_invariant_types::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState;
pub type NetworkRuntimeRemoteDeliveryNoEnforcementStage =
    remote_delivery_no_enforcement_invariant_types::NetworkRuntimeRemoteDeliveryNoEnforcementStage;
pub type NetworkRuntimeRemoteDeliveryOutboxCandidate =
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxCandidate;
pub type NetworkRuntimeRemoteDeliveryOutboxHandoffError =
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffError;
pub type NetworkRuntimeRemoteDeliveryOutboxHandoffReport =
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffReport;
pub type NetworkRuntimeRemoteDeliveryOutboxState =
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxState;
pub type NetworkRuntimeRemoteDeliveryProviderChildReadinessError =
    remote_delivery_provider_child_readiness_types::NetworkRuntimeRemoteDeliveryProviderChildReadinessError;
pub type NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord =
    remote_delivery_provider_child_readiness_types::NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord;
pub type NetworkRuntimeRemoteDeliveryProviderChildReadinessReport =
    remote_delivery_provider_child_readiness_types::NetworkRuntimeRemoteDeliveryProviderChildReadinessReport;
pub type NetworkRuntimeRemoteDeliveryProviderChildReadinessState =
    remote_delivery_provider_child_readiness_types::NetworkRuntimeRemoteDeliveryProviderChildReadinessState;
pub type NetworkRuntimeRemoteDeliveryReceiptLedgerError =
    remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptLedgerError;
pub type NetworkRuntimeRemoteDeliveryReceiptLedgerReport =
    remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptLedgerReport;
pub type NetworkRuntimeRemoteDeliveryReceiptRecord =
    remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptRecord;
pub type NetworkRuntimeRemoteDeliveryState = remote_delivery_status::NetworkRuntimeRemoteDeliveryState;
pub type NetworkRuntimeRemoteDeliveryStatusError =
    remote_delivery_status::NetworkRuntimeRemoteDeliveryStatusError;
pub type NetworkRuntimeRemoteDeliveryStatusReport =
    remote_delivery_status::NetworkRuntimeRemoteDeliveryStatusReport;
pub type NetworkRuntimeRemoteDeliveryTransportDispatchState =
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryTransportDispatchState;
pub type NetworkRuntimeRemoteDeliveryTransportDispatchStateError =
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryTransportDispatchStateError;
pub type NetworkRuntimeRemoteDeliveryTransportDispatchStateReport =
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryTransportDispatchStateReport;
pub type NetworkRuntimeRemoteEventChainJournalError =
    remote_delivery_event_chain_journal_types::NetworkRuntimeRemoteEventChainJournalError;
#[cfg(test)]
pub type NetworkRuntimeRemoteEventChainJournalReport =
    remote_delivery_event_chain_journal_types::NetworkRuntimeRemoteEventChainJournalReport;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuntimeEventPayload {
    pub phase: NetworkRuntimePhase,
    pub capability_status: ActivityCaptureCapabilityStatus,
    pub domain_attribution_status: ActivityDomainAttributionStatus,
    pub process_attribution_status: ActivityProcessAttributionStatus,
    pub protocol: Option<ActivityNetworkProtocol>,
    pub tcp_state: Option<ActivityNetworkTcpState>,
    pub local_ip: Option<String>,
    pub local_port: Option<u16>,
    pub destination_ip: Option<String>,
    pub destination_port: Option<u16>,
    pub destination_domain: Option<String>,
    pub process_id: Option<u32>,
    pub process_name: Option<String>,
    pub evidence_scope: NetworkEvidenceScope,
    pub evidence_grade: NetworkEvidenceGrade,
    pub ai_audit_state: NetworkAiAuditState,
    pub risk_budget_state: NetworkRiskBudgetState,
    pub intervention_state: NetworkInterventionState,
    pub claim_boundary: NetworkRuntimeClaimBoundary,
    pub previous_phase_ref: Option<String>,
    pub evidence_ref: String,
    pub ai_request_ref: Option<String>,
    pub ai_analysis_ref: Option<String>,
    pub policy_evaluation_ref: Option<String>,
    pub policy_decision_ref: Option<String>,
    pub adapter_capability_ref: Option<String>,
    pub enforcement_command_ref: Option<String>,
    pub enforcement_result_ref: Option<String>,
    pub audit_entry_ref: Option<String>,
    pub observed_at: String,
}

impl NetworkRuntimeEventPayload {
    fn from_observation(
        phase: NetworkRuntimePhase,
        observation: &NetworkObservation,
        observed_at: &str,
    ) -> Self {
        let chain_refs = NetworkRuntimeChainRefs::for_phase(phase, observation, observed_at);
        Self {
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
}

impl DomainEvent for NetworkRuntimeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::network_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(network_aggregate_key(self))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::network_flow::IDEMPOTENCY_NETWORK_RUNTIME_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&network_aggregate_key(self));
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeReport {
    pub publish_reports: Vec<PublishReport>,
    pub stored_events: Vec<StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
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

pub async fn publish_network_runtime_chain_for_observation(
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<NetworkRuntimeReport, EventingError> {
    let spine = NetworkRuntimeSpine::with_default_handlers().await?;
    spine
        .publish_observation_chain(observation, observed_at)
        .await
}

pub async fn prove_network_runtime_remote_delivery_cross_process_custody_readiness(
) -> Result<
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
> {
    remote_delivery_cross_process_custody_readiness::prove_network_runtime_remote_delivery_cross_process_custody_readiness().await
}

pub async fn prove_network_runtime_remote_delivery_cross_process_replay(
) -> Result<
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
> {
    remote_delivery_cross_process_replay::prove_network_runtime_remote_delivery_cross_process_replay().await
}

pub async fn prove_network_runtime_remote_delivery_delete_export_propagation(
) -> Result<
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
> {
    remote_delivery_delete_export_propagation::prove_network_runtime_remote_delivery_delete_export_propagation().await
}

pub async fn prove_network_runtime_remote_delivery_dispatch_readiness(
) -> Result<
    NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    NetworkRuntimeRemoteDeliveryDispatchReadinessError,
> {
    remote_delivery_dispatch_readiness::prove_network_runtime_remote_delivery_dispatch_readiness().await
}

pub async fn prove_network_runtime_remote_delivery_durable_envelope(
) -> Result<
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
> {
    remote_delivery_durable_envelope::prove_network_runtime_remote_delivery_durable_envelope().await
}

pub async fn prove_network_runtime_remote_delivery_external_cross_process_transport(
) -> Result<
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError,
> {
    remote_delivery_external_cross_process_transport::prove_network_runtime_remote_delivery_external_cross_process_transport().await
}

pub async fn prove_network_runtime_remote_delivery_fixture_transport(
) -> Result<
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
> {
    remote_delivery_fixture_transport::prove_network_runtime_remote_delivery_fixture_transport().await
}

pub async fn prove_network_runtime_remote_delivery_no_enforcement_invariant(
) -> Result<
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
> {
    remote_delivery_no_enforcement_invariant::prove_network_runtime_remote_delivery_no_enforcement_invariant().await
}

pub async fn prove_network_runtime_remote_delivery_outbox_handoff(
) -> Result<
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    remote_delivery_outbox_handoff::prove_network_runtime_remote_delivery_outbox_handoff().await
}

pub async fn prove_network_runtime_remote_delivery_provider_child_readiness(
) -> Result<
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
> {
    remote_delivery_provider_child_readiness::prove_network_runtime_remote_delivery_provider_child_readiness().await
}

pub async fn prove_network_runtime_remote_delivery_transport_dispatch_state(
) -> Result<
    NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
> {
    remote_delivery_transport_dispatch_state::prove_network_runtime_remote_delivery_transport_dispatch_state().await
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
                NetworkRuntimeEventPayload::from_observation(phase, &observation, observed_at);
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
    EventCustody::parse(value)
        .expect(constants::eventing_source::ERROR_EVENT_CUSTODY_CONSTANT_PARSES)
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
