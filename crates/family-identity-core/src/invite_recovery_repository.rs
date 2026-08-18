#![forbid(unsafe_code)]

//! Account-owned durable invite and recovery custody. The provider, account,
//! membership, and custody owners mint the opaque proof/receipt values used by
//! this repository; bearer values and serialized DTOs never mint authority.

use std::fmt;
use std::time::Duration;

use ocentra_eventing::error::EventingError;
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject, AccountIdentityRole,
    AccountIdentitySupportScope,
};
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};
use rusqlite::Connection;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
use crate::family_identity::{RecoveryId, SetupInviteId};
use crate::recovery_lifecycle::{RecoveryCustodyHandoff, RecoveryKind, RecoveryState};
use crate::setup_lifecycle::{RecoverySupportChannel, SetupInvitePurpose, SetupInviteTargetRole};

const INVITE_TOKEN_DIGEST_DOMAIN: &[u8] = b"ocentra-account-invite-v1";
const MAX_INVITE_TTL: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const HANDOFF_LEASE_MILLIS: i64 = 5 * 60 * 1_000;
const MAX_FORWARD_SKEW_MILLIS: i64 = 24 * 60 * 60 * 1_000;

pub(crate) fn validate_schema(connection: &Connection) -> Result<(), ()> {
    schema::validate(connection)
}

pub const INVITE_RECOVERY_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_runtime_clock (
         clock_id INTEGER PRIMARY KEY CHECK (clock_id = 1),
         last_epoch_millis INTEGER NOT NULL CHECK (last_epoch_millis > 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_mutation_effect (
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         action TEXT NOT NULL CHECK (action IN (
             'revoke-child-device','revoke-setup-invite','revoke-recovery'
         )),
         target_kind TEXT NOT NULL CHECK (target_kind IN (
             'child-device','setup-invite','recovery'
         )),
         target_id TEXT NOT NULL CHECK (
             length(trim(target_id)) > 0 AND length(target_id) <= 256
         ),
         idempotency_key TEXT NOT NULL CHECK (
             length(trim(idempotency_key)) > 0 AND length(idempotency_key) <= 256
         ),
         payload_digest TEXT NOT NULL UNIQUE CHECK (
             length(payload_digest) = 71
             AND substr(payload_digest, 1, 7) = 'sha256:'
             AND substr(payload_digest, 8) NOT GLOB '*[^0-9a-f]*'
         ),
         key_id TEXT NOT NULL CHECK (
             length(key_id) = 71
             AND substr(key_id, 1, 7) = 'sha256:'
             AND substr(key_id, 8) NOT GLOB '*[^0-9a-f]*'
         ),
         token_expires_at_epoch_millis INTEGER NOT NULL CHECK (
             token_expires_at_epoch_millis > 0
         ),
         status TEXT NOT NULL CHECK (status IN ('pending','completed')),
         result_code TEXT CHECK (result_code IS NULL OR result_code IN (
             'setup-invite-revoked','recovery-revoked'
         )),
         created_at_epoch_millis INTEGER NOT NULL CHECK (created_at_epoch_millis > 0),
         updated_at_epoch_millis INTEGER NOT NULL CHECK (
             updated_at_epoch_millis >= created_at_epoch_millis
         ),
         completed_at_epoch_millis INTEGER,
         retain_until_epoch_millis INTEGER NOT NULL CHECK (
             retain_until_epoch_millis > token_expires_at_epoch_millis
         ),
         PRIMARY KEY (
             account_id, household_id, action, target_kind, target_id, idempotency_key
         ),
         CHECK (
             (status = 'pending' AND result_code IS NULL
                 AND completed_at_epoch_millis IS NULL)
             OR (status = 'completed' AND result_code IS NOT NULL
                 AND completed_at_epoch_millis = updated_at_epoch_millis)
         )
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_mutation_effect_retention
         ON account_identity_mutation_effect(status, retain_until_epoch_millis);
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
         state TEXT NOT NULL CHECK (state IN ('pending','in-flight','committed','rejected')),
         created_at_epoch_millis INTEGER NOT NULL CHECK (created_at_epoch_millis > 0),
         active_attempt_id TEXT UNIQUE CHECK (active_attempt_id IS NULL OR length(trim(active_attempt_id)) > 0),
         lease_expires_at_epoch_millis INTEGER,
         attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0),
         CHECK ((state = 'pending' AND active_attempt_id IS NULL AND lease_expires_at_epoch_millis IS NULL)
             OR (state = 'in-flight' AND active_attempt_id IS NOT NULL AND lease_expires_at_epoch_millis > created_at_epoch_millis)
             OR (state IN ('committed','rejected') AND active_attempt_id IS NULL AND lease_expires_at_epoch_millis IS NULL))
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
         support_authorization_id TEXT,
         support_authorization_issuer TEXT,
         support_authorization_scope TEXT CHECK (support_authorization_scope IS NULL OR support_authorization_scope IN ('household','device-control')),
         support_authorization_expires_at_epoch_millis INTEGER,
         owner_effect_kind INTEGER NOT NULL CHECK (owner_effect_kind IN (1,2,3,4)),
         state TEXT NOT NULL CHECK (state IN ('owner-approval-required','approved','completed','revoked')),
         created_at_epoch_millis INTEGER NOT NULL CHECK (created_at_epoch_millis > 0),
         last_transition_at_epoch_millis INTEGER NOT NULL CHECK (last_transition_at_epoch_millis >= created_at_epoch_millis),
         reserved_owner_receipt_id TEXT CHECK (reserved_owner_receipt_id IS NULL OR length(reserved_owner_receipt_id) = 64),
         reserved_owner_transition_id TEXT CHECK (reserved_owner_transition_id IS NULL OR length(trim(reserved_owner_transition_id)) > 0),
         reserved_owner_receipt_expires_at_epoch_millis INTEGER,
         CHECK ((support_channel = 'support-assisted'
                 AND support_authorization_id IS NOT NULL
                 AND support_authorization_issuer IS NOT NULL
                 AND support_authorization_scope IS NOT NULL
                 AND support_authorization_expires_at_epoch_millis IS NOT NULL)
             OR (support_channel <> 'support-assisted'
                 AND support_authorization_id IS NULL
                 AND support_authorization_issuer IS NULL
                 AND support_authorization_scope IS NULL
                 AND support_authorization_expires_at_epoch_millis IS NULL))
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_recovery_household ON account_identity_recovery(household_id, state);
     CREATE TABLE IF NOT EXISTS account_identity_recovery_rate_limit (
         subject_digest TEXT PRIMARY KEY CHECK (length(subject_digest) = 64),
         window_started_at_epoch_millis INTEGER NOT NULL CHECK (window_started_at_epoch_millis > 0),
         attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_invite_rate_limit (
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
         owner_transition_id TEXT CHECK (owner_transition_id IS NULL OR length(trim(owner_transition_id)) > 0),
         owner_receipt_digest TEXT CHECK (owner_receipt_digest IS NULL OR (length(owner_receipt_digest) = 64 AND owner_receipt_digest NOT GLOB '*[^0-9a-f]*')),
         CHECK ((state = 'pending' AND lease_expires_at_epoch_millis IS NULL AND active_attempt_id IS NULL)
             OR (state = 'in-flight' AND lease_expires_at_epoch_millis > requested_at_epoch_millis AND active_attempt_id IS NOT NULL)
             OR (state = 'delivered' AND lease_expires_at_epoch_millis IS NULL
                 AND owner_transition_id IS NOT NULL AND owner_receipt_digest IS NOT NULL))
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
#[derive(PartialEq, Eq)]
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

pub struct VerifiedSupportRecoveryAuthorization {
    authorization_id: String,
    issuer: String,
    household_id: FamilyId,
    account_id: ParentAccountId,
    kind: RecoveryKind,
    scope: AccountIdentitySupportScope,
    expires_at_epoch_millis: i64,
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

pub struct ProviderCredentialSessionOwnerReceipt {
    handoff_id: String,
    correlation_id: String,
    recovery_id: RecoveryId,
    attempt_id: String,
    transition_id: String,
    receipt_digest: String,
}

pub struct DeviceTrustRevokeOwnerReceipt {
    handoff_id: String,
    correlation_id: String,
    recovery_id: RecoveryId,
    attempt_id: String,
    transition_id: String,
    receipt_digest: String,
}

pub struct DeviceTrustReinstallOwnerReceipt {
    handoff_id: String,
    correlation_id: String,
    recovery_id: RecoveryId,
    attempt_id: String,
    transition_id: String,
    receipt_digest: String,
}

pub struct HouseholdAuthorityMutationOwnerReceipt {
    handoff_id: String,
    correlation_id: String,
    recovery_id: RecoveryId,
    attempt_id: String,
    transition_id: String,
    receipt_digest: String,
}

/// A membership-owner delivery lease. The account repository only persists
/// the lease; it cannot mint a membership commit receipt or make the pending
/// row authoritative.
pub struct InviteMembershipDeliveryAttempt {
    invite_id: SetupInviteId,
    household_id: FamilyId,
    recipient_provider: AccountIdentityProvider,
    recipient_provider_subject: AccountIdentityProviderSubject,
    recipient_account_id: ParentAccountId,
    target_role: SetupInviteTargetRole,
    attempt_id: String,
    lease_expires_at: String,
}

/// Opaque membership-owner receipt. No constructor exists in this crate: the
/// membership owner must provide the exact attempt-bound receipt.
pub struct InviteMembershipCommitReceipt {
    invite_id: SetupInviteId,
    household_id: FamilyId,
    recipient_provider: AccountIdentityProvider,
    recipient_provider_subject: AccountIdentityProviderSubject,
    recipient_account_id: ParentAccountId,
    target_role: SetupInviteTargetRole,
    attempt_id: String,
}

#[path = "invite_recovery_repository_authority.rs"]
pub(super) mod authority;
#[path = "invite_recovery_repository_invite_ops.rs"]
mod invite_ops;
#[path = "invite_recovery_repository_membership_ops.rs"]
mod membership_ops;
#[path = "invite_recovery_repository_membership_types.rs"]
mod membership_types;
#[path = "recovery_owner_ack_ops.rs"]
mod owner_ack_ops;
#[path = "invite_recovery_repository_owner_receipt_types.rs"]
mod owner_receipt_types;
#[path = "invite_recovery_repository_recovery_begin_ops.rs"]
mod recovery_begin_ops;
#[path = "invite_recovery_repository_recovery_completion_ops.rs"]
mod recovery_completion_ops;
#[path = "invite_recovery_repository_recovery_handoff_ops.rs"]
mod recovery_handoff_ops;
#[path = "invite_recovery_repository_recovery_ops.rs"]
mod recovery_ops;
#[path = "invite_recovery_repository_schema.rs"]
mod schema;
#[path = "invite_recovery_repository_security_effect_codes.rs"]
mod security_effect_codes;
#[path = "invite_recovery_repository_security_effects.rs"]
mod security_effects;
#[path = "invite_recovery_repository_security_entropy.rs"]
mod security_entropy;
#[path = "invite_recovery_repository_security_rate_invite.rs"]
mod security_rate_invite;
#[path = "invite_recovery_repository_security_rate_recovery.rs"]
mod security_rate_recovery;
#[path = "invite_recovery_repository_support_invite_identity.rs"]
mod support_invite_identity;
#[path = "invite_recovery_repository_support_invite_policy.rs"]
mod support_invite_policy;
#[path = "invite_recovery_repository_support_invite_purpose.rs"]
mod support_invite_purpose;
#[path = "invite_recovery_repository_support_invite_target_role.rs"]
mod support_invite_target_role;
#[path = "invite_recovery_repository_support_recovery_channel.rs"]
mod support_recovery_channel;
#[path = "invite_recovery_repository_support_recovery_handoff.rs"]
mod support_recovery_handoff;
#[path = "invite_recovery_repository_support_recovery_kind_from_label.rs"]
mod support_recovery_kind_from_label;
#[path = "invite_recovery_repository_support_recovery_kind_label.rs"]
mod support_recovery_kind_label;
#[path = "invite_recovery_repository_support_recovery_labels.rs"]
mod support_recovery_labels;
#[path = "invite_recovery_repository_support_recovery_policy.rs"]
mod support_recovery_policy;
#[path = "invite_recovery_repository_support_recovery_scope.rs"]
mod support_recovery_scope;
#[path = "invite_recovery_repository_support_recovery_scope_from_label.rs"]
mod support_recovery_scope_from_label;
#[path = "invite_recovery_repository_support_recovery_scope_label.rs"]
mod support_recovery_scope_label;
#[path = "invite_recovery_repository_types.rs"]
mod types;
#[path = "invite_recovery_repository_types_identity.rs"]
mod types_identity;
