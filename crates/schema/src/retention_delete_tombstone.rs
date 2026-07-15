use serde::{Deserialize, Serialize};

mod enum_types;
mod identifiers;
mod policy_rows;
mod sample_rows;
mod text_types_core;
mod text_types_refs;

use self::identifiers::{action_ref, actor_id, contract_version, family_id, request_id, timestamp};

pub const RETENTION_DELETE_TOMBSTONE_SCHEMA_VERSION: &str = "retention-delete-tombstone-proof";

const RETENTION_DELETE_ACTOR_ROLE_PARENT: &str = "parent";
const RETENTION_DELETE_ACTOR_ROLE_GUARDIAN: &str = "guardian";
const RETENTION_DELETE_ACTOR_ROLE_SUPPORT: &str = "support";
const RETENTION_DELETE_ACTOR_ROLE_SYSTEM: &str = "system";

const RETENTION_DELETE_STATE_DELETE_REQUESTED: &str = "deleteRequested";
const RETENTION_DELETE_STATE_DELETE_VALIDATED: &str = "deleteValidated";
const RETENTION_DELETE_STATE_TOMBSTONE_WRITTEN: &str = "tombstoneWritten";
const RETENTION_DELETE_STATE_LOCAL_REDACTED: &str = "localRedacted";
const RETENTION_DELETE_STATE_PROPAGATION_PENDING: &str = "propagationPending";
const RETENTION_DELETE_STATE_PROPAGATED: &str = "propagated";
const RETENTION_DELETE_STATE_REPLAY_PROTECTED: &str = "replayProtected";
const RETENTION_DELETE_STATE_AUDIT_RETAINED: &str = "auditRetained";
const RETENTION_DELETE_STATE_HARD_DELETED: &str = "hardDeleted";

const RETENTION_DELETE_DATA_CLASS_CONFIG_METADATA: &str = "config-metadata";
const RETENTION_DELETE_DATA_CLASS_ACCOUNT_METADATA: &str = "account-metadata";
const RETENTION_DELETE_DATA_CLASS_POLICY_HISTORY: &str = "policy-history";
const RETENTION_DELETE_DATA_CLASS_EVIDENCE_JOURNAL: &str = "evidence-journal";
const RETENTION_DELETE_DATA_CLASS_LOGS: &str = "logs";
const RETENTION_DELETE_DATA_CLASS_SCREENSHOTS: &str = "screenshots";
const RETENTION_DELETE_DATA_CLASS_NETWORK_ARTIFACTS: &str = "network-artifacts";
const RETENTION_DELETE_DATA_CLASS_AI_OUTPUTS: &str = "ai-outputs";
const RETENTION_DELETE_DATA_CLASS_REPORTS: &str = "reports";
const RETENTION_DELETE_DATA_CLASS_NOTIFICATIONS: &str = "notifications";
const RETENTION_DELETE_DATA_CLASS_BILLING_REFERENCES: &str = "billing-references";

const RETENTION_DELETE_SOURCE_OF_TRUTH_HOUSEHOLD_CONTROL_PLANE: &str = "household-control-plane";
const RETENTION_DELETE_SOURCE_OF_TRUTH_ACCOUNT_CONTROL_PLANE: &str = "account-control-plane";
const RETENTION_DELETE_SOURCE_OF_TRUTH_BILLING_CONTROL_PLANE: &str = "billing-control-plane";
const RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_JOURNAL: &str =
    "child-device-local-journal";
const RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_EVIDENCE: &str =
    "child-device-local-evidence";
const RETENTION_DELETE_SOURCE_OF_TRUTH_PARENT_OWNED_OUTPUT: &str = "parent-owned-output";
const RETENTION_DELETE_SOURCE_OF_TRUTH_NOTIFICATION_SERVICE: &str = "notification-service";

const RETENTION_DELETE_RETENTION_CLASS_ACTIVE_WINDOW: &str = "active-window";
const RETENTION_DELETE_RETENTION_CLASS_DELETE_REQUESTED: &str = "delete-requested";
const RETENTION_DELETE_RETENTION_CLASS_DELETE_CONFIRMED: &str = "delete-confirmed";
const RETENTION_DELETE_RETENTION_CLASS_AUDIT_MINIMAL: &str = "audit-minimal";
const RETENTION_DELETE_RETENTION_CLASS_HARD_DELETED: &str = "hard-deleted";

const RETENTION_DELETE_AUDIT_MODE_MINIMAL_REF_ONLY: &str = "minimal-ref-only";
const RETENTION_DELETE_AUDIT_MODE_EXTERNAL_RETAINED: &str = "external-retained";

const RETENTION_DELETE_DERIVED_BOUNDARY_REDACTED_DERIVED_ONLY: &str = "redacted-derived-only";
const RETENTION_DELETE_DERIVED_BOUNDARY_BLOCKED_FROM_DERIVED_OUTPUTS: &str =
    "blocked-from-derived-outputs";

const RETENTION_DELETE_NON_CLAIM_NO_UI_HIDE_ONLY: &str = "no-ui-hide-only";
const RETENTION_DELETE_NON_CLAIM_NO_RESURRECTION: &str = "no-resurrection";
const RETENTION_DELETE_NON_CLAIM_NO_PLAIN_AUDIT_PAYLOAD: &str = "no-plain-audit-payload";
const RETENTION_DELETE_NON_CLAIM_NO_TS_BUSINESS_OWNER: &str = "no-ts-business-owner";
const RETENTION_DELETE_NON_CLAIM_NO_LAN_OWNERSHIP: &str = "no-lan-ownership";

const RETENTION_DELETE_POLICY_NOTE_CONFIG_METADATA: &str =
    "Config metadata may be redacted locally and hard-deleted after retention confirmation.";
const RETENTION_DELETE_POLICY_NOTE_ACCOUNT_METADATA: &str = "Account metadata keeps minimal audit refs while account control-plane ownership remains explicit.";
const RETENTION_DELETE_POLICY_NOTE_POLICY_HISTORY: &str =
    "Policy history keeps delete and approval refs but not deleted sensitive payloads.";
const RETENTION_DELETE_POLICY_NOTE_EVIDENCE_JOURNAL: &str = "Journal segments are local truth and require tombstone plus replay protection before hard delete.";
const RETENTION_DELETE_POLICY_NOTE_LOGS: &str =
    "Logs may retain minimal audit refs only after payload redaction.";
const RETENTION_DELETE_POLICY_NOTE_SCREENSHOTS: &str =
    "Screenshots require local redaction and may not survive as report or notification payloads.";
const RETENTION_DELETE_POLICY_NOTE_NETWORK_ARTIFACTS: &str =
    "Network artifacts keep only tombstone and replay-safe minimal refs after delete.";
const RETENTION_DELETE_POLICY_NOTE_AI_OUTPUTS: &str =
    "AI outputs may persist only redacted delete-safe references, never deleted payload copies.";
const RETENTION_DELETE_POLICY_NOTE_REPORTS: &str =
    "Reports are derived outputs and must purge deleted payload while keeping minimal audit refs.";
const RETENTION_DELETE_POLICY_NOTE_NOTIFICATIONS: &str =
    "Notifications keep routing metadata only and must not leak deleted payloads.";
const RETENTION_DELETE_POLICY_NOTE_BILLING_REFERENCES: &str = "Billing references are retained externally and may not become a child-evidence delete truth owner.";

const RETENTION_DELETE_SAMPLE_CONTRACT_VERSION: &str = "v0.4";
const RETENTION_DELETE_SAMPLE_REQUEST_TIMESTAMP: &str = "2026-06-28T18:00:00.000Z";
const RETENTION_DELETE_SAMPLE_REQUEST_ID: &str = "retention-delete-request-proof-1";
const RETENTION_DELETE_SAMPLE_FAMILY_ID: &str = "family-retention-delete-proof-1";
const RETENTION_DELETE_SAMPLE_ACTION_REF: &str = "retention-delete-action-proof-1";
const RETENTION_DELETE_SAMPLE_ACTOR_ID: &str = "parent-retention-delete-proof-1";
const RETENTION_DELETE_SAMPLE_REQUEST_EXPIRES_AT: &str = "2026-06-28T20:00:00.000Z";
const RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_ONE: &str = "tombstone-proof-1";
const RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_TWO: &str = "tombstone-proof-2";
const RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_THREE: &str = "tombstone-proof-3";
const RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_FOUR: &str = "tombstone-proof-4";
const RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_FIVE: &str = "tombstone-proof-5";
const RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_SIX: &str = "tombstone-proof-6";
const RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_SEVEN: &str = "tombstone-proof-7";
const RETENTION_DELETE_SAMPLE_REPLAY_REF_ONE: &str = "replay-proof-1";
const RETENTION_DELETE_SAMPLE_REPLAY_REF_TWO: &str = "replay-proof-2";
const RETENTION_DELETE_SAMPLE_REPLAY_REF_THREE: &str = "replay-proof-3";

const RETENTION_DELETE_EXPECT_CONTRACT_VERSION: &str = "contract version";
const RETENTION_DELETE_EXPECT_REQUEST_ID: &str = "request id";
const RETENTION_DELETE_EXPECT_ROW_ID: &str = "row id";
const RETENTION_DELETE_EXPECT_FAMILY_ID: &str = "family id";
const RETENTION_DELETE_EXPECT_ACTOR_ID: &str = "actor id";
const RETENTION_DELETE_EXPECT_ACTION_REF: &str = "action ref";
const RETENTION_DELETE_EXPECT_TOMBSTONE_REF: &str = "tombstone ref";
const RETENTION_DELETE_EXPECT_REPLAY_REF: &str = "replay ref";
const RETENTION_DELETE_EXPECT_PROOF_REF: &str = "proof ref";
const RETENTION_DELETE_EXPECT_TIMESTAMP: &str = "timestamp";

pub type RetentionDeleteActorRole = enum_types::RetentionDeleteActorRole;
pub type RetentionDeleteState = enum_types::RetentionDeleteState;
pub type RetentionDeleteDataClass = enum_types::RetentionDeleteDataClass;
pub type RetentionDeleteSourceOfTruth = enum_types::RetentionDeleteSourceOfTruth;
pub type RetentionDeleteRetentionClass = enum_types::RetentionDeleteRetentionClass;
pub type RetentionDeleteAuditMode = enum_types::RetentionDeleteAuditMode;
pub type RetentionDeleteDerivedBoundary = enum_types::RetentionDeleteDerivedBoundary;
pub type RetentionDeleteNonClaim = enum_types::RetentionDeleteNonClaim;

pub type RetentionDeleteContractVersion = text_types_core::RetentionDeleteContractVersion;
pub type RetentionDeleteRequestId = text_types_core::RetentionDeleteRequestId;
pub type RetentionDeleteRowId = text_types_core::RetentionDeleteRowId;
pub type RetentionDeleteFamilyId = text_types_core::RetentionDeleteFamilyId;
pub type RetentionDeleteActorId = text_types_core::RetentionDeleteActorId;
pub type RetentionDeleteActionRef = text_types_refs::RetentionDeleteActionRef;
pub type RetentionDeleteTombstoneRef = text_types_refs::RetentionDeleteTombstoneRef;
pub type RetentionDeleteReplayRef = text_types_refs::RetentionDeleteReplayRef;
pub type RetentionDeleteProofRef = text_types_refs::RetentionDeleteProofRef;
pub type RetentionDeleteTimestamp = text_types_refs::RetentionDeleteTimestamp;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionDeleteFamilyReference {
    pub family_id: RetentionDeleteFamilyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionDeleteActorReference {
    pub actor_id: RetentionDeleteActorId,
    pub role: RetentionDeleteActorRole,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionDeleteActionReference {
    pub action_ref: RetentionDeleteActionRef,
    pub actor: RetentionDeleteActorReference,
    pub requested_at: RetentionDeleteTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionDeleteRequest {
    pub schema_version: String,
    pub request_id: RetentionDeleteRequestId,
    pub family: RetentionDeleteFamilyReference,
    pub action: RetentionDeleteActionReference,
    pub requested_data_classes: Vec<RetentionDeleteDataClass>,
    pub request_expires_at: RetentionDeleteTimestamp,
    pub parent_authorized: bool,
    pub raw_payload_delete_requested: bool,
    pub derived_outputs_must_redact: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionDeletePolicyRow {
    pub data_class: RetentionDeleteDataClass,
    pub source_of_truth: RetentionDeleteSourceOfTruth,
    pub ocentra_hosted_by_default: bool,
    pub encrypted_before_upload: bool,
    pub derived_boundary: RetentionDeleteDerivedBoundary,
    pub audit_mode: RetentionDeleteAuditMode,
    pub hard_delete_allowed: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionDeleteRow {
    pub row_id: RetentionDeleteRowId,
    pub request_id: RetentionDeleteRequestId,
    pub data_class: RetentionDeleteDataClass,
    pub state: RetentionDeleteState,
    pub retention_class: RetentionDeleteRetentionClass,
    pub tombstone_ref: Option<RetentionDeleteTombstoneRef>,
    pub replay_ref: Option<RetentionDeleteReplayRef>,
    pub proof_ref: RetentionDeleteProofRef,
    pub request_expired: bool,
    pub parent_authorized: bool,
    pub tombstone_written: bool,
    pub local_payload_present: bool,
    pub local_payload_redacted: bool,
    pub propagation_pending: bool,
    pub propagated: bool,
    pub replay_blocked: bool,
    pub restore_revival_blocked: bool,
    pub minimal_audit_ref_retained: bool,
    pub audit_payload_redacted: bool,
    pub report_export_leak_blocked: bool,
    pub assistant_leak_blocked: bool,
    pub notification_leak_blocked: bool,
    pub hard_deleted: bool,
    pub claim_safe: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionDeleteTombstoneContractProof {
    pub schema_version: String,
    pub contract_version: RetentionDeleteContractVersion,
    pub request: RetentionDeleteRequest,
    pub retention_matrix: Vec<RetentionDeletePolicyRow>,
    pub rows: Vec<RetentionDeleteRow>,
    pub non_claims: Vec<RetentionDeleteNonClaim>,
    pub report_runtime_claimed: bool,
    pub notification_runtime_claimed: bool,
    pub restore_runtime_claimed: bool,
    pub ts_business_owner_claimed: bool,
    pub updated_at: RetentionDeleteTimestamp,
}

pub fn required_retention_delete_states() -> Vec<RetentionDeleteState> {
    vec![
        RetentionDeleteState::DeleteRequested,
        RetentionDeleteState::DeleteValidated,
        RetentionDeleteState::TombstoneWritten,
        RetentionDeleteState::LocalRedacted,
        RetentionDeleteState::PropagationPending,
        RetentionDeleteState::Propagated,
        RetentionDeleteState::ReplayProtected,
        RetentionDeleteState::AuditRetained,
        RetentionDeleteState::HardDeleted,
    ]
}

pub fn required_retention_delete_non_claims() -> Vec<RetentionDeleteNonClaim> {
    vec![
        RetentionDeleteNonClaim::UiHideOnly,
        RetentionDeleteNonClaim::Resurrection,
        RetentionDeleteNonClaim::PlainAuditPayload,
        RetentionDeleteNonClaim::TsBusinessOwner,
        RetentionDeleteNonClaim::LanOwnership,
    ]
}

pub fn retention_delete_policy_matrix() -> Vec<RetentionDeletePolicyRow> {
    policy_rows::retention_delete_policy_matrix()
}

pub fn sample_retention_delete_tombstone_contract_proof() -> RetentionDeleteTombstoneContractProof {
    let request_timestamp = timestamp(RETENTION_DELETE_SAMPLE_REQUEST_TIMESTAMP);
    let request = RetentionDeleteRequest {
        schema_version: RETENTION_DELETE_TOMBSTONE_SCHEMA_VERSION.to_string(),
        request_id: request_id(RETENTION_DELETE_SAMPLE_REQUEST_ID),
        family: RetentionDeleteFamilyReference {
            family_id: family_id(RETENTION_DELETE_SAMPLE_FAMILY_ID),
        },
        action: RetentionDeleteActionReference {
            action_ref: action_ref(RETENTION_DELETE_SAMPLE_ACTION_REF),
            actor: RetentionDeleteActorReference {
                actor_id: actor_id(RETENTION_DELETE_SAMPLE_ACTOR_ID),
                role: RetentionDeleteActorRole::Parent,
            },
            requested_at: request_timestamp.clone(),
        },
        requested_data_classes: vec![
            RetentionDeleteDataClass::EvidenceJournal,
            RetentionDeleteDataClass::Screenshots,
            RetentionDeleteDataClass::Reports,
            RetentionDeleteDataClass::Notifications,
        ],
        request_expires_at: timestamp(RETENTION_DELETE_SAMPLE_REQUEST_EXPIRES_AT),
        parent_authorized: true,
        raw_payload_delete_requested: true,
        derived_outputs_must_redact: true,
    };

    RetentionDeleteTombstoneContractProof {
        schema_version: RETENTION_DELETE_TOMBSTONE_SCHEMA_VERSION.to_string(),
        contract_version: contract_version(RETENTION_DELETE_SAMPLE_CONTRACT_VERSION),
        request: request.clone(),
        retention_matrix: retention_delete_policy_matrix(),
        rows: sample_rows::sample_retention_delete_rows(&request),
        non_claims: required_retention_delete_non_claims(),
        report_runtime_claimed: false,
        notification_runtime_claimed: false,
        restore_runtime_claimed: false,
        ts_business_owner_claimed: false,
        updated_at: request_timestamp,
    }
}
