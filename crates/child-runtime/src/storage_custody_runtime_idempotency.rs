use ocentra_eventing::envelope::{EventMetadata, StoredEventEnvelope};
use ocentra_storage_custody_core::storage_custody::{
    StorageCustodyActionPlannedEvent, StorageCustodyEffect, StorageCustodyEffectKind,
};

use super::storage_custody_effect_store::{
    StorageCustodyEffectRecord, StorageCustodyEffectRecordPreparation, StorageCustodyEffectStatus,
};
use super::storage_custody_runtime_existing::existing_record;
use super::{ChildStorageCustodyAuthorityHandle, ChildStorageCustodyRuntime};
use crate::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
    runtime_gate_tombstone::ChildRuntimeTombstonePublicationOutcome,
    service::ChildAgentServiceError,
};

pub(crate) fn existing_pending(
    runtime: &ChildStorageCustodyRuntime,
    operation_ref: &str,
    effect: StorageCustodyEffectKind,
    request: &StorageCustodyEffect,
) -> Result<Option<StorageCustodyEffectRecord>, ChildAgentServiceError> {
    let Some(existing) = existing_record(runtime, operation_ref, effect, request)? else {
        return Ok(None);
    };
    Ok(matches!(
        existing.status(),
        StorageCustodyEffectStatus::Prepared
            | StorageCustodyEffectStatus::Journaled
            | StorageCustodyEffectStatus::Applying
    )
    .then_some(existing))
}

pub(crate) fn prepare_record(
    authority: &ChildStorageCustodyAuthorityHandle,
    operation_ref: String,
    effect: &StorageCustodyEffect,
    input: ocentra_storage_custody_core::storage_custody::StorageCustodyInput,
    action: StorageCustodyActionPlannedEvent,
    envelope: StoredEventEnvelope,
) -> StorageCustodyEffectRecord {
    let effect_kind = effect.kind();
    let relative_path = match effect {
        StorageCustodyEffect::DeleteLocal { relative_path } => {
            Some(relative_path.display().to_string())
        }
        _ => None,
    };
    StorageCustodyEffectRecord::prepared(StorageCustodyEffectRecordPreparation {
        operation_ref,
        effect_kind,
        effect_ref: effect.reference(),
        relative_path,
        household_id: authority.household_id().to_owned(),
        child_profile_id: authority.child_profile_id().to_owned(),
        target_device_id: authority.target_device_id().to_owned(),
        authority_generation: authority.authority_generation(),
        session_generation: authority.session_generation(),
        custody_input: input,
        action,
        envelope,
    })
}

pub(crate) async fn publish_action(
    flow: &ChildRuntimeTombstoneEventFlow,
    effect: StorageCustodyEffectKind,
    action: StorageCustodyActionPlannedEvent,
    metadata: EventMetadata,
) -> Result<bool, ChildAgentServiceError> {
    if effect == StorageCustodyEffectKind::LocalDelete {
        return match flow.publish_action(action, metadata).await {
            Ok(ChildRuntimeTombstonePublicationOutcome::Journaled(_)) => Ok(true),
            Ok(ChildRuntimeTombstonePublicationOutcome::PendingJournalRetry(_)) => Ok(false),
            Err(error) => Err(ChildAgentServiceError::Storage(error)),
        };
    }
    flow.publish_action_to_journal(action, metadata)
        .await
        .map(|_| true)
        .map_err(ChildAgentServiceError::Storage)
}
