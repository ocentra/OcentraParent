import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../src/app-game-source-gated-policy-preview-timer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness } from '../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import { buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence } from '../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState,
} from '../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
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

describe('app/game source-gated policy preview timer scheduler persistence', () => {
  it('keeps only runtime-ready rows eligible for scheduler persistence proof', () => {
    const persistence = buildPersistence();

    expect(persistence.schedulerPersistenceProofRequiredCount).toBe(1);
    expect(persistence.blockedBySourceFreshnessCount).toBe(1);
    expect(persistence.blockedByCompilerDecisionCount).toBe(1);
    expect(persistence.rows.map((row) => row.schedulerPersistenceState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired,
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision,
    ]);
  });

  it('requires runtime, persistence, state-store, audit, and rollback proof before scheduling', () => {
    const persistence = buildPersistence();

    expect(persistence.rows[0]?.requiredProofRefs).toEqual([
      'future-service-timer-runtime-proof',
      'future-scheduler-persistence-proof',
      'future-scheduler-state-store-proof',
      'future-timer-audit-proof',
      'future-timer-rollback-proof',
    ]);
    expect(persistence.rows[0]?.serviceTimerRuntimeProofRequired).toBe(true);
    expect(persistence.rows[0]?.schedulerPersistenceProofRequired).toBe(true);
    expect(persistence.rows[0]?.schedulerStateStoreProofRequired).toBe(true);
    expect(persistence.rows[0]?.auditProofRequired).toBe(true);
    expect(persistence.rows[0]?.rollbackProofRequired).toBe(true);
  });

  it('rejects persistence runtime claims, durable storage claims, scheduled timers, and count drift', () => {
    const persistence = buildPersistence();

    expect(
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.safeParse({
        ...persistence,
        schedulerPersistenceRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.safeParse({
        ...persistence,
        durableSchedulerStorageClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.safeParse({
        ...persistence,
        timerScheduled: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.safeParse({
        ...persistence,
        schedulerPersistenceProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildPersistence() {
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
