use crate::constants;
use crate::household_mesh::{
    HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeState, HouseholdMeshTransportEnvelope,
};

pub(super) fn rejection_reason(
    message: &HouseholdMeshTransportEnvelope,
) -> Option<HouseholdMeshBridgeRejectionReason> {
    if message.schema_version != constants::household_mesh::EVENT_SCHEMA_VERSION {
        return Some(HouseholdMeshBridgeRejectionReason::UnsupportedLanMessage);
    }
    if message.message_id.is_empty()
        || message.idempotency_key.is_empty()
        || message.local_event_ref.is_empty()
        || message.source_peer_id.is_empty()
    {
        return Some(HouseholdMeshBridgeRejectionReason::MismatchedMessageRef);
    }
    if message.bridge_state != HouseholdMeshBridgeState::ExportSelected {
        return Some(HouseholdMeshBridgeRejectionReason::MismatchedMessageRef);
    }
    None
}
