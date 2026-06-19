import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamParentSurfaceReadModelHandoffPath = new URL(
  '../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof/handoff.json',
  import.meta.url
);

const ParentSurfaceReadModelOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  generatedAt: '2026-06-06T11:30:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
} as const;

it('builds parent-safe parent-surface read-model rows from WP100 handoff rows', () => {
  const readModel = buildParentSurfaceReadModel();

  expect(readModel.readyForParentSurfaceReadModelCount).toBe(1);
  expect(readModel.blockedBySourceFreshnessCount).toBe(1);
  expect(readModel.blockedByCompilerDecisionCount).toBe(1);
  expect(readModel.rows.map((row) => row.parentSurfaceReadModelState)).toEqual([
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.ReadyForParentSurfaceReadModel,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedBySourceFreshness,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelState.BlockedByCompilerDecision,
  ]);
});

it('preserves proof refs and source refs without raw private source rows', () => {
  const readModel = buildParentSurfaceReadModel();

  expect(readModel.rows[0]?.requiredParentSurfaceReadModelProofRefs).toEqual([
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  ]);
  expect(readModel.rows[0]?.parentSafeSummary).toBe(
    'native-app:ready-for-parent-surface-status-read-model-parent-surface-read-model-contract'
  );
  expect(readModel.rows[1]?.requiredParentSurfaceReadModelProofRefs).toEqual([]);
  expect(readModel.rawPrivateSourceRowsIncluded).toBe(false);
});

it('rejects runtime, rendering, persistence, adapter, and count overclaims', () => {
  const readModel = buildParentSurfaceReadModel();

  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse(
      {
        ...readModel,
        parentSurfaceReadModelRuntimeImplemented: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse(
      {
        ...readModel,
        parentSurfaceReadModelPersisted: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse(
      {
        ...readModel,
        portalUiRendered: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse(
      {
        ...readModel,
        adapterDispatchClaimed: true,
      }
    ).success
  ).toBe(false);
  expect(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.safeParse(
      {
        ...readModel,
        readyForParentSurfaceReadModelCount: 0,
      }
    ).success
  ).toBe(false);
});

function buildParentSurfaceReadModel() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel(
    ParentSurfaceReadModelOptions,
    readUpstreamHandoff()
  );
}

function readUpstreamHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamParentSurfaceReadModelHandoffPath, 'utf8'))
  );
}
