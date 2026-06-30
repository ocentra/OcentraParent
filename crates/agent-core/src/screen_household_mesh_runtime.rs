use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventCustody,
    ids::EventId, ids::EventType, ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenHouseholdMeshEventPayload as ProtocolScreenHouseholdMeshEventPayload,
    ScreenHouseholdMeshPhase, ScreenHouseholdMeshResultValidation, ScreenMeshCustodyBoundary,
    ScreenMeshPayloadMode, ScreenMeshResultRejectionReason,
};

use crate::{
    screen_household_mesh_runtime_refs::{
        child_validation_state, claim_state, lease_state, mesh_aggregate_key, policy_state,
        previous_mesh_phase_ref, provider_result_state,
    },
    screen_household_mesh_runtime_state::custody_label,
};

pub type ScreenHouseholdMeshEventPayload = ProtocolScreenHouseholdMeshEventPayload;
pub type ScreenHouseholdMeshInput =
    ocentra_parent_agent_protocol::screen_evidence::screen_household_mesh_input::ScreenHouseholdMeshInput;
pub type ScreenHouseholdMeshResultSubmission =
    ocentra_parent_agent_protocol::screen_evidence::screen_household_mesh_input::ScreenHouseholdMeshResultSubmission;

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

fn screen_household_mesh_event_payload_from_input(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> ScreenHouseholdMeshEventPayload {
    ScreenHouseholdMeshEventPayload {
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
            let payload = screen_household_mesh_event_payload_from_input(*phase, &input);
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
