import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema } from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff,
} from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceReadModelHandoffPath = new URL(
  '../../../test-results/app-game-timer-parent-rm-proof/handoff.json',
  import.meta.url
);

const StatusHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff-proof',
  generatedAt: '2026-06-06T10:05:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceStatusProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-proof',
  ],
  parentSurfaceStatusRef: 'future-service-readiness-response-consumer-parent-surface-status-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness response consumer parent-surface status handoff', () => {
  it('projects parent-surface read-model handoff rows into parent-surface status states', () => {
    const handoff = buildStatusHandoff();

    expect(handoff.parentSurfaceStatusProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.responseConsumerParentSurfaceStatusHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves read-model refs and adds status proof refs only for eligible rows', () => {
    const handoff = buildStatusHandoff();

    expect(handoff.rows[0]?.inheritedParentSurfaceReadModelProofRefs).toEqual([
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof',
    ]);
    expect(handoff.rows[0]?.requiredParentSurfaceStatusProofRefs).toEqual([
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-proof',
    ]);
    expect(handoff.rows[1]?.inheritedParentSurfaceReadModelProofRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredParentSurfaceStatusProofRefs).toEqual([]);
  });

  it('rejects status, read-model, service, portal, adapter, and count overclaims', () => {
    const handoff = buildStatusHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceStatusImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceReadModelImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiResponseConsumerImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
        ...handoff,
        portalResponseConsumerRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceStatusProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildStatusHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff(
    StatusHandoffOptions,
    readUpstreamParentSurfaceReadModelHandoff()
  );
}

function readUpstreamParentSurfaceReadModelHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceReadModelHandoffPath, 'utf8'))
  );
}
