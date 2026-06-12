import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema } from '../../src/app-game-timer-service-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoff,
} from '../../src/app-game-timer-service-event-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState } from '../../src/app-game-timer-service-event-handoff-rules';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

const UpstreamServiceReadModelHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-read-model-handoff-proof/handoff.json',
  import.meta.url
);

const ServiceEventHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceEventHandoffId: 'app-game-timer-service-event-handoff-proof',
  generatedAt: '2026-06-06T12:10:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceEventProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-event-proof',
  ],
} as const;

it('projects service read-model rows into service event handoff states', () => {
  const handoff = buildServiceEventHandoff();

  expect(handoff.serviceEventProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.parentSurfaceReadModelServiceEventHandoffState)).toEqual([
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.ServiceEventProofRequired,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.BlockedBySourceFreshness,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.BlockedByCompilerDecision,
  ]);
});

it('inherits service read-model proof refs and adds service event refs only for proof rows', () => {
  const handoff = buildServiceEventHandoff();

  expect(handoff.rows[0]?.inheritedServiceReadModelProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-proof',
  ]);
  expect(handoff.rows[0]?.requiredServiceEventProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-event-proof',
  ]);
  expect(handoff.rows[1]?.requiredServiceEventProofRefs).toEqual([]);
  expect(handoff.rows[2]?.requiredServiceEventProofRefs).toEqual([]);
});

it('rejects service event runtime, rendering, adapter, raw-source, and count overclaims', () => {
  const handoff = buildServiceEventHandoff();

  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.safeParse(
      {
        ...handoff,
        serviceEventRuntimeEmitted: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.safeParse(
      {
        ...handoff,
        serviceEventEmitted: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.safeParse(
      {
        ...handoff,
        parentSurfaceReadModelRuntimeImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.safeParse(
      {
        ...handoff,
        portalUiRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.safeParse(
      {
        ...handoff,
        adapterDispatchClaimed: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.safeParse(
      {
        ...handoff,
        rawPrivateSourceRowsIncluded: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.safeParse(
      {
        ...handoff,
        serviceEventProofRequiredCount: 0,
      }
    ).success
  ).toBe(false);
});

function buildServiceEventHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoff(
    ServiceEventHandoffOptions,
    readUpstreamServiceReadModelHandoff()
  );
}

function readUpstreamServiceReadModelHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamServiceReadModelHandoffPath, 'utf8'))
  );
}
