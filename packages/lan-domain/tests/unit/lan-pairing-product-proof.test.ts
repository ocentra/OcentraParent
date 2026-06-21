import { describe, expect, it } from 'vitest';
import {
  LanHouseholdCloudRelayDecisionSchema,
  LanHouseholdProductProofReadModelSchema,
  LanHouseholdProductProofStateSchema,
  LanHouseholdRouteCheckOutcomeSchema,
  LanHouseholdSelectedProviderPolicyEvidenceSchema,
  LanHouseholdSelectedRouteEvidenceSchema,
} from '@ocentra-parent/schema-domain/lan-pairing-product-proof';

const checkedAt = '2026-05-30T13:05:00.000Z';
const routeId = 'lan-route-household-product-proof';
const sensitiveMarkers = ['rawEvidence', 'rawToken', 'activity.sqlite', 'activity.ndjson'] as const;
const expectedRouteChecks = [
  'paired-route-accepted',
  'failed-unpaired-rejected',
  'wrong-origin-rejected',
  'wrong-device-rejected',
  'replay-rejected',
  'revocation-rejected',
  'stale-selected-device-rejected',
  'offline-selected-device-rejected',
  'unavailable-route-rejected',
  'manual-required-physical-household-lan',
] as const;

const productProofReadModel = {
  schemaVersion: 'v0.9',
  checkedAt,
  productReadinessDecision: 'not-ready-for-product-ready-household-lan-claim',
  localMultiServiceProofState: 'ci-mechanical-proof',
  physicalHouseholdLanProofState: 'manual-required',
  parentMobileControllerProofState: 'manual-required',
  cloudRelayState: 'not-implemented',
  cloudRelayDecision: cloudRelayDecision(),
  routeCheckOutcomes: [
    routeCheckOutcome('paired-route-accepted', 'paired', 'paired', 'online', null, 'paired route accepted'),
    routeCheckOutcome(
      'failed-unpaired-rejected',
      'unavailable',
      'unpaired',
      'online',
      'anonymous',
      'unpaired route rejected'
    ),
    routeCheckOutcome(
      'wrong-origin-rejected',
      'unavailable',
      'paired',
      'online',
      'wrong-origin',
      'wrong origin route rejected'
    ),
    routeCheckOutcome(
      'wrong-device-rejected',
      'unavailable',
      'paired',
      'online',
      'wrong-device',
      'wrong device route rejected'
    ),
    routeCheckOutcome('replay-rejected', 'unavailable', 'paired', 'online', 'replayed', 'replay rejected'),
    routeCheckOutcome('revocation-rejected', 'revoked', 'revoked', 'online', 'revoked', 'revoked rejected'),
    routeCheckOutcome(
      'stale-selected-device-rejected',
      'stale',
      'paired',
      'stale',
      'stale',
      'stale selected route rejected'
    ),
    routeCheckOutcome(
      'offline-selected-device-rejected',
      'offline',
      'paired',
      'offline',
      'offline',
      'offline selected route rejected'
    ),
    routeCheckOutcome(
      'unavailable-route-rejected',
      'unavailable',
      'paired',
      'online',
      'unsupported-route',
      'unavailable route rejected'
    ),
    routeCheckOutcome(
      'manual-required-physical-household-lan',
      'unavailable',
      'unpaired',
      'offline',
      'local-network-disabled',
      'physical household LAN manual gate',
      'manual-required'
    ),
  ],
  selectedRouteEvidence: [
    selectedRouteEvidence('paired', 'online', null, 'paired local route accepted'),
    selectedRouteEvidence('paired', 'stale', 'stale', 'stale selected route rejected'),
    selectedRouteEvidence('paired', 'offline', 'offline', 'offline selected route rejected'),
    selectedRouteEvidence('revoked', 'online', 'revoked', 'revoked route rejected'),
  ],
  selectedProviderPolicyEvidence: [
    selectedProviderEvidence('authorized-result', 'paired', 'online', null, 'available provider route'),
    selectedProviderEvidence('unavailable', 'unpaired', 'online', 'anonymous', 'unpaired provider route'),
    selectedProviderEvidence('degraded', 'paired', 'online', null, 'degraded provider route'),
    selectedProviderEvidence('unavailable', 'paired', 'stale', 'stale', 'stale provider route'),
    selectedProviderEvidence('unavailable', 'revoked', 'online', 'revoked', 'revoked provider route'),
  ],
  manualProofGates: [
    manualGate('two-physical-hosts', 'two named household devices on the same LAN'),
    manualGate('household-router-reachability', 'router or network note proving child service reachability'),
    manualGate('os-firewall-or-local-network-permission', 'OS firewall or local network permission artifact'),
    manualGate('physical-stale-offline-selected-device', 'selected child service stopped before control'),
    manualGate('real-mobile-controller-package', 'Android or iOS parent mobile package evidence'),
    manualGate('real-lan-ai-provider-host', 'opted-in provider host evidence from a real household device'),
    manualGate('cloud-relay-separate-proof', 'separate authenticated cloud relay proof before any relay claim'),
  ],
};

describe('LAN household product proof contracts', () => {
  it('parses product proof states and keeps physical household LAN evidence manual-required', () => {
    const parsed = LanHouseholdProductProofReadModelSchema.parse(productProofReadModel);

    expect(parsed.physicalHouseholdLanProofState).toBe('manual-required');
    expect(parsed.cloudRelayDecision.implementationState).toBe('not-implemented');
    expect(parsed.cloudRelayDecision.decisionState).toBe('manual-decision-required');
    expect(parsed.productReadinessDecision).toBe('not-ready-for-product-ready-household-lan-claim');
    expect(parsed.routeCheckOutcomes.map((outcome) => outcome.check)).toEqual(expectedRouteChecks);
    expect(parsed.selectedRouteEvidence.map((evidence) => evidence.rejectionReason)).toContain('offline');
    expect(parsed.selectedProviderPolicyEvidence.map((evidence) => evidence.routingState)).toContain('degraded');
    expect(JSON.stringify(parsed)).not.toContain('cloud-relay-implemented');
    for (const marker of sensitiveMarkers) {
      expect(JSON.stringify(parsed)).not.toContain(marker);
    }
  });

  it('rejects product proof states that would claim physical LAN readiness', () => {
    expect(LanHouseholdProductProofStateSchema.safeParse('product-ready')).toMatchObject({ success: false });
    expect(
      LanHouseholdCloudRelayDecisionSchema.safeParse({
        ...cloudRelayDecision(),
        implementationState: 'implemented',
      })
    ).toMatchObject({ success: false });
    expect(
      LanHouseholdRouteCheckOutcomeSchema.safeParse({
        ...routeCheckOutcome(
          'manual-required-physical-household-lan',
          'unavailable',
          'unpaired',
          'offline',
          'local-network-disabled',
          'physical household LAN manual gate',
          'product-ready'
        ),
      })
    ).toMatchObject({ success: false });
    expect(
      LanHouseholdSelectedRouteEvidenceSchema.safeParse(
        selectedRouteEvidence('paired', 'online', 'wrong-origin', 'invalid rejection on accepted route')
      )
    ).toMatchObject({ success: true });
    expect(
      LanHouseholdSelectedProviderPolicyEvidenceSchema.safeParse(
        selectedProviderEvidence('authorized-result', 'paired', 'online', null, 'accepted provider route')
      )
    ).toMatchObject({ success: true });
  });
});

function selectedRouteEvidence(
  trustState: unknown,
  reachability: unknown,
  rejectionReason: unknown,
  evidenceLabel: unknown
) {
  return {
    schemaVersion: 'v0.9',
    routeId,
    discoveryState: reachability === 'offline' || reachability === 'stale' ? reachability : trustState,
    trustState,
    reachability,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    evidenceLabel,
  };
}

function routeCheckOutcome(
  check: unknown,
  discoveryState: unknown,
  trustState: unknown,
  reachability: unknown,
  rejectionReason: unknown,
  evidenceLabel: unknown,
  proofState: unknown = 'ci-mechanical-proof'
) {
  return {
    schemaVersion: 'v0.9',
    routeId,
    check,
    discoveryState,
    trustState,
    reachability,
    rejectionReason,
    proofState,
    evidenceLabel,
  };
}

function selectedProviderEvidence(
  routingState: unknown,
  selectedRouteTrustState: unknown,
  selectedDeviceReachability: unknown,
  rejectionReason: unknown,
  evidenceLabel: unknown
) {
  return {
    schemaVersion: 'v0.9',
    routeId,
    routingState,
    selectedRouteTrustState,
    selectedDeviceReachability,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    evidenceLabel,
  };
}

function cloudRelayDecision() {
  return {
    schemaVersion: 'v0.9',
    implementationState: 'not-implemented',
    decisionState: 'manual-decision-required',
    requiredDecisionSummary: 'separate authenticated cloud relay proof is required before any relay claim',
    proofBoundary: 'no cloud relay runtime, storage, authentication, or routing exists in this proof',
  };
}

function manualGate(gate: unknown, requiredArtifactSummary: unknown) {
  return {
    schemaVersion: 'v0.9',
    gate,
    state: 'manual-required',
    requiredArtifactSummary,
  };
}
