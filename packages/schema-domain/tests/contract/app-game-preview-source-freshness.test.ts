import { describe, expect, it } from 'vitest';
import { AppGamePolicyPreviewHandoffReadModelSchema } from '../../src/app-game-policy-preview-handoff';
import {
  AppGameSourceFreshnessPolicyConsumptionMatrix,
  AppGameSourceFreshnessPolicyConsumptionRequests,
} from '../../src/generated-app-game-source-freshness-policy-consumption-data';
import {
  AppGameSourceFreshnessPolicyConsumptionMatrixId,
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessPolicyTargetKind,
  AppGameSourceFreshnessReadModelState,
} from '../../src/app-game-source-freshness-policy-consumption-values';
import { AppGameSourceFreshnessPreviewGateReadModelSchema } from '../../src/generated-app-game-source-freshness-preview-gate';
import { AppGameSourceGatedPolicyPreviewReadModelSchema } from '../../src/generated-app-game-source-gated-policy-preview-read-model';
import {
  previewGateReadModelInput,
  previewGateRow,
  previewHandoffReadModelInput,
  sourceGatedReadModelInput,
} from './app-game-preview-source-freshness.fixtures';

describe('schema-domain app-game preview/source-freshness contracts', () => {
  keepsSourceFreshnessMatrixGeneratedAndParseable();
  parsesPreviewHandoffReadModelThroughRustGeneratedRules();
  parsesPreviewGateAndSourceGatedReadModelsThroughGeneratedRules();
});

function keepsSourceFreshnessMatrixGeneratedAndParseable(): void {
  it('keeps the source-freshness matrix generated and parseable', () => {
    expect(AppGameSourceFreshnessPolicyConsumptionMatrix.matrixId).toBe(
      AppGameSourceFreshnessPolicyConsumptionMatrixId
    );
    expect(AppGameSourceFreshnessPolicyConsumptionMatrix.readiness).toHaveLength(3);
    expect(AppGameSourceFreshnessPolicyConsumptionMatrix.readiness[0]?.readinessState).toBe(
      AppGameSourceFreshnessPolicyReadinessState.PolicyReady
    );
    expect(AppGameSourceFreshnessPolicyConsumptionMatrix.readiness[2]?.readinessState).toBe(
      AppGameSourceFreshnessPolicyReadinessState.ManualRequired
    );
    expect(AppGameSourceFreshnessPolicyConsumptionRequests[0]?.target.targetKind).toBe(
      AppGameSourceFreshnessPolicyTargetKind.NativeApp
    );
    expect(AppGameSourceFreshnessPolicyConsumptionRequests[2]?.sourceStatusRows[0]?.state).toBe(
      AppGameSourceFreshnessReadModelState.Stale
    );
  });
}

function parsesPreviewHandoffReadModelThroughRustGeneratedRules(): void {
  it('parses a preview handoff read model through the Rust-generated rules', () => {
    expect(AppGamePolicyPreviewHandoffReadModelSchema.parse(previewHandoffReadModelInput)).toEqual(
      previewHandoffReadModelInput
    );
  });
}

function parsesPreviewGateAndSourceGatedReadModelsThroughGeneratedRules(): void {
  it('parses the preview gate and source-gated read models through the generated rules', () => {
    const readiness = AppGameSourceFreshnessPolicyConsumptionMatrix.readiness[2];

    expect(readiness?.readinessId).toBe(previewGateRow.sourceReadinessId);
    expect(readiness?.request.policyRequestId).toBe(previewGateRow.sourcePolicyRequestId);

    const gateReadModel = AppGameSourceFreshnessPreviewGateReadModelSchema.parse(previewGateReadModelInput);

    expect(gateReadModel.rows[0]).toEqual(previewGateRow);

    expect(AppGameSourceGatedPolicyPreviewReadModelSchema.parse(sourceGatedReadModelInput)).toEqual(
      sourceGatedReadModelInput
    );
  });
}
