import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff,
} from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId:
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof',
  generatedAt: '2026-06-06T18:50:00Z',
  sourceContractRefs: [
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceReadModelProofRefs: [
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  ],
  parentSurfaceReadModelRef:
    'future-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
} as const;

describe('app-game timer service response consumer parent-surface status read-model parent-surface read-model handoff', () => {
  it('projects parent-surface rows into parent-surface read-model handoff states', () => {
    const handoff = buildParentSurfaceReadModelHandoff();

    expect(handoff.parentSurfaceReadModelProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(
      handoff.rows.map(
        (row) => row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState
      )
    ).toEqual([
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedBySourceFreshness,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('inherits parent-surface proof refs and adds parent-surface read-model refs only for proof rows', () => {
    const handoff = buildParentSurfaceReadModelHandoff();

    expect(handoff.rows[0]?.inheritedParentSurfaceProofRefs).toEqual([
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-proof',
    ]);
    expect(handoff.rows[0]?.requiredParentSurfaceReadModelProofRefs).toEqual([
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
    ]);
    expect(handoff.rows[0]?.parentSurfaceReadModelRef).toBe(
      'future-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-read-model'
    );
    expect(handoff.rows[1]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
    expect(handoff.rows[2]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
  });

  it('rejects parent-surface read-model, rendering, adapter, raw-source, and count overclaims', () => {
    const handoff = buildParentSurfaceReadModelHandoff();

    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceStatusReadModelParentSurfaceReadModelRuntimeImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse({
        ...handoff,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse({
        ...handoff,
        rawPrivateSourceRowsIncluded: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceReadModelProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildParentSurfaceReadModelHandoff() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff(
    ParentSurfaceReadModelHandoffOptions,
    readUpstreamParentSurfaceHandoff()
  );
}

function readUpstreamParentSurfaceHandoff() {
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceHandoffPath, 'utf8'))
  );
}
