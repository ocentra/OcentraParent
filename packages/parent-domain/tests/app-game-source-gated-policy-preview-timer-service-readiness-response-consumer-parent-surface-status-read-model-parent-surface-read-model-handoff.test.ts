import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema } from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff,
} from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState } from '../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceHandoffPath = new URL(
  '../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof/timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff.json',
  import.meta.url
);

const ParentSurfaceReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof',
  generatedAt: '2026-06-06T11:05:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  ],
  parentSurfaceReadModelRef:
    'future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
} as const;

it('projects parent-surface rows into parent-surface read-model handoff states', () => {
  const handoff = buildParentSurfaceReadModelHandoff();

  expect(handoff.parentSurfaceReadModelProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(
    handoff.rows.map((row) => row.responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState)
  ).toEqual([
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedByCompilerDecision,
  ]);
});

it('preserves parent-surface refs and adds read-model refs only for eligible rows', () => {
  const handoff = buildParentSurfaceReadModelHandoff();

  expect(handoff.rows[0]?.inheritedParentSurfaceStatusReadModelProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
  ]);
  expect(handoff.rows[0]?.requiredParentSurfaceReadModelProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  ]);
  expect(handoff.rows[1]?.inheritedParentSurfaceStatusReadModelProofRefs).toEqual([]);
  expect(handoff.rows[1]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
});

it('rejects parent-surface, status read-model, portal, adapter, and count overclaims', () => {
  const handoff = buildParentSurfaceReadModelHandoff();

  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceStatusReadModelParentSurfaceImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceStatusReadModelParentSurfaceReadModelImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceStatusReadModelImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse(
      {
        ...handoff,
        portalUiRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse(
      {
        ...handoff,
        adapterDispatchClaimed: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceReadModelProofRequiredCount: 0,
      }
    ).success
  ).toBe(false);
});

function buildParentSurfaceReadModelHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff(
    ParentSurfaceReadModelHandoffOptions,
    readUpstreamStatusReadModelHandoff()
  );
}

function readUpstreamStatusReadModelHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceHandoffPath, 'utf8'))
  );
}
