use super::*;

pub(super) fn retention_delete_policy_matrix() -> Vec<RetentionDeletePolicyRow> {
    let mut rows = retention_delete_policy_rows_primary();
    rows.extend(retention_delete_policy_rows_secondary());
    rows.extend(retention_delete_policy_rows_tertiary());
    rows
}

fn retention_delete_policy_rows_primary() -> Vec<RetentionDeletePolicyRow> {
    vec![
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
}

fn retention_delete_policy_rows_secondary() -> Vec<RetentionDeletePolicyRow> {
    vec![
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
}

fn retention_delete_policy_rows_tertiary() -> Vec<RetentionDeletePolicyRow> {
    vec![
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
