import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema } from '../src/app-game-timer-service-response-consumer-parent-surface-read-model-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoff,
} from '../src/app-game-timer-service-response-consumer-parent-surface-status-handoff';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState } from '../src/app-game-timer-service-response-consumer-parent-surface-status-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceReadModelHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-response-consumer-parent-surface-read-model-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceStatusHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceResponseConsumerParentSurfaceStatusHandoffId:
    'app-game-timer-service-response-consumer-parent-surface-status-handoff-proof',
  generatedAt: '2026-06-06T16:10:00Z',
  sourceContractRefs: [
    'app-game-timer-service-response-consumer-parent-surface-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceStatusProofRefs: [
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-proof',
  ],
} as const;

it('projects parent-surface read-model rows into parent-surface status handoff states', () => {
  const handoff = buildParentSurfaceStatusHandoff();

  expect(handoff.parentSurfaceStatusProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.serviceResponseConsumerParentSurfaceStatusHandoffState)).toEqual([
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired,
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness,
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision,
  ]);
});

it('inherits parent-surface read-model proof refs and adds status refs only for proof rows', () => {
  const handoff = buildParentSurfaceStatusHandoff();

  expect(handoff.rows[0]?.inheritedParentSurfaceReadModelProofRefs).toEqual([
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-read-model-proof',
  ]);
  expect(handoff.rows[0]?.requiredParentSurfaceStatusProofRefs).toEqual([
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-proof',
  ]);
  expect(handoff.rows[1]?.requiredParentSurfaceStatusProofRefs).toEqual([]);
  expect(handoff.rows[2]?.requiredParentSurfaceStatusProofRefs).toEqual([]);
});

it('rejects parent-surface status, rendering, adapter, raw-source, and count overclaims', () => {
  const handoff = buildParentSurfaceStatusHandoff();

  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceStatusRuntimeImplemented: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceStatusPersisted: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceRendered: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
      ...handoff,
      adapterDispatchClaimed: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
      ...handoff,
      rawPrivateSourceRowsIncluded: true,
    }).success
  ).toBe(false);
  expect(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.safeParse({
      ...handoff,
      parentSurfaceStatusProofRequiredCount: 0,
    }).success
  ).toBe(false);
});

function buildParentSurfaceStatusHandoff() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoff(
    ParentSurfaceStatusHandoffOptions,
    readUpstreamParentSurfaceReadModelHandoff()
  );
}

function readUpstreamParentSurfaceReadModelHandoff() {
  return AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceReadModelHandoffPath, 'utf8'))
  );
}
