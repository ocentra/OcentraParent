import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness } from '../../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import { buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence } from '../../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import { buildAppGameSourceGatedPolicyPreviewTimerStatus } from '../../src/app-game-source-gated-policy-preview-timer-status';
import { buildAppGameSourceFreshnessPreviewGateReadModel } from '../../src/app-game-source-freshness-preview-gate';
import { AppGameSourceFreshnessPolicyConsumptionMatrix } from '../../src/app-game-source-freshness-policy-consumption-data';
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

const TimerHandoffOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  handoffId: 'source-gated-policy-preview-timer-handoff-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-read-model'],
} as const;

const TimerStatusOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  statusId: 'source-gated-policy-preview-timer-status-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-handoff'],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  sourceFreshnessProofRef: 'source-freshness-proof-required',
  compilerDecisionProofRef: 'compiler-decision-proof-required',
} as const;

const RuntimeReadinessOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readinessId: 'source-gated-policy-preview-timer-runtime-readiness-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-status'],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  auditProofRef: 'future-timer-audit-proof',
  rollbackProofRef: 'future-timer-rollback-proof',
} as const;

const SchedulerPersistenceOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  persistenceId: 'source-gated-policy-preview-timer-scheduler-persistence-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-runtime-readiness'],
  serviceTimerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  schedulerStateStoreProofRef: 'future-scheduler-state-store-proof',
  auditProofRef: 'future-timer-audit-proof',
  rollbackProofRef: 'future-timer-rollback-proof',
} as const;

const AuditRollbackOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  handoffId: 'source-gated-policy-preview-timer-audit-rollback-handoff-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-scheduler-persistence'],
  serviceTimerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  schedulerStateStoreProofRef: 'future-scheduler-state-store-proof',
  auditTrailProofRef: 'future-timer-audit-trail-proof',
  rollbackPlanProofRef: 'future-timer-rollback-plan-proof',
  auditRollbackReadModelProofRef: 'future-timer-audit-rollback-read-model-proof',
} as const;

const AuditRollbackReadModelOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readModelId: 'source-gated-policy-preview-timer-audit-rollback-read-model-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-audit-rollback-handoff'],
  parentVisibleSummaryRef: 'future-parent-visible-audit-rollback-summary-proof',
} as const;

const ParentSurfaceIntentOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  intentId: 'source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-audit-rollback-read-model'],
  parentSurfaceDrillInRef: 'future-parent-surface-audit-rollback-drill-in-proof',
  parentSurfaceProofRef: 'future-parent-surface-audit-rollback-intent-proof',
} as const;

const ServiceReadinessOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  handoffId: 'source-gated-policy-preview-timer-service-readiness-handoff-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent'],
  serviceReadinessProofRef: 'future-service-readiness-proof',
  serviceReadApiProofRef: 'future-service-read-api-proof',
  serviceReadApiRef: 'future-service-read-api-contract-ref',
} as const;

const ServiceReadinessReadModelOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readModelId: 'source-gated-policy-preview-timer-service-readiness-read-model-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-service-readiness-handoff'],
  serviceReadinessSummaryRef: 'future-service-readiness-read-model-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness read model', () => {
  it('projects service-readiness handoff rows into service read-model proof states', () => {
    const readModel = buildServiceReadinessReadModel();

    expect(readModel.serviceReadModelProofRequiredCount).toBe(1);
    expect(readModel.blockedBySourceFreshnessCount).toBe(1);
    expect(readModel.blockedByCompilerDecisionCount).toBe(1);
    expect(readModel.rows.map((row) => row.serviceReadinessReadModelState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.ServiceReadModelProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves service-readiness proof refs without implementing the service read API', () => {
    const readModel = buildServiceReadinessReadModel();

    expect(readModel.rows[0]?.requiredProofRefs).toEqual([
      'future-service-timer-runtime-proof',
      'future-scheduler-persistence-proof',
      'future-scheduler-state-store-proof',
      'future-timer-audit-trail-proof',
      'future-timer-rollback-plan-proof',
      'future-timer-audit-rollback-read-model-proof',
      'future-parent-surface-audit-rollback-intent-proof',
      'future-service-readiness-proof',
      'future-service-read-api-proof',
    ]);
    expect(readModel.rows[0]?.serviceReadinessSummaryRef).toBe('future-service-readiness-read-model-summary-proof');
    expect(readModel.rows[0]?.serviceReadApiRef).toBe('future-service-read-api-contract-ref');
    expect(readModel.rows[0]?.serviceReadApiImplemented).toBe(false);
    expect(readModel.rows[1]?.serviceReadApiProofRequired).toBe(false);
  });

  it('rejects service read API, UI, timer, adapter, and count overclaims', () => {
    const readModel = buildServiceReadinessReadModel();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.safeParse({
        ...readModel,
        serviceReadApiImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.safeParse({
        ...readModel,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.safeParse({
        ...readModel,
        timerScheduled: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.safeParse({
        ...readModel,
        serviceReadModelProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildServiceReadinessReadModel() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel(
    ServiceReadinessReadModelOptions,
    buildServiceReadinessHandoff()
  );
}

function buildServiceReadinessHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff(
    ServiceReadinessOptions,
    buildParentSurfaceIntent()
  );
}

function buildParentSurfaceIntent() {
  return buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent(
    ParentSurfaceIntentOptions,
    buildAuditRollbackReadModel()
  );
}

function buildAuditRollbackReadModel() {
  return buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel(
    AuditRollbackReadModelOptions,
    buildAuditRollbackHandoff()
  );
}

function buildAuditRollbackHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff(
    AuditRollbackOptions,
    buildSchedulerPersistence()
  );
}

function buildSchedulerPersistence() {
  return buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
    SchedulerPersistenceOptions,
    buildRuntimeReadiness()
  );
}

function buildRuntimeReadiness() {
  return buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness(RuntimeReadinessOptions, buildStatus());
}

function buildStatus() {
  return buildAppGameSourceGatedPolicyPreviewTimerStatus(TimerStatusOptions, buildTimerHandoff());
}

function buildTimerHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerHandoff(TimerHandoffOptions, buildReadModel());
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
