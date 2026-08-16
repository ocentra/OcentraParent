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

pub type BrowserRuntimeSocialProviderReceiptStatusReport =
    ocentra_parent_agent_protocol::browser::social_provider_receipt::BrowserRuntimeSocialProviderReceiptStatusReport;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BrowserRuntimeSocialProviderReceiptStatusRequest {
    payload: BrowserRuntimeEventPayload,
    request_id: RequestId,
}

impl DomainEvent for BrowserRuntimeSocialProviderReceiptStatusRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED,
            )?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(browser_aggregate_key(&self.payload.source_ref))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(
            constants::browser::IDEMPOTENCY_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_PREFIX,
        );
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for BrowserRuntimeSocialProviderReceiptStatusRequest {
    type Response = BrowserRuntimeSocialProviderReceiptStatusResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

pub type BrowserRuntimeSocialProviderReceiptStatusResponse =
    ocentra_parent_agent_protocol::browser::social_provider_receipt::BrowserRuntimeSocialProviderReceiptStatusResponse;

fn social_provider_receipt_status_response_from_payload(
    payload: &BrowserRuntimeEventPayload,
) -> BrowserRuntimeSocialProviderReceiptStatusResponse {
    let provider_dispatch_required = social_provider_dispatch_required(payload);
    BrowserRuntimeSocialProviderReceiptStatusResponse {
        receipt_boundary_row_count: 1,
        provider_dispatch_required_count: usize::from(provider_dispatch_required),
        manual_receipt_required_count: usize::from(!provider_dispatch_required),
        provider_attempt_ref: provider_dispatch_required.then(|| {
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REF.to_string()
        }),
        provider_receipt_proof_ref: Some(
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REF.to_string(),
        ),
        source_ref: payload.source_ref.clone(),
        evidence_ref: payload.evidence_ref.clone(),
        action_intent_id: provider_dispatch_required
            .then(|| payload.action_intent_id.clone())
            .flatten(),
        receipt_boundary_state: receipt_boundary_state(provider_dispatch_required).to_string(),
        receipt_runtime_state:
            constants::browser::SOCIAL_PROVIDER_RECEIPT_RUNTIME_STATE_MANUAL_REQUIRED.to_string(),
        provider_receipt_count: 0,
        provider_dispatch_count: 0,
        provider_webhook_count: 0,
        provider_credentials_count: 0,
        parent_notification_ui_delivery_count: 0,
        report_delivery_execution_count: 0,
        final_policy_execution_count: 0,
        connector_native_runtime_count: 0,
        enforcement_execution_count: 0,
    }
}

pub async fn request_browser_runtime_social_provider_receipt_status_for_input(
    input: BrowserRuntimeInput,
) -> Result<BrowserRuntimeSocialProviderReceiptStatusReport, EventingError> {
    let bus = EventBus::new();
    bus.subscribe::<BrowserRuntimeSocialProviderReceiptStatusRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
            )?,
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED,
            )?,
            TargetHandler::parse(
                constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
            )?,
        ),
        |context| async move {
            context
                .complete_request(social_provider_receipt_status_response_from_payload(
                    &context.payload().payload,
                ))
                .await?;
            Ok(())
        },
    )
    .await?;

    let phase = BrowserRuntimePhase::PolicyDecisionCompleted;
    let payload = super::browser_runtime_event_payload_from_input(phase, &input);
    let request = BrowserRuntimeSocialProviderReceiptStatusRequest {
        request_id: RequestId::parse(social_provider_receipt_status_request_id(&payload))?,
        payload,
    };
    let metadata = browser_event_metadata(
        phase,
        &input,
        constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
    )?;
    let request_report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(BrowserRuntimeSocialProviderReceiptStatusReport {
        request_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}

pub fn browser_runtime_social_provider_receipt_status_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    let payload = super::browser_runtime_event_payload_from_input(
        BrowserRuntimePhase::PolicyDecisionCompleted,
        &BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    );
    let request = BrowserRuntimeSocialProviderReceiptStatusRequest {
        request_id: RequestId::parse(social_provider_receipt_status_request_id(&payload))?,
        payload,
    };
    let mut registry = EventContractRegistry::new();
    registry.register_event(&request)?;
    Ok(EventTopologyManifest::from_registry(
        &registry,
        &[EventTopologyPublisher {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED,
            )?,
            source_component: SourceComponent::parse(
                constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
            )?,
        }],
        &[EventTopologySubscriber {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED,
            )?,
            subscriber_id: SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
            )?,
            target_handler: TargetHandler::parse(
                constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
            )?,
        }],
        &[],
        &[],
    ))
}

fn social_provider_dispatch_required(payload: &BrowserRuntimeEventPayload) -> bool {
    payload.phase == BrowserRuntimePhase::PolicyDecisionCompleted
        && payload.dry_run
        && payload.policy_authority
        && payload.action_intent_id.is_some()
}

fn receipt_boundary_state(provider_dispatch_required: bool) -> &'static str {
    if provider_dispatch_required {
        constants::browser::SOCIAL_PROVIDER_RECEIPT_STATE_PROVIDER_DISPATCH_REQUIRED
    } else {
        constants::browser::SOCIAL_PROVIDER_RECEIPT_STATE_MANUAL_REQUIRED
    }
}

fn social_provider_receipt_status_request_id(payload: &BrowserRuntimeEventPayload) -> String {
    let mut value =
        String::from(constants::browser::REQUEST_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_PREFIX);
    value.push_str(&payload.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value
}
