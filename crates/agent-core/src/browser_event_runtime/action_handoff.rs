use std::time::Duration;

use ocentra_eventing::{
    AggregateKey, DomainEvent, EventBus, EventContract, EventContractRegistry,
    EventResponseContract, EventSubscriber, EventTopologyManifest, EventTopologyPublisher,
    EventTopologySubscriber, EventType, EventingError, IdempotencyKey, RequestEvent, RequestId,
    RequestOptions, SchemaVersion, SourceComponent, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::{
    BrowserRuntimeEventPayload, BrowserRuntimeInput, BrowserRuntimePhase, BrowserRuntimeReport,
};

use super::{browser_aggregate_key, browser_event_metadata};

#[derive(Clone, Debug)]
pub struct BrowserRuntimeActionIntentHandoffReport {
    pub request_report: ocentra_eventing::RequestReport<BrowserRuntimeActionIntentHandoffResponse>,
    pub stored_events: Vec<ocentra_eventing::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::DeadLetter>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BrowserRuntimeActionIntentHandoffRequest {
    payload: BrowserRuntimeEventPayload,
    request_id: RequestId,
}

impl DomainEvent for BrowserRuntimeActionIntentHandoffRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED)?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(browser_aggregate_key(&self.payload.source_ref))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value =
            String::from(constants::browser::IDEMPOTENCY_BROWSER_ACTION_INTENT_HANDOFF_PREFIX);
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for BrowserRuntimeActionIntentHandoffRequest {
    type Response = BrowserRuntimeActionIntentHandoffResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserRuntimeActionIntentHandoffResponse {
    pub candidate_count: usize,
    pub policy_preview_id: Option<String>,
    pub action_intent_id: Option<String>,
    pub source_event_ref: Option<String>,
    pub outbox_ref: Option<String>,
    pub handoff_ref: Option<String>,
    pub source_ref: String,
    pub evidence_ref: String,
    pub dry_run_only: bool,
    pub policy_authority_only: bool,
    pub dispatch_attempt_count: u8,
    pub adapter_execution_count: u8,
    pub browser_mutation_count: u8,
    pub child_intervention_execution_count: u8,
    pub enforcement_execution_count: u8,
}

impl BrowserRuntimeActionIntentHandoffResponse {
    fn from_payload(payload: &BrowserRuntimeEventPayload) -> Self {
        let candidate = candidate_refs_from_payload(payload);
        Self {
            candidate_count: usize::from(candidate.is_some()),
            policy_preview_id: candidate
                .as_ref()
                .map(|refs| refs.policy_preview_id.clone()),
            action_intent_id: candidate.as_ref().map(|refs| refs.action_intent_id.clone()),
            source_event_ref: candidate.as_ref().map(|refs| refs.source_event_ref.clone()),
            outbox_ref: candidate.as_ref().map(|refs| refs.outbox_ref.clone()),
            handoff_ref: candidate.as_ref().map(|refs| refs.handoff_ref.clone()),
            source_ref: payload.source_ref.clone(),
            evidence_ref: payload.evidence_ref.clone(),
            dry_run_only: true,
            policy_authority_only: true,
            dispatch_attempt_count: 0,
            adapter_execution_count: 0,
            browser_mutation_count: 0,
            child_intervention_execution_count: 0,
            enforcement_execution_count: 0,
        }
    }
}

impl EventResponseContract for BrowserRuntimeActionIntentHandoffResponse {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActionIntentHandoffRefs {
    policy_preview_id: String,
    action_intent_id: String,
    source_event_ref: String,
    outbox_ref: String,
    handoff_ref: String,
}

pub(super) fn handoff_summary(
    report: &BrowserRuntimeReport,
) -> Option<(usize, String, String, String, String, String)> {
    if report.intervention_command_published() {
        return None;
    }
    let candidates = report
        .stored_events
        .iter()
        .filter_map(candidate_refs)
        .collect::<Vec<_>>();
    let refs = candidates.first()?.clone();
    Some((
        candidates.len(),
        refs.policy_preview_id,
        refs.action_intent_id,
        refs.source_event_ref,
        refs.outbox_ref,
        refs.handoff_ref,
    ))
}

pub async fn request_browser_runtime_action_intent_handoff_for_input(
    input: BrowserRuntimeInput,
) -> Result<BrowserRuntimeActionIntentHandoffReport, EventingError> {
    let bus = EventBus::new();
    bus.subscribe::<BrowserRuntimeActionIntentHandoffRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_HANDOFF)?,
            EventType::parse(constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED)?,
            TargetHandler::parse(constants::browser::TARGET_BROWSER_ACTION_INTENT_HANDOFF)?,
        ),
        |context| async move {
            context
                .complete_request(BrowserRuntimeActionIntentHandoffResponse::from_payload(
                    &context.payload().payload,
                ))
                .await?;
            Ok(())
        },
    )
    .await?;

    let phase = BrowserRuntimePhase::PolicyDecisionCompleted;
    let payload = BrowserRuntimeEventPayload::from_input(phase, &input);
    let request = BrowserRuntimeActionIntentHandoffRequest {
        request_id: RequestId::parse(action_intent_handoff_request_id(&payload))?,
        payload,
    };
    let metadata = browser_event_metadata(
        phase,
        &input,
        constants::browser::TARGET_BROWSER_ACTION_INTENT_HANDOFF,
    )?;
    let request_report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_ACTION_INTENT_HANDOFF_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(BrowserRuntimeActionIntentHandoffReport {
        request_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}

pub fn browser_runtime_action_intent_handoff_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    let payload = BrowserRuntimeEventPayload::from_input(
        BrowserRuntimePhase::PolicyDecisionCompleted,
        &BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    );
    let request = BrowserRuntimeActionIntentHandoffRequest {
        request_id: RequestId::parse(action_intent_handoff_request_id(&payload))?,
        payload,
    };
    let mut registry = EventContractRegistry::new();
    registry.register_event(&request)?;
    Ok(EventTopologyManifest::from_registry(
        &registry,
        &[EventTopologyPublisher {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED,
            )?,
            source_component: SourceComponent::parse(
                constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
            )?,
        }],
        &[EventTopologySubscriber {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED,
            )?,
            subscriber_id: SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_HANDOFF,
            )?,
            target_handler: TargetHandler::parse(
                constants::browser::TARGET_BROWSER_ACTION_INTENT_HANDOFF,
            )?,
        }],
        &[],
        &[],
    ))
}

fn candidate_refs(
    event: &ocentra_eventing::StoredEventEnvelope,
) -> Option<ActionIntentHandoffRefs> {
    let decoded = event.decode::<BrowserRuntimeEventPayload>().ok()?;
    candidate_refs_from_payload(&decoded.payload)
}

fn candidate_refs_from_payload(
    payload: &BrowserRuntimeEventPayload,
) -> Option<ActionIntentHandoffRefs> {
    if payload.phase != BrowserRuntimePhase::PolicyDecisionCompleted
        || !payload.dry_run
        || !payload.policy_authority
    {
        return None;
    }
    Some(ActionIntentHandoffRefs {
        policy_preview_id: payload.policy_preview_id.clone()?,
        action_intent_id: payload.action_intent_id.clone()?,
        source_event_ref: browser_event_ref(payload),
        outbox_ref: constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF.to_string(),
        handoff_ref: constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF.to_string(),
    })
}

fn browser_event_ref(payload: &BrowserRuntimeEventPayload) -> String {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&payload.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(payload.phase.event_type());
    value
}

fn action_intent_handoff_request_id(payload: &BrowserRuntimeEventPayload) -> String {
    let mut value = String::from(constants::browser::REQUEST_BROWSER_ACTION_INTENT_HANDOFF_PREFIX);
    value.push_str(&payload.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value
}
