use std::time::Duration;

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, contract_registry::EventContractRegistry,
    envelope::DomainEvent, envelope::EventContract, error::EventingError, ids::AggregateKey,
    ids::EventType, ids::IdempotencyKey, ids::RequestId, ids::SchemaVersion, ids::SourceComponent,
    ids::SubscriberId, ids::TargetHandler, request::RequestEvent, request::RequestOptions,
    topology::EventTopologyManifest, topology::EventTopologyPublisher,
    topology::EventTopologySubscriber,
};
use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use super::{
    browser_aggregate_key, browser_event_metadata, BrowserRuntimeEventPayload, BrowserRuntimeInput,
    BrowserRuntimeReport,
};

pub type BrowserRuntimeActionIntentHandoffReport =
    ocentra_parent_agent_protocol::browser::action_handoff::BrowserRuntimeActionIntentHandoffReport;

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

pub type BrowserRuntimeActionIntentHandoffResponse =
    ocentra_parent_agent_protocol::browser::action_handoff::BrowserRuntimeActionIntentHandoffResponse;

fn action_intent_handoff_response_from_payload(
    payload: &BrowserRuntimeEventPayload,
) -> BrowserRuntimeActionIntentHandoffResponse {
    let candidate = candidate_refs_from_payload(payload);
    BrowserRuntimeActionIntentHandoffResponse {
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
    let bus = EventBus::root();
    bus.subscribe::<BrowserRuntimeActionIntentHandoffRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_HANDOFF)?,
            EventType::parse(constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED)?,
            TargetHandler::parse(constants::browser::TARGET_BROWSER_ACTION_INTENT_HANDOFF)?,
        ),
        |context| async move {
            context
                .complete_request(action_intent_handoff_response_from_payload(
                    &context.payload().payload,
                ))
                .await?;
            Ok(())
        },
    )
    .await?;

    let phase = BrowserRuntimePhase::PolicyDecisionCompleted;
    let payload = super::browser_runtime_event_payload_from_input(phase, &input);
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
    let payload = super::browser_runtime_event_payload_from_input(
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
    event: &ocentra_eventing::envelope::StoredEventEnvelope,
) -> Option<ActionIntentHandoffRefs> {
    let decoded = event.decode::<BrowserRuntimeEventPayload>().ok()?;
    candidate_refs_from_payload(decoded.payload())
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
        outbox_ref: action_intent_ref(
            constants::browser::ACTION_INTENT_OUTBOX_REF_PREFIX,
            payload.action_intent_id.as_deref()?,
        ),
        handoff_ref: action_intent_ref(
            constants::browser::ACTION_INTENT_HANDOFF_REF_PREFIX,
            payload.action_intent_id.as_deref()?,
        ),
    })
}

fn action_intent_ref(prefix: &str, action_intent_id: &str) -> String {
    let suffix = action_intent_id
        .strip_prefix(constants::browser::ACTION_INTENT_ID_PREFIX)
        .unwrap_or(action_intent_id);
    let mut value = String::from(prefix);
    value.push_str(suffix);
    value
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
