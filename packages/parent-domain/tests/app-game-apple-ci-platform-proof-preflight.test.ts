import { describe, expect, it } from 'vitest';
import {
  AppGameAppleCiPlatformProofPreflightReadModelSchema,
  createAppGameAppleCiPlatformProofPreflightReadModel,
  summarizeAppGameAppleCiPlatformProofPreflightReadModel,
} from '../src/app-game-apple-ci-platform-proof-preflight';

describe('app-game Apple CI platform proof preflight', () => {
  it('summarizes macOS and iOS manual artifact gates as CI-required and blocked', () => {
    const readModel = createAppGameAppleCiPlatformProofPreflightReadModel({
      generatedAt: '2026-06-08T18:30:00.000Z',
    });

    assertAppleCiPreflightSummary(readModel);
  });

  it('turns macOS manual artifact gates into a CI-required platform row', () => {
    const readModel = createAppGameAppleCiPlatformProofPreflightReadModel({
      generatedAt: '2026-06-08T18:30:00.000Z',
    });

    assertMacosCiPreflightRow(readModel);
  });

  it('turns iOS manual artifact gates into a CI-required platform row', () => {
    const readModel = createAppGameAppleCiPlatformProofPreflightReadModel({
      generatedAt: '2026-06-08T18:30:00.000Z',
    });

    assertIosCiPreflightRow(readModel);
  });

  it('rejects Windows-local or adapter-dispatch claim upgrades for Apple platforms', () => {
    const readModel = createAppGameAppleCiPlatformProofPreflightReadModel({
      generatedAt: '2026-06-08T18:30:00.000Z',
    });

    expect(
      AppGameAppleCiPlatformProofPreflightReadModelSchema.safeParse({
        ...readModel,
        windowsLocalProofClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAppleCiPlatformProofPreflightReadModelSchema.safeParse({
        ...readModel,
        rows: [
          {
            ...readModel.rows[0],
            canDispatchAdapter: true,
          },
          readModel.rows[1],
        ],
      }).success
    ).toBe(false);
    expect(
      AppGameAppleCiPlatformProofPreflightReadModelSchema.safeParse({
        ...readModel,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
  });
});

function assertAppleCiPreflightSummary(
  readModel: ReturnType<typeof createAppGameAppleCiPlatformProofPreflightReadModel>
): void {
  expect(summarizeAppGameAppleCiPlatformProofPreflightReadModel(readModel)).toEqual({
    macosGateCount: 1,
    iosGateCount: 6,
    dispatchableRowCount: 0,
    blockedRowCount: 2,
    openBlockerCount: 10,
  });
}

function assertMacosCiPreflightRow(
  readModel: ReturnType<typeof createAppGameAppleCiPlatformProofPreflightReadModel>
): void {
  expect(readModel.rows[0]).toEqual({
    platform: 'macos',
    preflightState: 'ci-artifacts-required',
    sourceGateIds: ['v0-8-manual-gate-macos-service-package-permission'],
    requiredProofRefs: [
      'macos-ci-runner-ref',
      'macos-xcodebuild-ref',
      'macos-permission-profile-proof',
      'macos-mdm-endpoint-proof',
      'macos-rollback-audit-proof',
    ],
    blockerRefs: [
      'macos-ci-runner-not-proved',
      'macos-permission-profile-not-proved',
      'macos-mdm-endpoint-not-proved',
      'macos-rollback-audit-not-proved',
      'apple-platform-adapter-dispatch-blocked-before-ci-proof',
    ],
    canRunOnWindowsHost: false,
    canDispatchAdapter: false,
    ciRunnerClaimed: false,
    entitlementClaimed: false,
    platformEnforcementClaimed: false,
  });
}

function assertIosCiPreflightRow(
  readModel: ReturnType<typeof createAppGameAppleCiPlatformProofPreflightReadModel>
): void {
  expect(readModel.rows[1]).toEqual({
    platform: 'ios',
    preflightState: 'ci-artifacts-required',
    sourceGateIds: [
      'v0-8-manual-gate-ios-family-controls',
      'v0-8-manual-gate-ios-device-activity',
      'v0-8-manual-gate-ios-screen-time',
      'v0-8-manual-gate-ios-network-extension',
      'v0-8-manual-gate-ios-background-execution-signing',
      'v0-8-manual-gate-ios-testflight-device-install',
    ],
    requiredProofRefs: [
      'ios-ci-runner-ref',
      'ios-family-controls-entitlement-proof',
      'ios-device-activity-proof',
      'ios-managed-settings-proof',
      'ios-testflight-device-proof',
    ],
    blockerRefs: [
      'ios-ci-runner-not-proved',
      'ios-family-controls-not-proved',
      'ios-device-activity-not-proved',
      'ios-managed-settings-not-proved',
      'ios-testflight-device-not-proved',
      'apple-platform-adapter-dispatch-blocked-before-ci-proof',
    ],
    canRunOnWindowsHost: false,
    canDispatchAdapter: false,
    ciRunnerClaimed: false,
    entitlementClaimed: false,
    platformEnforcementClaimed: false,
  });
}
