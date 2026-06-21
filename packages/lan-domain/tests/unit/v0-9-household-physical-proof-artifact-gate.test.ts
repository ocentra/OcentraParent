import { describe, expect, it } from 'vitest';
import {
  V09HouseholdPhysicalProofArtifactGateReadModelSchema,
  V09HouseholdPhysicalProofArtifactRequirementSchema,
  V09HouseholdPhysicalProofManualEvidenceStatusSchema,
  V09HouseholdPhysicalProofRouteHealthCheckSchema,
} from '@ocentra-parent/schema-domain/v0-9-household-physical-proof-artifact-gate';

const checkedAt = '2026-05-31T13:50:00.000Z';
const routeId = 'route-v0-9-household-physical-artifact-gate';

const readModel = {
  schemaVersion: 'v0-9-household-physical-proof-artifact-gate',
  checkedAt,
  readinessDecision: 'manual-evidence-required-before-physical-household-lan-readiness',
  physicalHouseholdLanClaimState: 'manual-required',
  cloudRelayState: 'not-implemented',
  sourceProofs: [
    sourceProof('v0-9-household-discovery-mobile-controller-product-proof'),
    sourceProof('v0-9-production-discovery-household-proof'),
    sourceProof('v0-9-production-lan-mobile-controller-proof'),
  ],
  artifactRequirements: [
    requirement('two-physical-household-hosts', 'two named household devices on the same LAN'),
    requirement('same-router-or-subnet-evidence', 'router, subnet, or network artifact tying both devices together'),
    requirement('child-service-router-reachability', 'physical child service reachable through household router'),
    requirement('os-firewall-or-local-network-permission', 'OS firewall or local-network permission artifact'),
    requirement('controller-origin-allowlist-artifact', 'physical controller origin allowlist evidence'),
    requirement('selected-device-route-recovery', 'selected route recovery after service restart on the child device'),
    requirement('controller-observer-route-health', 'controller and observer route health from physical clients'),
    requirement('revoked-route-rejection', 'revoked route rejected before control is accepted'),
    requirement('stale-offline-device-rejection', 'stale and offline selected device rejection artifacts'),
    requirement('real-mobile-controller-package', 'real Android or iOS parent package route artifact'),
    requirement('manual-evidence-custody-record', 'reviewable custody record for the manual evidence bundle'),
  ],
  deviceReadiness: [
    deviceReadiness('discovered-child-agent', routeId, 'discovered', 'paired', 'online', 'ci-mechanical-proof'),
    deviceReadiness('selected-child-route', routeId, 'paired', 'paired', 'online', 'ci-mechanical-proof'),
    deviceReadiness('parent-controller-origin', routeId, 'paired', 'paired', 'online', 'ci-mechanical-proof'),
    deviceReadiness('parent-observer-route', routeId, 'paired', 'paired', 'online', 'manual-required'),
  ],
  routeHealth: [
    routeHealth('selected-route-accepted', routeId, 'active-controller-backend-proof', null, 'ci-mechanical-proof'),
    routeHealth('observer-read-only', routeId, 'observer-read-only', 'observer-read-only', 'ci-mechanical-proof'),
    routeHealth(
      'controller-takeover-manual-required',
      null,
      'controller-takeover-manual-required',
      'takeover-denied',
      'manual-required'
    ),
    routeHealth('revoked-route-rejected', routeId, 'active-controller-backend-proof', 'revoked', 'ci-mechanical-proof'),
    routeHealth('stale-offline-route-rejected', routeId, 'unavailable', 'stale', 'ci-mechanical-proof'),
  ],
  manualEvidenceStatus: {
    custodyState: 'not-collected',
    requiredArtifactCount: 11,
    collectedArtifactCount: 0,
    missingArtifactCount: 11,
    reviewerSummary: 'manual physical household evidence has not been collected or reviewed',
  },
  claimsProved: ['local service proof identifies the remaining physical household artifact gates'],
  claimsNotProved: [
    'physical household LAN readiness',
    'two-device router/firewall path from real household hardware',
    'cloud relay routing or authentication',
  ],
} as const;

describe('V0.9 household physical proof artifact gate contracts', () => {
  it('accepts a complete artifact gate while preserving the manual-required physical LAN claim', () => {
    const parsed = V09HouseholdPhysicalProofArtifactGateReadModelSchema.parse(readModel);

    expect(parsed.physicalHouseholdLanClaimState).toBe('manual-required');
    expect(parsed.cloudRelayState).toBe('not-implemented');
    expect(parsed.artifactRequirements.map((artifact) => artifact.requirement)).toEqual([
      'two-physical-household-hosts',
      'same-router-or-subnet-evidence',
      'child-service-router-reachability',
      'os-firewall-or-local-network-permission',
      'controller-origin-allowlist-artifact',
      'selected-device-route-recovery',
      'controller-observer-route-health',
      'revoked-route-rejection',
      'stale-offline-device-rejection',
      'real-mobile-controller-package',
      'manual-evidence-custody-record',
    ]);
    expect(parsed.manualEvidenceStatus.collectedArtifactCount).toBe(0);
    expect(parsed.manualEvidenceStatus.missingArtifactCount).toBe(11);
  });

  it('rejects physical LAN, cloud relay, and incomplete route-health overclaims', () => {
    expect(
      V09HouseholdPhysicalProofArtifactGateReadModelSchema.safeParse({
        ...readModel,
        physicalHouseholdLanClaimState: 'ci-mechanical-proof',
      }).success
    ).toBe(false);
    expect(
      V09HouseholdPhysicalProofArtifactGateReadModelSchema.safeParse({
        ...readModel,
        cloudRelayState: 'ci-mechanical-proof',
      }).success
    ).toBe(false);
    expect(
      V09HouseholdPhysicalProofArtifactGateReadModelSchema.safeParse({
        ...readModel,
        routeHealth: readModel.routeHealth.filter((entry) => entry.check !== 'observer-read-only'),
      }).success
    ).toBe(false);
  });

  it('keeps artifact and route-health vocabularies explicit', () => {
    expect(V09HouseholdPhysicalProofArtifactRequirementSchema.parse('manual-evidence-custody-record')).toBe(
      'manual-evidence-custody-record'
    );
    expect(V09HouseholdPhysicalProofManualEvidenceStatusSchema.parse('manual-required')).toBe('manual-required');
    expect(V09HouseholdPhysicalProofRouteHealthCheckSchema.parse('revoked-route-rejected')).toBe(
      'revoked-route-rejected'
    );
    expect(V09HouseholdPhysicalProofArtifactRequirementSchema.safeParse('cloud-relay-ready').success).toBe(false);
  });
});

function sourceProof(source: unknown) {
  return {
    source,
    path: `test-results/${String(source)}/proof.json`,
    command: `node scripts/test/${String(source)}.mjs`,
  };
}

function requirement(requirementName: unknown, requiredArtifactSummary: unknown) {
  return {
    requirement: requirementName,
    status: 'manual-required',
    requiredArtifactSummary,
    evidencePath: null,
    evidenceCapturedAt: null,
  };
}

function deviceReadiness(
  check: unknown,
  deviceRouteId: unknown,
  discoveryState: unknown,
  trustState: unknown,
  reachability: unknown,
  runtimeProofState: unknown
) {
  return {
    check,
    routeId: deviceRouteId,
    discoveryState,
    trustState,
    reachability,
    runtimeProofState,
    physicalArtifactStatus: 'manual-required',
    evidenceLabel: `${String(check)} device readiness`,
  };
}

function routeHealth(
  check: unknown,
  healthRouteId: unknown,
  commandAuthorityState: unknown,
  rejectionReason: unknown,
  runtimeProofState: unknown
) {
  return {
    check,
    routeId: healthRouteId,
    commandAuthorityState,
    rejectionReason,
    runtimeProofState,
    physicalArtifactStatus: 'manual-required',
    evidenceLabel: `${String(check)} route health`,
  };
}
