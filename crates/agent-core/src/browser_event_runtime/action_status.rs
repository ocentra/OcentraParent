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
};

pub type BrowserRuntimeActionIntentStatusReport =
    ocentra_parent_agent_protocol::browser::action_status::BrowserRuntimeActionIntentStatusReport;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BrowserRuntimeActionIntentStatusRequest {
    payload: BrowserRuntimeEventPayload,
    request_id: RequestId,
}

impl DomainEvent for BrowserRuntimeActionIntentStatusRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED)?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(browser_aggregate_key(&self.payload.source_ref))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value =
            String::from(constants::browser::IDEMPOTENCY_BROWSER_ACTION_INTENT_STATUS_PREFIX);
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for BrowserRuntimeActionIntentStatusRequest {
    type Response = BrowserRuntimeActionIntentStatusResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

pub type BrowserRuntimeActionIntentStatusResponse =
    ocentra_parent_agent_protocol::browser::action_status::BrowserRuntimeActionIntentStatusResponse;

fn action_intent_status_response_from_payload(
    payload: &BrowserRuntimeEventPayload,
) -> BrowserRuntimeActionIntentStatusResponse {
    let has_candidate = payload.phase == BrowserRuntimePhase::PolicyDecisionCompleted
        && payload.dry_run
        && payload.policy_authority
        && payload.policy_preview_id.is_some()
        && payload.action_intent_id.is_some();
    BrowserRuntimeActionIntentStatusResponse {
        candidate_count: usize::from(has_candidate),
        policy_preview_id: has_candidate
            .then(|| payload.policy_preview_id.clone())
            .flatten(),
        action_intent_id: has_candidate
            .then(|| payload.action_intent_id.clone())
            .flatten(),
        source_ref: payload.source_ref.clone(),
        evidence_ref: payload.evidence_ref.clone(),
        dry_run_only: true,
        policy_authority_only: true,
        dispatch_attempt_count: 0,
        adapter_execution_count: 0,
        child_intervention_execution_count: 0,
        enforcement_execution_count: 0,
    }
}

pub async fn request_browser_runtime_action_intent_status_for_input(
    input: BrowserRuntimeInput,
) -> Result<BrowserRuntimeActionIntentStatusReport, EventingError> {
    let bus = EventBus::root();
    bus.subscribe::<BrowserRuntimeActionIntentStatusRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_STATUS)?,
            EventType::parse(constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED)?,
            TargetHandler::parse(constants::browser::TARGET_BROWSER_ACTION_INTENT_STATUS)?,
        ),
        |context| async move {
            context
                .complete_request(action_intent_status_response_from_payload(
                    &context.payload().payload,
                ))
                .await?;
            Ok(())
        },
    )
    .await?;

    let phase = BrowserRuntimePhase::PolicyDecisionCompleted;
    let payload = super::browser_runtime_event_payload_from_input(phase, &input);
    let request = BrowserRuntimeActionIntentStatusRequest {
        request_id: RequestId::parse(action_intent_status_request_id(&payload))?,
        payload,
    };
    let metadata = browser_event_metadata(
        phase,
        &input,
        constants::browser::TARGET_BROWSER_ACTION_INTENT_STATUS,
    )?;
    let request_report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_ACTION_INTENT_STATUS_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(BrowserRuntimeActionIntentStatusReport {
        request_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}

pub fn browser_runtime_action_intent_status_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    let payload = super::browser_runtime_event_payload_from_input(
        BrowserRuntimePhase::PolicyDecisionCompleted,
        &BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    );
    let request = BrowserRuntimeActionIntentStatusRequest {
        request_id: RequestId::parse(action_intent_status_request_id(&payload))?,
        payload,
    };
    let mut registry = EventContractRegistry::new();
    registry.register_event(&request)?;
    Ok(EventTopologyManifest::from_registry(
        &registry,
        &[EventTopologyPublisher {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED,
            )?,
            source_component: SourceComponent::parse(
                constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
            )?,
        }],
        &[EventTopologySubscriber {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED,
            )?,
            subscriber_id: SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_STATUS,
            )?,
            target_handler: TargetHandler::parse(
                constants::browser::TARGET_BROWSER_ACTION_INTENT_STATUS,
            )?,
        }],
        &[],
        &[],
    ))
}

fn action_intent_status_request_id(payload: &BrowserRuntimeEventPayload) -> String {
    let mut value = String::from(constants::browser::REQUEST_BROWSER_ACTION_INTENT_STATUS_PREFIX);
    value.push_str(&payload.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value
}
