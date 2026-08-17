use ocentra_storage_custody_core::storage_custody_effect_store::StorageCustodyEffectStatus;

use super::{ChildStorageCustodyReadiness, ChildStorageCustodyRuntime};
use crate::service::ChildAgentServiceError;

impl ChildStorageCustodyRuntime {
    pub(crate) fn readiness(&self) -> ChildStorageCustodyReadiness {
        let Ok(records) = self.effects.records() else {
            return ChildStorageCustodyReadiness::ManualRequired;
        };
        let pending_operation_refs = records
            .iter()
            .filter(|record| {
                matches!(
                    record.status,
                    StorageCustodyEffectStatus::Prepared
                        | StorageCustodyEffectStatus::Journaled
                        | StorageCustodyEffectStatus::Applying
                )
            })
            .map(|record| record.operation_ref.clone())
            .collect::<Vec<_>>();
        if !pending_operation_refs.is_empty() {
            return ChildStorageCustodyReadiness::PendingRecovery {
                operation_refs: pending_operation_refs,
            };
        }
        let manual_operation_refs = records
            .iter()
            .filter(|record| record.status == StorageCustodyEffectStatus::ManualRequired)
            .map(|record| record.operation_ref.clone())
            .collect::<Vec<_>>();
        if !manual_operation_refs.is_empty() {
            return ChildStorageCustodyReadiness::ManualRecoveryRequired {
                operation_refs: manual_operation_refs,
            };
        }
        if self.authority.has_current_binding() {
            ChildStorageCustodyReadiness::CurrentAuthority
        } else {
            ChildStorageCustodyReadiness::ManualRequired
        }
    }

    pub(crate) fn ensure_action_dispatchable(&self) -> Result<(), ChildAgentServiceError> {
        match self.readiness() {
            ChildStorageCustodyReadiness::CurrentAuthority => Ok(()),
            ChildStorageCustodyReadiness::PendingRecovery { operation_refs } => {
                Err(ChildAgentServiceError::Configuration(format!(
                    "storage custody recovery is pending for {:?}",
                    operation_refs
                )))
            }
            ChildStorageCustodyReadiness::ManualRequired
            | ChildStorageCustodyReadiness::ManualRecoveryRequired { .. } => {
                Err(ChildAgentServiceError::Configuration(
                    "storage custody requires manual authority or recovery".to_owned(),
                ))
            }
        }
    }
}
