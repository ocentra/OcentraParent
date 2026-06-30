/* generated from crates/schema/src/parent_storage_settings_apply_flow.rs */

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
