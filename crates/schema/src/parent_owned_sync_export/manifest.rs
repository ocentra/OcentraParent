use super::identifiers::{item_id, policy_ref, version_label};
use super::*;

pub(super) fn sample_manifest_items() -> Vec<ParentOwnedSyncExportManifestItem> {
    vec![
        manifest_item(
            "journal",
            ParentOwnedSyncExportDataClass::EncryptedJournalSegment,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "query",
            ParentOwnedSyncExportDataClass::SqliteQueryRow,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "rule",
            ParentOwnedSyncExportDataClass::ParentRule,
            ParentOwnedSyncExportFormat::EncryptedSupportBundle,
        ),
        manifest_item(
            "approval",
            ParentOwnedSyncExportDataClass::ApprovalDecision,
            ParentOwnedSyncExportFormat::EncryptedSupportBundle,
        ),
        manifest_item(
            "registry",
            ParentOwnedSyncExportDataClass::DeviceRegistryEntry,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "notifications",
            ParentOwnedSyncExportDataClass::NotificationHistory,
            ParentOwnedSyncExportFormat::EncryptedSupportBundle,
        ),
        manifest_item(
            "audit",
            ParentOwnedSyncExportDataClass::AuditEvent,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "summary",
            ParentOwnedSyncExportDataClass::GeneratedSummary,
            ParentOwnedSyncExportFormat::HumanReadableParentReport,
        ),
    ]
}

fn manifest_item(
    suffix: impl AsRef<str>,
    data_class: ParentOwnedSyncExportDataClass,
    export_format: ParentOwnedSyncExportFormat,
) -> ParentOwnedSyncExportManifestItem {
    let suffix = suffix.as_ref();
    let human_readable = export_format == ParentOwnedSyncExportFormat::HumanReadableParentReport;

    ParentOwnedSyncExportManifestItem {
        item_id: item_id(format!("manifest-item-{suffix}")),
        data_class,
        export_format,
        destination_ownership:
            ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
        schema_version_label: version_label(format!("{suffix}.v1")),
        encryption: ParentOwnedSyncExportEncryptionMetadata {
            encryption_state: if human_readable {
                ParentOwnedSyncExportEncryptionState::HumanReadableParentAuthorized
            } else {
                ParentOwnedSyncExportEncryptionState::EncryptedAtRest
            },
            encrypted_before_upload: !human_readable,
            key_owner: ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            proof_requirement_ref: policy_ref(format!("encryption-proof-{suffix}")),
        },
        parent_action_required: true,
        raw_child_evidence_uploaded_by_default: false,
        ocentra_hosted_family_data_stored: false,
        claim_safe: true,
    }
}
