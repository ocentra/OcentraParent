use serde::{Deserialize, Serialize};

use super::super::constants::{
    PARENT_STORAGE_APPLY_STATE_APPLIED, PARENT_STORAGE_APPLY_STATE_APPLY_PENDING,
    PARENT_STORAGE_APPLY_STATE_APPLY_REQUIRES_CONFIRMATION,
    PARENT_STORAGE_APPLY_STATE_BLOCKED_MANUAL_REQUIRED, PARENT_STORAGE_APPLY_STATE_NOT_STARTED,
    PARENT_STORAGE_APPLY_STATE_PARTIAL, PARENT_STORAGE_APPLY_STATE_ROLLBACK_MANUAL_REQUIRED,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageApplyState {
    #[serde(rename = "notStarted")]
    NotStarted,
    #[serde(rename = "applyRequiresConfirmation")]
    ApplyRequiresConfirmation,
    #[serde(rename = "applyPending")]
    ApplyPending,
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "rollbackManualRequired")]
    RollbackManualRequired,
    #[serde(rename = "blockedManualRequired")]
    BlockedManualRequired,
}

impl ParentStorageApplyState {
    pub fn as_str(&self) -> &'static str {
        const VALUES: &[&str] = &[
            PARENT_STORAGE_APPLY_STATE_NOT_STARTED,
            PARENT_STORAGE_APPLY_STATE_APPLY_REQUIRES_CONFIRMATION,
            PARENT_STORAGE_APPLY_STATE_APPLY_PENDING,
            PARENT_STORAGE_APPLY_STATE_APPLIED,
            PARENT_STORAGE_APPLY_STATE_PARTIAL,
            PARENT_STORAGE_APPLY_STATE_ROLLBACK_MANUAL_REQUIRED,
            PARENT_STORAGE_APPLY_STATE_BLOCKED_MANUAL_REQUIRED,
        ];
        VALUES[*self as usize]
    }
}
