import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness } from '../../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import { AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-runtime-readiness';
import { AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-runtime-readiness-rules';
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

describe('app/game source-gated policy preview timer runtime readiness', () => {
  it('keeps only timer-runtime status rows eligible for runtime proof', () => {
    const readiness = buildReadiness();

    expect(readiness.runtimeProofRequiredCount).toBe(1);
    expect(readiness.blockedBySourceFreshnessCount).toBe(1);
    expect(readiness.blockedByCompilerDecisionCount).toBe(1);
    expect(readiness.rows.map((row) => row.runtimeReadinessState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired,
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision,
    ]);
  });

  it('requires timer runtime, persistence, audit, and rollback proof before scheduling', () => {
    const readiness = buildReadiness();

    expect(readiness.rows[0]?.requiredProofRefs).toEqual([
      'future-service-timer-runtime-proof',
      'future-scheduler-persistence-proof',
      'future-timer-audit-proof',
      'future-timer-rollback-proof',
    ]);
    expect(readiness.rows[0]?.timerRuntimeProofRequired).toBe(true);
    expect(readiness.rows[0]?.schedulerPersistenceProofRequired).toBe(true);
    expect(readiness.rows[0]?.auditProofRequired).toBe(true);
    expect(readiness.rows[0]?.rollbackProofRequired).toBe(true);
    expect(readiness.rows.every((row) => row.timerScheduled === false)).toBe(true);
  });

  it('rejects runtime claims, scheduled timers, and readiness count drift', () => {
    const readiness = buildReadiness();

    expect(
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema.safeParse({
        ...readiness,
        timerRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema.safeParse({
        ...readiness,
        timerScheduled: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema.safeParse({
        ...readiness,
        runtimeProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildReadiness() {
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
