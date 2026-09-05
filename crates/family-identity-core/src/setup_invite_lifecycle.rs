#![forbid(unsafe_code)]

//! Durable-adapter-neutral invite lifecycle state.

use crate::family_identity::{HouseholdRole, SetupInvite};
use crate::setup_lifecycle::SetupInvitePurpose;

/// Invite redemption is modeled as a state transition rather than a boolean
/// helper. The durable adapter owns persistence and token custody; this record
/// owns expiry, single-use, and revocation semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupInviteLifecycleRecord {
    pub(crate) invite: SetupInvite,
    pub(crate) purpose: SetupInvitePurpose,
    pub(crate) inviter_role: HouseholdRole,
    pub(crate) issued_at: String,
    pub(crate) accepted_at: Option<String>,
    pub(crate) revoked_at: Option<String>,
    pub(crate) use_count: u32,
}

impl SetupInviteLifecycleRecord {}
