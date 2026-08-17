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
} from './generated-parent-storage-settings-apply-flow-contracts';

export function parentStorageModeCardIsHonestGenerated(card: GeneratedParentStorageModeCard): boolean {
  if (!card.restorePreviewAvailable || !card.summary.trim()) {
    return false;
  }

  const check = parentStorageModeCardChecksGenerated[card.currentModeLabel];
  return check?.(card) ?? true;
}

export function parentStorageRestorePreviewIsHonestGenerated(preview: GeneratedParentStorageRestorePreview): boolean {
  if (
    !preview.confirmationRequired ||
    !preview.localTruthAuthoritative ||
    !preview.tombstonesPreserved ||
    !preview.householdRef.trim() ||
    !preview.productVersion.trim() ||
    !preview.schemaVersion.trim()
  ) {
    return false;
  }

  const check = parentStorageRestorePreviewChecksGenerated[preview.previewState];
  return check?.(preview) ?? true;
}

export function parentStorageApplyDecisionIsHonestGenerated(decision: GeneratedParentStorageApplyDecision): boolean {
  if (
    !decision.confirmationRequired ||
    !/^[0-9a-f]{64}$/.test(decision.applyIntentDigest) ||
    decision.rollbackAvailable ||
    decision.applyState === 'applyPending' ||
    decision.applyState === 'applied' ||
    decision.applyState === 'partial' ||
    decision.applyState === 'rollbackManualRequired'
  ) {
    return false;
  }
  const check = parentStorageApplyDecisionChecksGenerated[decision.applyState];
  return check?.(decision) ?? true;
}

export function parentStorageDeleteActionRowIsHonestGenerated(row: GeneratedParentStorageDeleteActionRow): boolean {
  return row.separateFromDisconnect && row.proofRequired && row.notes.trim().length > 0;
}

export function parentStorageDisconnectRowIsHonestGenerated(row: GeneratedParentStorageDisconnectRow): boolean {
  return row.existingFilesMayRemain && row.providerDeleteRequestedSeparately && row.notes.trim().length > 0;
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

const parentStorageModeCardChecksGenerated: Partial<
  Record<GeneratedParentStorageModeCard['currentModeLabel'], (card: GeneratedParentStorageModeCard) => boolean>
> = {
  'manual-required': (card: GeneratedParentStorageModeCard) =>
    card.manualRequiredVisible && card.uiState === 'manualRequired' && !card.applyBackAvailable,
  'provider-disconnected': (card: GeneratedParentStorageModeCard) =>
    card.providerStatus === 'disconnected' &&
    card.disconnectVisible &&
    card.uiState === 'remoteDisabled' &&
    !card.applyBackAvailable,
  'provider-error': (card: GeneratedParentStorageModeCard) =>
    card.lastFailureAt !== null && card.providerStatus !== 'ready' && card.providerStatus !== 'disconnected',
  'local-plus-encrypted-backup': (card: GeneratedParentStorageModeCard) =>
    card.providerMode === 'local-folder' &&
    card.providerStatus === 'ready' &&
    card.encryptionStatus === 'encrypted-before-upload',
  disabled: (card: GeneratedParentStorageModeCard) =>
    (card.providerStatus === 'disabled' || card.providerStatus === 'not-configured') &&
    !card.deleteVisible &&
    !card.applyBackAvailable,
};

const parentStorageRestorePreviewChecksGenerated: Partial<
  Record<
    GeneratedParentStorageRestorePreview['previewState'],
    (preview: GeneratedParentStorageRestorePreview) => boolean
  >
> = {
  partialRestore: (preview: GeneratedParentStorageRestorePreview) =>
    preview.partialRestore && preview.rejectedSections.length > 0,
  wrongHousehold: (preview: GeneratedParentStorageRestorePreview) => !preview.householdMatch,
  tombstoneConflict: (preview: GeneratedParentStorageRestorePreview) => preview.rejectedSections.length > 0,
  manualRequired: (preview: GeneratedParentStorageRestorePreview) =>
    preview.manualRequiredNote !== null && preview.manualRequiredNote.trim().length > 0,
};

const parentStorageApplyDecisionChecksGenerated: Partial<
  Record<GeneratedParentStorageApplyDecision['applyState'], (decision: GeneratedParentStorageApplyDecision) => boolean>
> = {
  blockedManualRequired: (decision: GeneratedParentStorageApplyDecision) =>
    decision.manualRequiredNote !== null && decision.manualRequiredNote.trim().length > 0,
  rollbackManualRequired: (decision: GeneratedParentStorageApplyDecision) =>
    decision.manualRequiredNote !== null && decision.manualRequiredNote.trim().length > 0,
  applyRequiresConfirmation: (decision: GeneratedParentStorageApplyDecision) => decision.manualRequiredNote === null,
};
