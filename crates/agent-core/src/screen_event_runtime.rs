use ocentra_eventing::{
    AggregateKey, CorrelationId, DomainEvent, EventBus, EventContract, EventCustody, EventId,
    EventMetadata, EventSource, EventSubscriber, EventType, EventingError, IdempotencyKey,
    RecordedAt, RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::{
    screen_event_runtime_phase::ScreenRuntimePhase,
    screen_event_runtime_refs::{
        action_ref, ai_request_ref, ai_result_ref, deletion_proof_ref, parent_rule_ref,
        policy_action, policy_decision_ref, portal_read_model_ref, previous_phase_ref,
        queue_event_ref, screen_aggregate_key, screen_correlation_id, summary_ref,
    },
    screen_event_runtime_state::{
        action_state, ai_audit_state, custody_state, deletion_state, evidence_scope, policy_state,
        ScreenActionState, ScreenAiAuditState, ScreenDeletionState, ScreenEvidenceScope,
        ScreenPolicyState, ScreenRuntimeClaimBoundary,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenRuntimeInput {
    pub queue_job_id: String,
    pub screen_analysis_result_id: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub image_digest: String,
    pub summary: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub prompt_or_template_version: String,
    pub policy_decision_ref: String,
    pub policy_action: String,
    pub parent_rule_ref: String,
    pub action_ref: String,
    pub deletion_proof_ref: String,
    pub portal_read_model_ref: String,
}

impl ScreenRuntimeInput {
    pub fn proof_fixture() -> Self {
        Self {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
            capture_scope: constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW.to_string(),
            image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
            summary: constants::activity_store::TEST_SCREEN_SUMMARY.to_string(),
            model_runtime_ref: constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string(),
            model_id: constants::activity_store::TEST_SCREEN_MODEL_ID.to_string(),
            prompt_or_template_version: constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION
                .to_string(),
            policy_decision_ref: constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            policy_action: constants::activity_store::TEST_POLICY_ACTION_ALLOW.to_string(),
            parent_rule_ref: constants::screen_flow::TEST_SCREEN_POLICY_RULE_REF.to_string(),
            action_ref: constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string(),
            deletion_proof_ref: constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string(),
            portal_read_model_ref: constants::screen_flow::TEST_SCREEN_PORTAL_READ_MODEL_REF
                .to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenRuntimeEventPayload {
    pub phase: ScreenRuntimePhase,
    pub queue_job_id: String,
    pub screen_analysis_result_id: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub image_digest: String,
    pub summary: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub prompt_or_template_version: String,
    pub policy_decision_ref: Option<String>,
    pub policy_action: Option<String>,
    pub parent_rule_ref: Option<String>,
    pub action_ref: Option<String>,
    pub deletion_proof_ref: Option<String>,
    pub portal_read_model_ref: Option<String>,
    pub previous_phase_ref: Option<String>,
    pub capture_event_ref: String,
    pub queue_event_ref: Option<String>,
    pub ai_request_ref: Option<String>,
    pub ai_result_ref: Option<String>,
    pub summary_ref: Option<String>,
    pub evidence_scope: ScreenEvidenceScope,
    pub ai_audit_state: ScreenAiAuditState,
    pub policy_state: ScreenPolicyState,
    pub action_state: ScreenActionState,
    pub deletion_state: ScreenDeletionState,
    pub custody_state: String,
    pub claim_boundary: ScreenRuntimeClaimBoundary,
    pub observed_at: String,
}

impl ScreenRuntimeEventPayload {
    fn from_input(
        phase: ScreenRuntimePhase,
        input: &ScreenRuntimeInput,
        observed_at: &str,
    ) -> Self {
        Self {
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
            policy_decision_ref: policy_decision_ref(phase, input),
            policy_action: policy_action(phase, input),
            parent_rule_ref: parent_rule_ref(phase, input),
            action_ref: action_ref(phase, input),
            deletion_proof_ref: deletion_proof_ref(phase, input),
            portal_read_model_ref: portal_read_model_ref(phase, input),
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
}

impl DomainEvent for ScreenRuntimeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::screen_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(screen_aggregate_key(&self.queue_job_id))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::screen_flow::IDEMPOTENCY_SCREEN_RUNTIME_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&screen_aggregate_key(&self.queue_job_id));
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

#[derive(Clone, Debug)]
pub struct ScreenRuntimeReport {
    pub publish_reports: Vec<ocentra_eventing::PublishReport>,
    pub stored_events: Vec<ocentra_eventing::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::DeadLetter>,
}

impl ScreenRuntimeReport {
    pub fn raw_image_escaped(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<ScreenRuntimeEventPayload>()
                .map(|envelope| {
                    envelope
                        .payload
                        .claim_boundary
                        .raw_image_available_to_ai_provider
                        || envelope
                            .payload
                            .claim_boundary
                            .raw_image_available_to_policy
                        || envelope
                            .payload
                            .claim_boundary
                            .raw_image_available_to_portal
                })
                .unwrap_or(true)
        })
    }
}

pub async fn publish_screen_runtime_chain_for_input(
    input: ScreenRuntimeInput,
    observed_at: &str,
) -> Result<ScreenRuntimeReport, EventingError> {
    let spine = ScreenRuntimeSpine::with_default_handlers().await?;
    spine.publish_input_chain(input, observed_at).await
}

struct ScreenRuntimeSpine {
    bus: EventBus,
}

impl ScreenRuntimeSpine {
    async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        for phase in ScreenRuntimePhase::ordered_chain() {
            bus.subscribe::<ScreenRuntimeEventPayload, _, _>(
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
        input: ScreenRuntimeInput,
        observed_at: &str,
    ) -> Result<ScreenRuntimeReport, EventingError> {
        let mut reports = Vec::new();
        for phase in ScreenRuntimePhase::ordered_chain() {
            let payload = ScreenRuntimeEventPayload::from_input(*phase, &input, observed_at);
            let metadata = screen_event_metadata(*phase, &input, observed_at)?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        Ok(ScreenRuntimeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }
}

fn screen_event_metadata(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
    observed_at: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(screen_correlation_id(&input.queue_job_id))?,
        screen_event_source(phase)?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(phase.target_handler())?),
    ))
}

fn screen_event_source(phase: ScreenRuntimePhase) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(custody_state(phase))?,
        phase.runtime_role(),
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::screen_flow::RUNTIME_COMPONENT_SCREEN_SPINE)?,
        RuntimeInstanceId::parse(constants::screen_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
