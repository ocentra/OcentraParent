use std::time::Duration;

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::DomainEvent,
    envelope::EventContract, error::EventingError, ids::AggregateKey, ids::EventType,
    ids::IdempotencyKey, ids::RequestId, ids::SchemaVersion, ids::SubscriberId, ids::TargetHandler,
    request::RequestEvent, request::RequestOptions,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkInterventionState;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase;
use serde::{Deserialize, Serialize};

use crate::NetworkObservation;

use super::{network_aggregate_key, network_event_metadata, NetworkRuntimeEventPayload};

pub type NetworkRuntimeReviewReport =
    ocentra_parent_agent_protocol::network_flow::review::NetworkRuntimeReviewReport;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct NetworkRuntimeReviewRequest {
    payload: NetworkRuntimeEventPayload,
    request_id: RequestId,
}

impl DomainEvent for NetworkRuntimeReviewRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::network_flow::EVENT_NETWORK_REVIEW_REQUESTED)?,
            SchemaVersion::new(constants::network_flow::REVIEW_EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(network_aggregate_key(&self.payload))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::network_flow::IDEMPOTENCY_NETWORK_REVIEW_PREFIX);
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for NetworkRuntimeReviewRequest {
    type Response = NetworkRuntimeReviewResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

pub type NetworkRuntimeReviewResponse =
    ocentra_parent_agent_protocol::network_flow::review::NetworkRuntimeReviewResponse;

fn network_runtime_review_response_from_payload(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkRuntimeReviewResponse {
    NetworkRuntimeReviewResponse {
        evidence_grade: payload.evidence_grade,
        risk_budget_state: payload.risk_budget_state,
        intervention_state: payload.intervention_state,
        review_required: payload.intervention_state != NetworkInterventionState::DryRunOnly,
        claim_boundary: payload.claim_boundary,
    }
}

pub async fn request_network_runtime_review_for_observation(
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<NetworkRuntimeReviewReport, EventingError> {
    let bus = EventBus::new();
    bus.subscribe::<NetworkRuntimeReviewRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::network_flow::SUBSCRIBER_NETWORK_REVIEW)?,
            EventType::parse(constants::network_flow::EVENT_NETWORK_REVIEW_REQUESTED)?,
            TargetHandler::parse(constants::network_flow::TARGET_NETWORK_REVIEW)?,
        ),
        |context| async move {
            context
                .complete_request(network_runtime_review_response_from_payload(
                    &context.payload().payload,
                ))
                .await?;
            Ok(())
        },
    )
    .await?;

    let phase = NetworkRuntimePhase::PolicyEvaluationRequested;
    let decision = super::network_runtime_decision_from_observation(&observation);
    let payload = super::network_runtime_event_payload_from_observation(
        phase,
        &observation,
        observed_at,
        decision,
    );
    let request = NetworkRuntimeReviewRequest {
        request_id: RequestId::parse(network_review_request_id(&payload))?,
        payload,
    };
    let metadata = network_event_metadata(
        phase,
        &observation,
        observed_at,
        constants::network_flow::TARGET_NETWORK_REVIEW,
    )?;
    let request_report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::network_flow::REQUEST_NETWORK_REVIEW_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(NetworkRuntimeReviewReport {
        request_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}

fn network_review_request_id(payload: &NetworkRuntimeEventPayload) -> String {
    let mut value = String::from(constants::network_flow::REQUEST_NETWORK_REVIEW_PREFIX);
    value.push_str(&network_aggregate_key(payload));
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value
}
