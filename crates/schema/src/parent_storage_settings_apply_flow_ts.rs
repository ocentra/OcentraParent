use super::parent_storage_settings_apply_flow::{
    parent_storage_settings_apply_flow_known_gaps,
    sample_parent_storage_settings_apply_flow_contract_proof,
    PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION,
};

const PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_EXPECTATION: &str =
    "parent storage settings apply flow proof json";
const PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAP_SEPARATOR: &str = "\n";

pub fn parent_storage_settings_apply_flow_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_parent_storage_settings_apply_flow_contract_proof()),
        PARENT_STORAGE_SETTINGS_APPLY_FLOW_PROOF_JSON_EXPECTATION,
    );
    let known_gaps = parent_storage_settings_apply_flow_known_gaps()
        .iter()
        .map(|gap| format!("  {:?},", gap))
        .collect::<Vec<_>>()
        .join(PARENT_STORAGE_SETTINGS_APPLY_FLOW_KNOWN_GAP_SEPARATOR);

    format!(
        r#"/* generated from crates/schema/src/parent_storage_settings_apply_flow.rs */

import type {{
  GeneratedParentOwnedSyncDeleteVisibilityState,
  GeneratedParentOwnedSyncDisconnectVisibilityState,
  GeneratedParentOwnedSyncExportDataClass,
  GeneratedParentOwnedSyncProviderMode,
  GeneratedParentOwnedSyncProviderStatus,
  GeneratedParentOwnedSyncState,
}} from './parent-owned-sync-export-contracts';

export const ParentStorageSettingsApplyFlowContractRuntime = {{
  SchemaVersion: '{schema_version}',
}} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export type GeneratedParentStorageSettingsRowId = string;
export type GeneratedParentStoragePreviewId = string;
export type GeneratedParentStorageApplyId = string;
export type GeneratedParentStorageActionId = string;
export type GeneratedParentStorageTimestamp = string;
export type GeneratedParentStorageModeLabel =
  | 'local-only'
  | 'local-plus-encrypted-backup'
  | 'local-plus-encrypted-provider-sync'
  | 'provider-disconnected'
  | 'provider-error'
  | 'manual-required'
  | 'disabled';
export type GeneratedParentStorageUiState =
  | 'providerNotConfigured'
  | 'providerAuthExpired'
  | 'providerPermissionMissing'
  | 'providerRevoked'
  | 'providerQuotaExceeded'
  | 'providerUnavailable'
  | 'localStoreUnavailable'
  | 'keyUnavailable'
  | 'keyRevoked'
  | 'wrongHousehold'
  | 'wrongDevice'
  | 'schemaUnsupported'
  | 'bundleCorrupt'
  | 'tombstoneConflict'
  | 'manualRequired'
  | 'offlineQueued'
  | 'syncDisabled'
  | 'remoteDisabled'
  | 'ocentraHostedStorageNotUsed'
  | 'ready';
export type GeneratedParentStorageEncryptionStatus =
  | 'encrypted-before-upload'
  | 'human-readable-parent-authorized'
  | 'not-applicable'
  | 'manual-required';
export type GeneratedParentStorageKeyStatus =
  | 'keyAvailable'
  | 'keyUnavailable'
  | 'keyRevoked'
  | 'manualRequired';
export type GeneratedParentStoragePreviewState =
  | 'importPreviewPassed'
  | 'partialRestore'
  | 'wrongHousehold'
  | 'wrongKey'
  | 'schemaUnsupported'
  | 'bundleCorrupt'
  | 'tombstoneConflict'
  | 'manualRequired';
export type GeneratedParentStorageApplyState =
  | 'notStarted'
  | 'applyRequiresConfirmation'
  | 'applyPending'
  | 'applied'
  | 'partial'
  | 'rollbackManualRequired'
  | 'blockedManualRequired';
export type GeneratedParentStorageDeleteActionKind =
  | 'delete-local-child-evidence'
  | 'delete-parent-portal-cache'
  | 'delete-generated-report'
  | 'delete-provider-backup-copy'
  | 'delete-support-bundle'
  | 'delete-ocentra-metadata';
export type GeneratedParentStorageCopyKey =
  | 'custody-boundary'
  | 'metadata-leakage'
  | 'sensitive-encrypted-before-upload'
  | 'lost-key-may-be-unrecoverable'
  | 'disconnect-does-not-delete'
  | 'tombstones-may-be-required'
  | 'backup-queued'
  | 'provider-upload-pending'
  | 'provider-upload-failed'
  | 'provider-upload-confirmed'
  | 'import-preview-passed'
  | 'apply-requires-confirmation'
  | 'deleted-locally-provider-delete-pending'
  | 'provider-disconnected-existing-files-may-remain'
  | 'manual-proof-required';
export type GeneratedParentStorageNoClaim =
  | 'no-portal-implementation-ready'
  | 'no-provider-runtime-ready'
  | 'no-auto-apply'
  | 'no-disconnect-deletes-provider-data'
  | 'no-delete-disconnect-collapse'
  | 'no-ts-business-owner'
  | 'no-lan-ownership';

export const GeneratedParentStorageModeLabels = [
  'local-only',
  'local-plus-encrypted-backup',
  'local-plus-encrypted-provider-sync',
  'provider-disconnected',
  'provider-error',
  'manual-required',
  'disabled',
] as const satisfies readonly GeneratedParentStorageModeLabel[];
export const GeneratedParentStorageUiStates = [
  'providerNotConfigured',
  'providerAuthExpired',
  'providerPermissionMissing',
  'providerRevoked',
  'providerQuotaExceeded',
  'providerUnavailable',
  'localStoreUnavailable',
  'keyUnavailable',
  'keyRevoked',
  'wrongHousehold',
  'wrongDevice',
  'schemaUnsupported',
  'bundleCorrupt',
  'tombstoneConflict',
  'manualRequired',
  'offlineQueued',
  'syncDisabled',
  'remoteDisabled',
  'ocentraHostedStorageNotUsed',
  'ready',
] as const satisfies readonly GeneratedParentStorageUiState[];
export const GeneratedParentStorageEncryptionStatuses = [
  'encrypted-before-upload',
  'human-readable-parent-authorized',
  'not-applicable',
  'manual-required',
] as const satisfies readonly GeneratedParentStorageEncryptionStatus[];
export const GeneratedParentStorageKeyStatuses = [
  'keyAvailable',
  'keyUnavailable',
  'keyRevoked',
  'manualRequired',
] as const satisfies readonly GeneratedParentStorageKeyStatus[];
export const GeneratedParentStoragePreviewStates = [
  'importPreviewPassed',
  'partialRestore',
  'wrongHousehold',
  'wrongKey',
  'schemaUnsupported',
  'bundleCorrupt',
  'tombstoneConflict',
  'manualRequired',
] as const satisfies readonly GeneratedParentStoragePreviewState[];
export const GeneratedParentStorageApplyStates = [
  'notStarted',
  'applyRequiresConfirmation',
  'applyPending',
  'applied',
  'partial',
  'rollbackManualRequired',
  'blockedManualRequired',
] as const satisfies readonly GeneratedParentStorageApplyState[];
export const GeneratedParentStorageDeleteActionKinds = [
  'delete-local-child-evidence',
  'delete-parent-portal-cache',
  'delete-generated-report',
  'delete-provider-backup-copy',
  'delete-support-bundle',
  'delete-ocentra-metadata',
] as const satisfies readonly GeneratedParentStorageDeleteActionKind[];
export const GeneratedParentStorageCopyKeys = [
  'custody-boundary',
  'metadata-leakage',
  'sensitive-encrypted-before-upload',
  'lost-key-may-be-unrecoverable',
  'disconnect-does-not-delete',
  'tombstones-may-be-required',
  'backup-queued',
  'provider-upload-pending',
  'provider-upload-failed',
  'provider-upload-confirmed',
  'import-preview-passed',
  'apply-requires-confirmation',
  'deleted-locally-provider-delete-pending',
  'provider-disconnected-existing-files-may-remain',
  'manual-proof-required',
] as const satisfies readonly GeneratedParentStorageCopyKey[];
export const GeneratedParentStorageNoClaims = [
  'no-portal-implementation-ready',
  'no-provider-runtime-ready',
  'no-auto-apply',
  'no-disconnect-deletes-provider-data',
  'no-delete-disconnect-collapse',
  'no-ts-business-owner',
  'no-lan-ownership',
] as const satisfies readonly GeneratedParentStorageNoClaim[];

export interface GeneratedParentStorageModeCard {{
  rowId: GeneratedParentStorageSettingsRowId;
  currentModeLabel: GeneratedParentStorageModeLabel;
  uiState: GeneratedParentStorageUiState;
  providerMode: GeneratedParentOwnedSyncProviderMode;
  providerStatus: GeneratedParentOwnedSyncProviderStatus;
  syncState: GeneratedParentOwnedSyncState;
  encryptionStatus: GeneratedParentStorageEncryptionStatus;
  keyStatus: GeneratedParentStorageKeyStatus;
  manualRequiredVisible: boolean;
  disconnectVisible: boolean;
  deleteVisible: boolean;
  restorePreviewAvailable: boolean;
  applyBackAvailable: boolean;
  lastSuccessAt: GeneratedParentStorageTimestamp | null;
  lastFailureAt: GeneratedParentStorageTimestamp | null;
  summary: string;
}}

export interface GeneratedParentStorageRestorePreview {{
  previewId: GeneratedParentStoragePreviewId;
  previewState: GeneratedParentStoragePreviewState;
  createdAt: GeneratedParentStorageTimestamp;
  productVersion: string;
  schemaVersion: string;
  householdMatch: boolean;
  deviceMatch: boolean;
  dataClasses: readonly GeneratedParentOwnedSyncExportDataClass[];
  conflicts: readonly string[];
  rejectedSections: readonly GeneratedParentOwnedSyncExportDataClass[];
  partialRestore: boolean;
  confirmationRequired: boolean;
  localTruthAuthoritative: boolean;
  tombstonesPreserved: boolean;
  manualRequiredNote: string | null;
}}

export interface GeneratedParentStorageApplyDecision {{
  applyId: GeneratedParentStorageApplyId;
  applyState: GeneratedParentStorageApplyState;
  confirmationRequired: boolean;
  willChange: readonly GeneratedParentOwnedSyncExportDataClass[];
  willNotChange: readonly GeneratedParentOwnedSyncExportDataClass[];
  preservedTombstones: readonly GeneratedParentOwnedSyncExportDataClass[];
  manualReviewRequired: readonly string[];
  rollbackAvailable: boolean;
  manualRequiredNote: string | null;
}}

export interface GeneratedParentStorageDeleteActionRow {{
  actionId: GeneratedParentStorageActionId;
  actionKind: GeneratedParentStorageDeleteActionKind;
  state: GeneratedParentOwnedSyncDeleteVisibilityState;
  separateFromDisconnect: boolean;
  proofRequired: boolean;
  notes: string;
}}

export interface GeneratedParentStorageDisconnectRow {{
  actionId: GeneratedParentStorageActionId;
  state: GeneratedParentOwnedSyncDisconnectVisibilityState;
  existingFilesMayRemain: boolean;
  providerDeleteRequestedSeparately: boolean;
  notes: string;
}}

export interface GeneratedParentStorageClaimSafeCopyRow {{
  copyKey: GeneratedParentStorageCopyKey;
  statement: string;
  forbiddenWithoutState: boolean;
  notes: string;
}}

export interface GeneratedParentStorageSettingsApplyFlowContractProof {{
  schemaVersion: typeof ParentStorageSettingsApplyFlowContractRuntime.SchemaVersion;
  contractVersion: GeneratedParentContractSchemaVersion;
  modeCard: GeneratedParentStorageModeCard;
  restorePreview: GeneratedParentStorageRestorePreview;
  applyDecision: GeneratedParentStorageApplyDecision;
  deleteActions: readonly GeneratedParentStorageDeleteActionRow[];
  disconnectAction: GeneratedParentStorageDisconnectRow;
  claimSafeCopy: readonly GeneratedParentStorageClaimSafeCopyRow[];
  noClaims: readonly GeneratedParentStorageNoClaim[];
  updatedAt: GeneratedParentStorageTimestamp;
}}

export const GeneratedParentStorageKnownGaps = [
{known_gaps}
] as const;

export const GeneratedParentStorageSettingsApplyFlowContractProof = {proof_json} as const satisfies GeneratedParentStorageSettingsApplyFlowContractProof;
"#,
        schema_version = PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION,
        known_gaps = known_gaps,
        proof_json = proof_json,
    )
}

pub fn parent_storage_settings_apply_flow_contract_rules_typescript() -> String {
    r#"/* generated from crates/schema/src/parent_storage_settings_apply_flow.rs */

import {
  GeneratedParentStorageCopyKeys,
  GeneratedParentStorageDeleteActionKinds,
  GeneratedParentStorageModeLabels,
  GeneratedParentStorageNoClaims,
  type GeneratedParentStorageApplyDecision,
  type GeneratedParentStorageClaimSafeCopyRow,
  type GeneratedParentStorageDeleteActionRow,
  type GeneratedParentStorageDisconnectRow,
  type GeneratedParentStorageModeCard,
  type GeneratedParentStorageRestorePreview,
  type GeneratedParentStorageSettingsApplyFlowContractProof,
} from './parent-storage-settings-apply-flow-contracts';

export function parentStorageModeCardIsHonestGenerated(card: GeneratedParentStorageModeCard): boolean {
  if (!card.restorePreviewAvailable || !card.summary.trim()) {
    return false;
  }

  if (card.currentModeLabel === 'manual-required') {
    return card.manualRequiredVisible && card.uiState === 'manualRequired' && !card.applyBackAvailable;
  }
  if (card.currentModeLabel === 'provider-disconnected') {
    return (
      card.providerStatus === 'disconnected' &&
      card.disconnectVisible &&
      card.uiState === 'remoteDisabled' &&
      !card.applyBackAvailable
    );
  }
  if (card.currentModeLabel === 'provider-error') {
    return card.lastFailureAt !== null && card.providerStatus !== 'ready' && card.providerStatus !== 'disconnected';
  }
  if (card.currentModeLabel === 'local-plus-encrypted-backup') {
    return (
      card.providerMode === 'local-folder' &&
      card.providerStatus === 'ready' &&
      card.encryptionStatus === 'encrypted-before-upload'
    );
  }
  if (card.currentModeLabel === 'disabled') {
    return (
      (card.providerStatus === 'disabled' || card.providerStatus === 'not-configured') &&
      !card.deleteVisible &&
      !card.applyBackAvailable
    );
  }

  return true;
}

export function parentStorageRestorePreviewIsHonestGenerated(preview: GeneratedParentStorageRestorePreview): boolean {
  if (
    !preview.confirmationRequired ||
    !preview.localTruthAuthoritative ||
    !preview.tombstonesPreserved ||
    !preview.productVersion.trim() ||
    !preview.schemaVersion.trim()
  ) {
    return false;
  }

  if (preview.previewState === 'partialRestore') {
    return preview.partialRestore && preview.rejectedSections.length > 0;
  }
  if (preview.previewState === 'wrongHousehold') {
    return !preview.householdMatch;
  }
  if (preview.previewState === 'tombstoneConflict') {
    return preview.rejectedSections.length > 0;
  }
  if (preview.previewState === 'manualRequired') {
    return preview.manualRequiredNote !== null && preview.manualRequiredNote.trim().length > 0;
  }

  return true;
}

export function parentStorageApplyDecisionIsHonestGenerated(decision: GeneratedParentStorageApplyDecision): boolean {
  if (!decision.confirmationRequired) {
    return false;
  }

  if (decision.applyState === 'blockedManualRequired' || decision.applyState === 'rollbackManualRequired') {
    return decision.manualRequiredNote !== null && decision.manualRequiredNote.trim().length > 0;
  }
  if (decision.applyState === 'applyRequiresConfirmation') {
    return decision.manualRequiredNote === null;
  }

  return true;
}

export function parentStorageDeleteActionRowIsHonestGenerated(row: GeneratedParentStorageDeleteActionRow): boolean {
  return row.separateFromDisconnect && row.proofRequired && row.notes.trim().length > 0;
}

export function parentStorageDisconnectRowIsHonestGenerated(row: GeneratedParentStorageDisconnectRow): boolean {
  if (!row.existingFilesMayRemain || !row.providerDeleteRequestedSeparately || !row.notes.trim()) {
    return false;
  }

  return row.state !== 'manual-required' || row.notes.trim().length > 0;
}

export function parentStorageClaimSafeCopyRowIsHonestGenerated(row: GeneratedParentStorageClaimSafeCopyRow): boolean {
  return row.forbiddenWithoutState && row.statement.trim().length > 0 && row.notes.trim().length > 0;
}

export function parentStorageSettingsApplyFlowProofIsHonestGenerated(
  proof: GeneratedParentStorageSettingsApplyFlowContractProof
): boolean {
  return (
    GeneratedParentStorageModeLabels.includes(proof.modeCard.currentModeLabel) &&
    GeneratedParentStorageNoClaims.every((nonClaim) => proof.noClaims.includes(nonClaim)) &&
    hasExactCoverageGenerated(
      proof.deleteActions.map((row) => row.actionKind),
      GeneratedParentStorageDeleteActionKinds
    ) &&
    hasExactCoverageGenerated(
      proof.claimSafeCopy.map((row) => row.copyKey),
      GeneratedParentStorageCopyKeys
    ) &&
    parentStorageModeCardIsHonestGenerated(proof.modeCard) &&
    parentStorageRestorePreviewIsHonestGenerated(proof.restorePreview) &&
    parentStorageApplyDecisionIsHonestGenerated(proof.applyDecision) &&
    proof.deleteActions.every((row) => parentStorageDeleteActionRowIsHonestGenerated(row)) &&
    parentStorageDisconnectRowIsHonestGenerated(proof.disconnectAction) &&
    proof.claimSafeCopy.every((row) => parentStorageClaimSafeCopyRowIsHonestGenerated(row))
  );
}

function hasExactCoverageGenerated<T extends string>(values: readonly T[], expected: readonly T[]): boolean {
  return values.length === expected.length && expected.every((value, index) => values[index] === value);
}
"#
    .to_owned()
}
