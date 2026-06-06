import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema } from '../src/app-game-timer-service-read-api-response-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoff,
} from '../src/app-game-timer-service-read-api-response-consumer-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState } from '../src/app-game-timer-service-read-api-response-consumer-handoff-rules';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamServiceReadApiResponseHandoffPath = new URL(
  '../../../test-results/app-game-timer-service-read-api-response-handoff-proof/handoff.json',
  import.meta.url
);

const ServiceReadApiResponseConsumerHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadApiResponseConsumerHandoffId: 'app-game-timer-service-read-api-response-consumer-handoff-proof',
  generatedAt: '2026-06-06T14:35:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-api-response-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadApiResponseConsumerProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-consumer-proof',
  ],
} as const;

it('projects service read API rows into service read API response consumer handoff states', () => {
  const handoff = buildServiceReadApiResponseConsumerHandoff();

  expect(handoff.serviceReadApiResponseConsumerProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.parentSurfaceReadModelServiceReadApiResponseConsumerHandoffState)).toEqual([
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState.ServiceReadApiResponseConsumerProofRequired,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState.BlockedBySourceFreshness,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffState.BlockedByCompilerDecision,
  ]);
});

it('inherits service read API response proof refs and adds service read API response consumer refs only for proof rows', () => {
  const handoff = buildServiceReadApiResponseConsumerHandoff();

  expect(handoff.rows[0]?.inheritedServiceReadApiResponseProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-proof',
  ]);
  expect(handoff.rows[0]?.requiredServiceReadApiResponseConsumerProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-consumer-proof',
  ]);
  expect(handoff.rows[1]?.requiredServiceReadApiResponseConsumerProofRefs).toEqual([]);
  expect(handoff.rows[2]?.requiredServiceReadApiResponseConsumerProofRefs).toEqual([]);
});

it('rejects service read API response consumer implementation, rendering, adapter, raw-source, and count overclaims', () => {
  const handoff = buildServiceReadApiResponseConsumerHandoff();

  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiResponseImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiResponseConsumerImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        serviceEventEmitted: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        portalUiRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        adapterDispatchClaimed: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        rawPrivateSourceRowsIncluded: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiResponseConsumerProofRequiredCount: 0,
      }
    ).success
  ).toBe(false);
});

function buildServiceReadApiResponseConsumerHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoff(
    ServiceReadApiResponseConsumerHandoffOptions,
    readUpstreamServiceReadApiResponseHandoff()
  );
}

function readUpstreamServiceReadApiResponseHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamServiceReadApiResponseHandoffPath, 'utf8'))
  );
}
