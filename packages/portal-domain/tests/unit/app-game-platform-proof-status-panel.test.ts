import { describe, expect, it } from 'vitest';
import { createAppGamePlatformProofStatusPanelIntent } from '../../src/app-game-platform-proof-status-panel';

const ReadModel = {
  generatedAt: '2026-06-08T16:20:00.000Z',
  returned: 3,
  hostVisibleCount: 1,
  hostNotDetectedCount: 1,
  localRuntimeNotApplicableCount: 1,
  enforcementReadyCount: 0,
  openGapCount: 8,
  rows: [
    {
      platform: 'windows',
      proofState: 'windows-policy-preflight-observed',
      authorityState: 'visibility-only',
      hostCapabilityState: 'available',
      hostCapabilityEvidenceRefs: ['windows-local-policy-evidence-proof-ref'],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: ['windows-broad-blocking-authority-preflight-ref'],
      openGaps: [
        'windows-applocker-enforce-not-proved',
        'windows-broad-blocking-not-proved',
        'cross-platform-child-delivery-not-proved',
      ],
    },
    {
      platform: 'linux',
      proofState: 'physical-device-observed',
      authorityState: 'visibility-only',
      hostCapabilityState: 'not-detected',
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: ['linux-host-local-probe-ref'],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: ['android-physical-device-proof-ref'],
      openGaps: [
        'android-device-owner-not-proved',
        'android-authority-preflight-not-attached',
        'cross-platform-child-delivery-not-proved',
      ],
    },
    {
      platform: 'ios',
      proofState: 'apple-ci-artifacts-required',
      authorityState: 'visibility-only',
      hostCapabilityState: 'not-applicable',
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: [],
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      proofRefs: ['apple-ci-platform-proof-preflight-ref'],
      openGaps: [
        'ios-family-controls-not-proved',
        'apple-platform-adapter-dispatch-blocked-before-ci-proof',
      ],
    },
  ],
} as const;

describe('app-game platform proof status panel intent', () => {
  it('renders normalized host-capability proof rows without enforcement upgrades', () => {
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
        expect.objectContaining({ label: 'Platform proofs', value: '3' }),
        expect.objectContaining({ label: 'Host-visible rows', value: '1' }),
        expect.objectContaining({ label: 'Host not-detected rows', value: '1' }),
        expect.objectContaining({ label: 'Not-applicable rows', value: '1' }),
        expect.objectContaining({ label: 'Enforcement-ready rows', value: '0' }),
        expect.objectContaining({ label: 'Open gaps', value: '8' }),
      ])
    );
    expect(intent.rows.map((row) => row.title)).toEqual(['windows', 'linux', 'ios']);
    expect(intent.rows[0].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform', value: 'windows' }),
        expect.objectContaining({ label: 'Authority state', value: 'visibility-only' }),
        expect.objectContaining({ label: 'Host capability', value: 'available' }),
        expect.objectContaining({ label: 'Host capability evidence', value: 'windows-local-policy-evidence-proof-ref' }),
        expect.objectContaining({ label: 'Host capability probe', value: 'windows-host-local-probe-ref' }),
        expect.objectContaining({ label: 'Platform state', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows[1].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform', value: 'linux' }),
        expect.objectContaining({ label: 'Host capability', value: 'not-detected' }),
        expect.objectContaining({ label: 'Host capability evidence', value: 'Not reported' }),
        expect.objectContaining({
          label: 'Host capability probe',
          value: 'linux-host-local-probe-ref',
        }),
        expect.objectContaining({ label: 'Broad blocking', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows[2].details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Platform', value: 'ios' }),
        expect.objectContaining({ label: 'Host capability', value: 'not-applicable' }),
        expect.objectContaining({ label: 'Evidence references', value: 'apple-ci-platform-proof-preflight-ref' }),
        expect.objectContaining({
          label: 'Open gaps',
          value: 'ios-family-controls-not-proved | apple-platform-adapter-dispatch-blocked-before-ci-proof',
        }),
        expect.objectContaining({ label: 'Platform state', value: 'Not claimed' }),
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
