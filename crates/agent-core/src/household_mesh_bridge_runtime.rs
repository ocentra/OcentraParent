use ocentra_eventing::bus::subscriber::EventSubscriber;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::bus::reports::{DeadLetter, PublishReport};
use ocentra_eventing::envelope::{DomainEvent, EventContract, StoredEventEnvelope};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventType, IdempotencyKey, SchemaVersion, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::{
    household_mesh_bridge_runtime_phase::HouseholdMeshBridgePhase,
    household_mesh_bridge_runtime_refs::{
        bridge_aggregate_key, bridge_event_state, bridge_message_type_for_local_event,
        previous_bridge_phase_ref,
    },
    household_mesh_bridge_runtime_source::bridge_event_metadata,
    household_mesh_bridge_runtime_state::{
        HouseholdMeshBridgeCustody, HouseholdMeshBridgeDirection, HouseholdMeshBridgeEnvelopeState,
        HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeValidationState,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HouseholdMeshBridgeInput {
    pub correlation_id: String,
    pub local_event_type: String,
    pub outbound_message_id: String,
    pub inbound_message_id: String,
    pub child_agent_peer_id: String,
    pub provider_peer_id: String,
    pub payload_ref: String,
    pub observed_at: String,
}

impl HouseholdMeshBridgeInput {
    pub fn proof_fixture() -> Self {
        Self {
            correlation_id: constants::household_mesh::TEST_BRIDGE_CORRELATION_ID.to_string(),
            local_event_type: constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED.to_string(),
            outbound_message_id: constants::household_mesh::TEST_BRIDGE_OUTBOUND_MESSAGE_ID
                .to_string(),
            inbound_message_id: constants::household_mesh::TEST_BRIDGE_INBOUND_MESSAGE_ID
                .to_string(),
            child_agent_peer_id: constants::household_mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID
                .to_string(),
            provider_peer_id: constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string(),
            payload_ref: constants::household_mesh::TEST_BRIDGE_PAYLOAD_REF.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HouseholdMeshBridgeExportCandidate {
    pub local_event_type: String,
    pub contains_raw_screenshot: bool,
    pub private_local_event: bool,
}

impl HouseholdMeshBridgeExportCandidate {
    pub fn selected_offer() -> Self {
        Self {
            local_event_type: constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED.to_string(),
            contains_raw_screenshot: false,
            private_local_event: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HouseholdMeshBridgeInboundEnvelope {
    pub lan_message_type: String,
    pub authenticated: bool,
    pub authorized: bool,
    pub direct_remote_publish_attempted: bool,
    pub contains_raw_screenshot: bool,
}

impl HouseholdMeshBridgeInboundEnvelope {
    pub fn accepted_offer() -> Self {
        Self {
            lan_message_type: constants::household_mesh::MESSAGE_AI_WORK_OFFER.to_string(),
            authenticated: true,
            authorized: true,
            direct_remote_publish_attempted: false,
            contains_raw_screenshot: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdMeshBridgeValidation {
    pub state: HouseholdMeshBridgeValidationState,
    pub rejection_reason: Option<HouseholdMeshBridgeRejectionReason>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HouseholdMeshBridgeEventPayload {
    pub phase: HouseholdMeshBridgePhase,
    pub envelope_state: HouseholdMeshBridgeEnvelopeState,
    pub direction: HouseholdMeshBridgeDirection,
    pub local_event_type: String,
    pub lan_message_type: String,
    pub outbound_message_id: String,
    pub inbound_message_id: String,
    pub child_agent_peer_id: String,
    pub provider_peer_id: String,
    pub payload_ref: String,
    pub previous_phase_ref: Option<String>,
    pub validation_state: HouseholdMeshBridgeValidationState,
    pub rejection_reason: Option<HouseholdMeshBridgeRejectionReason>,
    pub custody: HouseholdMeshBridgeCustody,
    pub observed_at: String,
}

impl HouseholdMeshBridgeEventPayload {
    fn from_input(phase: HouseholdMeshBridgePhase, input: &HouseholdMeshBridgeInput) -> Self {
        let lan_message_type = bridge_message_type_for_local_event(&input.local_event_type)
            .unwrap_or(constants::household_mesh::MESSAGE_AI_WORK_OFFER);
        Self {
            phase,
            envelope_state: bridge_event_state(phase),
            direction: bridge_direction_for_phase(phase),
            local_event_type: input.local_event_type.clone(),
            lan_message_type: lan_message_type.to_string(),
            outbound_message_id: input.outbound_message_id.clone(),
            inbound_message_id: input.inbound_message_id.clone(),
            child_agent_peer_id: input.child_agent_peer_id.clone(),
            provider_peer_id: input.provider_peer_id.clone(),
            payload_ref: input.payload_ref.clone(),
            previous_phase_ref: previous_bridge_phase_ref(phase),
            validation_state: HouseholdMeshBridgeValidationState::Accepted,
            rejection_reason: None,
            custody: HouseholdMeshBridgeCustody::selected_bridge_only(),
            observed_at: input.observed_at.clone(),
        }
    }
}

impl DomainEvent for HouseholdMeshBridgeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::household_mesh::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(bridge_aggregate_key(&self.outbound_message_id))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::household_mesh::IDEMPOTENCY_HOUSEHOLD_MESH_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.outbound_message_id);
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

#[derive(Clone, Debug)]
pub struct HouseholdMeshBridgeReport {
    pub publish_reports: Vec<PublishReport>,
    pub stored_events: Vec<StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

impl HouseholdMeshBridgeReport {
    pub fn violates_bridge_custody(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<HouseholdMeshBridgeEventPayload>()
                .map(|envelope| {
                    envelope.payload.custody.remote_direct_publish_allowed
                        || envelope.payload.custody.raw_screenshot_transferred
                        || envelope.payload.custody.private_local_event_exported
                })
                .unwrap_or(true)
        })
    }
}

pub async fn publish_household_mesh_bridge_chain_for_input(
    input: HouseholdMeshBridgeInput,
) -> Result<HouseholdMeshBridgeReport, EventingError> {
    let spine = HouseholdMeshBridgeSpine::with_default_handlers().await?;
    spine.publish_input_chain(input).await
}

pub fn validate_household_mesh_bridge_export(
    candidate: &HouseholdMeshBridgeExportCandidate,
) -> HouseholdMeshBridgeValidation {
    let rejection_reason = if candidate.private_local_event {
        Some(HouseholdMeshBridgeRejectionReason::PrivateLocalEvent)
    } else if candidate.contains_raw_screenshot {
        Some(HouseholdMeshBridgeRejectionReason::RawScreenPayload)
    } else if bridge_message_type_for_local_event(&candidate.local_event_type).is_none() {
        Some(HouseholdMeshBridgeRejectionReason::UnselectedEvent)
    } else {
        None
    };
    bridge_validation_for_rejection(rejection_reason)
}

pub fn validate_household_mesh_bridge_import(
    envelope: &HouseholdMeshBridgeInboundEnvelope,
) -> HouseholdMeshBridgeValidation {
    let rejection_reason = if envelope.direct_remote_publish_attempted {
        Some(HouseholdMeshBridgeRejectionReason::DirectRemotePublish)
    } else if !envelope.authenticated {
        Some(HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer)
    } else if !envelope.authorized {
        Some(HouseholdMeshBridgeRejectionReason::UnauthorizedPeer)
    } else if envelope.contains_raw_screenshot {
        Some(HouseholdMeshBridgeRejectionReason::RawScreenPayload)
    } else if envelope.lan_message_type != constants::household_mesh::MESSAGE_AI_WORK_OFFER
        && envelope.lan_message_type != constants::household_mesh::MESSAGE_AI_WORK_RESULT
    {
        Some(HouseholdMeshBridgeRejectionReason::UnsupportedLanMessage)
    } else {
        None
    };
    bridge_validation_for_rejection(rejection_reason)
}

struct HouseholdMeshBridgeSpine {
    bus: EventBus,
}

impl HouseholdMeshBridgeSpine {
    async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        for phase in HouseholdMeshBridgePhase::ordered_chain() {
            bus.subscribe::<HouseholdMeshBridgeEventPayload, _, _>(
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
        input: HouseholdMeshBridgeInput,
    ) -> Result<HouseholdMeshBridgeReport, EventingError> {
        let mut reports = Vec::new();
        for phase in HouseholdMeshBridgePhase::ordered_chain() {
            reports.push(
                self.bus
                    .publish(
                        HouseholdMeshBridgeEventPayload::from_input(*phase, &input),
                        bridge_event_metadata(*phase, &input)?,
                    )
                    .await?,
            );
        }
        Ok(HouseholdMeshBridgeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }
}

fn bridge_validation_for_rejection(
    rejection_reason: Option<HouseholdMeshBridgeRejectionReason>,
) -> HouseholdMeshBridgeValidation {
    HouseholdMeshBridgeValidation {
        state: if rejection_reason.is_none() {
            HouseholdMeshBridgeValidationState::Accepted
        } else {
            HouseholdMeshBridgeValidationState::Rejected
        },
        rejection_reason,
    }
}

fn bridge_direction_for_phase(phase: HouseholdMeshBridgePhase) -> HouseholdMeshBridgeDirection {
    match phase {
        HouseholdMeshBridgePhase::LocalEventSelected
        | HouseholdMeshBridgePhase::LanMessageExported => HouseholdMeshBridgeDirection::Export,
        HouseholdMeshBridgePhase::LanMessageReceived
        | HouseholdMeshBridgePhase::LocalEventRepublished => HouseholdMeshBridgeDirection::Import,
    }
}
