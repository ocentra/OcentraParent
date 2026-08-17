//! Child-service owner for the production custody source-to-effect chain.
//!
//! The service owns event publication, the durable effect ledger, replay, and
//! the only local delete executor. Account/provider/platform owners supply a
//! non-serializable current authority implementation; when they are absent,
//! every effect is recorded as manual-required and no local mutation occurs.

use std::{path::Path, sync::Arc};

use ocentra_storage_custody_core::storage_custody::{
    StorageCustodyEffectKind, StorageCustodyInput,
};

use super::child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow;

#[path = "storage_custody_runtime_authority.rs"]
mod storage_custody_runtime_authority;
#[path = "storage_custody_runtime_authority_reasons.rs"]
mod storage_custody_runtime_authority_reasons;
#[path = "storage_custody_runtime_delete.rs"]
mod storage_custody_runtime_delete;
#[path = "storage_custody_runtime_effect.rs"]
mod storage_custody_runtime_effect;
#[path = "storage_custody_runtime_existing.rs"]
mod storage_custody_runtime_existing;
#[path = "storage_custody_runtime_idempotency.rs"]
mod storage_custody_runtime_idempotency;
#[path = "storage_custody_runtime_lifecycle.rs"]
mod storage_custody_runtime_lifecycle;
#[path = "storage_custody_runtime_manual.rs"]
mod storage_custody_runtime_manual;
#[path = "storage_custody_runtime_readiness.rs"]
mod storage_custody_runtime_readiness;
#[path = "storage_custody_runtime_reasons.rs"]
mod storage_custody_runtime_reasons;
#[path = "storage_custody_runtime_reconciliation.rs"]
mod storage_custody_runtime_reconciliation;
#[path = "storage_custody_runtime_recovery.rs"]
mod storage_custody_runtime_recovery;
#[path = "storage_custody_runtime_replay.rs"]
mod storage_custody_runtime_replay;
#[path = "storage_custody_runtime_terminal.rs"]
mod storage_custody_runtime_terminal;
#[path = "storage_custody_runtime_validation.rs"]
mod storage_custody_runtime_validation;

/// Only the child-runtime composition owner may implement this source. The
/// account owner resolves a verified provider subject and durable current
/// member/device state before an adapter is added here; command callers never
/// receive this trait or supply a selector.
pub(crate) trait ChildStorageCustodyAuthority: Send + Sync {
    fn household_id(&self) -> &str;
    fn child_profile_id(&self) -> &str;
    fn target_device_id(&self) -> &str;
    fn authority_generation(&self) -> u64;
    fn session_generation(&self) -> u64;
    fn is_current(&self) -> bool;
    fn allows(&self, effect: StorageCustodyEffectKind) -> bool;
    fn custody_input(&self, effect: StorageCustodyEffectKind) -> Option<StorageCustodyInput>;
    fn allows_local_payload(&self, relative_path: &Path) -> bool;
}

/// Non-serializable current-authority handoff retained by the child service.
/// Its constructor is crate-private so JSON/TS callers cannot mint one.
#[derive(Clone)]
pub struct ChildStorageCustodyAuthorityHandle {
    source: Arc<dyn ChildStorageCustodyAuthority>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChildStorageCustodyAuthorityError {
    InvalidBinding,
    InvalidGeneration,
    StaleOrRevoked,
    EffectNotGranted,
}

#[derive(Debug)]
struct ManualRequiredChildStorageCustodyAuthority;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChildStorageCustodyOutcome {
    Applied {
        operation_ref: String,
        effect: StorageCustodyEffectKind,
    },
    AlreadyApplied {
        operation_ref: String,
        effect: StorageCustodyEffectKind,
    },
    PendingJournalRetry {
        operation_ref: String,
        effect: StorageCustodyEffectKind,
    },
    PendingRecovery {
        operation_ref: String,
        effect: StorageCustodyEffectKind,
    },
    ManualRequired {
        operation_ref: String,
        effect: StorageCustodyEffectKind,
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChildStorageCustodyReadiness {
    CurrentAuthority,
    PendingRecovery { operation_refs: Vec<String> },
    ManualRequired,
    ManualRecoveryRequired { operation_refs: Vec<String> },
}

#[derive(Clone)]
pub(super) struct ChildStorageCustodyRuntime {
    root: std::path::PathBuf,
    flow: ChildRuntimeTombstoneEventFlow,
    effects: ocentra_storage_custody_core::storage_custody_effect_store::StorageCustodyEffectStore,
    authority: ChildStorageCustodyAuthorityHandle,
    apply_lease_owner: String,
}
