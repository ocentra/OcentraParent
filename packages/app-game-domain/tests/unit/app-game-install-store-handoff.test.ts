import { describe, expect, it } from 'vitest';
import { AppGameInstallStoreHandoffProofMatrix } from '@ocentra-parent/schema-domain/app-game-install-store-handoff-proof';
import {
  AppGameInstallStoreHandoffMatrixSchema,
  AppGameInstallStoreHandoffRowSchema,
} from '@ocentra-parent/schema-domain/app-game-install-store-handoff';

const rowFor = (handoffId: string) => {
  const row = AppGameInstallStoreHandoffProofMatrix.rows.find((candidate) => candidate.handoffId === handoffId);

  if (row === undefined) {
    throw new Error(`Missing install/store handoff row ${handoffId}`);
  }

  return row;
};

describe('app/game install store handoff contracts', () => {
  recordsRequiredInstallStoreAndUninstallSignals();
  keepsStoreAndPurchaseSignalsContextOnly();
  requiresInstallApprovalRefsToCarryEvidence();
  routesUninstallAndTamperSignalsToTamperDocs();
  exposesManualRequiredStatesAndRejectsAdapterClaims();
});

function recordsRequiredInstallStoreAndUninstallSignals() {
  it('records each required handoff signal in one matrix', () => {
    const matrix = AppGameInstallStoreHandoffMatrixSchema.parse(AppGameInstallStoreHandoffProofMatrix);

    expect(matrix.matrixId).toBe('app-game-install-store-handoff-proof');
    expect(matrix.rows.map((row) => row.signalKind)).toEqual([
      'new-inventory-detected',
      'installer-updater-process',
      'store-package-install',
      'game-purchase-signal',
      'uninstall-detected',
      'tamper-uninstall-candidate',
    ]);
    expect(matrix.rows.every((row) => row.evidenceReferences.length > 0)).toBe(true);
  });
}

function keepsStoreAndPurchaseSignalsContextOnly() {
  it('keeps store and purchase signals as context rather than automatic decisions', () => {
    const storeInstall = rowFor('store-package-install-context-handoff');
    const gamePurchase = rowFor('native-game-purchase-signal-handoff');

    expect(storeInstall.storeSignalUse).toBe('context-only-not-decision');
    expect(gamePurchase.storeSignalUse).toBe('context-only-not-decision');
    expect(storeInstall.noClaimBoundaries).toContain('store-signal-not-safety-decision');
    expect(gamePurchase.noClaimBoundaries).toContain('no-billing-entitlement-logic');
    expect(
      AppGameInstallStoreHandoffRowSchema.safeParse({
        ...storeInstall,
        capabilityState: 'supported',
      }).success
    ).toBe(false);
  });
}

function requiresInstallApprovalRefsToCarryEvidence() {
  it('requires install approval handoff refs to cite evidence and the approval feature', () => {
    const newApp = rowFor('new-app-inventory-review-handoff');

    expect(newApp.approvalRequestRef).toBe('approval-request:new-app-inventory-review');
    expect(newApp.destinationFeatureDocs).toContain('docs/features/app-install-purchase-approval.md');
    expect(newApp.evidenceReferences.map((evidenceReference) => evidenceReference.evidenceReferenceId)).toEqual([
      'evidence:new-app-inventory-delta',
    ]);
    expect(
      AppGameInstallStoreHandoffRowSchema.safeParse({
        ...newApp,
        evidenceReferences: [],
      }).success
    ).toBe(false);
  });
}

function routesUninstallAndTamperSignalsToTamperDocs() {
  it('routes uninstall and tamper signals to tamper docs without approval request refs', () => {
    const uninstall = rowFor('uninstall-detected-tamper-handoff');
    const tamper = rowFor('tamper-uninstall-candidate-manual-handoff');

    expect(uninstall.approvalRequestRef).toBeNull();
    expect(tamper.approvalRequestRef).toBeNull();
    expect(uninstall.destinationFeatureDocs).toContain('docs/features/enforcement-integrity-tamper.md');
    expect(tamper.expectationDocRefs).toContain('docs/expectations/tamper-uninstall-protection.md');
    expect(
      AppGameInstallStoreHandoffRowSchema.safeParse({
        ...uninstall,
        destinationFeatureDocs: ['docs/features/app-game-control.md'],
      }).success
    ).toBe(false);
  });
}

function exposesManualRequiredStatesAndRejectsAdapterClaims() {
  it('requires parent-visible manual state and rejects adapter or policy claims', () => {
    const storeInstall = rowFor('store-package-install-context-handoff');

    expect(storeInstall.parentVisibleManualState).toContain('context for review');
    expect(storeInstall.manualRequirement).toContain('Microsoft Store API');
    expect(storeInstall.adapterExecutionClaim).toBe('not-claimed');
    expect(storeInstall.policyDecisionClaim).toBe('not-claimed');
    expect(
      AppGameInstallStoreHandoffRowSchema.safeParse({
        ...storeInstall,
        parentVisibleManualState: null,
      }).success
    ).toBe(false);
    expect(
      AppGameInstallStoreHandoffRowSchema.safeParse({
        ...storeInstall,
        adapterExecutionClaim: 'supported',
      }).success
    ).toBe(false);
  });
}
