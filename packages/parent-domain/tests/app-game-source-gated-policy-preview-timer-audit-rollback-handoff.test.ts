import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff } from '../src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState,
} from '../src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../src/app-game-source-gated-policy-preview-timer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness } from '../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import { buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence } from '../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import { buildAppGameSourceGatedPolicyPreviewTimerStatus } from '../src/app-game-source-gated-policy-preview-timer-status';
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

describe('app/game source-gated policy preview timer audit rollback handoff', () => {
  it('keeps only scheduler-persistence rows eligible for audit and rollback proof', () => {
    const handoff = buildAuditRollbackHandoff();

    expect(handoff.auditRollbackProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.auditRollbackState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired,
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('requires audit trail, rollback plan, and read model proof before scheduling', () => {
    const handoff = buildAuditRollbackHandoff();

    expect(handoff.rows[0]?.requiredProofRefs).toEqual([
      'future-service-timer-runtime-proof',
      'future-scheduler-persistence-proof',
      'future-scheduler-state-store-proof',
      'future-timer-audit-trail-proof',
      'future-timer-rollback-plan-proof',
      'future-timer-audit-rollback-read-model-proof',
    ]);
    expect(handoff.rows[0]?.auditTrailProofRequired).toBe(true);
    expect(handoff.rows[0]?.rollbackPlanProofRequired).toBe(true);
    expect(handoff.rows[0]?.auditRollbackReadModelProofRequired).toBe(true);
  });

  it('rejects durable audit, rollback execution, scheduled timers, and count drift', () => {
    const handoff = buildAuditRollbackHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.safeParse({
        ...handoff,
        durableAuditLogClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.safeParse({
        ...handoff,
        rollbackExecutionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.safeParse({
        ...handoff,
        timerScheduled: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.safeParse({
        ...handoff,
        auditRollbackProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

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
