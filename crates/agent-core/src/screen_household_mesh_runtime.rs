use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::DomainEvent,
    envelope::EventContract, envelope::EventMetadata, envelope::EventSource, error::EventingError,
    ids::AggregateKey, ids::CorrelationId, ids::EventCustody, ids::EventId, ids::EventType,
    ids::IdempotencyKey, ids::RecordedAt, ids::RuntimeInstanceId, ids::SchemaVersion,
    ids::SourceComponent, ids::SourceService, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::{
    screen_household_mesh_runtime_phase::ScreenHouseholdMeshPhase,
    screen_household_mesh_runtime_refs::{
        child_validation_state, claim_state, lease_state, mesh_aggregate_key, policy_state,
        previous_mesh_phase_ref, provider_result_state,
    },
    screen_household_mesh_runtime_state::{
        custody_label, ScreenMeshChildValidationState, ScreenMeshClaimState,
        ScreenMeshCustodyBoundary, ScreenMeshLeaseState, ScreenMeshPayloadMode,
        ScreenMeshPolicyState, ScreenMeshProviderResultState, ScreenMeshResultRejectionReason,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenHouseholdMeshInput {
    pub queue_job_id: String,
    pub screen_evidence_ref: String,
    pub payload_ref: String,
    pub provider_peer_id: String,
    pub claim_id: String,
    pub lease_id: String,
    pub provider_result_ref: String,
    pub policy_decision_ref: String,
    pub observed_at: String,
}

impl ScreenHouseholdMeshInput {
    pub fn proof_fixture() -> Self {
        Self {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_evidence_ref: constants::screen_flow::SCREEN_SUMMARY_EVENT_REF.to_string(),
            payload_ref: constants::screen_flow::TEST_SCREEN_MESH_PAYLOAD_REF.to_string(),
            provider_peer_id: constants::screen_flow::TEST_SCREEN_MESH_PROVIDER_PEER_ID.to_string(),
            claim_id: constants::screen_flow::TEST_SCREEN_MESH_CLAIM_ID.to_string(),
            lease_id: constants::screen_flow::TEST_SCREEN_MESH_LEASE_ID.to_string(),
            provider_result_ref: constants::screen_flow::TEST_SCREEN_MESH_RESULT_REF.to_string(),
            policy_decision_ref: constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenHouseholdMeshResultSubmission {
    pub provider_peer_id: String,
    pub claim_id: String,
    pub lease_id: String,
    pub screen_evidence_ref: String,
    pub custody_label: String,
    pub duplicate_result: bool,
    pub completed_after_lease_expiry: bool,
    pub raw_screenshot_transferred: bool,
    pub raw_screenshot_retained_by_provider: bool,
    pub provider_policy_event_attempted: bool,
    pub provider_enforcement_event_attempted: bool,
}

impl ScreenHouseholdMeshResultSubmission {
    pub fn accepted_for(input: &ScreenHouseholdMeshInput) -> Self {
        Self {
            provider_peer_id: input.provider_peer_id.clone(),
            claim_id: input.claim_id.clone(),
            lease_id: input.lease_id.clone(),
            screen_evidence_ref: input.screen_evidence_ref.clone(),
            custody_label: custody_label().to_string(),
            duplicate_result: false,
            completed_after_lease_expiry: false,
            raw_screenshot_transferred: false,
            raw_screenshot_retained_by_provider: false,
            provider_policy_event_attempted: false,
            provider_enforcement_event_attempted: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenHouseholdMeshResultValidation {
    pub accepted: bool,
    pub rejection_reason: Option<ScreenMeshResultRejectionReason>,
    pub policy_may_run: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenHouseholdMeshEventPayload {
    pub phase: ScreenHouseholdMeshPhase,
    pub queue_job_id: String,
    pub screen_evidence_ref: String,
    pub payload_ref: String,
    pub payload_mode: ScreenMeshPayloadMode,
    pub provider_peer_id: String,
    pub claim_id: String,
    pub lease_id: String,
    pub provider_result_ref: Option<String>,
    pub policy_decision_ref: Option<String>,
    pub previous_phase_ref: Option<String>,
    pub custody_label: String,
    pub claim_state: ScreenMeshClaimState,
    pub lease_state: ScreenMeshLeaseState,
    pub provider_result_state: ScreenMeshProviderResultState,
    pub child_validation_state: ScreenMeshChildValidationState,
    pub policy_state: ScreenMeshPolicyState,
    pub custody_boundary: ScreenMeshCustodyBoundary,
    pub observed_at: String,
}

impl ScreenHouseholdMeshEventPayload {
    fn from_input(phase: ScreenHouseholdMeshPhase, input: &ScreenHouseholdMeshInput) -> Self {
        Self {
            phase,
            queue_job_id: input.queue_job_id.clone(),
            screen_evidence_ref: input.screen_evidence_ref.clone(),
            payload_ref: input.payload_ref.clone(),
            payload_mode: ScreenMeshPayloadMode::RedactedSummary,
            provider_peer_id: input.provider_peer_id.clone(),
            claim_id: input.claim_id.clone(),
            lease_id: input.lease_id.clone(),
            provider_result_ref: provider_result_ref(phase, input),
            policy_decision_ref: policy_decision_ref(phase, input),
            previous_phase_ref: previous_mesh_phase_ref(phase),
            custody_label: custody_label().to_string(),
            claim_state: claim_state(phase),
            lease_state: lease_state(phase),
            provider_result_state: provider_result_state(phase),
            child_validation_state: child_validation_state(phase),
            policy_state: policy_state(phase),
            custody_boundary: ScreenMeshCustodyBoundary::child_owned_worker_only(),
            observed_at: input.observed_at.clone(),
        }
    }
}

impl DomainEvent for ScreenHouseholdMeshEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::screen_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(mesh_aggregate_key(&self.queue_job_id))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::screen_flow::IDEMPOTENCY_SCREEN_MESH_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&mesh_aggregate_key(&self.queue_job_id));
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

#[derive(Clone, Debug)]
pub struct ScreenHouseholdMeshReport {
    pub publish_reports: Vec<ocentra_eventing::bus::reports::PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
}

impl ScreenHouseholdMeshReport {
    pub fn raw_screenshot_escaped(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<ScreenHouseholdMeshEventPayload>()
                .map(|envelope| {
                    envelope.payload.custody_boundary.raw_screenshot_transferred
                        || envelope
                            .payload
                            .custody_boundary
                            .raw_screenshot_retained_by_provider
                })
                .unwrap_or(true)
        })
    }
}

pub async fn publish_screen_household_mesh_chain_for_input(
    input: ScreenHouseholdMeshInput,
) -> Result<ScreenHouseholdMeshReport, EventingError> {
    let spine = ScreenHouseholdMeshSpine::with_default_handlers().await?;
    spine.publish_input_chain(input).await
}

pub fn validate_screen_household_mesh_result(
    input: &ScreenHouseholdMeshInput,
    submission: &ScreenHouseholdMeshResultSubmission,
) -> ScreenHouseholdMeshResultValidation {
    let rejection_reason = screen_mesh_rejection_reason(input, submission);
    ScreenHouseholdMeshResultValidation {
        accepted: rejection_reason.is_none(),
        rejection_reason,
        policy_may_run: rejection_reason.is_none(),
    }
}

struct ScreenHouseholdMeshSpine {
    bus: EventBus,
}

impl ScreenHouseholdMeshSpine {
    async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        for phase in ScreenHouseholdMeshPhase::ordered_chain() {
            bus.subscribe::<ScreenHouseholdMeshEventPayload, _, _>(
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

    async fn publish_input_chain(
        &self,
        input: ScreenHouseholdMeshInput,
    ) -> Result<ScreenHouseholdMeshReport, EventingError> {
        let mut reports = Vec::new();
        for phase in ScreenHouseholdMeshPhase::ordered_chain() {
            let payload = ScreenHouseholdMeshEventPayload::from_input(*phase, &input);
            let metadata = screen_mesh_event_metadata(*phase, &input)?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        Ok(ScreenHouseholdMeshReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }
}

fn screen_mesh_rejection_reason(
    input: &ScreenHouseholdMeshInput,
    submission: &ScreenHouseholdMeshResultSubmission,
) -> Option<ScreenMeshResultRejectionReason> {
    if submission.duplicate_result {
        Some(ScreenMeshResultRejectionReason::DuplicateResult)
    } else if submission.completed_after_lease_expiry {
        Some(ScreenMeshResultRejectionReason::ExpiredLease)
    } else if submission.provider_peer_id != input.provider_peer_id {
        Some(ScreenMeshResultRejectionReason::WrongProvider)
    } else if submission.claim_id != input.claim_id || submission.lease_id != input.lease_id {
        Some(ScreenMeshResultRejectionReason::WrongClaim)
    } else if submission.screen_evidence_ref != input.screen_evidence_ref {
        Some(ScreenMeshResultRejectionReason::EvidenceMismatch)
    } else if submission.custody_label != custody_label() {
        Some(ScreenMeshResultRejectionReason::CustodyMismatch)
    } else if submission.raw_screenshot_transferred
        || submission.raw_screenshot_retained_by_provider
    {
        Some(ScreenMeshResultRejectionReason::RawImageTransfer)
    } else if submission.provider_policy_event_attempted
        || submission.provider_enforcement_event_attempted
    {
        Some(ScreenMeshResultRejectionReason::ProviderAuthorityViolation)
    } else {
        None
    }
}

fn provider_result_ref(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> Option<String> {
    match phase {
        ScreenHouseholdMeshPhase::ProviderResultReturned
        | ScreenHouseholdMeshPhase::ChildResultAccepted
        | ScreenHouseholdMeshPhase::PolicyRequested => Some(input.provider_result_ref.clone()),
        _ => None,
    }
}

fn policy_decision_ref(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> Option<String> {
    match phase {
        ScreenHouseholdMeshPhase::PolicyRequested => Some(input.policy_decision_ref.clone()),
        _ => None,
    }
}

fn screen_mesh_event_metadata(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(mesh_aggregate_key(&input.queue_job_id))?,
        screen_mesh_event_source(phase)?,
        RecordedAt::parse(&input.observed_at)?,
        Some(TargetHandler::parse(phase.target_handler())?),
    ))
}

fn screen_mesh_event_source(phase: ScreenHouseholdMeshPhase) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(custody_label())?,
        phase.runtime_role(),
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::screen_flow::RUNTIME_COMPONENT_SCREEN_SPINE)?,
        RuntimeInstanceId::parse(constants::screen_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
