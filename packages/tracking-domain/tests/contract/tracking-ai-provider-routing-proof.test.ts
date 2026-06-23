import { describe, expect, it } from 'vitest';
import { TrackingAiProviderRouteSchema } from '@ocentra-parent/schema-domain/tracking-location-policy';
import {
  TrackingAiProviderRoutingProofRowSchema,
  buildTrackingAiProviderRoutingProofRows,
  summarizeTrackingAiProviderRoutingProof,
} from '@ocentra-parent/schema-domain/tracking-ai-provider-routing-proof';

describe('tracking AI provider routing proof', () => {
  it('keeps child-local AI as the default child safety route', () => {
    const rows = buildTrackingAiProviderRoutingProofRows();
    const summary = summarizeTrackingAiProviderRoutingProof(rows);

    expect(summary.routeModes).toEqual([
      'child-local',
      'parent-local',
      'family-ai-hub',
      'parent-approved-remote',
      'metadata-only',
      'no-ai',
    ]);
    expect(summary.defaultChildSafetyRouteCount).toBe(1);
    expect(rows.find((row) => row.defaultChildSafetyPath)?.route.mode).toBe('child-local');
    expect(summary.allRowsHaveEvidenceAndCustody).toBe(true);
  });

  it('requires parent approval for the only remote-data route and preserves degraded states', () => {
    const summary = summarizeTrackingAiProviderRoutingProof(buildTrackingAiProviderRoutingProofRows());

    expect(summary.remoteAllowedRouteCount).toBe(1);
    expect(summary.remoteAllowedRoutesRequireParentApproval).toBe(true);
    expect(summary.degradedOrUnavailableRouteCount).toBe(4);
    expect(summary.capabilityStates).toContain('degraded');
    expect(summary.capabilityStates).toContain('unavailable');
    expect(summary.capabilityStates).toContain('disabled-by-default');
  });

  it('keeps assistant route rows preview-only with no direct writes or enforcement', () => {
    const summary = summarizeTrackingAiProviderRoutingProof(buildTrackingAiProviderRoutingProofRows());

    expect(summary.assistantCanWritePolicyDirectly).toBe(false);
    expect(summary.assistantCanEnforceDirectly).toBe(false);
  });

  it('rejects remote data without parent-approved route mode and recorded approval', () => {
    expect(
      TrackingAiProviderRouteSchema.safeParse({
        schemaVersion: 1,
        providerRouteId: 'metadata-only-tracking-ai-route',
        mode: 'metadata-only',
        capabilityState: 'available',
        remoteDataAllowed: true,
        unavailableReason: null,
        auditRefs: ['tracking-ai-provider-routing-proof'],
      }).success
    ).toBe(false);

    expect(
      TrackingAiProviderRoutingProofRowSchema.safeParse({
        ...buildTrackingAiProviderRoutingProofRows()[3],
        parentApprovalRecorded: false,
      }).success
    ).toBe(false);
  });

  it('rejects assistant direct policy-write upgrades', () => {
    expect(
      TrackingAiProviderRoutingProofRowSchema.safeParse({
        ...buildTrackingAiProviderRoutingProofRows()[0],
        assistantCanWritePolicyDirectly: true,
      }).success
    ).toBe(false);
  });
});
