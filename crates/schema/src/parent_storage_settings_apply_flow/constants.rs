pub(super) const PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION: &str =
    "parent-storage-settings-apply-flow-proof";
pub(super) const PARENT_STORAGE_CONTRACT_VERSION_VALUE: &str = "v0.6";
pub(super) const PARENT_STORAGE_ROW_ID_VALUE: &str = "parent-storage-settings-row-proof-1";
pub(super) const PARENT_STORAGE_LAST_SUCCESS_AT_VALUE: &str = "2026-06-28T19:10:00.000Z";
pub(super) const PARENT_STORAGE_LAST_FAILURE_AT_VALUE: &str = "2026-06-28T19:12:00.000Z";
pub(super) const PARENT_STORAGE_RESTORE_PREVIEW_ID_VALUE: &str = "restore-preview-proof-1";
pub(super) const PARENT_STORAGE_RESTORE_PREVIEW_CREATED_AT_VALUE: &str = "2026-06-28T19:14:00.000Z";
pub(super) const PARENT_STORAGE_PRODUCT_VERSION_VALUE: &str = "2026.06.28";
pub(super) const PARENT_STORAGE_EXPORT_SCHEMA_VERSION_VALUE: &str =
    "export-import-backup-recovery-proof";
pub(super) const PARENT_STORAGE_APPLY_DECISION_ID_VALUE: &str = "apply-decision-proof-1";
pub(super) const PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_ACTION_ID: &str = "delete-local-evidence";
pub(super) const PARENT_STORAGE_DELETE_PARENT_CACHE_ACTION_ID: &str = "delete-parent-cache";
pub(super) const PARENT_STORAGE_DELETE_GENERATED_REPORT_ACTION_ID: &str = "delete-generated-report";
pub(super) const PARENT_STORAGE_DELETE_PROVIDER_COPY_ACTION_ID: &str = "delete-provider-copy";
pub(super) const PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_ACTION_ID: &str = "delete-support-bundle";
pub(super) const PARENT_STORAGE_DELETE_OCENTRA_METADATA_ACTION_ID: &str = "delete-ocentra-metadata";
pub(super) const PARENT_STORAGE_DISCONNECT_ACTION_ID: &str = "disconnect-provider-proof-1";
pub(super) const PARENT_STORAGE_UPDATED_AT_VALUE: &str = "2026-06-28T19:16:00.000Z";
pub(super) const PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_PRESERVED: &str =
    "notification-history tombstone preserved";
pub(super) const PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_CONFLICT: &str =
    "notification-history tombstone conflict";
pub(super) const PARENT_STORAGE_SUMMARY_MANUAL_PROOF_REQUIRED: &str =
    "Manual proof required before provider-backed apply or delete proceeds.";
pub(super) const PARENT_STORAGE_MANUAL_REVIEW_REQUIRED_NOTE: &str =
    "Manual review is required before any blocked section can be reconsidered.";
pub(super) const PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_NOTE: &str =
    "Local delete remains separate from disconnect and provider delete.";
pub(super) const PARENT_STORAGE_DELETE_PARENT_CACHE_NOTE: &str =
    "Parent cache delete is separate from evidence delete.";
pub(super) const PARENT_STORAGE_DELETE_GENERATED_REPORT_NOTE: &str =
    "Generated report delete does not imply source evidence delete.";
pub(super) const PARENT_STORAGE_DELETE_PROVIDER_COPY_NOTE: &str =
    "Provider delete remains manual-required until provider runtime proof exists.";
pub(super) const PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_NOTE: &str =
    "Support bundle delete is separate from provider or local evidence delete.";
pub(super) const PARENT_STORAGE_DELETE_OCENTRA_METADATA_NOTE: &str =
    "Control-plane metadata delete remains a distinct parent action.";
pub(super) const PARENT_STORAGE_DISCONNECT_NOTE: &str =
    "Disconnect stops future sync only; existing provider files may remain.";
pub(super) const PARENT_STORAGE_CLAIM_SAFE_COPY_NOTE: &str =
    "Claim-safe copy only; no success-looking shorthand without proof state.";
pub(super) const PARENT_STORAGE_COPY_CUSTODY_BOUNDARY_STATEMENT: &str =
    "Ocentra does not store child activity data by default.";
pub(super) const PARENT_STORAGE_COPY_METADATA_LEAKAGE_STATEMENT: &str = "Your selected storage provider may see encrypted file metadata such as file name, size, and modified time.";
pub(super) const PARENT_STORAGE_COPY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD_STATEMENT: &str =
    "Sensitive data is encrypted before upload.";
pub(super) const PARENT_STORAGE_COPY_LOST_KEY_MAY_BE_UNRECOVERABLE_STATEMENT: &str = "If you lose your recovery key or device keys, Ocentra may not be able to recover encrypted child activity data.";
pub(super) const PARENT_STORAGE_COPY_DISCONNECT_DOES_NOT_DELETE_STATEMENT: &str = "Disconnecting a provider stops future sync but does not automatically delete files already written there unless you request deletion and proof succeeds.";
pub(super) const PARENT_STORAGE_COPY_TOMBSTONES_MAY_BE_REQUIRED_STATEMENT: &str =
    "Deleting local data may require tombstones so old backups do not restore deleted evidence.";
pub(super) const PARENT_STORAGE_COPY_BACKUP_QUEUED_STATEMENT: &str = "Backup queued.";
pub(super) const PARENT_STORAGE_COPY_PROVIDER_UPLOAD_PENDING_STATEMENT: &str =
    "Provider upload pending.";
pub(super) const PARENT_STORAGE_COPY_PROVIDER_UPLOAD_FAILED_STATEMENT: &str =
    "Provider upload failed.";
pub(super) const PARENT_STORAGE_COPY_PROVIDER_UPLOAD_CONFIRMED_STATEMENT: &str =
    "Provider upload confirmed.";
pub(super) const PARENT_STORAGE_COPY_IMPORT_PREVIEW_PASSED_STATEMENT: &str =
    "Import preview passed.";
pub(super) const PARENT_STORAGE_COPY_APPLY_REQUIRES_CONFIRMATION_STATEMENT: &str =
    "Apply requires confirmation.";
pub(super) const PARENT_STORAGE_COPY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING_STATEMENT: &str =
    "Deleted locally; provider delete pending.";
pub(super) const PARENT_STORAGE_COPY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN_STATEMENT:
    &str = "Provider disconnected; existing files may remain.";
pub(super) const PARENT_STORAGE_COPY_MANUAL_PROOF_REQUIRED_STATEMENT: &str =
    "Manual proof required.";
pub(super) const PARENT_STORAGE_KNOWN_GAP_FINAL_PORTAL_RENDERING: &str =
    "Final portal rendering remains owned by portal-ux-household-surfaces-plan.";
pub(super) const PARENT_STORAGE_KNOWN_GAP_DESKTOP_HOST_WIRING: &str =
    "Desktop host wiring remains owned by parent-client-runtime-distribution-plan.";
pub(super) const PARENT_STORAGE_KNOWN_GAP_PROVIDER_SDK_RUNTIME: &str =
    "Provider SDK runtime remains unclaimed for this packet.";
pub(super) const PARENT_STORAGE_KNOWN_GAP_AUTOMATIC_PROVIDER_DELETE_OR_APPLY: &str =
    "Automatic provider delete or apply execution remains unclaimed for this packet.";
pub(super) const PARENT_STORAGE_EXPECT_CONTRACT_VERSION: &str = "contract version";
pub(super) const PARENT_STORAGE_EXPECT_ROW_ID: &str = "row id";
pub(super) const PARENT_STORAGE_EXPECT_PREVIEW_ID: &str = "preview id";
pub(super) const PARENT_STORAGE_EXPECT_APPLY_ID: &str = "apply id";
pub(super) const PARENT_STORAGE_EXPECT_ACTION_ID: &str = "action id";
pub(super) const PARENT_STORAGE_EXPECT_TIMESTAMP: &str = "timestamp";

pub(super) const PARENT_STORAGE_MODE_LABEL_LOCAL_ONLY: &str = "local-only";
pub(super) const PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_BACKUP: &str =
    "local-plus-encrypted-backup";
pub(super) const PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_PROVIDER_SYNC: &str =
    "local-plus-encrypted-provider-sync";
pub(super) const PARENT_STORAGE_MODE_LABEL_PROVIDER_DISCONNECTED: &str = "provider-disconnected";
pub(super) const PARENT_STORAGE_MODE_LABEL_PROVIDER_ERROR: &str = "provider-error";
pub(super) const PARENT_STORAGE_MODE_LABEL_MANUAL_REQUIRED: &str = "manual-required";
pub(super) const PARENT_STORAGE_MODE_LABEL_DISABLED: &str = "disabled";

pub(super) const PARENT_STORAGE_UI_STATE_PROVIDER_NOT_CONFIGURED: &str = "providerNotConfigured";
pub(super) const PARENT_STORAGE_UI_STATE_PROVIDER_AUTH_EXPIRED: &str = "providerAuthExpired";
pub(super) const PARENT_STORAGE_UI_STATE_PROVIDER_PERMISSION_MISSING: &str =
    "providerPermissionMissing";
pub(super) const PARENT_STORAGE_UI_STATE_PROVIDER_REVOKED: &str = "providerRevoked";
pub(super) const PARENT_STORAGE_UI_STATE_PROVIDER_QUOTA_EXCEEDED: &str = "providerQuotaExceeded";
pub(super) const PARENT_STORAGE_UI_STATE_PROVIDER_UNAVAILABLE: &str = "providerUnavailable";
pub(super) const PARENT_STORAGE_UI_STATE_LOCAL_STORE_UNAVAILABLE: &str = "localStoreUnavailable";
pub(super) const PARENT_STORAGE_UI_STATE_KEY_UNAVAILABLE: &str = "keyUnavailable";
pub(super) const PARENT_STORAGE_UI_STATE_KEY_REVOKED: &str = "keyRevoked";
pub(super) const PARENT_STORAGE_UI_STATE_WRONG_HOUSEHOLD: &str = "wrongHousehold";
pub(super) const PARENT_STORAGE_UI_STATE_WRONG_DEVICE: &str = "wrongDevice";
pub(super) const PARENT_STORAGE_UI_STATE_SCHEMA_UNSUPPORTED: &str = "schemaUnsupported";
pub(super) const PARENT_STORAGE_UI_STATE_BUNDLE_CORRUPT: &str = "bundleCorrupt";
pub(super) const PARENT_STORAGE_UI_STATE_TOMBSTONE_CONFLICT: &str = "tombstoneConflict";
pub(super) const PARENT_STORAGE_UI_STATE_MANUAL_REQUIRED: &str = "manualRequired";
pub(super) const PARENT_STORAGE_UI_STATE_OFFLINE_QUEUED: &str = "offlineQueued";
pub(super) const PARENT_STORAGE_UI_STATE_SYNC_DISABLED: &str = "syncDisabled";
pub(super) const PARENT_STORAGE_UI_STATE_REMOTE_DISABLED: &str = "remoteDisabled";
pub(super) const PARENT_STORAGE_UI_STATE_OCENTRA_HOSTED_STORAGE_NOT_USED: &str =
    "ocentraHostedStorageNotUsed";
pub(super) const PARENT_STORAGE_UI_STATE_READY: &str = "ready";

pub(super) const PARENT_STORAGE_ENCRYPTION_STATUS_ENCRYPTED_BEFORE_UPLOAD: &str =
    "encrypted-before-upload";
pub(super) const PARENT_STORAGE_ENCRYPTION_STATUS_HUMAN_READABLE_PARENT_AUTHORIZED: &str =
    "human-readable-parent-authorized";
pub(super) const PARENT_STORAGE_ENCRYPTION_STATUS_NOT_APPLICABLE: &str = "not-applicable";
pub(super) const PARENT_STORAGE_ENCRYPTION_STATUS_MANUAL_REQUIRED: &str = "manual-required";

pub(super) const PARENT_STORAGE_KEY_STATUS_KEY_AVAILABLE: &str = "keyAvailable";
pub(super) const PARENT_STORAGE_KEY_STATUS_KEY_UNAVAILABLE: &str = "keyUnavailable";
pub(super) const PARENT_STORAGE_KEY_STATUS_KEY_REVOKED: &str = "keyRevoked";
pub(super) const PARENT_STORAGE_KEY_STATUS_MANUAL_REQUIRED: &str = "manualRequired";

pub(super) const PARENT_STORAGE_PREVIEW_STATE_IMPORT_PREVIEW_PASSED: &str = "importPreviewPassed";
pub(super) const PARENT_STORAGE_PREVIEW_STATE_PARTIAL_RESTORE: &str = "partialRestore";
pub(super) const PARENT_STORAGE_PREVIEW_STATE_WRONG_HOUSEHOLD: &str = "wrongHousehold";
pub(super) const PARENT_STORAGE_PREVIEW_STATE_WRONG_KEY: &str = "wrongKey";
pub(super) const PARENT_STORAGE_PREVIEW_STATE_SCHEMA_UNSUPPORTED: &str = "schemaUnsupported";
pub(super) const PARENT_STORAGE_PREVIEW_STATE_BUNDLE_CORRUPT: &str = "bundleCorrupt";
pub(super) const PARENT_STORAGE_PREVIEW_STATE_TOMBSTONE_CONFLICT: &str = "tombstoneConflict";
pub(super) const PARENT_STORAGE_PREVIEW_STATE_MANUAL_REQUIRED: &str = "manualRequired";

pub(super) const PARENT_STORAGE_APPLY_STATE_NOT_STARTED: &str = "notStarted";
pub(super) const PARENT_STORAGE_APPLY_STATE_APPLY_REQUIRES_CONFIRMATION: &str =
    "applyRequiresConfirmation";
pub(super) const PARENT_STORAGE_APPLY_STATE_APPLY_PENDING: &str = "applyPending";
pub(super) const PARENT_STORAGE_APPLY_STATE_APPLIED: &str = "applied";
pub(super) const PARENT_STORAGE_APPLY_STATE_PARTIAL: &str = "partial";
pub(super) const PARENT_STORAGE_APPLY_STATE_ROLLBACK_MANUAL_REQUIRED: &str =
    "rollbackManualRequired";
pub(super) const PARENT_STORAGE_APPLY_STATE_BLOCKED_MANUAL_REQUIRED: &str = "blockedManualRequired";

pub(super) const PARENT_STORAGE_DELETE_ACTION_KIND_LOCAL_CHILD_EVIDENCE: &str =
    "delete-local-child-evidence";
pub(super) const PARENT_STORAGE_DELETE_ACTION_KIND_PARENT_PORTAL_CACHE: &str =
    "delete-parent-portal-cache";
pub(super) const PARENT_STORAGE_DELETE_ACTION_KIND_GENERATED_REPORT: &str =
    "delete-generated-report";
pub(super) const PARENT_STORAGE_DELETE_ACTION_KIND_PROVIDER_BACKUP_COPY: &str =
    "delete-provider-backup-copy";
pub(super) const PARENT_STORAGE_DELETE_ACTION_KIND_SUPPORT_BUNDLE: &str = "delete-support-bundle";
pub(super) const PARENT_STORAGE_DELETE_ACTION_KIND_OCENTRA_METADATA: &str =
    "delete-ocentra-metadata";

pub(super) const PARENT_STORAGE_COPY_KEY_CUSTODY_BOUNDARY: &str = "custody-boundary";
pub(super) const PARENT_STORAGE_COPY_KEY_METADATA_LEAKAGE: &str = "metadata-leakage";
pub(super) const PARENT_STORAGE_COPY_KEY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD: &str =
    "sensitive-encrypted-before-upload";
pub(super) const PARENT_STORAGE_COPY_KEY_LOST_KEY_MAY_BE_UNRECOVERABLE: &str =
    "lost-key-may-be-unrecoverable";
pub(super) const PARENT_STORAGE_COPY_KEY_DISCONNECT_DOES_NOT_DELETE: &str =
    "disconnect-does-not-delete";
pub(super) const PARENT_STORAGE_COPY_KEY_TOMBSTONES_MAY_BE_REQUIRED: &str =
    "tombstones-may-be-required";
pub(super) const PARENT_STORAGE_COPY_KEY_BACKUP_QUEUED: &str = "backup-queued";
pub(super) const PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_PENDING: &str = "provider-upload-pending";
pub(super) const PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_FAILED: &str = "provider-upload-failed";
pub(super) const PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_CONFIRMED: &str =
    "provider-upload-confirmed";
pub(super) const PARENT_STORAGE_COPY_KEY_IMPORT_PREVIEW_PASSED: &str = "import-preview-passed";
pub(super) const PARENT_STORAGE_COPY_KEY_APPLY_REQUIRES_CONFIRMATION: &str =
    "apply-requires-confirmation";
pub(super) const PARENT_STORAGE_COPY_KEY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING: &str =
    "deleted-locally-provider-delete-pending";
pub(super) const PARENT_STORAGE_COPY_KEY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN: &str =
    "provider-disconnected-existing-files-may-remain";
pub(super) const PARENT_STORAGE_COPY_KEY_MANUAL_PROOF_REQUIRED: &str = "manual-proof-required";

pub(super) const PARENT_STORAGE_NO_CLAIM_PORTAL_IMPLEMENTATION_READY: &str =
    "no-portal-implementation-ready";
pub(super) const PARENT_STORAGE_NO_CLAIM_PROVIDER_RUNTIME_READY: &str = "no-provider-runtime-ready";
pub(super) const PARENT_STORAGE_NO_CLAIM_AUTO_APPLY: &str = "no-auto-apply";
pub(super) const PARENT_STORAGE_NO_CLAIM_DISCONNECT_DELETES_PROVIDER_DATA: &str =
    "no-disconnect-deletes-provider-data";
pub(super) const PARENT_STORAGE_NO_CLAIM_DELETE_DISCONNECT_COLLAPSE: &str =
    "no-delete-disconnect-collapse";
pub(super) const PARENT_STORAGE_NO_CLAIM_TS_BUSINESS_OWNER: &str = "no-ts-business-owner";
pub(super) const PARENT_STORAGE_NO_CLAIM_LAN_OWNERSHIP: &str = "no-lan-ownership";
