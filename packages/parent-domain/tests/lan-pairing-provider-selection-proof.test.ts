import { describe, expect, it } from 'vitest';
import {
  LanProviderSelectionCandidateEvidenceSchema,
  LanProviderSelectionLifecycleStateSchema,
  LanProviderSelectionPolicyDecisionSchema,
  LanProviderSelectionReadModelSchema,
} from '../src/lan-pairing';

const checkedAt = '2026-05-30T17:50:00.000Z';
const routeId = 'lan-route-provider-selection-proof';
const providerPeerId = 'provider-peer-parent-desktop';
const sensitiveMarkers = ['rawEvidence', 'rawToken', 'activity.sqlite', 'activity.ndjson'] as const;

const providerSelectionReadModel = {
  schemaVersion: 'v0.9',
  checkedAt,
  selectedProviderRouteId: routeId,
  authorizedProviderSelectionState: 'ci-mechanical-proof',
  physicalHouseholdProviderProofState: 'manual-required',
  cloudRelayImplementationState: 'not-implemented',
  cloudRelayDecisionState: 'manual-decision-required',
  candidates: [
    candidate(
      'candidate-selected',
      'paired',
      'paired',
      'online',
      'authorized-result',
      null,
      'select-authorized-provider',
      'selected local provider route'
    ),
    candidate(
      'candidate-rejected',
      'unavailable',
      'unpaired',
      'online',
      'unavailable',
      'anonymous',
      'refuse-unpaired-provider',
      'unpaired provider refused'
    ),
    candidate(
      'candidate-rejected',
      'unavailable',
      'paired',
      'online',
      'unsupported-capability',
      'lan-ai-job-unauthorized',
      'refuse-unsupported-capability',
      'unsupported capability refused'
    ),
    candidate(
      'candidate-degraded',
      'paired',
      'paired',
      'online',
      'busy',
      null,
      'degrade-busy-provider',
      'busy provider degraded'
    ),
    candidate(
      'candidate-unavailable',
      'offline',
      'paired',
      'offline',
      'unavailable',
      'offline',
      'refuse-route-blocked-provider',
      'offline provider route blocked'
    ),
    candidate(
      'not-implemented',
      'unavailable',
      'unpaired',
      'offline',
      'unavailable',
      'local-network-disabled',
      'require-cloud-relay-decision',
      'cloud relay provider route not implemented',
      'not-implemented'
    ),
  ],
  manualRequirements: [
    manualRequirement('physical-household-provider-host', 'real opted-in provider host on household LAN'),
    manualRequirement('provider-route-origin-allowlist', 'physical parent origin allowlist evidence'),
    manualRequirement('provider-route-stale-offline-artifact', 'stale and offline provider route artifacts'),
    manualRequirement('provider-revocation-artifact', 'revoked provider route rejection artifact'),
    manualRequirement('cloud-relay-provider-decision', 'separate authenticated cloud relay product decision'),
  ],
};

describe('LAN provider selection proof contracts', () => {
  it('parses provider-selection lifecycle and keeps physical household provider proof manual-required', () => {
    const parsed = LanProviderSelectionReadModelSchema.parse(providerSelectionReadModel);

    expect(parsed.selectedProviderRouteId).toBe(routeId);
    expect(parsed.authorizedProviderSelectionState).toBe('ci-mechanical-proof');
    expect(parsed.physicalHouseholdProviderProofState).toBe('manual-required');
    expect(parsed.cloudRelayImplementationState).toBe('not-implemented');
    expect(parsed.cloudRelayDecisionState).toBe('manual-decision-required');
    expect(parsed.candidates.map((entry) => entry.lifecycleState)).toEqual([
      'candidate-selected',
      'candidate-rejected',
      'candidate-rejected',
      'candidate-degraded',
      'candidate-unavailable',
      'not-implemented',
    ]);
    expect(parsed.candidates.map((entry) => entry.policyDecision)).toContain('refuse-route-blocked-provider');
    expect(parsed.candidates.map((entry) => entry.routingState)).toContain('unsupported-capability');
    expect(parsed.manualRequirements.map((entry) => entry.requirement)).toContain('cloud-relay-provider-decision');
    for (const marker of sensitiveMarkers) {
      expect(JSON.stringify(parsed)).not.toContain(marker);
    }
  });

  it('rejects provider-selection states that would claim product-ready household routing', () => {
    expect(LanProviderSelectionLifecycleStateSchema.safeParse('product-ready')).toMatchObject({ success: false });
    expect(LanProviderSelectionPolicyDecisionSchema.safeParse('silently-fallback-to-cloud')).toMatchObject({
      success: false,
    });
    expect(
      LanProviderSelectionReadModelSchema.safeParse({
        ...providerSelectionReadModel,
        cloudRelayImplementationState: 'implemented',
      })
    ).toMatchObject({ success: false });
    expect(
      LanProviderSelectionCandidateEvidenceSchema.safeParse({
        ...candidate(
          'candidate-selected',
          'paired',
          'paired',
          'online',
          'authorized-result',
          null,
          'select-authorized-provider',
          'invalid product-ready proof',
          'product-ready'
        ),
      })
    ).toMatchObject({ success: false });
  });
});

function candidate(
  lifecycleState: unknown,
  discoveryState: unknown,
  trustState: unknown,
  reachability: unknown,
  routingState: unknown,
  rejectionReason: unknown,
  policyDecision: unknown,
  evidenceLabel: unknown,
  proofState: unknown = 'ci-mechanical-proof'
) {
  return {
    schemaVersion: 'v0.9',
    providerPeerId,
    routeId,
    lifecycleState,
    discoveryState,
    trustState,
    reachability,
    routingState,
    rejectionReason,
    policyDecision,
    proofState,
    evidenceLabel,
  };
}

function manualRequirement(requirement: unknown, requiredArtifactSummary: unknown) {
  return {
    schemaVersion: 'v0.9',
    requirement,
    state: 'manual-required',
    requiredArtifactSummary,
  };
}
