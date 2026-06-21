import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema } from '../../src/app-game-timer-service-read-api-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoff,
} from '../../src/app-game-timer-service-read-api-response-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState } from '../../src/app-game-timer-service-read-api-response-handoff-rules';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamServiceReadApiHandoffPath = new URL(
  '../../../../test-results/app-game-timer-service-read-api-handoff-proof/handoff.json',
  import.meta.url
);

const ServiceReadApiResponseHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadApiResponseHandoffId: 'app-game-timer-service-read-api-response-handoff-proof',
  generatedAt: '2026-06-06T14:25:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-api-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadApiResponseProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-proof',
  ],
} as const;

it('projects service read API rows into service read API response handoff states', () => {
  const handoff = buildServiceReadApiResponseHandoff();

  expect(handoff.serviceReadApiResponseProofRequiredCount).toBe(1);
  expect(handoff.blockedBySourceFreshnessCount).toBe(1);
  expect(handoff.blockedByCompilerDecisionCount).toBe(1);
  expect(handoff.rows.map((row) => row.parentSurfaceReadModelServiceReadApiResponseHandoffState)).toEqual([
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.ServiceReadApiResponseProofRequired,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.BlockedBySourceFreshness,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffState.BlockedByCompilerDecision,
  ]);
});

it('inherits service read API proof refs and adds service read API response refs only for proof rows', () => {
  const handoff = buildServiceReadApiResponseHandoff();

  expect(handoff.rows[0]?.inheritedServiceReadApiProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-proof',
  ]);
  expect(handoff.rows[0]?.requiredServiceReadApiResponseProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-proof',
  ]);
  expect(handoff.rows[1]?.requiredServiceReadApiResponseProofRefs).toEqual([]);
  expect(handoff.rows[2]?.requiredServiceReadApiResponseProofRefs).toEqual([]);
});

it('rejects service read API response implementation, rendering, adapter, raw-source, and count overclaims', () => {
  const handoff = buildServiceReadApiResponseHandoff();

  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiResponseImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.safeParse(
      {
        ...handoff,
        serviceEventEmitted: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.safeParse(
      {
        ...handoff,
        portalUiRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.safeParse(
      {
        ...handoff,
        adapterDispatchClaimed: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.safeParse(
      {
        ...handoff,
        rawPrivateSourceRowsIncluded: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoffSchema.safeParse(
      {
        ...handoff,
        serviceReadApiResponseProofRequiredCount: 0,
      }
    ).success
  ).toBe(false);
});

function buildServiceReadApiResponseHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoff(
    ServiceReadApiResponseHandoffOptions,
    readUpstreamServiceReadApiHandoff()
  );
}

function readUpstreamServiceReadApiHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamServiceReadApiHandoffPath, 'utf8'))
  );
}
