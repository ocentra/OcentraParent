#![forbid(unsafe_code)]

//! Durable-adapter-neutral recovery lifecycle state.

use crate::family_identity::RecoveryId;
use crate::setup_lifecycle::{RecoveryKind, RecoveryOperation};
use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId,
};
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};

/// Recovery state held by the account adapter while an operation is proved
/// and approved. Completion never grants account or device authority; it only
/// makes the downstream custody/setup handoff explicit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLifecycleRecord {
    pub(crate) operation: RecoveryOperation,
    pub(crate) created_at: String,
    pub(crate) last_transition_at: String,
}

impl RecoveryLifecycleRecord {}

/// Opaque, account-owned handoff queued only after recovery approval commits.
/// It is a custody request reference, not evidence of custody execution or a
/// replacement for the current authority check.
#[derive(Debug, PartialEq, Eq)]
pub struct RecoveryCustodyHandoff {
    handoff_id: String,
    correlation_id: String,
    recovery_id: RecoveryId,
    household_id: FamilyId,
    account_id: ParentAccountId,
    member_id: AccountIdentityMemberId,
    device_id: AccountIdentityDeviceId,
    kind: RecoveryKind,
    requested_at: String,
}

pub(crate) struct RecoveryCustodyHandoffInput {
    pub(crate) handoff_id: String,
    pub(crate) correlation_id: String,
    pub(crate) recovery_id: RecoveryId,
    pub(crate) household_id: FamilyId,
    pub(crate) account_id: ParentAccountId,
    pub(crate) member_id: AccountIdentityMemberId,
    pub(crate) device_id: AccountIdentityDeviceId,
    pub(crate) kind: RecoveryKind,
    pub(crate) requested_at: String,
}

#[path = "recovery_lifecycle_handoff.rs"]
mod handoff;
