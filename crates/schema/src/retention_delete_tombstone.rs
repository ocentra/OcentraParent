use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

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

macro_rules! retention_delete_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! retention_delete_string_enum {
    ($name:ident { $($variant:ident => $value:expr),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                $(if value == $value { return Ok(Self::$variant); })+
                Err(serde::de::Error::unknown_variant(value.as_str(), &[$($value,)+]))
            }
        }
    };
}

retention_delete_string_enum!(RetentionDeleteActorRole {
    Parent => RETENTION_DELETE_ACTOR_ROLE_PARENT,
    Guardian => RETENTION_DELETE_ACTOR_ROLE_GUARDIAN,
    Support => RETENTION_DELETE_ACTOR_ROLE_SUPPORT,
    System => RETENTION_DELETE_ACTOR_ROLE_SYSTEM,
});

retention_delete_string_enum!(RetentionDeleteState {
    DeleteRequested => RETENTION_DELETE_STATE_DELETE_REQUESTED,
    DeleteValidated => RETENTION_DELETE_STATE_DELETE_VALIDATED,
    TombstoneWritten => RETENTION_DELETE_STATE_TOMBSTONE_WRITTEN,
    LocalRedacted => RETENTION_DELETE_STATE_LOCAL_REDACTED,
    PropagationPending => RETENTION_DELETE_STATE_PROPAGATION_PENDING,
    Propagated => RETENTION_DELETE_STATE_PROPAGATED,
    ReplayProtected => RETENTION_DELETE_STATE_REPLAY_PROTECTED,
    AuditRetained => RETENTION_DELETE_STATE_AUDIT_RETAINED,
    HardDeleted => RETENTION_DELETE_STATE_HARD_DELETED,
});

retention_delete_string_enum!(RetentionDeleteDataClass {
    ConfigMetadata => RETENTION_DELETE_DATA_CLASS_CONFIG_METADATA,
    AccountMetadata => RETENTION_DELETE_DATA_CLASS_ACCOUNT_METADATA,
    PolicyHistory => RETENTION_DELETE_DATA_CLASS_POLICY_HISTORY,
    EvidenceJournal => RETENTION_DELETE_DATA_CLASS_EVIDENCE_JOURNAL,
    Logs => RETENTION_DELETE_DATA_CLASS_LOGS,
    Screenshots => RETENTION_DELETE_DATA_CLASS_SCREENSHOTS,
    NetworkArtifacts => RETENTION_DELETE_DATA_CLASS_NETWORK_ARTIFACTS,
    AiOutputs => RETENTION_DELETE_DATA_CLASS_AI_OUTPUTS,
    Reports => RETENTION_DELETE_DATA_CLASS_REPORTS,
    Notifications => RETENTION_DELETE_DATA_CLASS_NOTIFICATIONS,
    BillingReferences => RETENTION_DELETE_DATA_CLASS_BILLING_REFERENCES,
});

retention_delete_string_enum!(RetentionDeleteSourceOfTruth {
    HouseholdControlPlane => RETENTION_DELETE_SOURCE_OF_TRUTH_HOUSEHOLD_CONTROL_PLANE,
    AccountControlPlane => RETENTION_DELETE_SOURCE_OF_TRUTH_ACCOUNT_CONTROL_PLANE,
    BillingControlPlane => RETENTION_DELETE_SOURCE_OF_TRUTH_BILLING_CONTROL_PLANE,
    ChildDeviceLocalJournal => RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_JOURNAL,
    ChildDeviceLocalEvidence => RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_EVIDENCE,
    ParentOwnedOutput => RETENTION_DELETE_SOURCE_OF_TRUTH_PARENT_OWNED_OUTPUT,
    NotificationService => RETENTION_DELETE_SOURCE_OF_TRUTH_NOTIFICATION_SERVICE,
});

retention_delete_string_enum!(RetentionDeleteRetentionClass {
    ActiveWindow => RETENTION_DELETE_RETENTION_CLASS_ACTIVE_WINDOW,
    DeleteRequested => RETENTION_DELETE_RETENTION_CLASS_DELETE_REQUESTED,
    DeleteConfirmed => RETENTION_DELETE_RETENTION_CLASS_DELETE_CONFIRMED,
    AuditMinimal => RETENTION_DELETE_RETENTION_CLASS_AUDIT_MINIMAL,
    HardDeleted => RETENTION_DELETE_RETENTION_CLASS_HARD_DELETED,
});

retention_delete_string_enum!(RetentionDeleteAuditMode {
    MinimalRefOnly => RETENTION_DELETE_AUDIT_MODE_MINIMAL_REF_ONLY,
    ExternalRetained => RETENTION_DELETE_AUDIT_MODE_EXTERNAL_RETAINED,
});

retention_delete_string_enum!(RetentionDeleteDerivedBoundary {
    RedactedDerivedOnly => RETENTION_DELETE_DERIVED_BOUNDARY_REDACTED_DERIVED_ONLY,
    BlockedFromDerivedOutputs =>
        RETENTION_DELETE_DERIVED_BOUNDARY_BLOCKED_FROM_DERIVED_OUTPUTS,
});

retention_delete_string_enum!(RetentionDeleteNonClaim {
    NoUiHideOnly => RETENTION_DELETE_NON_CLAIM_NO_UI_HIDE_ONLY,
    NoResurrection => RETENTION_DELETE_NON_CLAIM_NO_RESURRECTION,
    NoPlainAuditPayload => RETENTION_DELETE_NON_CLAIM_NO_PLAIN_AUDIT_PAYLOAD,
    NoTsBusinessOwner => RETENTION_DELETE_NON_CLAIM_NO_TS_BUSINESS_OWNER,
    NoLanOwnership => RETENTION_DELETE_NON_CLAIM_NO_LAN_OWNERSHIP,
});

retention_delete_text_identifier!(RetentionDeleteContractVersion);
retention_delete_text_identifier!(RetentionDeleteRequestId);
retention_delete_text_identifier!(RetentionDeleteRowId);
retention_delete_text_identifier!(RetentionDeleteFamilyId);
retention_delete_text_identifier!(RetentionDeleteActorId);
retention_delete_text_identifier!(RetentionDeleteActionRef);
retention_delete_text_identifier!(RetentionDeleteTombstoneRef);
retention_delete_text_identifier!(RetentionDeleteReplayRef);
retention_delete_text_identifier!(RetentionDeleteProofRef);
retention_delete_text_identifier!(RetentionDeleteTimestamp);

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
        RetentionDeleteNonClaim::NoUiHideOnly,
        RetentionDeleteNonClaim::NoResurrection,
        RetentionDeleteNonClaim::NoPlainAuditPayload,
        RetentionDeleteNonClaim::NoTsBusinessOwner,
        RetentionDeleteNonClaim::NoLanOwnership,
    ]
}

pub fn retention_delete_policy_matrix() -> Vec<RetentionDeletePolicyRow> {
    let mut rows = retention_delete_policy_rows_primary();
    rows.extend(retention_delete_policy_rows_secondary());
    rows.extend(retention_delete_policy_rows_tertiary());
    rows
}

fn retention_delete_policy_rows_primary() -> Vec<RetentionDeletePolicyRow> {
    [
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::ConfigMetadata,
            source_of_truth: RetentionDeleteSourceOfTruth::HouseholdControlPlane,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_CONFIG_METADATA,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::AccountMetadata,
            source_of_truth: RetentionDeleteSourceOfTruth::AccountControlPlane,
            ocentra_hosted_by_default: true,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: false,
            notes: RETENTION_DELETE_POLICY_NOTE_ACCOUNT_METADATA,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::PolicyHistory,
            source_of_truth: RetentionDeleteSourceOfTruth::HouseholdControlPlane,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_POLICY_HISTORY,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::EvidenceJournal,
            source_of_truth: RetentionDeleteSourceOfTruth::ChildDeviceLocalJournal,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::BlockedFromDerivedOutputs,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_EVIDENCE_JOURNAL,
        }),
    ]
    .iter()
    .map(policy_row)
    .collect()
}

fn retention_delete_policy_rows_secondary() -> Vec<RetentionDeletePolicyRow> {
    [
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::Logs,
            source_of_truth: RetentionDeleteSourceOfTruth::ChildDeviceLocalJournal,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_LOGS,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::Screenshots,
            source_of_truth: RetentionDeleteSourceOfTruth::ChildDeviceLocalEvidence,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::BlockedFromDerivedOutputs,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_SCREENSHOTS,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::NetworkArtifacts,
            source_of_truth: RetentionDeleteSourceOfTruth::ChildDeviceLocalEvidence,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::BlockedFromDerivedOutputs,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_NETWORK_ARTIFACTS,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::AiOutputs,
            source_of_truth: RetentionDeleteSourceOfTruth::ParentOwnedOutput,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_AI_OUTPUTS,
        }),
    ]
    .iter()
    .map(policy_row)
    .collect()
}

fn retention_delete_policy_rows_tertiary() -> Vec<RetentionDeletePolicyRow> {
    [
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::Reports,
            source_of_truth: RetentionDeleteSourceOfTruth::ParentOwnedOutput,
            ocentra_hosted_by_default: false,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_REPORTS,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::Notifications,
            source_of_truth: RetentionDeleteSourceOfTruth::NotificationService,
            ocentra_hosted_by_default: true,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::MinimalRefOnly,
            hard_delete_allowed: true,
            notes: RETENTION_DELETE_POLICY_NOTE_NOTIFICATIONS,
        }),
        policy_row(&RetentionDeletePolicyRowInput {
            data_class: RetentionDeleteDataClass::BillingReferences,
            source_of_truth: RetentionDeleteSourceOfTruth::BillingControlPlane,
            ocentra_hosted_by_default: true,
            encrypted_before_upload: true,
            derived_boundary: RetentionDeleteDerivedBoundary::RedactedDerivedOnly,
            audit_mode: RetentionDeleteAuditMode::ExternalRetained,
            hard_delete_allowed: false,
            notes: RETENTION_DELETE_POLICY_NOTE_BILLING_REFERENCES,
        }),
    ]
    .iter()
    .map(policy_row)
    .collect()
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
        rows: sample_retention_delete_rows(&request),
        non_claims: required_retention_delete_non_claims(),
        report_runtime_claimed: false,
        notification_runtime_claimed: false,
        restore_runtime_claimed: false,
        ts_business_owner_claimed: false,
        updated_at: request_timestamp,
    }
}

fn sample_retention_delete_rows(request: &RetentionDeleteRequest) -> Vec<RetentionDeleteRow> {
    let mut rows = sample_retention_delete_rows_primary(request);
    rows.extend(sample_retention_delete_rows_secondary(request));
    rows.extend(sample_retention_delete_rows_tertiary(request));
    rows
}

fn sample_retention_delete_rows_primary(request: &RetentionDeleteRequest) -> Vec<RetentionDeleteRow> {
    [
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::EvidenceJournal,
                state: RetentionDeleteState::DeleteRequested,
                retention_class: RetentionDeleteRetentionClass::DeleteRequested,
                tombstone_ref: None,
                replay_ref: None,
                tombstone_written: false,
                local_payload_present: true,
                local_payload_redacted: false,
                propagation_pending: false,
                propagated: false,
                replay_blocked: false,
                restore_revival_blocked: false,
                minimal_audit_ref_retained: false,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::PolicyHistory,
                state: RetentionDeleteState::DeleteValidated,
                retention_class: RetentionDeleteRetentionClass::DeleteRequested,
                tombstone_ref: None,
                replay_ref: None,
                tombstone_written: false,
                local_payload_present: true,
                local_payload_redacted: false,
                propagation_pending: false,
                propagated: false,
                replay_blocked: false,
                restore_revival_blocked: false,
                minimal_audit_ref_retained: false,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::EvidenceJournal,
                state: RetentionDeleteState::TombstoneWritten,
                retention_class: RetentionDeleteRetentionClass::DeleteConfirmed,
                tombstone_ref: Some(tombstone_ref(RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_ONE)),
                replay_ref: None,
                tombstone_written: true,
                local_payload_present: true,
                local_payload_redacted: false,
                propagation_pending: false,
                propagated: false,
                replay_blocked: false,
                restore_revival_blocked: false,
                minimal_audit_ref_retained: false,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
    ]
    .iter()
    .map(sample_row)
    .collect()
}

fn sample_retention_delete_rows_secondary(request: &RetentionDeleteRequest) -> Vec<RetentionDeleteRow> {
    [
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::Screenshots,
                state: RetentionDeleteState::LocalRedacted,
                retention_class: RetentionDeleteRetentionClass::DeleteConfirmed,
                tombstone_ref: Some(tombstone_ref(RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_TWO)),
                replay_ref: None,
                tombstone_written: true,
                local_payload_present: false,
                local_payload_redacted: true,
                propagation_pending: false,
                propagated: false,
                replay_blocked: false,
                restore_revival_blocked: false,
                minimal_audit_ref_retained: false,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::NetworkArtifacts,
                state: RetentionDeleteState::PropagationPending,
                retention_class: RetentionDeleteRetentionClass::DeleteConfirmed,
                tombstone_ref: Some(tombstone_ref(RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_THREE)),
                replay_ref: None,
                tombstone_written: true,
                local_payload_present: false,
                local_payload_redacted: true,
                propagation_pending: true,
                propagated: false,
                replay_blocked: false,
                restore_revival_blocked: false,
                minimal_audit_ref_retained: false,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::Reports,
                state: RetentionDeleteState::Propagated,
                retention_class: RetentionDeleteRetentionClass::DeleteConfirmed,
                tombstone_ref: Some(tombstone_ref(RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_FOUR)),
                replay_ref: None,
                tombstone_written: true,
                local_payload_present: false,
                local_payload_redacted: true,
                propagation_pending: false,
                propagated: true,
                replay_blocked: false,
                restore_revival_blocked: false,
                minimal_audit_ref_retained: false,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
    ]
    .iter()
    .map(sample_row)
    .collect()
}

fn sample_retention_delete_rows_tertiary(request: &RetentionDeleteRequest) -> Vec<RetentionDeleteRow> {
    [
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::AiOutputs,
                state: RetentionDeleteState::ReplayProtected,
                retention_class: RetentionDeleteRetentionClass::DeleteConfirmed,
                tombstone_ref: Some(tombstone_ref(RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_FIVE)),
                replay_ref: Some(replay_ref(RETENTION_DELETE_SAMPLE_REPLAY_REF_ONE)),
                tombstone_written: true,
                local_payload_present: false,
                local_payload_redacted: true,
                propagation_pending: false,
                propagated: true,
                replay_blocked: true,
                restore_revival_blocked: false,
                minimal_audit_ref_retained: false,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::Notifications,
                state: RetentionDeleteState::AuditRetained,
                retention_class: RetentionDeleteRetentionClass::AuditMinimal,
                tombstone_ref: Some(tombstone_ref(RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_SIX)),
                replay_ref: Some(replay_ref(RETENTION_DELETE_SAMPLE_REPLAY_REF_TWO)),
                tombstone_written: true,
                local_payload_present: false,
                local_payload_redacted: true,
                propagation_pending: false,
                propagated: true,
                replay_blocked: true,
                restore_revival_blocked: true,
                minimal_audit_ref_retained: true,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: false,
            }),
            sample_row(SampleRowInput {
                request: &request,
                data_class: RetentionDeleteDataClass::Logs,
                state: RetentionDeleteState::HardDeleted,
                retention_class: RetentionDeleteRetentionClass::HardDeleted,
                tombstone_ref: Some(tombstone_ref(RETENTION_DELETE_SAMPLE_TOMBSTONE_REF_SEVEN)),
                replay_ref: Some(replay_ref(RETENTION_DELETE_SAMPLE_REPLAY_REF_THREE)),
                tombstone_written: true,
                local_payload_present: false,
                local_payload_redacted: true,
                propagation_pending: false,
                propagated: true,
                replay_blocked: true,
                restore_revival_blocked: true,
                minimal_audit_ref_retained: true,
                report_export_leak_blocked: true,
                assistant_leak_blocked: true,
                notification_leak_blocked: true,
                hard_deleted: true,
            }),
    ]
    .iter()
    .map(sample_row)
    .collect()
}

struct RetentionDeletePolicyRowInput<'a> {
    data_class: RetentionDeleteDataClass,
    source_of_truth: RetentionDeleteSourceOfTruth,
    ocentra_hosted_by_default: bool,
    encrypted_before_upload: bool,
    derived_boundary: RetentionDeleteDerivedBoundary,
    audit_mode: RetentionDeleteAuditMode,
    hard_delete_allowed: bool,
    notes: &'a str,
}

fn policy_row(input: &RetentionDeletePolicyRowInput<'_>) -> RetentionDeletePolicyRow {
    RetentionDeletePolicyRow {
        data_class: input.data_class,
        source_of_truth: input.source_of_truth,
        ocentra_hosted_by_default: input.ocentra_hosted_by_default,
        encrypted_before_upload: input.encrypted_before_upload,
        derived_boundary: input.derived_boundary,
        audit_mode: input.audit_mode,
        hard_delete_allowed: input.hard_delete_allowed,
        notes: input.notes.to_string(),
    }
}

struct SampleRowInput<'a> {
    request: &'a RetentionDeleteRequest,
    data_class: RetentionDeleteDataClass,
    state: RetentionDeleteState,
    retention_class: RetentionDeleteRetentionClass,
    tombstone_ref: Option<RetentionDeleteTombstoneRef>,
    replay_ref: Option<RetentionDeleteReplayRef>,
    tombstone_written: bool,
    local_payload_present: bool,
    local_payload_redacted: bool,
    propagation_pending: bool,
    propagated: bool,
    replay_blocked: bool,
    restore_revival_blocked: bool,
    minimal_audit_ref_retained: bool,
    report_export_leak_blocked: bool,
    assistant_leak_blocked: bool,
    notification_leak_blocked: bool,
    hard_deleted: bool,
}

fn sample_row(input: SampleRowInput<'_>) -> RetentionDeleteRow {
    let SampleRowInput {
        request,
        data_class,
        state,
        retention_class,
        tombstone_ref,
        replay_ref,
        tombstone_written,
        local_payload_present,
        local_payload_redacted,
        propagation_pending,
        propagated,
        replay_blocked,
        restore_revival_blocked,
        minimal_audit_ref_retained,
        report_export_leak_blocked,
        assistant_leak_blocked,
        notification_leak_blocked,
        hard_deleted,
    } = input;

    RetentionDeleteRow {
        row_id: row_id(format!("retention-delete-row-{}", state.as_str())),
        request_id: request.request_id.clone(),
        data_class,
        state,
        retention_class,
        tombstone_ref,
        replay_ref,
        proof_ref: proof_ref(format!("proof-ref-{}", state.as_str())),
        request_expired: false,
        parent_authorized: request.parent_authorized,
        tombstone_written,
        local_payload_present,
        local_payload_redacted,
        propagation_pending,
        propagated,
        replay_blocked,
        restore_revival_blocked,
        minimal_audit_ref_retained,
        audit_payload_redacted: minimal_audit_ref_retained,
        report_export_leak_blocked,
        assistant_leak_blocked,
        notification_leak_blocked,
        hard_deleted,
        claim_safe: true,
    }
}

fn contract_version(value: &str) -> RetentionDeleteContractVersion {
    crate::schema_option_or_unreachable(
        RetentionDeleteContractVersion::parse(value),
        RETENTION_DELETE_EXPECT_CONTRACT_VERSION,
    )
}

fn request_id(value: &str) -> RetentionDeleteRequestId {
    crate::schema_option_or_unreachable(
        RetentionDeleteRequestId::parse(value),
        RETENTION_DELETE_EXPECT_REQUEST_ID,
    )
}

fn row_id(value: impl Into<String>) -> RetentionDeleteRowId {
    crate::schema_option_or_unreachable(
        RetentionDeleteRowId::parse(value),
        RETENTION_DELETE_EXPECT_ROW_ID,
    )
}

fn family_id(value: &str) -> RetentionDeleteFamilyId {
    crate::schema_option_or_unreachable(
        RetentionDeleteFamilyId::parse(value),
        RETENTION_DELETE_EXPECT_FAMILY_ID,
    )
}

fn actor_id(value: &str) -> RetentionDeleteActorId {
    crate::schema_option_or_unreachable(
        RetentionDeleteActorId::parse(value),
        RETENTION_DELETE_EXPECT_ACTOR_ID,
    )
}

fn action_ref(value: &str) -> RetentionDeleteActionRef {
    crate::schema_option_or_unreachable(
        RetentionDeleteActionRef::parse(value),
        RETENTION_DELETE_EXPECT_ACTION_REF,
    )
}

fn tombstone_ref(value: impl Into<String>) -> RetentionDeleteTombstoneRef {
    crate::schema_option_or_unreachable(
        RetentionDeleteTombstoneRef::parse(value),
        RETENTION_DELETE_EXPECT_TOMBSTONE_REF,
    )
}

fn replay_ref(value: impl Into<String>) -> RetentionDeleteReplayRef {
    crate::schema_option_or_unreachable(
        RetentionDeleteReplayRef::parse(value),
        RETENTION_DELETE_EXPECT_REPLAY_REF,
    )
}

fn proof_ref(value: impl Into<String>) -> RetentionDeleteProofRef {
    crate::schema_option_or_unreachable(
        RetentionDeleteProofRef::parse(value),
        RETENTION_DELETE_EXPECT_PROOF_REF,
    )
}

fn timestamp(value: &str) -> RetentionDeleteTimestamp {
    crate::schema_option_or_unreachable(
        RetentionDeleteTimestamp::parse(value),
        RETENTION_DELETE_EXPECT_TIMESTAMP,
    )
}
