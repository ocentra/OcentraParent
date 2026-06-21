use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, error::EventingError, ids::EventType,
    ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshBridgePhase, HouseholdMeshBridgeState,
};

use crate::{
    household_mesh_bridge_runtime_refs::{
        bridge_event_state, bridge_local_event_kind_for_local_event,
        bridge_message_type_for_local_event, previous_bridge_phase_ref,
    },
    household_mesh_bridge_runtime_source::bridge_event_metadata,
    household_mesh_bridge_runtime_state::{
        HouseholdMeshBridgeCustody, HouseholdMeshBridgeDirection,
        HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeValidationState,
    },
    household_mesh_event_bridge::{
        export_selected_local_event, validate_incoming_lan_message,
        HouseholdMeshAuthenticationState, HouseholdMeshBridgeRejection,
        HouseholdMeshExportDecision, HouseholdMeshImportDecision, HouseholdMeshLanMessage,
        HouseholdMeshPolicyAuthority,
    },
};

pub(crate) type HouseholdMeshBridgeValidation =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeValidation;
pub(crate) type HouseholdMeshBridgeEventPayload =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeEventPayload;
pub(crate) type HouseholdMeshBridgeInput =
    ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::HouseholdMeshBridgeInput;
pub(crate) type HouseholdMeshBridgeExportCandidate =
    ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::HouseholdMeshBridgeExportCandidate;
pub(crate) type HouseholdMeshBridgeInboundEnvelope =
    ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::HouseholdMeshBridgeInboundEnvelope;

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

fn household_mesh_bridge_event_payload_from_input(
    phase: HouseholdMeshBridgePhase,
    input: &HouseholdMeshBridgeInput,
) -> HouseholdMeshBridgeEventPayload {
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
    HouseholdMeshBridgeEventPayload {
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
                        household_mesh_bridge_event_payload_from_input(*phase, &input),
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
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
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
