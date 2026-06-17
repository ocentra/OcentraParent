use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::DomainEvent,
    envelope::EventContract, error::EventingError, ids::AggregateKey, ids::EventType,
    ids::IdempotencyKey, ids::SchemaVersion, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeState;
use serde::{Deserialize, Serialize};

use crate::{
    export_selected_local_event,
    household_mesh_bridge_runtime_phase::HouseholdMeshBridgePhase,
    household_mesh_bridge_runtime_refs::{
        bridge_aggregate_key, bridge_event_state, bridge_local_event_kind_for_local_event,
        bridge_message_type_for_local_event, previous_bridge_phase_ref,
    },
    household_mesh_bridge_runtime_source::bridge_event_metadata,
    household_mesh_bridge_runtime_state::{
        HouseholdMeshBridgeCustody, HouseholdMeshBridgeDirection, HouseholdMeshBridgeEnvelopeState,
        HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeValidationState,
    },
    validate_incoming_lan_message, HouseholdMeshBridgeRejection, HouseholdMeshExportDecision,
    HouseholdMeshImportDecision, HouseholdMeshLanMessage,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HouseholdMeshBridgeInput {
    pub correlation_id: String,
    pub local_event_type: String,
    pub family_id: String,
    pub target_child_device_id: String,
    pub outbound_message_id: String,
    pub outbound_idempotency_key: String,
    pub child_agent_peer_id: String,
    pub provider_peer_id: String,
    pub payload_ref: String,
    pub observed_at: String,
    pub received_at_epoch_seconds: u64,
    pub inbound_message: HouseholdMeshLanMessage,
    pub seen_message_ids: Vec<String>,
    pub seen_idempotency_keys: Vec<String>,
}

impl HouseholdMeshBridgeInput {
    pub fn proof_fixture() -> Self {
        let mut inbound_message = HouseholdMeshLanMessage::proof_fixture_for(
            constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN,
            constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        );
        inbound_message.source_peer_id =
            constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string();
        Self {
            correlation_id: constants::household_mesh::TEST_BRIDGE_CORRELATION_ID.to_string(),
            local_event_type: constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED.to_string(),
            family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
            target_child_device_id: constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
                .to_string(),
            outbound_message_id: constants::household_mesh::TEST_BRIDGE_OUTBOUND_MESSAGE_ID
                .to_string(),
            outbound_idempotency_key: constants::household_mesh::TEST_BRIDGE_IDEMPOTENCY_KEY
                .to_string(),
            child_agent_peer_id: constants::household_mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID
                .to_string(),
            provider_peer_id: constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string(),
            payload_ref: constants::household_mesh::TEST_BRIDGE_PAYLOAD_REF.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            received_at_epoch_seconds:
                constants::household_mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            inbound_message,
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
        }
    }

    fn inbound_envelope(&self) -> HouseholdMeshBridgeInboundEnvelope {
        HouseholdMeshBridgeInboundEnvelope {
            message: self.inbound_message.clone(),
            expected_family_id: self.family_id.clone(),
            expected_target_child_device_id: self.target_child_device_id.clone(),
            received_at_epoch_seconds: self.received_at_epoch_seconds,
            authorized: true,
            seen_message_ids: self.seen_message_ids.clone(),
            seen_idempotency_keys: self.seen_idempotency_keys.clone(),
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
    pub message: HouseholdMeshLanMessage,
    pub expected_family_id: String,
    pub expected_target_child_device_id: String,
    pub received_at_epoch_seconds: u64,
    pub authorized: bool,
    pub seen_message_ids: Vec<String>,
    pub seen_idempotency_keys: Vec<String>,
}

impl HouseholdMeshBridgeInboundEnvelope {
    pub fn accepted_offer() -> Self {
        let mut message = HouseholdMeshLanMessage::proof_fixture_for(
            constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER,
            constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER,
        );
        message.source_peer_id =
            constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string();
        Self {
            message,
            expected_family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
            expected_target_child_device_id:
                constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
            received_at_epoch_seconds:
                constants::household_mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            authorized: true,
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
        }
    }

    pub fn accepted_result() -> Self {
        let mut message = HouseholdMeshLanMessage::proof_fixture_for(
            constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN,
            constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        );
        message.source_peer_id =
            constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string();
        Self {
            message,
            expected_family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
            expected_target_child_device_id:
                constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
            received_at_epoch_seconds:
                constants::household_mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            authorized: true,
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
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
    pub local_event_ref: String,
    pub lan_message_type: String,
    pub family_id: String,
    pub target_child_device_id: String,
    pub source_peer_id: String,
    pub idempotency_key: String,
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
        let export_decision = bridge_export_decision_for_input(input);
        let bridge_message = bridge_message_for_phase(phase, input, &export_decision);
        let import_validation = matches!(
            phase,
            HouseholdMeshBridgePhase::LanMessageReceived
                | HouseholdMeshBridgePhase::LocalEventRepublished
        )
        .then(|| validate_household_mesh_bridge_import(&input.inbound_envelope()));
        let export_validation = match export_decision {
            HouseholdMeshExportDecision::Export(_) => HouseholdMeshBridgeValidation {
                state: HouseholdMeshBridgeValidationState::Accepted,
                rejection_reason: None,
            },
            HouseholdMeshExportDecision::Reject(rejection) => HouseholdMeshBridgeValidation {
                state: HouseholdMeshBridgeValidationState::Rejected,
                rejection_reason: Some(bridge_rejection_reason(rejection)),
            },
        };
        let validation = import_validation.unwrap_or(export_validation);
        Self {
            phase,
            envelope_state: bridge_event_state(phase),
            direction: bridge_direction_for_phase(phase),
            local_event_type: input.local_event_type.clone(),
            local_event_ref: bridge_message.local_event_ref.clone(),
            lan_message_type: bridge_message.lan_message_type.clone(),
            family_id: bridge_message.family_id.clone(),
            target_child_device_id: bridge_message.target_child_device_id.clone(),
            source_peer_id: bridge_message.source_peer_id.clone(),
            idempotency_key: bridge_message.idempotency_key.clone(),
            outbound_message_id: input.outbound_message_id.clone(),
            inbound_message_id: input.inbound_message.message_id.clone(),
            child_agent_peer_id: input.child_agent_peer_id.clone(),
            provider_peer_id: input.provider_peer_id.clone(),
            payload_ref: input.payload_ref.clone(),
            previous_phase_ref: previous_bridge_phase_ref(phase),
            validation_state: validation.state,
            rejection_reason: validation.rejection_reason,
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
    pub publish_reports: Vec<ocentra_eventing::bus::reports::PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
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
    let rejection_reason = if !envelope.authorized {
        Some(HouseholdMeshBridgeRejectionReason::UnauthorizedPeer)
    } else {
        let seen_message_ids = envelope
            .seen_message_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let seen_idempotency_keys = envelope
            .seen_idempotency_keys
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        match validate_incoming_lan_message(
            envelope.message.clone(),
            &envelope.expected_family_id,
            &envelope.expected_target_child_device_id,
            envelope.received_at_epoch_seconds,
            &seen_message_ids,
            &seen_idempotency_keys,
        ) {
            HouseholdMeshImportDecision::Republish(_) => None,
            HouseholdMeshImportDecision::Reject(rejection) => {
                Some(bridge_rejection_reason(rejection))
            }
        }
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

fn bridge_export_decision_for_input(
    input: &HouseholdMeshBridgeInput,
) -> HouseholdMeshExportDecision {
    let Some(local_event_kind) = bridge_local_event_kind_for_local_event(&input.local_event_type)
    else {
        return HouseholdMeshExportDecision::Reject(
            HouseholdMeshBridgeRejection::UnselectedLocalEvent,
        );
    };
    export_selected_local_event(
        local_event_kind,
        &input.family_id,
        &input.target_child_device_id,
        &input.child_agent_peer_id,
        &input.outbound_message_id,
        &input.outbound_idempotency_key,
        constants::household_mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
        constants::household_mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
    )
}

fn bridge_message_for_phase(
    phase: HouseholdMeshBridgePhase,
    input: &HouseholdMeshBridgeInput,
    export_decision: &HouseholdMeshExportDecision,
) -> HouseholdMeshLanMessage {
    match phase {
        HouseholdMeshBridgePhase::LocalEventSelected
        | HouseholdMeshBridgePhase::LanMessageExported => match export_decision {
            HouseholdMeshExportDecision::Export(message) => message.clone(),
            HouseholdMeshExportDecision::Reject(_) => fallback_export_message(input),
        },
        HouseholdMeshBridgePhase::LanMessageReceived
        | HouseholdMeshBridgePhase::LocalEventRepublished => input.inbound_message.clone(),
    }
}

fn fallback_export_message(input: &HouseholdMeshBridgeInput) -> HouseholdMeshLanMessage {
    let lan_message_type = bridge_message_type_for_local_event(&input.local_event_type)
        .unwrap_or(constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER);
    let local_event_ref = bridge_local_event_kind_for_local_event(&input.local_event_type)
        .map(crate::household_mesh_event_bridge::local_event_ref)
        .unwrap_or(constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER);
    HouseholdMeshLanMessage {
        schema_version: constants::household_mesh::EVENT_SCHEMA_VERSION,
        message_id: input.outbound_message_id.clone(),
        idempotency_key: input.outbound_idempotency_key.clone(),
        family_id: input.family_id.clone(),
        target_child_device_id: input.target_child_device_id.clone(),
        source_peer_id: input.child_agent_peer_id.clone(),
        local_event_ref: local_event_ref.to_string(),
        lan_message_type: lan_message_type.to_string(),
        bridge_state: HouseholdMeshBridgeState::ExportSelected,
        authentication_state: crate::HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: crate::HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
        sent_at_epoch_seconds: constants::household_mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
        stale_after_seconds: constants::household_mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
    }
}

fn bridge_rejection_reason(
    rejection: HouseholdMeshBridgeRejection,
) -> HouseholdMeshBridgeRejectionReason {
    match rejection {
        HouseholdMeshBridgeRejection::UnselectedLocalEvent => {
            HouseholdMeshBridgeRejectionReason::UnselectedEvent
        }
        HouseholdMeshBridgeRejection::UnauthenticatedMessage => {
            HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer
        }
        HouseholdMeshBridgeRejection::DirectRemotePublish => {
            HouseholdMeshBridgeRejectionReason::DirectRemotePublish
        }
        HouseholdMeshBridgeRejection::PolicyAuthorityEscalation => {
            HouseholdMeshBridgeRejectionReason::PolicyAuthorityEscalation
        }
        HouseholdMeshBridgeRejection::RawPayload => {
            HouseholdMeshBridgeRejectionReason::RawScreenPayload
        }
        HouseholdMeshBridgeRejection::MismatchedMessageRef => {
            HouseholdMeshBridgeRejectionReason::MismatchedMessageRef
        }
        HouseholdMeshBridgeRejection::ReplayedMessage => {
            HouseholdMeshBridgeRejectionReason::ReplayedMessage
        }
        HouseholdMeshBridgeRejection::StaleMessage => {
            HouseholdMeshBridgeRejectionReason::StaleMessage
        }
        HouseholdMeshBridgeRejection::FamilyMismatch => {
            HouseholdMeshBridgeRejectionReason::FamilyMismatch
        }
        HouseholdMeshBridgeRejection::WrongTargetDevice => {
            HouseholdMeshBridgeRejectionReason::WrongTargetDevice
        }
    }
}
