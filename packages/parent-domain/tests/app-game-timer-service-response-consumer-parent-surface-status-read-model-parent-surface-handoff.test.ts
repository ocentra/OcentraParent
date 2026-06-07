import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff,
} from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamStatusReadModelHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof',
  generatedAt: '2026-06-06T18:35:00Z',
  sourceContractRefs: [
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceProofRefs: [
    'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-proof',
  ],
  parentSurfaceRef: 'future-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface',
} as const;

describe('app-game timer service response consumer parent-surface status read-model parent-surface handoff', () => {
  it('projects status read-model rows into parent-surface handoff states', () => {
    const handoff = buildParentSurfaceHandoff();

    expect(handoff.parentSurfaceProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(
      handoff.rows.map((row) => row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState)
    ).toEqual([
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('inherits status read-model proof refs and adds parent-surface refs only for proof rows', () => {
    const handoff = buildParentSurfaceHandoff();

    expect(handoff.rows[0]?.inheritedParentSurfaceStatusReadModelProofRefs).toEqual([
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-proof',
    ]);
    expect(handoff.rows[0]?.requiredParentSurfaceProofRefs).toEqual([
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-proof',
    ]);
    expect(handoff.rows[0]?.parentSurfaceRef).toBe(
      'future-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface'
    );
    expect(handoff.rows[1]?.requiredParentSurfaceProofRefs).toEqual([]);
    expect(handoff.rows[2]?.requiredParentSurfaceProofRefs).toEqual([]);
  });

  it('rejects parent-surface, rendering, adapter, raw-source, and count overclaims', () => {
    const handoff = buildParentSurfaceHandoff();

    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceStatusReadModelParentSurfaceRuntimeImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        rawPrivateSourceRowsIncluded: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.safeParse({
        ...handoff,
        parentSurfaceProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildParentSurfaceHandoff() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff(
    ParentSurfaceHandoffOptions,
    readUpstreamStatusReadModelHandoff()
  );
}

function readUpstreamStatusReadModelHandoff() {
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamStatusReadModelHandoffPath, 'utf8'))
  );
}
