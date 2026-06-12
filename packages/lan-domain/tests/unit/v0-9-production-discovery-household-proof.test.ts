import { describe, expect, it } from 'vitest';
import {
  V09ProductionDiscoveryHouseholdCheckSchema,
  V09ProductionDiscoveryHouseholdProofBoundarySchema,
  V09ProductionDiscoveryHouseholdProofReadModelSchema,
  V09ProductionDiscoveryHouseholdRouteRecoveryStateSchema,
  V09ProductionDiscoveryHouseholdRuntimeOwnerSchema,
  V09ProductionDiscoveryHouseholdSourceStateSchema,
  V09ProductionDiscoveryHouseholdStateEvidenceSchema,
} from '../../src/lan-pairing';

const checkedAt = '2026-05-30T20:50:00.000Z';
const routeId = 'lan-route-production-discovery-household-proof';
const sensitiveMarkers = ['rawEvidence', 'rawToken', 'activity.sqlite', 'activity.ndjson'] as const;
const expectedChecks = [
  'production-discovery-states',
  'paired-route-accepted',
  'failed-unpaired-rejected',
  'replay-rejected',
  'restart-selected-route-recovered',
  'restart-registry-state-recovered',
  'stale-source-rejected',
  'offline-device-rejected',
  'revoked-pairing-rejected',
  'unavailable-route-rejected',
  'wrong-origin-rejected',
  'wrong-device-rejected',
  'manual-physical-household-checklist',
] as const;
const expectedSourceStates = [
  'discovered',
  'pending',
  'paired',
  'failed-unpaired',
  'restart-recovered',
  'stale',
  'offline',
  'revoked',
  'unavailable',
  'wrong-origin',
  'wrong-device',
  'manual-required',
] as const;

const readModel = {
  schemaVersion: 'v0.9',
  checkedAt,
  proofBoundary: 'local-real-service-not-physical-household-lan',
  productReadinessDecision: 'not-ready-for-product-ready-household-lan-claim',
  productionDiscoveryStates: [
    evidence('production-discovery-states', 'discovered', 'discovered', 'unpaired', 'online', null),
    evidence('production-discovery-states', 'pending', 'pending', 'pairing', 'online', null),
    evidence('production-discovery-states', 'paired', 'paired', 'paired', 'online', null),
    evidence('production-discovery-states', 'revoked', 'revoked', 'revoked', 'online', 'revoked'),
    evidence('production-discovery-states', 'stale', 'stale', 'paired', 'stale', 'stale'),
    evidence('production-discovery-states', 'offline', 'offline', 'paired', 'offline', 'offline'),
    evidence('production-discovery-states', 'unavailable', 'unavailable', 'unpaired', 'offline', 'unsupported-route'),
  ],
  routeChecks: [
    evidence('paired-route-accepted', 'paired', 'paired', 'paired', 'online', null),
    evidence('failed-unpaired-rejected', 'failed-unpaired', 'unavailable', 'unpaired', 'online', 'anonymous'),
    evidence('stale-source-rejected', 'stale', 'stale', 'paired', 'stale', 'stale'),
    evidence('offline-device-rejected', 'offline', 'offline', 'paired', 'offline', 'offline'),
    evidence('revoked-pairing-rejected', 'revoked', 'revoked', 'revoked', 'online', 'revoked'),
    evidence('unavailable-route-rejected', 'unavailable', 'unavailable', 'paired', 'online', 'unsupported-route'),
    evidence('wrong-origin-rejected', 'wrong-origin', 'unavailable', 'paired', 'online', 'wrong-origin'),
    evidence('wrong-device-rejected', 'wrong-device', 'unavailable', 'paired', 'online', 'wrong-device'),
  ],
  restartRecovery: [
    evidence(
      'restart-selected-route-recovered',
      'restart-recovered',
      'paired',
      'paired',
      'online',
      null,
      'registry-restored-after-restart'
    ),
    evidence(
      'restart-registry-state-recovered',
      'restart-recovered',
      'paired',
      'paired',
      'online',
      null,
      'selected-route-persisted'
    ),
  ],
  sourceDeviceStates: [
    evidence('stale-source-rejected', 'stale', 'stale', 'paired', 'stale', 'stale'),
    evidence('offline-device-rejected', 'offline', 'offline', 'paired', 'offline', 'offline'),
    evidence('revoked-pairing-rejected', 'revoked', 'revoked', 'revoked', 'online', 'revoked'),
    evidence(
      'unavailable-route-rejected',
      'unavailable',
      'unavailable',
      'unpaired',
      'offline',
      'local-network-disabled'
    ),
    evidence(
      'manual-physical-household-checklist',
      'manual-required',
      'unavailable',
      'unpaired',
      'offline',
      'local-network-disabled',
      'manual-required-physical-route-recovery',
      'manual-required',
      'manual-proof'
    ),
  ],
  manualHouseholdProofChecklist: [
    manualChecklistItem('two-physical-hosts', 'two named household devices on the same LAN'),
    manualChecklistItem('household-router-reachability', 'router or network reachability artifact'),
    manualChecklistItem('os-firewall-or-local-network-permission', 'firewall or OS local-network permission artifact'),
    manualChecklistItem('allowed-origin-on-physical-controller', 'allowed origin from the physical controller host'),
    manualChecklistItem('physical-route-selection-and-takeover', 'physical route selection and takeover artifact'),
    manualChecklistItem('physical-revocation-and-rejection', 'physical revocation before rejected follow-up control'),
    manualChecklistItem('physical-stale-offline-selected-device', 'stopped or paused selected child service artifact'),
    manualChecklistItem('real-mobile-controller-package', 'real Android or iOS controller package proof'),
    manualChecklistItem('real-mobile-observer-package', 'real Android or iOS observer package proof'),
    manualChecklistItem('real-lan-ai-provider-host', 'real opted-in provider host proof'),
    manualChecklistItem('cloud-relay-separate-proof', 'separate authenticated cloud relay proof'),
  ],
  claimsProved: [
    'local real-service production discovery proof preserves route checks and restart recovery states',
    'wrong-origin and wrong-device evidence remain explicit rejection states',
  ],
  claimsNotProved: [
    'physical household LAN readiness',
    'cloud relay routing storage or authentication',
    'mobile background controller behavior',
  ],
};

describe('V0.9 production discovery household proof contracts', () => {
  it('parses the household discovery read model with route checks restart recovery and manual proof gates', () => {
    const parsed = V09ProductionDiscoveryHouseholdProofReadModelSchema.parse(readModel);

    expect(parsed.proofBoundary).toBe('local-real-service-not-physical-household-lan');
    expect(parsed.productReadinessDecision).toBe('not-ready-for-product-ready-household-lan-claim');
    expect(new Set(parsed.routeChecks.map((entry) => entry.check))).toEqual(
      new Set([
        'paired-route-accepted',
        'failed-unpaired-rejected',
        'stale-source-rejected',
        'offline-device-rejected',
        'revoked-pairing-rejected',
        'unavailable-route-rejected',
        'wrong-origin-rejected',
        'wrong-device-rejected',
      ])
    );
    expect(new Set(parsed.productionDiscoveryStates.map((entry) => entry.sourceState))).toEqual(
      new Set(['discovered', 'pending', 'paired', 'revoked', 'stale', 'offline', 'unavailable'])
    );
    expect(parsed.restartRecovery.map((entry) => entry.routeRecoveryState)).toEqual([
      'registry-restored-after-restart',
      'selected-route-persisted',
    ]);
    expect(parsed.manualHouseholdProofChecklist).toHaveLength(11);
    expect(parsed.manualHouseholdProofChecklist.every((entry) => entry.state === 'manual-required')).toBe(true);
    expect(JSON.stringify(parsed)).not.toContain('"proofState":"product-ready"');
    for (const marker of sensitiveMarkers) {
      expect(JSON.stringify(parsed)).not.toContain(marker);
    }
  });

  it('keeps accepted state vocabulary explicit and rejects product-ready overclaims', () => {
    for (const expected of expectedChecks) {
      expect(V09ProductionDiscoveryHouseholdCheckSchema.parse(expected)).toBe(expected);
    }
    for (const expected of expectedSourceStates) {
      expect(V09ProductionDiscoveryHouseholdSourceStateSchema.parse(expected)).toBe(expected);
    }
    expect(V09ProductionDiscoveryHouseholdProofBoundarySchema.safeParse('physical-household-lan-ready')).toMatchObject({
      success: false,
    });
    expect(V09ProductionDiscoveryHouseholdRouteRecoveryStateSchema.safeParse('silently-restored')).toMatchObject({
      success: false,
    });
    expect(V09ProductionDiscoveryHouseholdRuntimeOwnerSchema.safeParse('portal-owned-runtime')).toMatchObject({
      success: false,
    });
    expect(
      V09ProductionDiscoveryHouseholdStateEvidenceSchema.safeParse({
        ...evidence('wrong-origin-rejected', 'wrong-origin', 'unavailable', 'paired', 'online', 'wrong-origin'),
        proofState: 'product-ready',
      })
    ).toMatchObject({ success: false });
  });
});

function evidence(
  check: unknown,
  sourceState: unknown,
  discoveryState: unknown,
  trustState: unknown,
  reachability: unknown,
  rejectionReason: unknown,
  routeRecoveryState: unknown = 'fail-closed-unpaired',
  proofState: unknown = 'ci-mechanical-proof',
  runtimeOwner: unknown = 'proof-harness'
) {
  return {
    schemaVersion: 'v0.9',
    check,
    sourceState,
    routeId,
    discoveryState,
    trustState,
    reachability,
    rejectionReason,
    routeRecoveryState,
    proofState,
    runtimeOwner,
    evidenceLabel: `${String(check)} evidence`,
  };
}

function manualChecklistItem(gate: unknown, requiredArtifactSummary: unknown) {
  return {
    schemaVersion: 'v0.9',
    gate,
    state: 'manual-required',
    requiredArtifactSummary,
    runtimeOwner: 'manual-proof',
  };
}
