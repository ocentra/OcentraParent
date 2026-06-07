import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel,
} from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceReadModelHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceReadModelOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelId:
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  generatedAt: '2026-06-06T19:05:00Z',
  sourceContractRefs: [
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
} as const;

describe('app-game timer service response consumer parent-surface status read-model parent-surface read-model contract', () => {
  it('builds parent-safe parent-surface read-model rows from WP113 handoff rows', () => {
    const readModel = buildParentSurfaceReadModel();

    expect(readModel.readyForParentSurfaceReadModelCount).toBe(1);
    expect(readModel.blockedBySourceFreshnessCount).toBe(1);
    expect(readModel.blockedByCompilerDecisionCount).toBe(1);
    expect(readModel.rows.map((row) => row.parentSurfaceReadModelState)).toEqual([
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves parent-surface read-model proof refs without raw private source rows', () => {
    const readModel = buildParentSurfaceReadModel();

    expect(readModel.rows[0]?.requiredParentSurfaceReadModelProofRefs).toEqual([
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
    ]);
    expect(readModel.rows[0]?.parentSafeSummary).toBe(
      'native-app:ready-for-parent-surface-status-read-model-parent-surface-read-model-contract'
    );
    expect(readModel.rows[1]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
    expect(readModel.rawPrivateSourceRowsIncluded).toBe(false);
  });

  it('rejects package export, runtime, rendering, persistence, adapter, and count overclaims', () => {
    const readModel = buildParentSurfaceReadModel();

    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse({
        ...readModel,
        packageExported: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse({
        ...readModel,
        parentSurfaceReadModelRuntimeImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse({
        ...readModel,
        runtimeReadModelPersisted: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse({
        ...readModel,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse({
        ...readModel,
        readyForParentSurfaceReadModelCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildParentSurfaceReadModel() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel(
    ParentSurfaceReadModelOptions,
    readUpstreamHandoff()
  );
}

function readUpstreamHandoff() {
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceReadModelHandoffPath, 'utf8'))
  );
}
