use serde::{Deserialize, Serialize};

use super::{
    RETENTION_DELETE_ACTOR_ROLE_GUARDIAN, RETENTION_DELETE_ACTOR_ROLE_PARENT,
    RETENTION_DELETE_ACTOR_ROLE_SUPPORT, RETENTION_DELETE_ACTOR_ROLE_SYSTEM,
    RETENTION_DELETE_AUDIT_MODE_EXTERNAL_RETAINED, RETENTION_DELETE_AUDIT_MODE_MINIMAL_REF_ONLY,
    RETENTION_DELETE_DATA_CLASS_ACCOUNT_METADATA, RETENTION_DELETE_DATA_CLASS_AI_OUTPUTS,
    RETENTION_DELETE_DATA_CLASS_BILLING_REFERENCES, RETENTION_DELETE_DATA_CLASS_CONFIG_METADATA,
    RETENTION_DELETE_DATA_CLASS_EVIDENCE_JOURNAL, RETENTION_DELETE_DATA_CLASS_LOGS,
    RETENTION_DELETE_DATA_CLASS_NETWORK_ARTIFACTS, RETENTION_DELETE_DATA_CLASS_NOTIFICATIONS,
    RETENTION_DELETE_DATA_CLASS_POLICY_HISTORY, RETENTION_DELETE_DATA_CLASS_REPORTS,
    RETENTION_DELETE_DATA_CLASS_SCREENSHOTS,
    RETENTION_DELETE_DERIVED_BOUNDARY_BLOCKED_FROM_DERIVED_OUTPUTS,
    RETENTION_DELETE_DERIVED_BOUNDARY_REDACTED_DERIVED_ONLY,
    RETENTION_DELETE_NON_CLAIM_NO_LAN_OWNERSHIP, RETENTION_DELETE_NON_CLAIM_NO_PLAIN_AUDIT_PAYLOAD,
    RETENTION_DELETE_NON_CLAIM_NO_RESURRECTION, RETENTION_DELETE_NON_CLAIM_NO_TS_BUSINESS_OWNER,
    RETENTION_DELETE_NON_CLAIM_NO_UI_HIDE_ONLY, RETENTION_DELETE_RETENTION_CLASS_ACTIVE_WINDOW,
    RETENTION_DELETE_RETENTION_CLASS_AUDIT_MINIMAL,
    RETENTION_DELETE_RETENTION_CLASS_DELETE_CONFIRMED,
    RETENTION_DELETE_RETENTION_CLASS_DELETE_REQUESTED,
    RETENTION_DELETE_RETENTION_CLASS_HARD_DELETED,
    RETENTION_DELETE_SOURCE_OF_TRUTH_ACCOUNT_CONTROL_PLANE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_BILLING_CONTROL_PLANE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_EVIDENCE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_JOURNAL,
    RETENTION_DELETE_SOURCE_OF_TRUTH_HOUSEHOLD_CONTROL_PLANE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_NOTIFICATION_SERVICE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_PARENT_OWNED_OUTPUT, RETENTION_DELETE_STATE_AUDIT_RETAINED,
    RETENTION_DELETE_STATE_DELETE_REQUESTED, RETENTION_DELETE_STATE_DELETE_VALIDATED,
    RETENTION_DELETE_STATE_HARD_DELETED, RETENTION_DELETE_STATE_LOCAL_REDACTED,
    RETENTION_DELETE_STATE_PROPAGATED, RETENTION_DELETE_STATE_PROPAGATION_PENDING,
    RETENTION_DELETE_STATE_REPLAY_PROTECTED, RETENTION_DELETE_STATE_TOMBSTONE_WRITTEN,
};

const RETENTION_DELETE_ACTOR_ROLE_VARIANTS: [&str; 4] = [
    RETENTION_DELETE_ACTOR_ROLE_PARENT,
    RETENTION_DELETE_ACTOR_ROLE_GUARDIAN,
    RETENTION_DELETE_ACTOR_ROLE_SUPPORT,
    RETENTION_DELETE_ACTOR_ROLE_SYSTEM,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteActorRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "guardian")]
    Guardian,
    #[serde(rename = "support")]
    Support,
    #[serde(rename = "system")]
    System,
}

impl RetentionDeleteActorRole {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_ACTOR_ROLE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const RETENTION_DELETE_STATE_VARIANTS: [&str; 9] = [
    RETENTION_DELETE_STATE_DELETE_REQUESTED,
    RETENTION_DELETE_STATE_DELETE_VALIDATED,
    RETENTION_DELETE_STATE_TOMBSTONE_WRITTEN,
    RETENTION_DELETE_STATE_LOCAL_REDACTED,
    RETENTION_DELETE_STATE_PROPAGATION_PENDING,
    RETENTION_DELETE_STATE_PROPAGATED,
    RETENTION_DELETE_STATE_REPLAY_PROTECTED,
    RETENTION_DELETE_STATE_AUDIT_RETAINED,
    RETENTION_DELETE_STATE_HARD_DELETED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteState {
    #[serde(rename = "deleteRequested")]
    DeleteRequested,
    #[serde(rename = "deleteValidated")]
    DeleteValidated,
    #[serde(rename = "tombstoneWritten")]
    TombstoneWritten,
    #[serde(rename = "localRedacted")]
    LocalRedacted,
    #[serde(rename = "propagationPending")]
    PropagationPending,
    #[serde(rename = "propagated")]
    Propagated,
    #[serde(rename = "replayProtected")]
    ReplayProtected,
    #[serde(rename = "auditRetained")]
    AuditRetained,
    #[serde(rename = "hardDeleted")]
    HardDeleted,
}

impl RetentionDeleteState {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const RETENTION_DELETE_DATA_CLASS_VARIANTS: [&str; 11] = [
    RETENTION_DELETE_DATA_CLASS_CONFIG_METADATA,
    RETENTION_DELETE_DATA_CLASS_ACCOUNT_METADATA,
    RETENTION_DELETE_DATA_CLASS_POLICY_HISTORY,
    RETENTION_DELETE_DATA_CLASS_EVIDENCE_JOURNAL,
    RETENTION_DELETE_DATA_CLASS_LOGS,
    RETENTION_DELETE_DATA_CLASS_SCREENSHOTS,
    RETENTION_DELETE_DATA_CLASS_NETWORK_ARTIFACTS,
    RETENTION_DELETE_DATA_CLASS_AI_OUTPUTS,
    RETENTION_DELETE_DATA_CLASS_REPORTS,
    RETENTION_DELETE_DATA_CLASS_NOTIFICATIONS,
    RETENTION_DELETE_DATA_CLASS_BILLING_REFERENCES,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteDataClass {
    #[serde(rename = "config-metadata")]
    ConfigMetadata,
    #[serde(rename = "account-metadata")]
    AccountMetadata,
    #[serde(rename = "policy-history")]
    PolicyHistory,
    #[serde(rename = "evidence-journal")]
    EvidenceJournal,
    #[serde(rename = "logs")]
    Logs,
    #[serde(rename = "screenshots")]
    Screenshots,
    #[serde(rename = "network-artifacts")]
    NetworkArtifacts,
    #[serde(rename = "ai-outputs")]
    AiOutputs,
    #[serde(rename = "reports")]
    Reports,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "billing-references")]
    BillingReferences,
}

impl RetentionDeleteDataClass {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_DATA_CLASS_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const RETENTION_DELETE_SOURCE_OF_TRUTH_VARIANTS: [&str; 7] = [
    RETENTION_DELETE_SOURCE_OF_TRUTH_HOUSEHOLD_CONTROL_PLANE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_ACCOUNT_CONTROL_PLANE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_BILLING_CONTROL_PLANE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_JOURNAL,
    RETENTION_DELETE_SOURCE_OF_TRUTH_CHILD_DEVICE_LOCAL_EVIDENCE,
    RETENTION_DELETE_SOURCE_OF_TRUTH_PARENT_OWNED_OUTPUT,
    RETENTION_DELETE_SOURCE_OF_TRUTH_NOTIFICATION_SERVICE,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteSourceOfTruth {
    #[serde(rename = "household-control-plane")]
    HouseholdControlPlane,
    #[serde(rename = "account-control-plane")]
    AccountControlPlane,
    #[serde(rename = "billing-control-plane")]
    BillingControlPlane,
    #[serde(rename = "child-device-local-journal")]
    ChildDeviceLocalJournal,
    #[serde(rename = "child-device-local-evidence")]
    ChildDeviceLocalEvidence,
    #[serde(rename = "parent-owned-output")]
    ParentOwnedOutput,
    #[serde(rename = "notification-service")]
    NotificationService,
}

impl RetentionDeleteSourceOfTruth {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_SOURCE_OF_TRUTH_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const RETENTION_DELETE_RETENTION_CLASS_VARIANTS: [&str; 5] = [
    RETENTION_DELETE_RETENTION_CLASS_ACTIVE_WINDOW,
    RETENTION_DELETE_RETENTION_CLASS_DELETE_REQUESTED,
    RETENTION_DELETE_RETENTION_CLASS_DELETE_CONFIRMED,
    RETENTION_DELETE_RETENTION_CLASS_AUDIT_MINIMAL,
    RETENTION_DELETE_RETENTION_CLASS_HARD_DELETED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteRetentionClass {
    #[serde(rename = "active-window")]
    ActiveWindow,
    #[serde(rename = "delete-requested")]
    DeleteRequested,
    #[serde(rename = "delete-confirmed")]
    DeleteConfirmed,
    #[serde(rename = "audit-minimal")]
    AuditMinimal,
    #[serde(rename = "hard-deleted")]
    HardDeleted,
}

impl RetentionDeleteRetentionClass {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_RETENTION_CLASS_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const RETENTION_DELETE_AUDIT_MODE_VARIANTS: [&str; 2] = [
    RETENTION_DELETE_AUDIT_MODE_MINIMAL_REF_ONLY,
    RETENTION_DELETE_AUDIT_MODE_EXTERNAL_RETAINED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteAuditMode {
    #[serde(rename = "minimal-ref-only")]
    MinimalRefOnly,
    #[serde(rename = "external-retained")]
    ExternalRetained,
}

impl RetentionDeleteAuditMode {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_AUDIT_MODE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const RETENTION_DELETE_DERIVED_BOUNDARY_VARIANTS: [&str; 2] = [
    RETENTION_DELETE_DERIVED_BOUNDARY_REDACTED_DERIVED_ONLY,
    RETENTION_DELETE_DERIVED_BOUNDARY_BLOCKED_FROM_DERIVED_OUTPUTS,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteDerivedBoundary {
    #[serde(rename = "redacted-derived-only")]
    RedactedDerivedOnly,
    #[serde(rename = "blocked-from-derived-outputs")]
    BlockedFromDerivedOutputs,
}

impl RetentionDeleteDerivedBoundary {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_DERIVED_BOUNDARY_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const RETENTION_DELETE_NON_CLAIM_VARIANTS: [&str; 5] = [
    RETENTION_DELETE_NON_CLAIM_NO_UI_HIDE_ONLY,
    RETENTION_DELETE_NON_CLAIM_NO_RESURRECTION,
    RETENTION_DELETE_NON_CLAIM_NO_PLAIN_AUDIT_PAYLOAD,
    RETENTION_DELETE_NON_CLAIM_NO_TS_BUSINESS_OWNER,
    RETENTION_DELETE_NON_CLAIM_NO_LAN_OWNERSHIP,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum RetentionDeleteNonClaim {
    #[serde(rename = "no-ui-hide-only")]
    UiHideOnly,
    #[serde(rename = "no-resurrection")]
    Resurrection,
    #[serde(rename = "no-plain-audit-payload")]
    PlainAuditPayload,
    #[serde(rename = "no-ts-business-owner")]
    TsBusinessOwner,
    #[serde(rename = "no-lan-ownership")]
    LanOwnership,
}

impl RetentionDeleteNonClaim {
    pub const VARIANTS: &'static [&'static str] = &RETENTION_DELETE_NON_CLAIM_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}
