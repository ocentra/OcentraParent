import { describe, expect, it } from 'vitest';
import {
  TamperUninstallArtifactStatusEntrySchema,
  TamperUninstallArtifactStatusReadModelSchema,
  type TamperUninstallArtifactSurface,
} from '../../src/tamper-uninstall-artifact-status';
import { TamperUninstallArtifactStatusReadModel } from '../../src/tamper-uninstall-artifact-status-read-model';

describe('tamper uninstall artifact status contract', () => {
  coversDesktopMobileAndAdminRemovalRows();
  keepsArtifactsManualOrDeviceProofRequired();
  documentsAdminRemovalWithoutBlocking();
  keepsClaimFieldsFalse();
  rejectsClaimUpgradesMissingRefsAndIncoherentStates();
});

function coversDesktopMobileAndAdminRemovalRows() {
  it('covers desktop mobile and admin removal artifact status rows', () => {
    const readModel = TamperUninstallArtifactStatusReadModelSchema.parse(TamperUninstallArtifactStatusReadModel);

    expect(readModel.readModelId).toBe('tamper-uninstall-artifact-status-proof');
    expect(readModel.entries).toHaveLength(8);
    expect(countBy(readModel.entries.map((entry) => entry.platform))).toEqual({
      windows: 3,
      linux: 1,
      macos: 1,
      android: 2,
      ios: 1,
    });
    expect(readModel.entries.map((entry) => entry.surface)).toEqual([
      'windows-service-stop',
      'windows-package-uninstall',
      'linux-service-package',
      'macos-launchd-package',
      'android-package-removed',
      'android-device-owner-managed-profile',
      'ios-family-controls-device-activity',
      'admin-removal-flow',
    ]);
  });
}

function keepsArtifactsManualOrDeviceProofRequired() {
  it('keeps uninstall detection artifacts manual required or device proof required', () => {
    expect(entryFor('windows-service-stop')).toMatchObject({
      artifactState: 'manual-required',
      parentVisibleStatus: 'artifact-needed',
      custodyState: 'manual-review-required',
      capability: 'headless-agent-service',
    });
    expect(entryFor('windows-package-uninstall').requiredArtifacts).toContain(
      'Windows installer uninstall or rollback artifact'
    );
    expect(entryFor('android-package-removed')).toMatchObject({
      artifactState: 'device-proof-required',
      parentVisibleStatus: 'device-proof-needed',
      custodyState: 'not-collected',
      capability: 'package-lifecycle',
    });
    expect(entryFor('ios-family-controls-device-activity').requiredArtifacts).toContain(
      'iOS Family Controls authorization artifact'
    );
  });
}

function documentsAdminRemovalWithoutBlocking() {
  it('documents admin removal without blocking parent or admin removal', () => {
    const adminRemoval = entryFor('admin-removal-flow');

    expect(adminRemoval).toMatchObject({
      artifactState: 'documented-admin-removal',
      parentVisibleStatus: 'admin-removal-documented',
      custodyState: 'documented-flow-only',
      adminRemovalBlockingClaimed: false,
    });
    expect(adminRemoval.adminRemovalFlowRefs).toEqual(['documented-parent-admin-removal-flow-ref']);
    expect(adminRemoval.boundary).toContain('is not blocked by this proof');
  });
}

function keepsClaimFieldsFalse() {
  it('keeps anti tamper provider delivery privilege stealth and raw child data as non claims', () => {
    for (const entry of TamperUninstallArtifactStatusReadModel.entries) {
      expect(entry.uninstallDetectionClaimed).toBe(false);
      expect(entry.tamperResistanceClaimed).toBe(false);
      expect(entry.stealthPersistenceClaimed).toBe(false);
      expect(entry.privilegeEscalationClaimed).toBe(false);
      expect(entry.adminRemovalBlockingClaimed).toBe(false);
      expect(entry.providerDeliveryClaimed).toBe(false);
      expect(entry.rawChildDataIncluded).toBe(false);
      expect(entry.auditRefs).toEqual(['tamper-integrity-audit-ref']);
      expect(entry.integrityRefs).toEqual(['integrity-alert-status-bridge-ref']);
    }
  });
}

function rejectsClaimUpgradesMissingRefsAndIncoherentStates() {
  it('rejects claim upgrades missing refs missing artifacts and incoherent states', () => {
    const windowsStop = entryFor('windows-service-stop');
    const androidRemoved = entryFor('android-package-removed');
    const adminRemoval = entryFor('admin-removal-flow');

    for (const invalidEntry of [
      { ...windowsStop, statusEntryId: 'invalid-uninstall-claim', uninstallDetectionClaimed: true },
      { ...windowsStop, statusEntryId: 'invalid-tamper-claim', tamperResistanceClaimed: true },
      { ...windowsStop, statusEntryId: 'invalid-provider-claim', providerDeliveryClaimed: true },
      { ...windowsStop, statusEntryId: 'invalid-raw-child-data', rawChildDataIncluded: true },
      { ...windowsStop, statusEntryId: 'invalid-no-audit-ref', auditRefs: [] },
      { ...windowsStop, statusEntryId: 'invalid-no-artifact', requiredArtifacts: [] },
      { ...androidRemoved, statusEntryId: 'invalid-mobile-upgrade', artifactState: 'manual-required' },
      { ...adminRemoval, statusEntryId: 'invalid-admin-no-flow-ref', adminRemovalFlowRefs: [] },
      { ...adminRemoval, statusEntryId: 'invalid-admin-blocking', adminRemovalBlockingClaimed: true },
    ]) {
      expect(() => TamperUninstallArtifactStatusEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
}

function entryFor(surface: TamperUninstallArtifactSurface) {
  const entry = TamperUninstallArtifactStatusReadModel.entries.find((candidate) => candidate.surface === surface);
  if (entry === undefined) {
    throw new Error(`Missing tamper uninstall artifact status entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
