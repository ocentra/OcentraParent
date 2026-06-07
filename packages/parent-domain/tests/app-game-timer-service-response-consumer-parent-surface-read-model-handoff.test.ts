import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoffSchema as AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema } from '../src/app-game-timer-service-read-api-response-consumer-parent-surface-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoff,
} from '../src/app-game-timer-service-response-consumer-parent-surface-read-model-handoff';
import { AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState } from '../src/app-game-timer-service-response-consumer-parent-surface-read-model-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceResponseConsumerParentSurfaceReadModelHandoffId:
    'app-game-timer-service-response-consumer-parent-surface-read-model-handoff-proof',
  generatedAt: '2026-06-06T15:30:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-api-response-consumer-parent-surface-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceReadModelProofRefs: [
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-read-model-proof',
  ],
} as const;

it('projects parent-surface handoff rows into parent-surface read-model handoff states', () => {
  const handoff = buildParentSurfaceReadModelHandoff();

  expect(handoff.parentSurfaceReadModelProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.serviceResponseConsumerParentSurfaceReadModelHandoffState)).toEqual([
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired,
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness,
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision,
  ]);
});

it('inherits parent-surface proof refs and adds read-model refs only for proof rows', () => {
  const handoff = buildParentSurfaceReadModelHandoff();

  expect(handoff.rows[0]?.inheritedParentSurfaceProofRefs).toEqual([
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-proof',
  ]);
  expect(handoff.rows[0]?.requiredParentSurfaceReadModelProofRefs).toEqual([
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-read-model-proof',
  ]);
  expect(handoff.rows[1]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
  expect(handoff.rows[2]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
});

it('rejects parent-surface read-model, rendering, adapter, raw-source, and count overclaims', () => {
  const handoff = buildParentSurfaceReadModelHandoff();

  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceReadModelRuntimeImplemented: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceReadModelPersisted: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceRendered: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse({
      ...handoff,
      adapterDispatchClaimed: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse({
      ...handoff,
      rawPrivateSourceRowsIncluded: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceReadModelProofRequiredCount: 0,
    }).success
  ).toBe(false);
});

function buildParentSurfaceReadModelHandoff() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoff(
    ParentSurfaceReadModelHandoffOptions,
    readUpstreamParentSurfaceHandoff()
  );
}

function readUpstreamParentSurfaceHandoff() {
  return AppGameTimerServiceResponseConsumerParentSurfaceHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceHandoffPath, 'utf8'))
  );
}
