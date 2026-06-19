import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema } from '../../src/app-game-timer-service-event-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoff,
} from '../../src/app-game-timer-service-read-api-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState } from '../../src/app-game-timer-service-read-api-handoff-rules';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamServiceEventHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-event-handoff-proof/handoff.json',
  import.meta.url
);

const ServiceReadApiHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadApiHandoffId: 'app-game-timer-service-read-api-handoff-proof',
  generatedAt: '2026-06-06T12:20:00Z',
  sourceContractRefs: [
    'app-game-timer-service-event-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadApiProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-proof',
  ],
} as const;

it('projects service event rows into service read API handoff states', () => {
  const handoff = buildServiceReadApiHandoff();

  expect(handoff.serviceReadApiProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.parentSurfaceReadModelServiceReadApiHandoffState)).toEqual([
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.ServiceReadApiProofRequired,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedBySourceFreshness,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedByCompilerDecision,
  ]);
});

it('inherits service event proof refs and adds service read API refs only for proof rows', () => {
  const handoff = buildServiceReadApiHandoff();

  expect(handoff.rows[0]?.inheritedServiceEventProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-event-proof',
  ]);
  expect(handoff.rows[0]?.requiredServiceReadApiProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-proof',
  ]);
  expect(handoff.rows[1]?.requiredServiceReadApiProofRefs).toEqual([]);
  expect(handoff.rows[2]?.requiredServiceReadApiProofRefs).toEqual([]);
});

it('rejects service read API implementation, rendering, adapter, raw-source, and count overclaims', () => {
  const handoff = buildServiceReadApiHandoff();

  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiResponseImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.safeParse(
      {
        ...handoff,
        serviceEventEmitted: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.safeParse(
      {
        ...handoff,
        portalUiRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.safeParse(
      {
        ...handoff,
        adapterDispatchClaimed: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.safeParse(
      {
        ...handoff,
        rawPrivateSourceRowsIncluded: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiProofRequiredCount: 0,
      }
    ).success
  ).toBe(false);
});

function buildServiceReadApiHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoff(
    ServiceReadApiHandoffOptions,
    readUpstreamServiceEventHandoff()
  );
}

function readUpstreamServiceEventHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamServiceEventHandoffPath, 'utf8'))
  );
}
