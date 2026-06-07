import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema } from '../src/app-game-timer-service-response-consumer-parent-surface-status-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoff,
} from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceStatusHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-response-consumer-parent-surface-status-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceStatusReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceResponseConsumerParentSurfaceStatusReadModelHandoffId:
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof',
  generatedAt: '2026-06-06T16:25:00Z',
  sourceContractRefs: [
    'app-game-timer-service-response-consumer-parent-surface-status-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceStatusReadModelProofRefs: [
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-proof',
  ],
} as const;

describe('app-game timer service response consumer parent-surface status read-model handoff', () => {
  it('projects parent-surface status rows into parent-surface status read-model handoff states', () => {
    const handoff = buildParentSurfaceStatusReadModelHandoff();

    expect(handoff.parentSurfaceStatusReadModelProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState)).toEqual([
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('inherits parent-surface status proof refs and adds status read-model refs only for proof rows', () => {
    const handoff = buildParentSurfaceStatusReadModelHandoff();

    expect(handoff.rows[0]?.inheritedParentSurfaceStatusProofRefs).toEqual([
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-proof',
    ]);
    expect(handoff.rows[0]?.requiredParentSurfaceStatusReadModelProofRefs).toEqual([
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-proof',
    ]);
    expect(handoff.rows[1]?.requiredParentSurfaceStatusReadModelProofRefs).toEqual([]);
    expect(handoff.rows[2]?.requiredParentSurfaceStatusReadModelProofRefs).toEqual([]);
  });

  it('rejects parent-surface status read-model, rendering, adapter, raw-source, and count overclaims', () => {
    const handoff = buildParentSurfaceStatusReadModelHandoff();

    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceStatusReadModelRuntimeImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceStatusReadModelPersisted: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse({
        ...handoff,
        rawPrivateSourceRowsIncluded: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceStatusReadModelProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildParentSurfaceStatusReadModelHandoff() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoff(
    ParentSurfaceStatusReadModelHandoffOptions,
    readUpstreamParentSurfaceStatusHandoff()
  );
}

function readUpstreamParentSurfaceStatusHandoff() {
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceStatusHandoffPath, 'utf8'))
  );
}
