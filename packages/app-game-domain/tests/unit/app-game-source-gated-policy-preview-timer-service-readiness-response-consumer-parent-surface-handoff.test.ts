import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamResponseHandoffPath = new URL(
  '../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof/timer-service-readiness-read-api-response-consumer-handoff.json',
  import.meta.url
);

const ResponseConsumerHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof',
  generatedAt: '2026-06-06T08:56:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceProofRefs: ['future-app-game-timer-service-readiness-response-consumer-parent-surface-proof'],
  parentSurfaceSummaryRef: 'future-service-readiness-response-consumer-parent-surface-handoff-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness response consumer parent-surface handoff', () => {
  it('projects response handoff rows into response consumer proof states', () => {
    const handoff = buildResponseConsumerHandoff();

    expect(handoff.parentSurfaceProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.responseConsumerParentSurfaceHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves response proof refs and adds consumer proof refs only for eligible rows', () => {
    const handoff = buildResponseConsumerHandoff();

    expect(handoff.rows[0]?.inheritedReadApiResponseProofRefs).toEqual([
      'future-app-game-timer-service-readiness-read-api-response-proof',
    ]);
    expect(handoff.rows[0]?.requiredParentSurfaceProofRefs).toEqual([
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-proof',
    ]);
    expect(handoff.rows[1]?.inheritedReadApiResponseProofRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredParentSurfaceProofRefs).toEqual([]);
  });

  it('rejects response consumer, service, portal, adapter, and count overclaims', () => {
    const handoff = buildResponseConsumerHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiResponseConsumerImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiResponseImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        portalResponseConsumerRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildResponseConsumerHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff(
    ResponseConsumerHandoffOptions,
    readUpstreamResponseHandoff()
  );
}

function readUpstreamResponseHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamResponseHandoffPath, 'utf8'))
  );
}
