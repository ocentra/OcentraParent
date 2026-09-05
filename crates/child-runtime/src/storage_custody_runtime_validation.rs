use std::io;

use ocentra_eventing::envelope::DomainEvent;
use ocentra_storage_custody_core::storage_custody::{
    plan_storage_custody_actions, LocalPayloadRetentionAction, StorageCustodyActionPlannedEvent,
    StorageCustodyEffectKind, StorageCustodyLocation, StorageTombstoneState,
};

use super::storage_custody_effect_store::StorageCustodyEffectRecord;
use crate::service::ChildAgentServiceError;

pub(crate) fn validate_effect_location(
    effect: StorageCustodyEffectKind,
    location: StorageCustodyLocation,
) -> Result<(), ChildAgentServiceError> {
    if effect == StorageCustodyEffectKind::LocalDelete
        && location != StorageCustodyLocation::ChildDeviceLocal
    {
        return Err(invalid_custody(
            "child runtime local delete requires child-device-local custody",
        ));
    }
    Ok(())
}

pub(crate) fn coherent_local_delete(record: &StorageCustodyEffectRecord) -> bool {
    record.action.action_plan.local_payload_retention_action == LocalPayloadRetentionAction::Delete
        && record.action.action_plan.tombstone_state == StorageTombstoneState::Write
        && record
            .relative_path
            .as_deref()
            .is_some_and(|path| !path.trim().is_empty())
}

pub(crate) fn local_delete_action_is_allowed(action: &StorageCustodyActionPlannedEvent) -> bool {
    action.action_plan.local_payload_retention_action == LocalPayloadRetentionAction::Delete
        && action.action_plan.tombstone_state == StorageTombstoneState::Write
}

pub(crate) fn invalid_custody(message: &str) -> ChildAgentServiceError {
    ChildAgentServiceError::Storage(io::Error::new(
        io::ErrorKind::InvalidInput,
        message.to_owned(),
    ))
}

pub(crate) fn validate_effect_record_shape(
    record: &StorageCustodyEffectRecord,
) -> Result<(), ChildAgentServiceError> {
    if !base_record_shape_is_valid(record) || !record_path_shape_is_valid(record) {
        return Err(invalid_custody(
            "durable custody effect record shape is invalid",
        ));
    }
    if plan_storage_custody_actions(record.custody_input) != record.action.action_plan {
        return Err(invalid_custody(
            "durable custody action does not match its recorded decision input",
        ));
    }
    let decoded = record
        .envelope
        .decode::<StorageCustodyActionPlannedEvent>()
        .map_err(|error| {
            invalid_custody(&format!("durable custody envelope decode failed: {error}"))
        })?;
    let expected_aggregate_key = record
        .action
        .aggregate_key()
        .map_err(ChildAgentServiceError::Runtime)?;
    let expected_idempotency_key = record
        .action
        .idempotency_key()
        .map_err(ChildAgentServiceError::Runtime)?;
    if decoded.payload() != &record.action
        || decoded.aggregate_key() != &expected_aggregate_key
        || decoded.idempotency_key() != &expected_idempotency_key
    {
        return Err(invalid_custody(
            "durable custody effect envelope identity is invalid",
        ));
    }
    Ok(())
}

fn base_record_shape_is_valid(record: &StorageCustodyEffectRecord) -> bool {
    record.schema_version == 1
        && !record.operation_ref.trim().is_empty()
        && !record.effect_ref.trim().is_empty()
        && !record.household_id.trim().is_empty()
        && !record.child_profile_id.trim().is_empty()
        && !record.target_device_id.trim().is_empty()
        && record.authority_generation != 0
        && record.session_generation != 0
}

fn record_path_shape_is_valid(record: &StorageCustodyEffectRecord) -> bool {
    match record.effect_kind {
        StorageCustodyEffectKind::LocalDelete => {
            record.custody_input.location == StorageCustodyLocation::ChildDeviceLocal
                && record
                    .relative_path
                    .as_deref()
                    .is_some_and(|path| !path.trim().is_empty() && coherent_local_delete(record))
        }
        _ => record.relative_path.is_none(),
    }
}
