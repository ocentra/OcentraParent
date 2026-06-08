use ocentra_eventing::{
    AggregateKey, CorrelationId, DomainEvent, EventBus, EventContract, EventCustody, EventId,
    EventMetadata, EventSource, EventSubscriber, EventType, EventingError, IdempotencyKey,
    RecordedAt, RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    TargetHandler,
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

mod broker_delivery;
mod queue;
mod refs;
mod remote_delivery_delete_export_propagation;
mod remote_delivery_delete_export_propagation_types;
mod remote_delivery_dispatch_readiness;
mod remote_delivery_dispatch_readiness_types;
mod remote_delivery_durable_envelope;
mod remote_delivery_durable_envelope_types;
#[cfg(test)]
mod remote_delivery_event_chain_journal;
mod remote_delivery_event_chain_journal_types;
mod remote_delivery_event_chain_store;
mod remote_delivery_fixture_transport;
mod remote_delivery_fixture_transport_types;
mod remote_delivery_no_enforcement_invariant;
mod remote_delivery_no_enforcement_invariant_types;
mod remote_delivery_outbox_handoff;
mod remote_delivery_outbox_handoff_types;
mod remote_delivery_provider_child_readiness;
mod remote_delivery_provider_child_readiness_types;
mod remote_delivery_receipt_ledger;
mod remote_delivery_receipt_ledger_types;
mod remote_delivery_status;
mod remote_delivery_transport_dispatch_state;
mod remote_delivery_transport_dispatch_state_types;
#[cfg(test)]
mod review;

pub use broker_delivery::{
    prove_network_runtime_broker_delivery_semantics, NetworkRuntimeBrokerDeliveryProofError,
    NetworkRuntimeBrokerDeliverySemantics, NetworkRuntimeBrokerDeliverySemanticsReport,
};
#[cfg(test)]
pub(crate) use queue::{
    queue_network_runtime_flow_expires_before_drain, queue_network_runtime_flow_until_subscriber,
    NetworkRuntimeQueueDrainReport, NetworkRuntimeQueueIdempotencyReport,
    NetworkRuntimeQueueOverflowReport, NetworkRuntimeQueueTtlReport,
};
pub(crate) use queue::{
    queue_network_runtime_flow_overflow_dead_letters,
    queue_network_runtime_flow_rejects_duplicate_idempotency,
};
use refs::NetworkRuntimeChainRefs;
pub use remote_delivery_delete_export_propagation::prove_network_runtime_remote_delivery_delete_export_propagation;
#[cfg(test)]
pub(crate) use remote_delivery_delete_export_propagation::prove_network_runtime_remote_delivery_delete_export_propagation_from_fixture_transport;
pub use remote_delivery_delete_export_propagation_types::{
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationState,
};
pub use remote_delivery_dispatch_readiness::prove_network_runtime_remote_delivery_dispatch_readiness;
pub use remote_delivery_dispatch_readiness_types::{
    NetworkRuntimeRemoteDeliveryDispatchReadinessError,
    NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    NetworkRuntimeRemoteDeliveryDispatchReadinessState,
};
pub use remote_delivery_durable_envelope::prove_network_runtime_remote_delivery_durable_envelope;
pub use remote_delivery_durable_envelope_types::{
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
};
#[cfg(test)]
pub(crate) use remote_delivery_event_chain_journal::prove_network_runtime_remote_event_chain_journal;
pub use remote_delivery_event_chain_journal_types::NetworkRuntimeRemoteEventChainJournalError;
#[cfg(test)]
pub(crate) use remote_delivery_event_chain_journal_types::NetworkRuntimeRemoteEventChainJournalReport;
pub use remote_delivery_fixture_transport::prove_network_runtime_remote_delivery_fixture_transport;
#[cfg(test)]
pub(crate) use remote_delivery_fixture_transport::prove_network_runtime_remote_delivery_fixture_transport_from_outbox;
pub use remote_delivery_fixture_transport_types::{
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
    NetworkRuntimeRemoteDeliveryFixtureTransportRecord,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportState,
};
pub use remote_delivery_no_enforcement_invariant::prove_network_runtime_remote_delivery_no_enforcement_invariant;
#[cfg(test)]
pub(crate) use remote_delivery_no_enforcement_invariant::prove_network_runtime_remote_delivery_no_enforcement_invariant_from_dispatch_readiness;
pub use remote_delivery_no_enforcement_invariant_types::{
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState,
    NetworkRuntimeRemoteDeliveryNoEnforcementStage,
};
pub use remote_delivery_outbox_handoff::prove_network_runtime_remote_delivery_outbox_handoff;
pub use remote_delivery_outbox_handoff_types::{
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport, NetworkRuntimeRemoteDeliveryOutboxState,
};
pub use remote_delivery_provider_child_readiness::prove_network_runtime_remote_delivery_provider_child_readiness;
pub use remote_delivery_provider_child_readiness_types::{
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
};
pub use remote_delivery_receipt_ledger::prove_network_runtime_remote_delivery_receipt_ledger;
pub use remote_delivery_receipt_ledger_types::{
    NetworkRuntimeRemoteDeliveryReceiptLedgerError, NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
};
pub use remote_delivery_status::{
    prove_network_runtime_remote_delivery_status, NetworkRuntimeRemoteDeliveryState,
    NetworkRuntimeRemoteDeliveryStatusError, NetworkRuntimeRemoteDeliveryStatusReport,
};
pub use remote_delivery_transport_dispatch_state::prove_network_runtime_remote_delivery_transport_dispatch_state;
pub use remote_delivery_transport_dispatch_state_types::{
    NetworkRuntimeRemoteDeliveryBlockedDispatchRecord,
    NetworkRuntimeRemoteDeliveryTransportDispatchState,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
};
#[cfg(test)]
pub use review::{
    request_network_runtime_review_for_observation, NetworkRuntimeReviewReport,
    NetworkRuntimeReviewResponse,
};

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
    pub publish_reports: Vec<ocentra_eventing::PublishReport>,
    pub stored_events: Vec<ocentra_eventing::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::DeadLetter>,
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
