import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-rules';
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
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  sourceFreshnessProofRef: 'source-freshness-proof-required',
  compilerDecisionProofRef: 'compiler-decision-proof-required',
} as const;

const RuntimeReadinessOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readinessId: 'source-gated-policy-preview-timer-runtime-readiness-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-status',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  auditProofRef: 'future-timer-audit-proof',
  rollbackProofRef: 'future-timer-rollback-proof',
} as const;

const SchedulerPersistenceOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  persistenceId: 'source-gated-policy-preview-timer-scheduler-persistence-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-runtime-readiness',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
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
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-scheduler-persistence',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
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
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-audit-rollback-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  parentVisibleSummaryRef: 'future-parent-visible-audit-rollback-summary-proof',
} as const;

describe('app/game source-gated policy preview timer audit rollback read model', () => {
  it('projects audit rollback handoff rows into parent-visible read-model proof states', () => {
    const readModel = buildAuditRollbackReadModel();

    expect(readModel.auditRollbackReadModelProofRequiredCount).toBe(1);
    expect(readModel.blockedBySourceFreshnessCount).toBe(1);
    expect(readModel.blockedByCompilerDecisionCount).toBe(1);
    expect(readModel.rows.map((row) => row.readModelState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.AuditRollbackReadModelProofRequired,
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves audit trail, rollback plan, and read model proof requirements without scheduling', () => {
    const readModel = buildAuditRollbackReadModel();

    expect(readModel.rows[0]?.requiredProofRefs).toEqual([
      'future-service-timer-runtime-proof',
      'future-scheduler-persistence-proof',
      'future-scheduler-state-store-proof',
      'future-timer-audit-trail-proof',
      'future-timer-rollback-plan-proof',
      'future-timer-audit-rollback-read-model-proof',
    ]);
    expect(readModel.rows[0]?.auditTrailProofRequired).toBe(true);
    expect(readModel.rows[0]?.rollbackPlanProofRequired).toBe(true);
    expect(readModel.rows[0]?.auditRollbackReadModelProofRequired).toBe(true);
    expect(readModel.rows[0]?.parentVisibleSummaryRef).toBe('future-parent-visible-audit-rollback-summary-proof');
    expect(readModel.rows[0]?.timerScheduled).toBe(false);
  });

  it('rejects durable audit, rollback execution, scheduled timers, and count drift', () => {
    const readModel = buildAuditRollbackReadModel();

    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.safeParse({
        ...readModel,
        durableAuditLogClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.safeParse({
        ...readModel,
        rollbackExecutionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.safeParse({
        ...readModel,
        timerScheduled: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.safeParse({
        ...readModel,
        auditRollbackReadModelProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

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
  return buildAppSourceFreshnessPreviewGateReadModel([
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

function buildAppSourceFreshnessPreviewGateReadModel(
  rows: Parameters<typeof buildAppGameSourceFreshnessPreviewGateReadModel>[1]
) {
  return buildAppGameSourceFreshnessPreviewGateReadModel(GateOptions, rows);
}
