import { describe, expect, it } from 'vitest';
import {
  AppGamePolicyPreviewHandoffReadModelSchema,
  AppGamePolicyPreviewStatus,
  AppGamePolicyPreviewTargetDomain,
} from '../../src/app-game-policy-preview-handoff';
import {
  AppGameSourceFreshnessPolicyConsumptionMatrix,
  AppGameSourceFreshnessPolicyConsumptionRequests,
} from '../../src/app-game-source-freshness-policy-consumption-data';
import {
  AppGameSourceFreshnessCapabilityStatus,
  AppGameSourceFreshnessPolicyConsumptionMatrixId,
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessPolicyTargetKind,
  AppGameSourceFreshnessReadModelState,
  AppGameSourceFreshnessReasonCode,
  AppGameSourceFreshnessRequirementKind,
  AppGameSourceFreshnessRequirementState,
  AppGameSourceFreshnessSourceKind,
} from '../../src/app-game-source-freshness-policy-consumption-values';
import {
  AppGameSourceFreshnessPreviewGateReadModelSchema,
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
} from '../../src/app-game-source-freshness-preview-gate';
import {
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSchema,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
} from '../../src/app-game-source-gated-policy-preview-read-model';
import { RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims } from '../../src/app-game-source-gated-policy-preview-read-model-rules';
import { ParentContractSchemaVersion } from '../../src/family-reference-primitives';
import { PolicyDecisionHandoffState } from '../../src/policy-contracts';

describe('schema-domain app-game preview/source-freshness contracts', () => {
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

  it('parses a preview handoff read model through the Rust-generated rules', () => {
    const row = {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      rowId: 'preview-row-1',
      targetDomain: AppGamePolicyPreviewTargetDomain.NativeGame,
      sourceCompiledDecisionId: 'compiled-decision-1',
      sourceCompileRequestId: 'compile-request-1',
      sourceTargetKind: 'specific-game',
      device: {
        deviceId: 'device-1',
        childProfileId: 'child-1',
        label: 'Child Tablet',
        platform: 'android',
      },
      policyVersion: 'policy-v0-6',
      policyTarget: {
        targetType: 'app',
        targetId: 'game-1',
        targetValue: 'game-1',
      },
      policyDecisionId: 'policy-decision-1',
      policyAction: 'allow',
      outcomeState: 'dry-run-ready',
      previewStatus: AppGamePolicyPreviewStatus.PreviewReady,
      rejectionReason: 'none',
      ruleRefs: ['rule-1'],
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-1',
          kind: 'policy-decision',
          observedAt: '2026-06-04T12:55:00.000Z',
        },
      ],
      capabilityRefs: ['capability-1'],
      authorityRefs: ['authority-1'],
      auditRefs: ['audit-1'],
      dryRun: true,
      enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
      policyEvaluatorRuntimeClaimState: 'not-claimed',
      timerRuntimeClaimState: 'not-claimed',
      adapterDispatchState: 'not-dispatched',
      childDeliveryClaimState: 'not-claimed',
      platformEnforcementClaimState: 'not-claimed',
      policyEvaluatorRuntimeClaimed: false,
      timerRuntimeClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      generatedAt: '2026-06-04T12:55:00.000Z',
    } as const;

    expect(
      AppGamePolicyPreviewHandoffReadModelSchema.parse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        handoffId: 'handoff-1',
        generatedAt: '2026-06-04T12:55:00.000Z',
        sourceContractRefs: ['contract-preview'],
        rows: [row],
        nativeAppRowCount: 0,
        nativeGameRowCount: 1,
        previewReadyCount: 1,
        manualRequiredCount: 0,
        rejectedCount: 0,
        policyEvaluatorRuntimeClaimed: false,
        timerRuntimeClaimed: false,
        adapterDispatchClaimed: false,
        childDeliveryClaimed: false,
        platformEnforcementClaimed: false,
      })
    ).toMatchObject({ rows: [row] });
  });

  it('parses the preview gate and source-gated read models through the generated rules', () => {
    const readiness = AppGameSourceFreshnessPolicyConsumptionMatrix.readiness[2];

    expect(readiness?.readinessId).toBe('source-freshness-policy-readiness-3');

    const gateRow = {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      rowId: 'gate-row-1',
      targetDomain: AppGamePolicyPreviewTargetDomain.NativeGame,
      sourceReadinessId: readiness!.readinessId,
      sourcePolicyRequestId: readiness!.request.policyRequestId,
      sourceReadinessState: AppGameSourceFreshnessPolicyReadinessState.ManualRequired,
      sourcePolicyCompileAllowed: false,
      sourceRequirementStates: [
        AppGameSourceFreshnessRequirementState.Stale,
        AppGameSourceFreshnessRequirementState.NotClaimed,
      ],
      sourceReasonCodes: [
        AppGameSourceFreshnessReasonCode.StaleSourceStatusRow,
        AppGameSourceFreshnessReasonCode.NotClaimedSourceStatus,
      ],
      sourceEvidenceRefs: ['evidence-game-runtime-stale', 'evidence-game-launcher-not-claimed'],
      compiledDecisionProvided: false,
      previewStatus: AppGameSourceFreshnessPreviewGateStatus.ManualRequired,
      gateState: AppGameSourceFreshnessPreviewGateState.SourceManualRequired,
      previewRow: null,
      policyEvaluatorRuntimeClaimed: false,
      timerRuntimeClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      generatedAt: '2026-06-04T12:55:00.000Z',
    } as const;

    const gateReadModel = AppGameSourceFreshnessPreviewGateReadModelSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      gateId: 'gate-1',
      generatedAt: '2026-06-04T12:55:00.000Z',
      sourceContractRefs: ['contract-source-freshness', 'contract-preview'],
      rows: [gateRow],
      nativeAppRowCount: 0,
      nativeGameRowCount: 1,
      previewReadyCount: 0,
      manualRequiredCount: 1,
      sourceManualRequiredCount: 1,
      compilerManualRequiredCount: 0,
      policyEvaluatorRuntimeClaimed: false,
      timerRuntimeClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
    });

    expect(gateReadModel.rows[0]).toMatchObject(gateRow);

    expect(
      AppGameSourceGatedPolicyPreviewReadModelSchema.parse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        readModelId: 'source-gated-read-model-1',
        sourceGateId: 'gate-1',
        generatedAt: '2026-06-04T12:55:00.000Z',
        sourceContractRefs: ['contract-preview'],
        sourceGateContractRefs: ['contract-gate'],
        rows: [
          {
            schemaVersion: ParentContractSchemaVersion.V0_6,
            rowId: 'source-gated-row-1',
            sourceGateRowId: 'gate-row-1',
            sourceGateId: 'gate-1',
            targetDomain: AppGamePolicyPreviewTargetDomain.NativeGame,
            sourceReadinessId: readiness!.readinessId,
            sourcePolicyRequestId: readiness!.request.policyRequestId,
            sourceReadinessState: AppGameSourceFreshnessPolicyReadinessState.ManualRequired,
            sourceRequirementStates: [
              AppGameSourceFreshnessRequirementState.Stale,
              AppGameSourceFreshnessRequirementState.NotClaimed,
            ],
            sourcePolicyCompileAllowed: false,
            sourceEvidenceRefs: ['evidence-game-runtime-stale'],
            gateState: AppGameSourceFreshnessPreviewGateState.SourceManualRequired,
            projectionState:
              AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible,
            previewStatus: AppGameSourceFreshnessPreviewGateStatus.ManualRequired,
            previewDecisionRef: null,
            previewCompilerStatus: null,
            sensitiveDetailBoundary:
              AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary.RedactedEvidenceRefsOnly,
            serviceRuntimeEventClaimed: false,
            portalUiRendered: false,
            policyEvaluatorRuntimeClaimed: false,
            timerRuntimeClaimed: false,
            adapterDispatchClaimed: false,
            childDeliveryClaimed: false,
            platformEnforcementClaimed: false,
            rawPrivateSourceRowsIncluded: false,
            generatedAt: '2026-06-04T12:55:00.000Z',
          },
        ],
        nativeAppRowCount: 0,
        nativeGameRowCount: 1,
        previewReadyVisibleCount: 0,
        sourceManualRequiredVisibleCount: 1,
        compilerManualRequiredVisibleCount: 0,
        readModelNonClaims: [...RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims],
        serviceRuntimeEventClaimed: false,
        portalUiRendered: false,
        policyEvaluatorRuntimeClaimed: false,
        timerRuntimeClaimed: false,
        adapterDispatchClaimed: false,
        childDeliveryClaimed: false,
        platformEnforcementClaimed: false,
        rawPrivateSourceRowsIncluded: false,
      })
    ).toMatchObject({
      readModelNonClaims: [...RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims],
    });
  });
});
