use crate::household_mesh::{HouseholdMeshBridgeRejectionReason, HouseholdMeshTransportEnvelope};

use super::super::HouseholdMeshBridgeInboundEnvelope;

pub(super) fn rejection_reason(
    message: &HouseholdMeshTransportEnvelope,
    input: &HouseholdMeshBridgeInboundEnvelope,
) -> Option<HouseholdMeshBridgeRejectionReason> {
    if message.family_id != input.expected_family_id {
        return Some(HouseholdMeshBridgeRejectionReason::FamilyMismatch);
    }
    if message.target_child_device_id != input.expected_target_child_device_id {
        return Some(HouseholdMeshBridgeRejectionReason::WrongTargetDevice);
    }
    if message.is_stale_at(input.received_at_epoch_seconds) {
        return Some(HouseholdMeshBridgeRejectionReason::StaleMessage);
    }
    if input
        .seen_message_ids
        .iter()
        .any(|seen| seen == &message.message_id)
        || input
            .seen_idempotency_keys
            .iter()
            .any(|seen| seen == &message.idempotency_key)
    {
        return Some(HouseholdMeshBridgeRejectionReason::ReplayedMessage);
    }
    None
}
