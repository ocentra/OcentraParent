import { describe, expect, it } from 'vitest';
import {
  AppGameSourceFreshnessPreviewGateReadModelSchema,
  AppGameSourceFreshnessPreviewGateRowSchema,
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
  buildAppGameSourceFreshnessPreviewGateReadModel,
  buildAppGameSourceFreshnessPreviewGateRow,
} from '../../src/app-game-source-freshness-preview-gate';
import { AppGameSourceFreshnessPolicyConsumptionMatrix } from '../../src/app-game-source-freshness-policy-consumption-data';
import { AppGameSourceFreshnessPolicyReadinessState } from '../../src/app-game-source-freshness-policy-consumption-values';
import { AppGamePolicyPreviewStatus, AppGamePolicyPreviewTargetDomain } from '../../src/app-game-policy-preview-handoff';
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

describe('app/game source freshness preview gate', () => {
  it('creates preview-ready native app rows only after source freshness allows policy compile', () => {
    assertCreatesPreviewReadyNativeAppRows();
  });

  it('blocks stale or missing source freshness before policy preview rows are built', () => {
    assertBlocksManualSourceFreshness();
  });

  it('keeps source-fresh native game block-launch previews manual-required when compiler proof is manual', () => {
    assertCompilerManualGameStaysManualRequired();
  });

  it('builds read-model counts across source-blocked and compiler-blocked preview states', () => {
    assertBuildsReadModelCounts();
  });

  it('rejects mismatched source and compiled preview target domains plus runtime overclaims', () => {
    assertRejectsMismatchesAndOverclaims();
  });
});

function assertCreatesPreviewReadyNativeAppRows() {
  const row = buildReadyAppRow();

  expect(row.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeApp);
  expect(row.sourceReadinessState).toBe(AppGameSourceFreshnessPolicyReadinessState.PolicyReady);
  expect(row.sourcePolicyCompileAllowed).toBe(true);
  expect(row.previewStatus).toBe(AppGameSourceFreshnessPreviewGateStatus.PreviewReady);
  expect(row.gateState).toBe(AppGameSourceFreshnessPreviewGateState.SourceFresh);
  expect(row.previewRow?.previewStatus).toBe(AppGamePolicyPreviewStatus.PreviewReady);
  expect(row.sourceEvidenceRefs).toEqual([
    'evidence-app-inventory-parental-controls-helper',
    'evidence-app-runtime-parental-controls-helper',
    'evidence-app-foreground-parental-controls-helper',
  ]);
  expect(row.adapterDispatchClaimed).toBe(false);
  expect(row.policyEvaluatorRuntimeClaimed).toBe(false);
}

function assertBlocksManualSourceFreshness() {
  const row = buildManualGameRow();

  expect(row.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeGame);
  expect(row.sourceReadinessState).toBe(AppGameSourceFreshnessPolicyReadinessState.ManualRequired);
  expect(row.sourcePolicyCompileAllowed).toBe(false);
  expect(row.previewStatus).toBe(AppGameSourceFreshnessPreviewGateStatus.ManualRequired);
  expect(row.gateState).toBe(AppGameSourceFreshnessPreviewGateState.SourceManualRequired);
  expect(row.previewRow).toBeNull();
  expect(row.compiledDecisionProvided).toBe(false);
  expect(row.sourceRequirementStates).toEqual(['stale', 'missing', 'not-claimed']);
  expect(row.adapterDispatchClaimed).toBe(false);
}

function assertCompilerManualGameStaysManualRequired() {
  const row = buildCompilerManualGameRow();

  expect(row.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeGame);
  expect(row.sourcePolicyCompileAllowed).toBe(true);
  expect(row.previewStatus).toBe(AppGameSourceFreshnessPreviewGateStatus.ManualRequired);
  expect(row.gateState).toBe(AppGameSourceFreshnessPreviewGateState.CompilerManualRequired);
  expect(row.previewRow?.previewStatus).toBe(AppGamePolicyPreviewStatus.ManualRequired);
  expect(row.adapterDispatchClaimed).toBe(false);
  expect(row.platformEnforcementClaimed).toBe(false);
}

function assertBuildsReadModelCounts() {
  const readModel = buildFullReadModel();

  expect(readModel.nativeAppRowCount).toBe(1);
  expect(readModel.nativeGameRowCount).toBe(2);
  expect(readModel.previewReadyCount).toBe(1);
  expect(readModel.manualRequiredCount).toBe(2);
  expect(readModel.sourceManualRequiredCount).toBe(1);
  expect(readModel.compilerManualRequiredCount).toBe(1);
  expect(readModel.adapterDispatchClaimed).toBe(false);
}

function assertRejectsMismatchesAndOverclaims() {
  expect(() =>
    buildAppGameSourceFreshnessPreviewGateRow(GateOptions, {
      rowId: 'source-gate-row-domain-mismatch',
      sourceReadiness: readyGameSource,
      compiledDecision: appCompiledDecision,
    })
  ).toThrow('Expected source freshness target domain to match compiled policy preview target domain');

  expect(
    AppGameSourceFreshnessPreviewGateRowSchema.safeParse({ ...buildReadyAppRow(), adapterDispatchClaimed: true })
      .success
  ).toBe(false);
  expect(
    AppGameSourceFreshnessPreviewGateReadModelSchema.safeParse({
      ...buildReadyAppReadModel(),
      previewReadyCount: 0,
    }).success
  ).toBe(false);
}

function buildReadyAppRow() {
  return buildAppGameSourceFreshnessPreviewGateRow(GateOptions, {
    rowId: 'source-gate-row-ready-app',
    sourceReadiness: readyAppSource,
    compiledDecision: appCompiledDecision,
  });
}

function buildManualGameRow() {
  return buildAppGameSourceFreshnessPreviewGateRow(GateOptions, {
    rowId: 'source-gate-row-manual-game',
    sourceReadiness: manualGameSource,
    compiledDecision: null,
  });
}

function buildCompilerManualGameRow() {
  return buildAppGameSourceFreshnessPreviewGateRow(GateOptions, {
    rowId: 'source-gate-row-compiler-manual-game',
    sourceReadiness: readyGameSource,
    compiledDecision: gameManualCompiledDecision,
  });
}

function buildReadyAppReadModel() {
  return buildAppGameSourceFreshnessPreviewGateReadModel(GateOptions, [
    {
      rowId: 'source-gate-row-ready-app',
      sourceReadiness: readyAppSource,
      compiledDecision: appCompiledDecision,
    },
  ]);
}

function buildFullReadModel() {
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
