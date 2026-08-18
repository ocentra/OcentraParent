#![forbid(unsafe_code)]

//! Account-owned durable invite and recovery custody. The provider, account,
//! membership, and custody owners mint the opaque proof/receipt values used by
//! this repository; bearer values and serialized DTOs never mint authority.

use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, SecondsFormat, Utc};
use getrandom::fill;
use ocentra_eventing::error::EventingError;
use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityMappingStatus,
    AccountIdentityProvider, AccountIdentityProviderSubject, AccountIdentityRole,
    AccountIdentitySupportScope,
};
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use sha2::{Digest, Sha256};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::{RecoveryId, SetupInviteId};
use crate::recovery_lifecycle::{RecoveryCustodyHandoff, RecoveryKind, RecoveryState};
use crate::setup_lifecycle::{RecoverySupportChannel, SetupInvitePurpose, SetupInviteTargetRole};

const INVITE_TOKEN_DIGEST_DOMAIN: &[u8] = b"ocentra-account-invite-v1";
const MAX_INVITE_TTL: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const HANDOFF_LEASE_MILLIS: i64 = 5 * 60 * 1_000;

pub(crate) fn validate_schema(connection: &Connection) -> Result<(), ()> {
    let integrity = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_| ())?;
    if integrity != "ok" {
        return Err(());
    }
    let owned_tables = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master
             WHERE type = 'table' AND name IN (
                 'account_identity_runtime_clock', 'account_identity_setup_invite',
                 'account_identity_pending_invite_membership', 'account_identity_recovery',
                 'account_identity_recovery_rate_limit',
                 'account_identity_recovery_custody_handoff'
             )",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    if owned_tables != 6 {
        return Err(());
    }
    let unowned_objects = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master
             WHERE type IN ('trigger','view') AND name LIKE 'account_identity_%'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (unowned_objects == 0).then_some(()).ok_or(())
}

pub const INVITE_RECOVERY_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_runtime_clock (
         clock_id INTEGER PRIMARY KEY CHECK (clock_id = 1),
         last_epoch_millis INTEGER NOT NULL CHECK (last_epoch_millis > 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_setup_invite (
         invite_id TEXT PRIMARY KEY CHECK (length(trim(invite_id)) > 0),
         token_digest TEXT NOT NULL UNIQUE CHECK (length(token_digest) = 64 AND token_digest NOT GLOB '*[^0-9a-f]*'),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         inviter_account_id TEXT NOT NULL CHECK (length(trim(inviter_account_id)) > 0),
         inviter_member_id TEXT NOT NULL CHECK (length(trim(inviter_member_id)) > 0),
         inviter_device_id TEXT NOT NULL CHECK (length(trim(inviter_device_id)) > 0),
         inviter_authority_generation INTEGER NOT NULL CHECK (inviter_authority_generation > 0),
         inviter_session_generation INTEGER NOT NULL CHECK (inviter_session_generation > 0),
         inviter_role TEXT NOT NULL CHECK (inviter_role IN ('parent-owner','co-parent-guardian')),
         purpose TEXT NOT NULL CHECK (purpose IN ('co-parent-invite','observer-invite','child-device-pairing','household-transfer')),
         target_role TEXT NOT NULL CHECK (target_role IN ('co-parent-guardian','observer','child-device-agent','parent-owner')),
         recipient_provider TEXT NOT NULL CHECK (recipient_provider IN ('authjs','firebase')),
         recipient_provider_subject TEXT NOT NULL CHECK (length(trim(recipient_provider_subject)) > 0),
         recipient_account_id TEXT NOT NULL CHECK (length(trim(recipient_account_id)) > 0),
         invitee_email_digest TEXT NOT NULL CHECK (length(invitee_email_digest) = 64 AND invitee_email_digest NOT GLOB '*[^0-9a-f]*'),
         issued_at_epoch_millis INTEGER NOT NULL CHECK (issued_at_epoch_millis > 0),
         expires_at_epoch_millis INTEGER NOT NULL CHECK (expires_at_epoch_millis > issued_at_epoch_millis),
         state TEXT NOT NULL CHECK (state IN ('pending','accepted','expired','revoked')),
         accepted_at_epoch_millis INTEGER,
         revoked_at_epoch_millis INTEGER,
         use_count INTEGER NOT NULL CHECK (use_count IN (0,1)),
         CHECK ((state = 'pending' AND accepted_at_epoch_millis IS NULL AND revoked_at_epoch_millis IS NULL AND use_count = 0)
             OR (state = 'accepted' AND accepted_at_epoch_millis >= issued_at_epoch_millis AND revoked_at_epoch_millis IS NULL AND use_count = 1)
             OR (state = 'expired' AND accepted_at_epoch_millis IS NULL AND revoked_at_epoch_millis IS NULL AND use_count = 0)
             OR (state = 'revoked' AND revoked_at_epoch_millis >= issued_at_epoch_millis AND accepted_at_epoch_millis IS NULL AND use_count = 0))
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_setup_invite_household ON account_identity_setup_invite(household_id, state);
     CREATE TABLE IF NOT EXISTS account_identity_pending_invite_membership (
         invite_id TEXT PRIMARY KEY REFERENCES account_identity_setup_invite(invite_id),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         recipient_provider TEXT NOT NULL CHECK (recipient_provider IN ('authjs','firebase')),
         recipient_provider_subject TEXT NOT NULL CHECK (length(trim(recipient_provider_subject)) > 0),
         recipient_account_id TEXT NOT NULL CHECK (length(trim(recipient_account_id)) > 0),
         target_role TEXT NOT NULL CHECK (target_role IN ('co-parent-guardian','observer','child-device-agent','parent-owner')),
         state TEXT NOT NULL CHECK (state IN ('pending','committed','rejected')),
         created_at_epoch_millis INTEGER NOT NULL CHECK (created_at_epoch_millis > 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_recovery (
         recovery_id TEXT PRIMARY KEY CHECK (length(trim(recovery_id)) > 0),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         requester_member_id TEXT NOT NULL CHECK (length(trim(requester_member_id)) > 0),
         requester_device_id TEXT NOT NULL CHECK (length(trim(requester_device_id)) > 0),
         requester_role TEXT NOT NULL CHECK (requester_role IN ('parent-owner','co-parent-guardian','observer','child-device-agent','support-admin')),
         kind TEXT NOT NULL CHECK (kind IN ('forgot-login','lost-parent-device','compromised-account','child-reinstall','household-transfer')),
         support_channel TEXT NOT NULL CHECK (support_channel IN ('self-serve','household-owner-assisted','support-assisted')),
         identity_proof_id TEXT NOT NULL UNIQUE CHECK (length(trim(identity_proof_id)) > 0),
         identity_proof_provider TEXT NOT NULL CHECK (identity_proof_provider IN ('authjs','firebase')),
         identity_proof_subject TEXT NOT NULL CHECK (length(trim(identity_proof_subject)) > 0),
         identity_proof_expires_at_epoch_millis INTEGER NOT NULL CHECK (identity_proof_expires_at_epoch_millis > 0),
         identity_proof_state TEXT NOT NULL CHECK (identity_proof_state IN ('verified','pending','failed')),
         owner_effect TEXT NOT NULL CHECK (owner_effect IN ('provider-credential-session','device-trust-revoke','device-trust-reinstall','household-authority-mutation','data-custody-export-delete')),
         state TEXT NOT NULL CHECK (state IN ('owner-approval-required','approved','completed','revoked')),
         delete_export_handoff_required INTEGER NOT NULL CHECK (delete_export_handoff_required IN (0,1)),
         created_at_epoch_millis INTEGER NOT NULL CHECK (created_at_epoch_millis > 0),
         last_transition_at_epoch_millis INTEGER NOT NULL CHECK (last_transition_at_epoch_millis >= created_at_epoch_millis)
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_recovery_household ON account_identity_recovery(household_id, state);
     CREATE TABLE IF NOT EXISTS account_identity_recovery_rate_limit (
         subject_digest TEXT PRIMARY KEY CHECK (length(subject_digest) = 64),
         window_started_at_epoch_millis INTEGER NOT NULL CHECK (window_started_at_epoch_millis > 0),
         attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_recovery_custody_handoff (
         handoff_id TEXT PRIMARY KEY CHECK (length(trim(handoff_id)) > 0),
         correlation_id TEXT NOT NULL UNIQUE CHECK (length(trim(correlation_id)) > 0),
         recovery_id TEXT NOT NULL UNIQUE REFERENCES account_identity_recovery(recovery_id),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         member_id TEXT NOT NULL CHECK (length(trim(member_id)) > 0),
         device_id TEXT NOT NULL CHECK (length(trim(device_id)) > 0),
         kind TEXT NOT NULL CHECK (kind IN ('forgot-login','lost-parent-device','compromised-account','child-reinstall','household-transfer')),
         requested_at_epoch_millis INTEGER NOT NULL CHECK (requested_at_epoch_millis > 0),
         state TEXT NOT NULL CHECK (state IN ('pending','in-flight','delivered')),
         active_attempt_id TEXT UNIQUE CHECK (active_attempt_id IS NULL OR length(trim(active_attempt_id)) > 0),
         lease_expires_at_epoch_millis INTEGER,
         attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0),
         CHECK ((state = 'pending' AND lease_expires_at_epoch_millis IS NULL AND active_attempt_id IS NULL)
             OR (state = 'in-flight' AND lease_expires_at_epoch_millis > requested_at_epoch_millis AND active_attempt_id IS NOT NULL)
             OR (state = 'delivered' AND lease_expires_at_epoch_millis IS NULL))
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_recovery_handoff_ready ON account_identity_recovery_custody_handoff(household_id, state, lease_expires_at_epoch_millis);";

#[derive(Debug)]
pub enum InviteRecoveryRepositoryError {
    Unavailable,
    AuthorityUnavailable,
    AuthorityExpired,
    AuthorityNotCurrent,
    EntropyUnavailable,
    InvalidValue(EventingError),
    InvalidInvite,
    InviteRejected,
    RecoveryRejected,
    Missing,
    HandoffConflict,
    ClockUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryOwnerEffect {
    ProviderCredentialSession,
    DeviceTrustRevoke,
    DeviceTrustReinstall,
    HouseholdAuthorityMutation,
    DataCustodyExportDelete,
}

pub struct SetupInviteCode {
    invite_id: SetupInviteId,
    token: String,
}

pub struct VerifiedInviteRecipient {
    provider: AccountIdentityProvider,
    provider_subject: AccountIdentityProviderSubject,
    account_id: ParentAccountId,
    email_digest: String,
}
pub(crate) struct VerifiedInviteRecipientInput {
    pub(crate) provider: AccountIdentityProvider,
    pub(crate) provider_subject: AccountIdentityProviderSubject,
    pub(crate) account_id: ParentAccountId,
    pub(crate) canonical_email: String,
}
#[derive(Debug, PartialEq, Eq)]
pub struct InviteMembershipHandoff {
    invite_id: SetupInviteId,
    household_id: FamilyId,
    recipient_provider: AccountIdentityProvider,
    recipient_provider_subject: AccountIdentityProviderSubject,
    recipient_account_id: ParentAccountId,
    target_role: SetupInviteTargetRole,
}

pub struct VerifiedRecoveryIdentityProof {
    proof_id: String,
    provider: AccountIdentityProvider,
    provider_subject: AccountIdentityProviderSubject,
    account_id: ParentAccountId,
    household_id: FamilyId,
    member_id: String,
    device_id: String,
    role: AccountIdentityRole,
    kind: RecoveryKind,
    support_channel: RecoverySupportChannel,
    expires_at_epoch_millis: i64,
}
pub(crate) struct VerifiedRecoveryIdentityProofInput {
    pub(crate) proof_id: String,
    pub(crate) provider: AccountIdentityProvider,
    pub(crate) provider_subject: AccountIdentityProviderSubject,
    pub(crate) account_id: ParentAccountId,
    pub(crate) household_id: FamilyId,
    pub(crate) member_id: String,
    pub(crate) device_id: String,
    pub(crate) role: AccountIdentityRole,
    pub(crate) kind: RecoveryKind,
    pub(crate) support_channel: RecoverySupportChannel,
    pub(crate) expires_at_epoch_millis: i64,
}

pub struct VerifiedSupportRecoveryAuthorization {
    authorization_id: String,
    issuer: String,
    household_id: FamilyId,
    account_id: ParentAccountId,
    kind: RecoveryKind,
    scope: AccountIdentitySupportScope,
    expires_at_epoch_millis: i64,
}
pub(crate) struct VerifiedSupportRecoveryAuthorizationInput {
    pub(crate) authorization_id: String,
    pub(crate) issuer: String,
    pub(crate) household_id: FamilyId,
    pub(crate) account_id: ParentAccountId,
    pub(crate) kind: RecoveryKind,
    pub(crate) scope: AccountIdentitySupportScope,
    pub(crate) expires_at_epoch_millis: i64,
}

pub struct IssuedSetupInvite {
    code: SetupInviteCode,
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
    expires_at: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RedeemedSetupInvite {
    invite_id: SetupInviteId,
    household_id: FamilyId,
    target_role: SetupInviteTargetRole,
    accepted_at: String,
    membership_handoff: InviteMembershipHandoff,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RecoveryCompletion {
    state: RecoveryState,
    handoff_enqueued: bool,
    owner_effect: RecoveryOwnerEffect,
}

pub struct RecoveryHandoffDeliveryAttempt {
    handoff: RecoveryCustodyHandoff,
    attempt_id: String,
    lease_expires_at: String,
}

pub struct RecoveryCustodyDeliveryReceipt {
    handoff_id: String,
    correlation_id: String,
    attempt_id: String,
}

#[path = "invite_recovery_repository_authority.rs"]
mod authority;
#[path = "invite_recovery_repository_invite_ops.rs"]
mod invite_ops;
#[path = "invite_recovery_repository_recovery_ops.rs"]
mod recovery_ops;
#[path = "invite_recovery_repository_support_invite.rs"]
mod support_invite;
#[path = "invite_recovery_repository_support_recovery.rs"]
mod support_recovery;
#[path = "invite_recovery_repository_support_security.rs"]
mod support_security;
#[path = "invite_recovery_repository_types.rs"]
mod types;
#[path = "invite_recovery_repository_types_identity.rs"]
mod types_identity;
