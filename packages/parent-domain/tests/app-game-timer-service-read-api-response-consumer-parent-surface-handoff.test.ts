import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema } from '../src/app-game-timer-service-read-api-response-consumer-handoff';
import {
  AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema,
  buildAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoff,
} from '../src/app-game-timer-service-read-api-response-consumer-parent-surface-handoff';
import { AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState } from '../src/app-game-timer-service-read-api-response-consumer-parent-surface-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamResponseConsumerHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-read-api-response-consumer-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceHandoffId: 'app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof',
  generatedAt: '2026-06-07T13:55:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-api-response-consumer-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceProofRefs: ['future-app-game-timer-service-read-api-response-consumer-parent-surface-proof'],
} as const;

it('projects response consumer rows into parent-surface handoff states', () => {
  const handoff = buildParentSurfaceHandoff();

  expect(handoff.parentSurfaceProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.parentSurfaceHandoffState)).toEqual([
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired,
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness,
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision,
  ]);
});

it('inherits response consumer proof refs and adds parent-surface refs only for proof rows', () => {
  const handoff = buildParentSurfaceHandoff();

  expect(handoff.rows[0]?.inheritedServiceReadApiResponseConsumerProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-consumer-proof',
  ]);
  expect(handoff.rows[0]?.requiredParentSurfaceProofRefs).toEqual([
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-proof',
  ]);
  expect(handoff.rows[1]?.requiredParentSurfaceProofRefs).toEqual([]);
  expect(handoff.rows[2]?.requiredParentSurfaceProofRefs).toEqual([]);
});

it('rejects parent-surface rendering, runtime, adapter, raw-source, and count overclaims', () => {
  const handoff = buildParentSurfaceHandoff();

  expect(
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceRendered: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceReadModelRuntimeImplemented: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.safeParse({
      ...handoff,
      portalUiRendered: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.safeParse({
      ...handoff,
      serviceReadApiResponseConsumerImplemented: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.safeParse({
      ...handoff,
      adapterDispatchClaimed: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.safeParse({
      ...handoff,
      rawPrivateSourceRowsIncluded: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceProofRequiredCount: 0,
    }).success
  ).toBe(false);
});

function buildParentSurfaceHandoff() {
  return buildAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoff(
    ParentSurfaceHandoffOptions,
    readUpstreamResponseConsumerHandoff()
  );
}

function readUpstreamResponseConsumerHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamResponseConsumerHandoffPath, 'utf8'))
  );
}
