import { describe, expect, it } from 'vitest';
import {
  AppGamePolicyPreviewTargetDomain,
  AppGamePolicyPreviewStatus,
} from '../../src/app-game-policy-preview-handoff';
import {
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessRequirementState,
} from '../../src/app-game-source-freshness-policy-consumption-values';
import {
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
} from '../../src/app-game-source-freshness-preview-gate';
import {
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
} from '../../src/app-game-source-gated-policy-preview-read-model';
import {
  buildAppGameSourceGatedPolicyPreviewTimerHandoff,
  AppGameSourceGatedPolicyPreviewTimerHandoffState,
} from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import {
  buildAppGameSourceGatedPolicyPreviewTimerStatus,
  AppGameSourceGatedPolicyPreviewTimerStatusState,
} from '../../src/app-game-source-gated-policy-preview-timer-status';
import {
  buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState,
} from '../../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import {
  buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState,
} from '../../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import {
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model';
import { ParentContractSchemaVersion } from '../../src/family-reference-primitives';

const generatedAt = '2026-06-29T06:00:00.000Z';

describe('schema-domain app-game timer/service-readiness contracts', () => {
  it('builds timer handoff through scheduler persistence from the Rust-generated rules', () => {
    const readModel = {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      readModelId: 'source-gated-read-model-1',
      sourceGateId: 'gate-1',
      generatedAt,
      sourceContractRefs: ['contract-preview'],
      sourceGateContractRefs: ['contract-gate'],
      rows: [
        {
          schemaVersion: ParentContractSchemaVersion.V0_6,
          rowId: 'source-gated-row-1',
          sourceGateRowId: 'gate-row-1',
          sourceGateId: 'gate-1',
          targetDomain: AppGamePolicyPreviewTargetDomain.NativeApp,
          sourceReadinessId: 'readiness-app-1',
          sourcePolicyRequestId: 'request-app-1',
          sourceReadinessState: AppGameSourceFreshnessPolicyReadinessState.PolicyReady,
          sourceRequirementStates: [AppGameSourceFreshnessRequirementState.Satisfied],
          sourcePolicyCompileAllowed: true,
          sourceEvidenceRefs: ['evidence-app-ready'],
          gateState: AppGameSourceFreshnessPreviewGateState.SourceFresh,
          projectionState: AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible,
          previewStatus: AppGameSourceFreshnessPreviewGateStatus.PreviewReady,
          previewDecisionRef: 'preview-decision-app-1',
          previewCompilerStatus: AppGamePolicyPreviewStatus.PreviewReady,
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
          generatedAt,
        },
        {
          schemaVersion: ParentContractSchemaVersion.V0_6,
          rowId: 'source-gated-row-2',
          sourceGateRowId: 'gate-row-2',
          sourceGateId: 'gate-1',
          targetDomain: AppGamePolicyPreviewTargetDomain.NativeGame,
          sourceReadinessId: 'readiness-game-1',
          sourcePolicyRequestId: 'request-game-1',
          sourceReadinessState: AppGameSourceFreshnessPolicyReadinessState.ManualRequired,
          sourceRequirementStates: [AppGameSourceFreshnessRequirementState.Stale],
          sourcePolicyCompileAllowed: false,
          sourceEvidenceRefs: ['evidence-game-stale'],
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
          generatedAt,
        },
      ],
      nativeAppRowCount: 1,
      nativeGameRowCount: 1,
      previewReadyVisibleCount: 1,
      sourceManualRequiredVisibleCount: 1,
      compilerManualRequiredVisibleCount: 0,
      readModelNonClaims: [
        'no-service-runtime-event',
        'no-portal-ui-rendered',
        'no-policy-evaluator-runtime',
        'no-timer-runtime',
        'no-adapter-dispatch',
        'no-child-delivery',
        'no-platform-enforcement',
        'no-raw-private-source-rows',
      ],
      serviceRuntimeEventClaimed: false,
      portalUiRendered: false,
      policyEvaluatorRuntimeClaimed: false,
      timerRuntimeClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    } as const;

    const timerHandoff = buildAppGameSourceGatedPolicyPreviewTimerHandoff(
      {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        handoffId: 'timer-handoff-1',
        generatedAt,
        sourceContractRefs: ['contract-preview'],
      },
      readModel
    );

    expect(timerHandoff.timerSequenceCandidateCount).toBe(1);
    expect(timerHandoff.sourceManualBlockedCount).toBe(1);
    expect(timerHandoff.rows[0]?.timerHandoffState).toBe(
      AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing
    );
    expect(timerHandoff.rows[1]?.timerHandoffState).toBe(
      AppGameSourceGatedPolicyPreviewTimerHandoffState.SourceManualRequiredBeforeTimer
    );

    const timerStatus = buildAppGameSourceGatedPolicyPreviewTimerStatus(
      {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        statusId: 'timer-status-1',
        generatedAt,
        sourceContractRefs: ['contract-timer-status'],
        timerRuntimeProofRef: 'proof-timer-runtime',
        sourceFreshnessProofRef: 'proof-source-freshness',
        compilerDecisionProofRef: 'proof-compiler-decision',
      },
      timerHandoff
    );

    expect(timerStatus.timerRuntimeProofRequiredCount).toBe(1);
    expect(timerStatus.sourceFreshnessProofRequiredCount).toBe(1);
    expect(timerStatus.rows[0]?.timerStatusState).toBe(
      AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired
    );
    expect(timerStatus.rows[0]?.requiredProofRefs).toEqual(['proof-timer-runtime']);
    expect(timerStatus.rows[1]?.requiredProofRefs).toEqual(['proof-source-freshness']);

    const runtimeReadiness = buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness(
      {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        readinessId: 'timer-runtime-readiness-1',
        generatedAt,
        sourceContractRefs: ['contract-runtime-readiness'],
        timerRuntimeProofRef: 'proof-timer-runtime',
        schedulerPersistenceProofRef: 'proof-scheduler-persistence',
        auditProofRef: 'proof-audit',
        rollbackProofRef: 'proof-rollback',
      },
      timerStatus
    );

    expect(runtimeReadiness.runtimeProofRequiredCount).toBe(1);
    expect(runtimeReadiness.rows[0]?.runtimeReadinessState).toBe(
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired
    );
    expect(runtimeReadiness.rows[0]?.requiredProofRefs).toEqual([
      'proof-timer-runtime',
      'proof-scheduler-persistence',
      'proof-audit',
      'proof-rollback',
    ]);
    expect(runtimeReadiness.rows[1]?.requiredProofRefs).toEqual(['proof-source-freshness']);

    const schedulerPersistence = buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
      {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        persistenceId: 'timer-scheduler-persistence-1',
        generatedAt,
        sourceContractRefs: ['contract-scheduler-persistence'],
        serviceTimerRuntimeProofRef: 'proof-service-timer-runtime',
        schedulerPersistenceProofRef: 'proof-scheduler-persistence',
        schedulerStateStoreProofRef: 'proof-scheduler-store',
        auditProofRef: 'proof-audit',
        rollbackProofRef: 'proof-rollback',
      },
      runtimeReadiness
    );

    expect(schedulerPersistence.schedulerPersistenceProofRequiredCount).toBe(1);
    expect(schedulerPersistence.rows[0]?.schedulerPersistenceState).toBe(
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired
    );
    expect(schedulerPersistence.rows[0]?.requiredProofRefs).toEqual([
      'proof-service-timer-runtime',
      'proof-scheduler-persistence',
      'proof-scheduler-store',
      'proof-audit',
      'proof-rollback',
    ]);
    expect(schedulerPersistence.rows[1]?.requiredProofRefs).toEqual(['proof-source-freshness']);
  });

  it('builds service-readiness and protocol read models through the Rust-generated rules', () => {
    const serviceReadinessHandoff = {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      handoffId: 'service-readiness-handoff-1',
      sourceParentSurfaceIntentId: 'parent-surface-intent-1',
      generatedAt,
      sourceContractRefs: ['contract-service-readiness-handoff'],
      rows: [
        {
          schemaVersion: ParentContractSchemaVersion.V0_6,
          rowId: 'service-readiness-row-1',
          sourceParentSurfaceIntentRowId: 'parent-surface-row-1',
          sourceAuditRollbackReadModelRowId: 'audit-read-model-row-1',
          sourceAuditRollbackHandoffRowId: 'audit-handoff-row-1',
          sourceSchedulerPersistenceRowId: 'scheduler-persistence-row-1',
          targetDomain: AppGamePolicyPreviewTargetDomain.NativeApp,
          serviceReadinessHandoffState: 'service-read-api-proof-required',
          parentSurfaceProofRequired: true,
          serviceReadinessProofRequired: true,
          serviceReadApiProofRequired: true,
          requiredProofRefs: [
            'proof-service-timer-runtime',
            'proof-scheduler-persistence',
            'proof-scheduler-store',
            'proof-audit',
            'proof-rollback',
            'proof-service-readiness',
            'proof-service-read-api',
          ],
          sourceEvidenceRefs: ['evidence-app-ready'],
          serviceReadApiRef: 'service-read-api-app',
          serviceRuntimeEventClaimed: false,
          serviceReadApiImplemented: false,
          portalUiRendered: false,
          policyEvaluatorRuntimeClaimed: false,
          timerRuntimeClaimed: false,
          timerScheduled: false,
          schedulerPersistenceRuntimeClaimed: false,
          durableSchedulerStorageClaimed: false,
          auditRuntimeClaimed: false,
          durableAuditLogClaimed: false,
          rollbackRuntimeClaimed: false,
          rollbackExecutionClaimed: false,
          adapterDispatchClaimed: false,
          childDeliveryClaimed: false,
          platformEnforcementClaimed: false,
          rawPrivateSourceRowsIncluded: false,
          generatedAt,
        },
        {
          schemaVersion: ParentContractSchemaVersion.V0_6,
          rowId: 'service-readiness-row-2',
          sourceParentSurfaceIntentRowId: 'parent-surface-row-2',
          sourceAuditRollbackReadModelRowId: 'audit-read-model-row-2',
          sourceAuditRollbackHandoffRowId: 'audit-handoff-row-2',
          sourceSchedulerPersistenceRowId: 'scheduler-persistence-row-2',
          targetDomain: AppGamePolicyPreviewTargetDomain.NativeGame,
          serviceReadinessHandoffState: 'blocked-by-source-freshness',
          parentSurfaceProofRequired: false,
          serviceReadinessProofRequired: false,
          serviceReadApiProofRequired: false,
          requiredProofRefs: ['proof-source-freshness'],
          sourceEvidenceRefs: ['evidence-game-stale'],
          serviceReadApiRef: 'service-read-api-game',
          serviceRuntimeEventClaimed: false,
          serviceReadApiImplemented: false,
          portalUiRendered: false,
          policyEvaluatorRuntimeClaimed: false,
          timerRuntimeClaimed: false,
          timerScheduled: false,
          schedulerPersistenceRuntimeClaimed: false,
          durableSchedulerStorageClaimed: false,
          auditRuntimeClaimed: false,
          durableAuditLogClaimed: false,
          rollbackRuntimeClaimed: false,
          rollbackExecutionClaimed: false,
          adapterDispatchClaimed: false,
          childDeliveryClaimed: false,
          platformEnforcementClaimed: false,
          rawPrivateSourceRowsIncluded: false,
          generatedAt,
        },
      ],
      nativeAppRowCount: 1,
      nativeGameRowCount: 1,
      serviceReadApiProofRequiredCount: 1,
      blockedBySourceFreshnessCount: 1,
      blockedByCompilerDecisionCount: 0,
      serviceReadinessHandoffNonClaims: [
        'no-service-runtime-event',
        'no-service-read-api-implemented',
        'no-portal-ui-rendered',
        'no-policy-evaluator-runtime',
        'no-timer-runtime',
        'no-timer-scheduled',
        'no-scheduler-persistence-runtime',
        'no-durable-scheduler-storage',
        'no-audit-runtime',
        'no-durable-audit-log',
        'no-rollback-runtime',
        'no-rollback-execution',
        'no-adapter-dispatch',
        'no-child-delivery',
        'no-platform-enforcement',
        'no-raw-private-source-rows',
      ],
      serviceRuntimeEventClaimed: false,
      serviceReadApiImplemented: false,
      portalUiRendered: false,
      policyEvaluatorRuntimeClaimed: false,
      timerRuntimeClaimed: false,
      timerScheduled: false,
      schedulerPersistenceRuntimeClaimed: false,
      durableSchedulerStorageClaimed: false,
      auditRuntimeClaimed: false,
      durableAuditLogClaimed: false,
      rollbackRuntimeClaimed: false,
      rollbackExecutionClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    } as const;

    const serviceReadinessReadModel = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel(
      {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        readModelId: 'service-readiness-read-model-1',
        generatedAt,
        sourceContractRefs: ['contract-service-readiness-read-model'],
        serviceReadinessSummaryRef: 'service-readiness-summary-1',
      },
      serviceReadinessHandoff
    );

    expect(serviceReadinessReadModel.serviceReadModelProofRequiredCount).toBe(1);
    expect(serviceReadinessReadModel.rows[0]?.serviceReadinessReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.ServiceReadModelProofRequired
    );
    expect(serviceReadinessReadModel.rows[1]?.serviceReadinessReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedBySourceFreshness
    );

    const protocolHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff(
      {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        handoffId: 'protocol-handoff-1',
        generatedAt,
        sourceContractRefs: ['contract-protocol-handoff'],
        protocolCommandContractProofRef: 'proof-protocol-command',
        protocolEventContractProofRef: 'proof-protocol-event',
        rustProtocolMirrorProofRef: 'proof-rust-protocol-mirror',
        serviceHandlerProofRef: 'proof-service-handler',
      },
      serviceReadinessReadModel
    );

    const protocolReadModel = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel(
      {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        readModelId: 'protocol-read-model-1',
        generatedAt,
        sourceContractRefs: ['contract-protocol-read-model'],
        protocolSummaryRef: 'protocol-summary-1',
      },
      protocolHandoff
    );

    expect(protocolReadModel.protocolReadModelProofRequiredCount).toBe(1);
    expect(protocolReadModel.rows[0]?.protocolReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired
    );
    expect(protocolReadModel.rows[0]?.requiredProtocolProofRefs).toEqual([
      'proof-protocol-command',
      'proof-protocol-event',
      'proof-rust-protocol-mirror',
      'proof-service-handler',
    ]);
    expect(protocolReadModel.rows[1]?.protocolReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedBySourceFreshness
    );
  });
});
