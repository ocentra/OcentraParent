import { describe, expect, it } from 'vitest';
import {
  ParentStorageApplyDecisionSchema,
  ParentStorageDeleteActionRowSchema,
  ParentStorageSettingsApplyFlowContractProofReadModel,
  ParentStorageSettingsApplyFlowContractProofSchema,
  ParentStorageSettingsApplyFlowKnownGaps,
  ParentStorageRestorePreviewSchema,
  RequiredParentStorageDeleteActionKinds,
  RequiredParentStorageModeLabels,
  RequiredParentStorageNoClaims,
  summarizeParentStorageDeleteActionKinds,
  summarizeParentStorageModeLabels,
} from '@ocentra-parent/schema-domain/parent-storage-settings-apply-flow';

describe('parent storage settings apply flow contracts', () => {
  explicitStorageModeStateProof();
  restorePreviewBeforeApplyProof();
  deleteAndDisconnectStaySeparateProof();
  noAutomaticFallbackOrRuntimeOverclaimProof();
});

function explicitStorageModeStateProof(): void {
  it('keeps storage mode labels explicit and manual-required visibility honest', () => {
    const proof = ParentStorageSettingsApplyFlowContractProofSchema.parse(
      ParentStorageSettingsApplyFlowContractProofReadModel
    );

    expect(summarizeParentStorageModeLabels([proof.modeCard.currentModeLabel])).toEqual({
      'local-only': 0,
      'local-plus-encrypted-backup': 0,
      'local-plus-encrypted-provider-sync': 0,
      'provider-disconnected': 0,
      'provider-error': 0,
      'manual-required': 1,
      disabled: 0,
    });
    expect(RequiredParentStorageModeLabels).toEqual([
      'local-only',
      'local-plus-encrypted-backup',
      'local-plus-encrypted-provider-sync',
      'provider-disconnected',
      'provider-error',
      'manual-required',
      'disabled',
    ]);
    expect(proof.modeCard.uiState).toBe('manualRequired');
    expect(proof.modeCard.manualRequiredVisible).toBe(true);
    expect(proof.modeCard.summary.toLowerCase().includes('cloud')).toBe(false);
  });
}

function restorePreviewBeforeApplyProof(): void {
  it('keeps restore preview non-mutating and apply confirmation explicit', () => {
    const proof = ParentStorageSettingsApplyFlowContractProofReadModel;

    expect(proof.restorePreview.confirmationRequired).toBe(true);
    expect(proof.restorePreview.tombstonesPreserved).toBe(true);
    expect(proof.applyDecision.applyState).toBe('applyRequiresConfirmation');
    expect(proof.applyDecision.confirmationRequired).toBe(true);
    expect(
      ParentStorageRestorePreviewSchema.safeParse({
        ...proof.restorePreview,
        confirmationRequired: false,
      }).success
    ).toBe(false);
    expect(
      ParentStorageApplyDecisionSchema.safeParse({
        ...proof.applyDecision,
        confirmationRequired: false,
      }).success
    ).toBe(false);
  });
}

function deleteAndDisconnectStaySeparateProof(): void {
  it('keeps provider disconnect and delete actions separate with explicit visibility', () => {
    const proof = ParentStorageSettingsApplyFlowContractProofReadModel;

    expect(summarizeParentStorageDeleteActionKinds(proof.deleteActions)).toEqual({
      'delete-local-child-evidence': 1,
      'delete-parent-portal-cache': 1,
      'delete-generated-report': 1,
      'delete-provider-backup-copy': 1,
      'delete-support-bundle': 1,
      'delete-ocentra-metadata': 1,
    });
    expect(proof.deleteActions.map((row) => row.actionKind)).toEqual(RequiredParentStorageDeleteActionKinds);
    expect(proof.disconnectAction.existingFilesMayRemain).toBe(true);
    expect(proof.disconnectAction.providerDeleteRequestedSeparately).toBe(true);
    expect(
      ParentStorageDeleteActionRowSchema.safeParse({
        ...proof.deleteActions[0],
        separateFromDisconnect: false,
      }).success
    ).toBe(false);
  });
}

function noAutomaticFallbackOrRuntimeOverclaimProof(): void {
  it('keeps no-claim boundaries and manual-required provider gaps explicit', () => {
    const proof = ParentStorageSettingsApplyFlowContractProofReadModel;

    expect(proof.noClaims).toEqual(RequiredParentStorageNoClaims);
    expect(proof.deleteActions.find((row) => row.actionKind === 'delete-provider-backup-copy')?.state).toBe(
      'manual-required'
    );
    expect(ParentStorageSettingsApplyFlowKnownGaps).toContain(
      'Provider SDK runtime remains unclaimed for this packet.'
    );
    expect(ParentStorageSettingsApplyFlowKnownGaps).toContain(
      'Automatic provider delete or apply execution remains unclaimed for this packet.'
    );
    expect(proof.claimSafeCopy.some((row) => row.copyKey === 'manual-proof-required')).toBe(true);
  });
}
