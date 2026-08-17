use ocentra_storage_custody_core::{
    storage_custody::{StorageCustodyEffect, StorageCustodyEffectKind},
    storage_custody_effect_store::{StorageCustodyEffectRecord, StorageCustodyEffectStatus},
};

use super::storage_custody_runtime_validation::invalid_custody;
use super::{ChildStorageCustodyOutcome, ChildStorageCustodyRuntime};
use crate::service::ChildAgentServiceError;

pub(crate) fn existing_outcome(
    runtime: &ChildStorageCustodyRuntime,
    operation_ref: &str,
    effect: StorageCustodyEffectKind,
    request: &StorageCustodyEffect,
) -> Result<Option<ChildStorageCustodyOutcome>, ChildAgentServiceError> {
    let Some(existing) = existing_record(runtime, operation_ref, effect, request)? else {
        return Ok(None);
    };
    match existing.status {
        StorageCustodyEffectStatus::Applied => {
            Ok(Some(ChildStorageCustodyOutcome::AlreadyApplied {
                operation_ref: operation_ref.to_owned(),
                effect,
            }))
        }
        StorageCustodyEffectStatus::ManualRequired => {
            Ok(Some(ChildStorageCustodyOutcome::ManualRequired {
                operation_ref: operation_ref.to_owned(),
                effect,
                reason: existing
                    .manual_required_reason
                    .unwrap_or_else(|| "custody effect requires manual handling".to_owned()),
            }))
        }
        StorageCustodyEffectStatus::Prepared
        | StorageCustodyEffectStatus::Journaled
        | StorageCustodyEffectStatus::Applying => Ok(None),
    }
}

pub(crate) fn existing_record(
    runtime: &ChildStorageCustodyRuntime,
    operation_ref: &str,
    effect: StorageCustodyEffectKind,
    request: &StorageCustodyEffect,
) -> Result<Option<StorageCustodyEffectRecord>, ChildAgentServiceError> {
    let Some(existing) = runtime
        .effects
        .records()
        .map_err(ChildAgentServiceError::Storage)?
        .into_iter()
        .find(|record| record.operation_ref == operation_ref)
    else {
        return Ok(None);
    };
    let requested_path = match request {
        StorageCustodyEffect::DeleteLocal { relative_path } => {
            Some(relative_path.display().to_string())
        }
        _ => None,
    };
    if existing.effect_kind != effect || existing.relative_path != requested_path {
        return Err(invalid_custody(
            "custody operation reference was reused with a different effect",
        ));
    }
    Ok(Some(existing))
}
