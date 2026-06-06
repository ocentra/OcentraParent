import { describe, expect, it } from 'vitest';
import {
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSchema,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
  buildAppGameSourceGatedPolicyPreviewReadModel,
} from '../src/app-game-source-gated-policy-preview-read-model';
import {
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
  buildAppGameSourceFreshnessPreviewGateReadModel,
} from '../src/app-game-source-freshness-preview-gate';
import { AppGameSourceFreshnessPolicyConsumptionMatrix } from '../src/app-game-source-freshness-policy-consumption-data';
import { AppGamePolicyPreviewStatus, AppGamePolicyPreviewTargetDomain } from '../src/app-game-policy-preview-handoff';
import {
  PreviewOptions,
  appCompiledDecision,
  gameManualCompiledDecision,
} from './app-game-policy-preview-handoff-fixtures';

const [readyAppSource, readyGameSource, manualGameSource] = AppGameSourceFreshnessPolicyConsumptionMatrix.readiness;

const GateOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  gateId: 'source-freshness-preview-gate-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-freshness-policy-consumption',
    'app-game-policy-preview-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  policyPreviewOptions: PreviewOptions,
} as const;

const ReadModelOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readModelId: 'source-gated-policy-preview-read-model-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-freshness-preview-gate',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
} as const;

describe('app/game source-gated policy preview read model', () => {
  it('projects preview-ready app rows from the source freshness gate without runtime claims', () => {
    const readModel = buildReadModel();
    const row = readModel.rows[0];

    expect(row?.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeApp);
    expect(row?.gateState).toBe(AppGameSourceFreshnessPreviewGateState.SourceFresh);
    expect(row?.projectionState).toBe(AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible);
    expect(row?.previewStatus).toBe(AppGameSourceFreshnessPreviewGateStatus.PreviewReady);
    expect(row?.previewCompilerStatus).toBe(AppGamePolicyPreviewStatus.PreviewReady);
    expect(row?.previewDecisionRef).toBe('policy-decision-preview-app');
    expect(row?.sensitiveDetailBoundary).toBe(
      AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary.RedactedEvidenceRefsOnly
    );
    expect(row?.adapterDispatchClaimed).toBe(false);
    expect(row?.rawPrivateSourceRowsIncluded).toBe(false);
  });

  it('keeps stale or missing source freshness visible as source-manual-required with no preview decision ref', () => {
    const readModel = buildReadModel();
    const row = readModel.rows[1];

    expect(row?.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeGame);
    expect(row?.gateState).toBe(AppGameSourceFreshnessPreviewGateState.SourceManualRequired);
    expect(row?.projectionState).toBe(
      AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible
    );
    expect(row?.sourcePolicyCompileAllowed).toBe(false);
    expect(row?.previewDecisionRef).toBeNull();
    expect(row?.previewCompilerStatus).toBeNull();
    expect(row?.serviceRuntimeEventClaimed).toBe(false);
  });

  it('separates source-fresh native games that still need compiler/manual proof', () => {
    const readModel = buildReadModel();
    const row = readModel.rows[2];

    expect(row?.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeGame);
    expect(row?.gateState).toBe(AppGameSourceFreshnessPreviewGateState.CompilerManualRequired);
    expect(row?.projectionState).toBe(
      AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible
    );
    expect(row?.sourcePolicyCompileAllowed).toBe(true);
    expect(row?.previewDecisionRef).toBe('policy-decision-preview-game');
    expect(row?.previewCompilerStatus).toBe(AppGamePolicyPreviewStatus.ManualRequired);
    expect(row?.platformEnforcementClaimed).toBe(false);
  });

  it('rejects count drift and runtime overclaims', () => {
    const readModel = buildReadModel();

    expect(readModel.nativeAppRowCount).toBe(1);
    expect(readModel.nativeGameRowCount).toBe(2);
    expect(readModel.previewReadyVisibleCount).toBe(1);
    expect(readModel.sourceManualRequiredVisibleCount).toBe(1);
    expect(readModel.compilerManualRequiredVisibleCount).toBe(1);

    expect(
      AppGameSourceGatedPolicyPreviewReadModelSchema.safeParse({
        ...readModel,
        previewReadyVisibleCount: 0,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
  });
});

function buildReadModel() {
  return buildAppGameSourceGatedPolicyPreviewReadModel(ReadModelOptions, buildGateReadModel());
}

function buildGateReadModel() {
  return buildAppGameSourceFreshnessPreviewGateReadModel(GateOptions, [
    {
      rowId: 'source-gate-row-ready-app',
      sourceReadiness: readyAppSource,
      compiledDecision: appCompiledDecision,
    },
    {
      rowId: 'source-gate-row-manual-game',
      sourceReadiness: manualGameSource,
      compiledDecision: null,
    },
    {
      rowId: 'source-gate-row-compiler-manual-game',
      sourceReadiness: readyGameSource,
      compiledDecision: gameManualCompiledDecision,
    },
  ]);
}
