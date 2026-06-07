import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema,
  buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoff,
} from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-handoff';
import { AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState } from '../src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamServiceHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-proof/handoff.json',
  import.meta.url
);

const ServiceReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadModelHandoffId:
    'timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-handoff-proof',
  generatedAt: '2026-06-06T12:00:00Z',
  sourceContractRefs: [
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadModelProofRefs: [
    'future-app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-proof',
  ],
} as const;

describe('app-game timer service response consumer parent-surface status read-model parent-surface read-model service read-model handoff', () => {
  it('projects service handoff rows into service read-model handoff states', () => {
    const handoff = buildServiceReadModelHandoff();

    expect(handoff.serviceReadModelProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.parentSurfaceReadModelServiceReadModelHandoffState)).toEqual([
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('inherits service proof refs and adds service read-model refs only for proof rows', () => {
    const handoff = buildServiceReadModelHandoff();

    expect(handoff.rows[0]?.inheritedServiceProofRefs).toEqual([
      'future-app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-proof',
    ]);
    expect(handoff.rows[0]?.requiredServiceReadModelProofRefs).toEqual([
      'future-app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-proof',
    ]);
    expect(handoff.rows[1]?.requiredServiceReadModelProofRefs).toEqual([]);
    expect(handoff.rows[2]?.requiredServiceReadModelProofRefs).toEqual([]);
  });

  it('rejects service read-model runtime, rendering, adapter, raw-source, and count overclaims', () => {
    const handoff = buildServiceReadModelHandoff();

    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          serviceReadModelRuntimeEmitted: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          parentSurfaceReadModelRuntimeImplemented: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          portalUiRendered: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          adapterDispatchClaimed: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          rawPrivateSourceRowsIncluded: true,
        }
      ).success
    ).toBe(false);
    expect(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.safeParse(
        {
          ...handoff,
          serviceReadModelProofRequiredCount: 0,
        }
      ).success
    ).toBe(false);
  });
});

function buildServiceReadModelHandoff() {
  return buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoff(
    ServiceReadModelHandoffOptions,
    readUpstreamServiceHandoff()
  );
}

function readUpstreamServiceHandoff() {
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamServiceHandoffPath, 'utf8'))
  );
}
