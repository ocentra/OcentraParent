import { describe, expect, it } from 'vitest';
import {
  AppGameSourceGatedPolicyPreviewExportManifestState,
  AppGameSourceGatedPolicyPreviewExportReadinessSchema,
  AppGameSourceGatedPolicyPreviewExportReadinessState,
  buildAppGameSourceGatedPolicyPreviewExportReadiness,
} from '../src/app-game-source-gated-policy-preview-export-readiness';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceFreshnessPreviewGateReadModel } from '../src/app-game-source-freshness-preview-gate';
import { AppGameSourceFreshnessPolicyConsumptionMatrix } from '../src/app-game-source-freshness-policy-consumption-data';
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
  sourceContractRefs: ['app-game-source-freshness-policy-consumption', 'app-game-policy-preview-handoff'],
  policyPreviewOptions: PreviewOptions,
} as const;

const ReadModelOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readModelId: 'source-gated-policy-preview-read-model-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-freshness-preview-gate'],
} as const;

const ExportOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readinessId: 'source-gated-policy-preview-export-readiness-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-read-model',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
} as const;

describe('app/game source-gated policy preview export readiness', () => {
  it('names the future package export surface without editing the manifest', () => {
    const readiness = buildReadiness();

    expect(readiness.requiredExportSubpath).toBe('./app-game-source-gated-policy-preview-read-model');
    expect(readiness.requiredExportSymbols).toEqual([
      'AppGameSourceGatedPolicyPreviewReadModelSchema',
      'AppGameSourceGatedPolicyPreviewReadModelRowSchema',
      'buildAppGameSourceGatedPolicyPreviewReadModel',
      'decodeAppGameSourceGatedPolicyPreviewReadModel',
      'AppGameSourceGatedPolicyPreviewReadModelProjectionState',
      'AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary',
    ]);
    expect(readiness.readinessState).toBe(
      AppGameSourceGatedPolicyPreviewExportReadinessState.ReadyForManifestSequencing
    );
    expect(readiness.manifestState).toBe(
      AppGameSourceGatedPolicyPreviewExportManifestState.DeferredByPackageManifestLock
    );
    expect(readiness.packageManifestUpdated).toBe(false);
  });

  it('preserves source-gated app and game row coverage from WP76', () => {
    const readiness = buildReadiness();

    expect(readiness.nativeAppRowCount).toBe(1);
    expect(readiness.nativeGameRowCount).toBe(2);
    expect(readiness.previewReadyVisibleCount).toBe(1);
    expect(readiness.sourceManualRequiredVisibleCount).toBe(1);
    expect(readiness.compilerManualRequiredVisibleCount).toBe(1);
    expect(readiness.projectionStates).toEqual([
      'preview-ready-visible',
      'source-manual-required-visible',
      'compiler-manual-required-visible',
    ]);
  });

  it('rejects manifest edits, row-count drift, and runtime overclaims', () => {
    const readiness = buildReadiness();

    expect(
      AppGameSourceGatedPolicyPreviewExportReadinessSchema.safeParse({
        ...readiness,
        packageManifestUpdated: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewExportReadinessSchema.safeParse({
        ...readiness,
        nativeGameRowCount: 0,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewExportReadinessSchema.safeParse({
        ...readiness,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
  });
});

function buildReadiness() {
  return buildAppGameSourceGatedPolicyPreviewExportReadiness(ExportOptions, buildReadModel());
}

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
