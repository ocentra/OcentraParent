use ocentra_eventing::bus::reports::dead_letter::DeadLetter;
use ocentra_eventing::bus::reports::handler::PublishReport;
use ocentra_eventing::{
    bus::publisher::{EventPublisher, RootEventPublisher},
    bus::subscriber::EventSubscriber,
    bus::EventBus,
    error::EventingError,
    ids::EventType,
    ids::SubscriberId,
    ids::TargetHandler,
    journal::ndjson::NdjsonEventJournal,
    journal::policy::JournalPolicy,
    journal::policy::JournalSelector,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenActionState, ScreenAiAuditState, ScreenDeletionState, ScreenEvidenceScope,
    ScreenPolicyState, ScreenRuntimeClaimBoundary,
    ScreenRuntimeEventPayload as ProtocolScreenRuntimeEventPayload, ScreenRuntimePhase,
};

use crate::{
    screen_event_runtime_input::{
        ScreenRuntimeCaptureInput, ScreenRuntimeDegradedInput, ScreenRuntimeDeletionInput,
        ScreenRuntimeInput,
    },
    screen_event_runtime_metadata::{
        screen_capture_event_metadata, screen_deletion_event_metadata, screen_event_metadata,
    },
    screen_event_runtime_refs::{
        action_ref, ai_request_ref, ai_result_ref, deletion_proof_ref, parent_rule_ref,
        policy_action, policy_decision_ref, portal_read_model_ref, previous_phase_ref,
        queue_event_ref, summary_ref,
    },
    screen_event_runtime_state::{
        action_state, ai_audit_state, custody_state, deletion_state, evidence_scope, policy_state,
    },
};

pub type ScreenRuntimeEventPayload = ProtocolScreenRuntimeEventPayload;

#[derive(Clone, Debug)]
pub struct ScreenRuntimeReport {
    pub publish_reports: Vec<PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

impl ScreenRuntimeReport {
    pub fn raw_image_escaped(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<ScreenRuntimeEventPayload>()
                .map(|envelope| {
                    envelope
                        .payload()
                        .claim_boundary
                        .raw_image_available_to_ai_provider
                        || envelope
                            .payload()
                            .claim_boundary
                            .raw_image_available_to_policy
                        || envelope
                            .payload()
                            .claim_boundary
                            .raw_image_available_to_portal
                })
                .unwrap_or(true)
        })
    }
}

fn screen_runtime_event_payload_from_input(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
    observed_at: &str,
) -> ScreenRuntimeEventPayload {
    let mut payload = screen_runtime_event_payload_from_capture_input(
        phase,
        &ScreenRuntimeCaptureInput::from(input),
        observed_at,
    );
    payload.policy_decision_ref = policy_decision_ref(phase, input);
    payload.policy_action = policy_action(phase, input);
    payload.parent_rule_ref = parent_rule_ref(phase, input);
    payload.action_ref = action_ref(phase, input);
    payload.deletion_proof_ref = deletion_proof_ref(phase, input);
    payload.portal_read_model_ref = portal_read_model_ref(phase, input);
    payload
}

fn screen_runtime_event_payload_from_capture_input(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeCaptureInput,
    observed_at: &str,
) -> ScreenRuntimeEventPayload {
    ScreenRuntimeEventPayload {
        phase,
        queue_job_id: input.queue_job_id.clone(),
        screen_analysis_result_id: input.screen_analysis_result_id.clone(),
        capture_reason: input.capture_reason.clone(),
        capture_scope: input.capture_scope.clone(),
        image_digest: input.image_digest.clone(),
        summary: input.summary.clone(),
        model_runtime_ref: input.model_runtime_ref.clone(),
        model_id: input.model_id.clone(),
        prompt_or_template_version: input.prompt_or_template_version.clone(),
        policy_decision_ref: None,
        policy_action: None,
        parent_rule_ref: None,
        action_ref: None,
        deletion_proof_ref: None,
        portal_read_model_ref: None,
        previous_phase_ref: previous_phase_ref(phase),
        capture_event_ref: constants::screen_flow::SCREEN_CAPTURE_EVENT_REF.to_string(),
        queue_event_ref: queue_event_ref(phase),
        ai_request_ref: ai_request_ref(phase),
        ai_result_ref: ai_result_ref(phase),
        summary_ref: summary_ref(phase),
        evidence_scope: evidence_scope(phase),
        ai_audit_state: ai_audit_state(phase),
        policy_state: policy_state(phase),
        action_state: action_state(phase),
        deletion_state: deletion_state(phase),
        custody_state: custody_state(phase).to_string(),
        claim_boundary: ScreenRuntimeClaimBoundary::child_owned_no_raw_escape(),
        observed_at: observed_at.to_string(),
    }
}

fn screen_runtime_event_payload_from_deletion_input(
    input: &ScreenRuntimeDeletionInput,
    observed_at: &str,
) -> ScreenRuntimeEventPayload {
    let capture_input = ScreenRuntimeCaptureInput {
        queue_job_id: input.queue_job_id.clone(),
        screen_analysis_result_id: input.screen_analysis_result_id.clone(),
        capture_reason: input.capture_reason.clone(),
        capture_scope: input.capture_scope.clone(),
        image_digest: input.image_digest.clone(),
        summary: input.summary.clone(),
        model_runtime_ref: input.model_runtime_ref.clone(),
        model_id: input.model_id.clone(),
        prompt_or_template_version: input.prompt_or_template_version.clone(),
    };
    let mut payload = screen_runtime_event_payload_from_capture_input(
        ScreenRuntimePhase::DeletionCommitted,
        &capture_input,
        observed_at,
    );
    payload.previous_phase_ref = Some(constants::screen_flow::SCREEN_QUEUE_EVENT_REF.to_string());
    payload.ai_request_ref = None;
    payload.ai_result_ref = None;
    payload.summary_ref = None;
    payload.policy_decision_ref = None;
    payload.policy_action = None;
    payload.parent_rule_ref = None;
    payload.action_ref = None;
    payload.deletion_proof_ref = Some(input.deletion_proof_ref.clone());
    payload.ai_audit_state = ScreenAiAuditState::NotRequested;
    payload.policy_state = ScreenPolicyState::NotReady;
    payload.action_state = ScreenActionState::NotReady;
    payload.deletion_state = ScreenDeletionState::Committed;
    payload.evidence_scope = ScreenEvidenceScope::DeletedQueryStoreSummary;
    payload.custody_state = constants::eventing_source::CUSTODY_LOCAL_JOURNAL.to_string();
    payload
}

fn screen_runtime_event_payload_from_degraded_input(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeDegradedInput,
    observed_at: &str,
) -> ScreenRuntimeEventPayload {
    let capture_input = ScreenRuntimeCaptureInput::from(input);
    let mut payload =
        screen_runtime_event_payload_from_capture_input(phase, &capture_input, observed_at);
    if matches!(
        phase,
        ScreenRuntimePhase::DeletionCommitted | ScreenRuntimePhase::PortalReadModelUpdated
    ) {
        payload.ai_request_ref = None;
        payload.ai_result_ref = None;
        payload.summary_ref = None;
        payload.previous_phase_ref = Some(
            if phase == ScreenRuntimePhase::DeletionCommitted {
                constants::screen_flow::SCREEN_QUEUE_EVENT_REF
            } else {
                constants::screen_flow::SCREEN_DELETION_EVENT_REF
            }
            .to_string(),
        );
        payload.deletion_proof_ref = Some(input.deletion_proof_ref.clone());
        payload.evidence_scope = ScreenEvidenceScope::DeletedQueryStoreSummary;
        payload.custody_state = constants::eventing_source::CUSTODY_LOCAL_JOURNAL.to_string();
        payload.deletion_state = ScreenDeletionState::Committed;
        payload.ai_audit_state = ScreenAiAuditState::NotRequested;
    }
    if phase == ScreenRuntimePhase::PortalReadModelUpdated {
        payload.portal_read_model_ref = Some(input.portal_read_model_ref.clone());
        payload.custody_state = constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE.to_string();
    }
    payload.policy_decision_ref = None;
    payload.policy_action = None;
    payload.parent_rule_ref = None;
    payload.action_ref = None;
    payload.policy_state = ScreenPolicyState::NotReady;
    payload.action_state = ScreenActionState::NotReady;
    payload
}

pub async fn publish_screen_runtime_chain_for_input(
    publisher: &EventPublisher,
    target_bus: &EventBus,
    input: ScreenRuntimeInput,
    observed_at: &str,
) -> Result<ScreenRuntimeReport, EventingError> {
    let mut reports = Vec::new();
    for phase in ScreenRuntimePhase::ordered_chain() {
        let payload = screen_runtime_event_payload_from_input(*phase, &input, observed_at);
        let metadata = screen_event_metadata(*phase, &input, observed_at)?;
        reports.push(publisher.publish_on(target_bus, payload, metadata).await?);
    }
    causal_screen_runtime_report(target_bus, reports).await
}

pub async fn publish_screen_capture_queue_events_for_input(
    input: ScreenRuntimeCaptureInput,
    observed_at: &str,
) -> Result<ScreenRuntimeReport, EventingError> {
    let spine = ScreenRuntimeSpine::without_owner_handlers();
    spine.publish_capture_queue_events(input, observed_at).await
}

pub async fn publish_screen_deletion_event_for_input(
    input: ScreenRuntimeDeletionInput,
    observed_at: &str,
) -> Result<ScreenRuntimeReport, EventingError> {
    let spine = ScreenRuntimeSpine::without_owner_handlers();
    spine.publish_deletion_event(input, observed_at).await
}

pub async fn publish_screen_degraded_event_chain_for_input(
    publisher: &EventPublisher,
    target_bus: &EventBus,
    input: ScreenRuntimeDegradedInput,
    observed_at: &str,
) -> Result<ScreenRuntimeReport, EventingError> {
    let mut reports = Vec::new();
    for phase in [
        ScreenRuntimePhase::CaptureObserved,
        ScreenRuntimePhase::QueueEncrypted,
        ScreenRuntimePhase::DeletionCommitted,
        ScreenRuntimePhase::PortalReadModelUpdated,
    ] {
        let payload = screen_runtime_event_payload_from_degraded_input(phase, &input, observed_at);
        let metadata = screen_capture_event_metadata(
            phase,
            &ScreenRuntimeCaptureInput::from(&input),
            observed_at,
        )?;
        reports.push(publisher.publish_on(target_bus, payload, metadata).await?);
    }
    causal_screen_runtime_report(target_bus, reports).await
}

async fn causal_screen_runtime_report(
    target_bus: &EventBus,
    publish_reports: Vec<PublishReport>,
) -> Result<ScreenRuntimeReport, EventingError> {
    let published_event_ids = publish_reports
        .iter()
        .map(|report| report.event_id.clone())
        .collect::<Vec<_>>();
    let stored_events = target_bus
        .journal()
        .await
        .into_iter()
        .filter(|event| published_event_ids.contains(&event.event_id))
        .collect();
    let dead_letters = target_bus
        .dead_letters()
        .await
        .into_iter()
        .filter(|dead_letter| published_event_ids.contains(&dead_letter.envelope.event_id))
        .collect();
    Ok(ScreenRuntimeReport {
        publish_reports,
        stored_events,
        dead_letters,
    })
}

pub struct ScreenRuntimeSpine {
    bus: RootEventPublisher,
}

impl ScreenRuntimeSpine {
    fn without_owner_handlers() -> Self {
        Self {
            bus: EventBus::new(),
        }
    }

    pub async fn with_durable_deletion_handler(
        journal: NdjsonEventJournal,
    ) -> Result<Self, EventingError> {
        let deletion_event_type =
            EventType::parse(constants::screen_flow::EVENT_SCREEN_DELETION_COMMITTED)?;
        let bus = EventBus::with_journal(
            JournalPolicy::after_dispatch(JournalSelector::EventTypes(vec![
                deletion_event_type.clone()
            ])),
            journal.shared(),
        );
        bus.subscribe::<ScreenRuntimeEventPayload, _, _>(
            EventSubscriber::new(
                SubscriberId::parse(constants::screen_flow::SUBSCRIBER_SCREEN_DELETION_WORKER)?,
                deletion_event_type,
                TargetHandler::parse(constants::screen_flow::TARGET_SCREEN_DELETION_WORKER)?,
            ),
            |context| async move { handle_terminal_deletion_delivery(context.payload()) },
        )
        .await?;
        Ok(Self { bus })
    }

    async fn publish_capture_queue_events(
        &self,
        input: ScreenRuntimeCaptureInput,
        observed_at: &str,
    ) -> Result<ScreenRuntimeReport, EventingError> {
        let mut reports = Vec::new();
        for phase in [
            ScreenRuntimePhase::CaptureObserved,
            ScreenRuntimePhase::QueueEncrypted,
        ] {
            let payload =
                screen_runtime_event_payload_from_capture_input(phase, &input, observed_at);
            let metadata = screen_capture_event_metadata(phase, &input, observed_at)?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        Ok(ScreenRuntimeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }

    pub async fn publish_deletion_event(
        &self,
        input: ScreenRuntimeDeletionInput,
        observed_at: &str,
    ) -> Result<ScreenRuntimeReport, EventingError> {
        let payload = screen_runtime_event_payload_from_deletion_input(&input, observed_at);
        let metadata = screen_deletion_event_metadata(&input, observed_at)?;
        let report = self.bus.publish(payload, metadata).await?;
        let published_event_id = report.event_id.clone();
        let stored_events = self
            .bus
            .journal()
            .await
            .into_iter()
            .filter(|event| event.event_id == published_event_id)
            .collect();
        let dead_letters = self
            .bus
            .dead_letters()
            .await
            .into_iter()
            .filter(|dead_letter| dead_letter.envelope.event_id == published_event_id)
            .collect();
        Ok(ScreenRuntimeReport {
            publish_reports: vec![report],
            stored_events,
            dead_letters,
        })
    }

    pub async fn retained_event_count(&self) -> usize {
        self.bus.journal().await.len()
    }
}

fn handle_terminal_deletion_delivery(
    payload: &ScreenRuntimeEventPayload,
) -> Result<(), EventingError> {
    let deletion_proof_present = payload
        .deletion_proof_ref
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty());
    let no_raw_image_escape = !payload.claim_boundary.raw_image_available_to_ai_provider
        && !payload.claim_boundary.raw_image_available_to_policy
        && !payload.claim_boundary.raw_image_available_to_portal;
    if payload.phase == ScreenRuntimePhase::DeletionCommitted
        && payload.deletion_state == ScreenDeletionState::Committed
        && payload.evidence_scope == ScreenEvidenceScope::DeletedQueryStoreSummary
        && deletion_proof_present
        && no_raw_image_escape
    {
        return Ok(());
    }
    Err(EventingError::InvalidValue {
        field: constants::screen_flow::EVENT_SCREEN_DELETION_COMMITTED,
        value: constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES.to_string(),
    })
}
