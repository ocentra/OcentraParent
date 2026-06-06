import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema } from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff,
} from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState } from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamStatusReadModelHandoffPath = new URL(
  '../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof/timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.json',
  import.meta.url
);

const ParentSurfaceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof',
  generatedAt: '2026-06-06T10:45:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
  ],
  parentSurfaceRef: 'future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
} as const;

it('projects status read-model rows into parent-surface handoff states', () => {
  const handoff = buildParentSurfaceHandoff();

  expect(handoff.parentSurfaceProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.responseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState)).toEqual([
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedByCompilerDecision,
  ]);
});

it('preserves status read-model refs and adds parent-surface refs only for eligible rows', () => {
  const handoff = buildParentSurfaceHandoff();

  expect(handoff.rows[0]?.inheritedParentSurfaceStatusReadModelProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof',
  ]);
  expect(handoff.rows[0]?.requiredParentSurfaceProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
  ]);
  expect(handoff.rows[1]?.inheritedParentSurfaceStatusReadModelProofRefs).toEqual([]);
  expect(handoff.rows[1]?.requiredParentSurfaceProofRefs).toEqual([]);
});

it('rejects parent-surface, status read-model, portal, adapter, and count overclaims', () => {
  const handoff = buildParentSurfaceHandoff();

  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceStatusReadModelParentSurfaceImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceStatusReadModelImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse(
      {
        ...handoff,
        portalUiRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse(
      {
        ...handoff,
        adapterDispatchClaimed: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceProofRequiredCount: 0,
      }
    ).success
  ).toBe(false);
});

function buildParentSurfaceHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff(
    ParentSurfaceHandoffOptions,
    readUpstreamStatusReadModelHandoff()
  );
}

function readUpstreamStatusReadModelHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamStatusReadModelHandoffPath, 'utf8'))
  );
}
