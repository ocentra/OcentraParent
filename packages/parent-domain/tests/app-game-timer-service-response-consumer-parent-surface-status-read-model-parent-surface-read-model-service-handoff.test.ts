import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff,
} from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamParentSurfaceReadModelPath = new URL(
  '../../../test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof/read-model.json',
  import.meta.url
);

const ServiceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceHandoffId:
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-proof',
  generatedAt: '2026-06-07T15:55:00Z',
  sourceContractRefs: [
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceProofRefs: [
    'future-app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-proof',
  ],
} as const;

describe('app-game timer service response consumer parent-surface status read-model parent-surface read-model service handoff', () => {
  it('projects parent-surface read-model rows into service handoff states', () => {
    const handoff = buildServiceHandoff();

    expect(handoff.serviceProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.parentSurfaceReadModelServiceHandoffState)).toEqual([
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('carries parent-safe summaries and adds service refs only for service-proof rows', () => {
    const handoff = buildServiceHandoff();

    expect(handoff.rows[0]?.parentSafeSummary).toBe(
      'native-app:ready-for-parent-surface-status-read-model-parent-surface-read-model-contract'
    );
    expect(handoff.rows[0]?.requiredServiceProofRefs).toEqual([
      'future-app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-proof',
    ]);
    expect(handoff.rows[1]?.requiredServiceProofRefs).toEqual([]);
    expect(handoff.rows[2]?.requiredServiceProofRefs).toEqual([]);
  });

  it('rejects service, runtime, rendering, adapter, raw-source, and count overclaims', () => {
    const handoff = buildServiceHandoff();

    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.safeParse(
        {
          ...handoff,
          serviceReadModelEmitted: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceReadModelRuntimeImplemented: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.safeParse(
        {
          ...handoff,
          portalUiRendered: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.safeParse(
        {
          ...handoff,
          adapterDispatchClaimed: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.safeParse(
        {
          ...handoff,
          rawPrivateSourceRowsIncluded: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.safeParse(
        {
          ...handoff,
          serviceProofRequiredCount: 0,
        }
      ).success
    ).toBe(false);
  });
});

function buildServiceHandoff() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff(
    ServiceHandoffOptions,
    readUpstreamReadModel()
  );
}

function readUpstreamReadModel() {
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceReadModelPath, 'utf8'))
  );
}
