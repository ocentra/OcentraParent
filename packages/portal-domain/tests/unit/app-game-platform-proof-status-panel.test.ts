import { describe, expect, it } from 'vitest';
import { createAppGamePlatformProofStatusPanelIntent } from '../../src/app-game-platform-proof-status-panel';

const ReadModel = {
  schemaVersion: 1,
  readModelId: 'app-game-platform-proof-status',
  generatedAt: '2026-06-08T16:20:00.000Z',
  sourceReadModelIds: ['v0-8-supported-adapter-runtime-proof'],
  custodyLabel: 'app-game-platform-proof-status',
  capabilityStatus: 'app-game-platform-proof-status-partial',
  returned: 5,
  hostVisibleCount: 2,
  hostNotDetectedCount: 1,
  localRuntimeNotApplicableCount: 2,
  enforcementReadyCount: 0,
  openGapCount: 14,
  adapterDispatchClaimed: false,
  broadInstalledAppBlockingClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  childDeviceDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
  rows: [
    {
      schemaVersion: 1,
      rowId: 'app-game-platform-proof-status-android',
      platform: 'android',
      proofState: 'android-host-visible',
      authorityState: 'visibility-only',
      hostCapabilityState: 'available',
      hostCapabilityEvidenceRefs: ['android-adb-host-toolchain-ref'],
      hostCapabilityProbeRefs: ['android-adb-path-probe-ref'],
      productMeanings: ['native-app', 'native-game'],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: [
        'android-adb-host-toolchain-ref',
        'android-physical-device-proof-ref',
        'android-usage-events-foreground-ref',
      ],
      openGaps: [
        'android-device-owner-not-proved',
        'android-durable-usage-events-replay-not-proved',
        'platform-enforcement-not-proved',
        'child-device-delivery-not-proved',
      ],
      lastCheckedAt: '2026-06-08T16:20:00.000Z',
    },
    {
      schemaVersion: 1,
      rowId: 'app-game-platform-proof-status-linux',
      platform: 'linux',
      proofState: 'linux-host-not-detected',
      authorityState: 'visibility-only',
      hostCapabilityState: 'not-detected',
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: [],
      productMeanings: ['native-app', 'native-game'],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: [
        'linux-wsl-host-toolchain-ref',
        'linux-wslg-display-ref',
        'linux-wslg-x11-socket-ref',
        'linux-wslg-wayland-socket-ref',
      ],
      openGaps: [
        'linux-native-service-not-proved',
        'linux-foreground-capture-not-proved',
        'linux-rollback-not-proved',
        'platform-enforcement-not-proved',
        'child-device-delivery-not-proved',
      ],
      lastCheckedAt: '2026-06-08T16:20:00.000Z',
    },
    {
      schemaVersion: 1,
      rowId: 'app-game-platform-proof-status-windows',
      platform: 'windows',
      proofState: 'windows-policy-preflight-observed',
      authorityState: 'scoped-execution-only',
      hostCapabilityState: 'available',
      hostCapabilityEvidenceRefs: ['windows-host-local-probe-ref'],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      productMeanings: ['native-app', 'native-game'],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: ['windows-broad-blocking-authority-preflight-ref'],
      openGaps: [
        'windows-applocker-enforce-proof-not-attached',
        'windows-app-control-enforce-proof-not-attached',
        'rollback-proof-not-attached',
      ],
      lastCheckedAt: '2026-06-08T16:20:00.000Z',
    },
    {
      schemaVersion: 1,
      rowId: 'app-game-platform-proof-status-macos',
      platform: 'macos',
      proofState: 'apple-ci-artifacts-required',
      authorityState: 'not-locally-provable',
      hostCapabilityState: 'not-applicable',
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: [],
      productMeanings: ['native-app', 'native-game'],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: ['apple-ci-platform-proof-preflight-ref'],
      openGaps: [
        'macos-ci-runner-not-proved',
        'macos-mdm-endpoint-not-proved',
        'apple-platform-adapter-dispatch-blocked-before-ci-proof',
      ],
      lastCheckedAt: '2026-06-08T16:20:00.000Z',
    },
    {
      schemaVersion: 1,
      rowId: 'app-game-platform-proof-status-ios',
      platform: 'ios',
      proofState: 'apple-ci-artifacts-required',
      authorityState: 'not-locally-provable',
      hostCapabilityState: 'not-applicable',
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: [],
      productMeanings: ['native-app', 'native-game'],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: ['apple-ci-platform-proof-preflight-ref'],
      openGaps: [
        'ios-family-controls-not-proved',
        'ios-managed-settings-not-proved',
        'ios-testflight-device-not-proved',
        'apple-platform-adapter-dispatch-blocked-before-ci-proof',
      ],
      lastCheckedAt: '2026-06-08T16:20:00.000Z',
    },
  ],
} as const;

describe('app-game platform proof status panel intent', () => {
  it('renders five-platform proof rows as review-only visibility without enforcement upgrades', () => {
    const intent = createAppGamePlatformProofStatusPanelIntent(ReadModel);

    expect(intent.body).toBe(
      'Parent-safe platform proof status for Windows, Android, Linux, macOS, and iOS evidence and CI-required proof rows.'
    );
    expect(intent.productClaim).toBe(
      'Windows, Android, Linux, macOS, and iOS platform proof rows are parent-visible evidence only. Native enforcement, broad blocking, rollback, audit, Apple CI artifacts, and child delivery remain unclaimed until platform authority proof is attached.'
    );
    expect(intent.loadState).toBe('Review');
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform proofs', value: '5' }),
        expect.objectContaining({ label: 'Host-visible rows', value: '2' }),
        expect.objectContaining({ label: 'Host not detected rows', value: '1' }),
        expect.objectContaining({ label: 'Not-applicable rows', value: '2' }),
        expect.objectContaining({ label: 'Enforcement-ready rows', value: '0' }),
        expect.objectContaining({ label: 'Open gaps', value: '14' }),
      ])
    );
    expect(intent.rows.map((row) => row.title)).toEqual(['android', 'linux', 'windows', 'macos', 'ios']);
    expect(intent.rows[0].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform', value: 'android' }),
        expect.objectContaining({ label: 'Authority state', value: 'visibility-only' }),
        expect.objectContaining({ label: 'Host capability', value: 'available' }),
        expect.objectContaining({ label: 'Platform state', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Child delivery', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows[1].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform', value: 'linux' }),
        expect.objectContaining({ label: 'Host capability', value: 'not-detected' }),
        expect.objectContaining({
          label: 'Evidence references',
          value:
            'linux-wsl-host-toolchain-ref | linux-wslg-display-ref | linux-wslg-x11-socket-ref | linux-wslg-wayland-socket-ref',
        }),
        expect.objectContaining({ label: 'Enforcement', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows[3].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform', value: 'macos' }),
        expect.objectContaining({ label: 'Host capability', value: 'not-applicable' }),
        expect.objectContaining({ label: 'Evidence references', value: 'apple-ci-platform-proof-preflight-ref' }),
        expect.objectContaining({
          label: 'Open gaps',
          value:
            'macos-ci-runner-not-proved | macos-mdm-endpoint-not-proved | apple-platform-adapter-dispatch-blocked-before-ci-proof',
        }),
        expect.objectContaining({ label: 'Platform state', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows[4].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform', value: 'ios' }),
        expect.objectContaining({ label: 'Host capability', value: 'not-applicable' }),
        expect.objectContaining({
          label: 'Open gaps',
          value:
            'ios-family-controls-not-proved | ios-managed-settings-not-proved | ios-testflight-device-not-proved | apple-platform-adapter-dispatch-blocked-before-ci-proof',
        }),
        expect.objectContaining({ label: 'Child delivery', value: 'Not claimed' }),
      ])
    );
  });

  it('renders missing read models as unavailable without rows', () => {
    const intent = createAppGamePlatformProofStatusPanelIntent(null);

    expect(intent.loadState).toBe('Unavailable');
    expect(intent.rows).toHaveLength(0);
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([expect.objectContaining({ label: 'Status', value: 'Unavailable' })])
    );
  });
});
