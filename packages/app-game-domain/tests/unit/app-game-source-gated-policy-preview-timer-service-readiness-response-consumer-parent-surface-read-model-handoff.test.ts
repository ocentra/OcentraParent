import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

const UpstreamParentSurfaceHandoffPath = new URL(
  '../../../test-results/app-game-timer-parent-surface-proof/handoff.json',
  import.meta.url
);

const ReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff-proof',
  generatedAt: '2026-06-06T09:45:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof',
  ],
  parentSurfaceReadModelRef: 'future-service-readiness-response-consumer-parent-surface-read-model-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness response consumer parent-surface read-model handoff', () => {
  it('projects parent-surface handoff rows into parent-surface read-model states', () => {
    const handoff = buildReadModelHandoff();

    expect(handoff.parentSurfaceReadModelProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.responseConsumerParentSurfaceReadModelHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves parent-surface refs and adds read-model proof refs only for eligible rows', () => {
    const handoff = buildReadModelHandoff();

    expect(handoff.rows[0]?.inheritedParentSurfaceProofRefs).toEqual([
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-proof',
    ]);
    expect(handoff.rows[0]?.requiredParentSurfaceReadModelProofRefs).toEqual([
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof',
    ]);
    expect(handoff.rows[1]?.inheritedParentSurfaceProofRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
  });

  it('rejects read-model, parent-surface, service, portal, adapter, and count overclaims', () => {
    const handoff = buildReadModelHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceReadModelImplemented: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceRendered: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          serviceReadApiResponseConsumerImplemented: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          portalResponseConsumerRendered: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          adapterDispatchClaimed: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceReadModelProofRequiredCount: 0,
        }
      ).success
    ).toBe(false);
  });
});

function buildReadModelHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff(
    ReadModelHandoffOptions,
    readUpstreamParentSurfaceHandoff()
  );
}

function readUpstreamParentSurfaceHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceHandoffPath, 'utf8'))
  );
}
