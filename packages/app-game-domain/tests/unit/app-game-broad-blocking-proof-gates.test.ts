import { describe, expect, it } from 'vitest';
import { AppGameBroadBlockingGateMatrix } from '../../src/app-game-broad-blocking-proof-gate-data';
import {
  AppGameBroadBlockingGateMatrixSchema,
  AppGameBroadBlockingGateSchema,
} from '../../src/app-game-broad-blocking-proof-gates';
import { EnforcementMode } from '@ocentra-parent/schema-domain/enforcement';

const gateFor = (gateId: string) => {
  const gate = AppGameBroadBlockingGateMatrix.gates.find((candidate) => candidate.gateId === gateId);

  if (gate === undefined) {
    throw new Error(`Missing app/game broad blocking gate ${gateId}`);
  }

  return gate;
};

const Proof = {
  setup: { proofKind: 'setup-proof', artifactRef: 'proof/setup.md' },
  authority: { proofKind: 'authority-tier-proof', artifactRef: 'proof/authority.md' },
  rollback: { proofKind: 'rollback-proof', artifactRef: 'proof/rollback.md' },
  audit: { proofKind: 'audit-state-proof', artifactRef: 'proof/audit.md' },
  appControl: { proofKind: 'windows-app-control-proof', artifactRef: 'proof/windows-app-control.md' },
} as const;

describe('app/game broad blocking proof gates', () => {
  registerMatrixAndDispatchTests();
  registerPlatformProofRequirementTests();
  registerValidationRejectionTests();
});

function registerMatrixAndDispatchTests() {
  it('records broad app and game blocking as manual-required, unavailable, or not-claimed before adapter dispatch', () => {
    const matrix = AppGameBroadBlockingGateMatrixSchema.parse(AppGameBroadBlockingGateMatrix);
    const outcomeCounts = countBy(matrix.gates.map((gate) => gate.outcomeState));
    const platformCounts = countBy(matrix.gates.map((gate) => gate.platform));

    expect(matrix.matrixId).toBe('app-game-broad-blocking-proof-gates');
    expect(matrix.gates).toHaveLength(7);
    expect(outcomeCounts).toEqual({
      'manual-required': 5,
      unavailable: 1,
      'not-claimed': 1,
    });
    expect(platformCounts).toEqual({
      windows: 2,
      macos: 1,
      linux: 1,
      android: 1,
      ios: 2,
    });
    expect(matrix.gates.every((gate) => gate.canCallAdapter === false)).toBe(true);
    expect(matrix.gates.every((gate) => gate.broadBlockingClaimed === false)).toBe(true);
  });

  it('rejects manual-required and unavailable broad blocking rows that try to call adapters', () => {
    const windows = gateFor('windows-block-launch-applocker-app-control-manual-required');
    const linux = gateFor('linux-hard-block-mechanism-unavailable');

    expect(
      AppGameBroadBlockingGateSchema.safeParse({
        ...windows,
        canCallAdapter: true,
        supportedModes: [EnforcementMode.BlockProcess],
        adapterDispatchState: 'dispatch-eligible',
      }).success
    ).toBe(false);
    expect(
      AppGameBroadBlockingGateSchema.safeParse({
        ...linux,
        canCallAdapter: true,
        supportedModes: [EnforcementMode.BlockProcess],
        adapterDispatchState: 'dispatch-eligible',
      }).success
    ).toBe(false);
  });
}

function registerPlatformProofRequirementTests() {
  it('names setup, authority tier, rollback, audit, and platform proof before broad blocking can move up', () => {
    const windows = gateFor('windows-block-launch-applocker-app-control-manual-required');
    const macos = gateFor('macos-hard-block-endpoint-mdm-manual-required');
    const linux = gateFor('linux-hard-block-mechanism-unavailable');

    expect(windows.requiredProofKinds).toEqual([
      'setup-proof',
      'authority-tier-proof',
      'rollback-proof',
      'audit-state-proof',
      'windows-applocker-proof',
      'windows-app-control-proof',
      'windows-system-app-allowlist-proof',
    ]);
    expect(macos.requiredProofKinds).toContain('macos-endpoint-security-proof');
    expect(macos.requiredProofKinds).toContain('rollback-proof');
    expect(macos.requiredProofKinds).toContain('audit-state-proof');
    expect(linux.requiredProofKinds).toContain('linux-mechanism-proof');
    expect(linux.requiredProofKinds).toContain('linux-distro-proof');
    expect(linux.requiredProofKinds).toContain('linux-session-proof');
  });

  it('keeps Android normal mode, iOS shielding, and iOS process killing proof-gated', () => {
    const android = gateFor('android-normal-mode-hide-suspend-manual-required');
    const iosShield = gateFor('ios-managedsettings-shield-manual-required');
    const iosKill = gateFor('ios-process-kill-not-claimed');

    expect(android.parentVisibleReason).toContain('Device Owner or Profile Owner proof');
    expect(android.requiredProofKinds).toContain('android-device-owner-proof');
    expect(android.requiredProofKinds).toContain('android-profile-owner-proof');
    expect(iosShield.requiredProofKinds).toContain('ios-family-controls-proof');
    expect(iosShield.requiredProofKinds).toContain('ios-managed-settings-proof');
    expect(iosKill).toMatchObject({
      action: 'terminate-process',
      outcomeState: 'not-claimed',
      adapterDispatchState: 'not-dispatched',
      canCallAdapter: false,
    });
  });
}

function registerValidationRejectionTests() {
  it('rejects AppLocker audit-only evidence and incomplete supported broad block upgrades', () => {
    const auditOnly = gateFor('windows-applocker-audit-is-not-enforce-proof');
    const supportedCandidate = {
      ...auditOnly,
      gateId: 'supported-windows-block-launch',
      outcomeState: 'supported',
      adapterDispatchState: 'dispatch-eligible',
      authorityTier: 'root-or-admin-service',
      setupState: 'admin-or-root-required',
      capabilityState: 'supported',
      canCallAdapter: true,
      supportedModes: [EnforcementMode.BlockProcess],
      rollbackState: 'rollback-proof-attached',
      auditState: 'audit-proof-attached',
      broadBlockingClaimed: true,
      proofReferences: [Proof.setup, Proof.authority, Proof.rollback, Proof.audit, Proof.appControl],
      requiredProofKinds: [],
    } as const;

    expect(
      AppGameBroadBlockingGateSchema.safeParse({
        ...supportedCandidate,
        proofReferences: [Proof.setup, Proof.authority, Proof.rollback, Proof.audit],
      }).success
    ).toBe(false);
    expect(
      AppGameBroadBlockingGateSchema.safeParse({
        ...supportedCandidate,
        proofReferences: [Proof.setup, Proof.authority, Proof.appControl],
      }).success
    ).toBe(false);
    expect(AppGameBroadBlockingGateSchema.safeParse(supportedCandidate).success).toBe(true);
  });

  it('rejects generic UI reasons for manual-required platform gates', () => {
    const windows = gateFor('windows-block-launch-applocker-app-control-manual-required');

    expect(
      AppGameBroadBlockingGateSchema.safeParse({
        ...windows,
        parentVisibleReason: 'Unsupported',
      }).success
    ).toBe(false);
  });
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
