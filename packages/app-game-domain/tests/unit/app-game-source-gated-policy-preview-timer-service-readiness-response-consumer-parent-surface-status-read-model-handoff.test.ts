import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamParentSurfaceStatusHandoffPath = new URL(
  '../../../test-results/app-game-timer-parent-status-proof/handoff.json',
  import.meta.url
);

const StatusReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof',
  generatedAt: '2026-06-06T10:30:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceStatusReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof',
  ],
  parentSurfaceStatusReadModelRef: 'future-service-readiness-response-consumer-parent-surface-status-read-model-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness response consumer parent-surface status read-model handoff', () => {
  it('projects parent-surface status handoff rows into status read-model states', () => {
    const handoff = buildStatusReadModelHandoff();

    expect(handoff.parentSurfaceStatusReadModelProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.responseConsumerParentSurfaceStatusReadModelHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves status refs and adds status read-model refs only for eligible rows', () => {
    const handoff = buildStatusReadModelHandoff();

    expect(handoff.rows[0]?.inheritedParentSurfaceStatusProofRefs).toEqual([
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-proof',
    ]);
    expect(handoff.rows[0]?.requiredParentSurfaceStatusReadModelProofRefs).toEqual([
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof',
    ]);
    expect(handoff.rows[1]?.inheritedParentSurfaceStatusProofRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredParentSurfaceStatusReadModelProofRefs).toEqual([]);
  });

  it('rejects status read-model, status, service, portal, adapter, and count overclaims', () => {
    const handoff = buildStatusReadModelHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceStatusReadModelImplemented: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceStatusImplemented: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          serviceReadApiResponseConsumerImplemented: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          portalResponseConsumerRendered: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          adapterDispatchClaimed: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceStatusReadModelProofRequiredCount: 0,
        }
      ).success
    ).toBe(false);
  });
});

function buildStatusReadModelHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff(
    StatusReadModelHandoffOptions,
    readUpstreamParentSurfaceStatusHandoff()
  );
}

function readUpstreamParentSurfaceStatusHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceStatusHandoffPath, 'utf8'))
  );
}
