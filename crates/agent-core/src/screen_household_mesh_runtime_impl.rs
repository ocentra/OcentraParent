use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventCustody,
    ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenHouseholdMeshEventPayload, ScreenHouseholdMeshPhase, ScreenHouseholdMeshResultValidation,
    ScreenMeshCustodyBoundary, ScreenMeshPayloadMode, ScreenMeshResultRejectionReason,
};

use crate::screen_household_mesh_runtime_refs::{
    child_validation_state, claim_state, lease_state, mesh_aggregate_key, policy_state,
    previous_mesh_phase_ref, provider_result_state,
};
use crate::screen_household_mesh_runtime_state::custody_label;

pub(crate) async fn publish_screen_household_mesh_chain_for_input(
    input: crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
) -> Result<crate::screen_household_mesh_runtime::ScreenHouseholdMeshReport, EventingError> {
    let spine = ScreenHouseholdMeshSpine::without_owner_handlers();
    spine.publish_input_chain(input).await
}

pub(crate) fn validate_screen_household_mesh_result(
    input: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
    submission: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshResultSubmission,
) -> ScreenHouseholdMeshResultValidation {
    let rejection_reason = screen_mesh_rejection_reason(input, submission);
    ScreenHouseholdMeshResultValidation {
        accepted: rejection_reason.is_none(),
        rejection_reason,
        policy_may_run: rejection_reason.is_none(),
    }
}

pub(crate) fn screen_household_mesh_event_payload_from_input(
    phase: ScreenHouseholdMeshPhase,
    input: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
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

pub(crate) fn screen_mesh_rejection_reason(
    input: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
    submission: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshResultSubmission,
) -> Option<ScreenMeshResultRejectionReason> {
    [
        submission
            .duplicate_result
            .then_some(ScreenMeshResultRejectionReason::DuplicateResult),
        submission
            .completed_after_lease_expiry
            .then_some(ScreenMeshResultRejectionReason::ExpiredLease),
        (submission.provider_peer_id != input.provider_peer_id)
            .then_some(ScreenMeshResultRejectionReason::WrongProvider),
        (submission.claim_id != input.claim_id || submission.lease_id != input.lease_id)
            .then_some(ScreenMeshResultRejectionReason::WrongClaim),
        (submission.screen_evidence_ref != input.screen_evidence_ref)
            .then_some(ScreenMeshResultRejectionReason::EvidenceMismatch),
        (submission.custody_label != custody_label())
            .then_some(ScreenMeshResultRejectionReason::CustodyMismatch),
        (submission.raw_screenshot_transferred || submission.raw_screenshot_retained_by_provider)
            .then_some(ScreenMeshResultRejectionReason::RawImageTransfer),
        (submission.provider_policy_event_attempted
            || submission.provider_enforcement_event_attempted)
            .then_some(ScreenMeshResultRejectionReason::ProviderAuthorityViolation),
    ]
    .into_iter()
    .flatten()
    .next()
}

pub(crate) fn provider_result_ref(
    phase: ScreenHouseholdMeshPhase,
    input: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
) -> Option<String> {
    match phase {
        ScreenHouseholdMeshPhase::ProviderResultReturned
        | ScreenHouseholdMeshPhase::ChildResultAccepted
        | ScreenHouseholdMeshPhase::PolicyRequested => Some(input.provider_result_ref.clone()),
        _ => None,
    }
}

pub(crate) fn policy_decision_ref(
    phase: ScreenHouseholdMeshPhase,
    input: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
) -> Option<String> {
    match phase {
        ScreenHouseholdMeshPhase::PolicyRequested => Some(input.policy_decision_ref.clone()),
        _ => None,
    }
}

pub(crate) fn screen_mesh_event_metadata(
    phase: ScreenHouseholdMeshPhase,
    input: &crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(mesh_aggregate_key(&input.queue_job_id))?,
        screen_mesh_event_source(phase)?,
        RecordedAt::parse(&input.observed_at)?,
        Some(TargetHandler::parse(phase.target_handler())?),
    ))
}

pub(crate) fn screen_mesh_event_source(
    phase: ScreenHouseholdMeshPhase,
) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(custody_label())?,
        phase.runtime_role()?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::screen_flow::RUNTIME_COMPONENT_SCREEN_SPINE)?,
        RuntimeInstanceId::parse(constants::screen_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}

struct ScreenHouseholdMeshSpine {
    bus: RootEventPublisher,
}

impl ScreenHouseholdMeshSpine {
    fn without_owner_handlers() -> Self {
        Self {
            bus: EventBus::root(),
        }
    }

    async fn publish_input_chain(
        &self,
        input: crate::screen_household_mesh_runtime::ScreenHouseholdMeshInput,
    ) -> Result<crate::screen_household_mesh_runtime::ScreenHouseholdMeshReport, EventingError>
    {
        let mut reports = Vec::new();
        for phase in ScreenHouseholdMeshPhase::ordered_chain() {
            let payload = screen_household_mesh_event_payload_from_input(*phase, &input);
            let metadata = screen_mesh_event_metadata(*phase, &input)?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        Ok(
            crate::screen_household_mesh_runtime::ScreenHouseholdMeshReport {
                publish_reports: reports,
                stored_events: self.bus.journal().await,
                dead_letters: self.bus.dead_letters().await,
            },
        )
    }
}
