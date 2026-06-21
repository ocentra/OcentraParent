import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerStatus } from '../../src/app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerStatusSchema } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerStatusState } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-status-rules';
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

describe('app/game source-gated policy preview timer status', () => {
  it('classifies timer handoff rows into required proof statuses', () => {
    const status = buildStatus();

    expect(status.timerRuntimeProofRequiredCount).toBe(1);
    expect(status.sourceFreshnessProofRequiredCount).toBe(1);
    expect(status.compilerDecisionProofRequiredCount).toBe(1);
    expect(status.rows.map((row) => row.timerStatusState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired,
      AppGameSourceGatedPolicyPreviewTimerStatusState.SourceFreshnessProofRequired,
      AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired,
    ]);
  });

  it('pins a proof ref on each row without scheduling timers', () => {
    const status = buildStatus();

    expect(status.rows[0]?.requiredProofRefs).toEqual(['future-service-timer-runtime-proof']);
    expect(status.rows[1]?.requiredProofRefs).toEqual(['source-freshness-proof-required']);
    expect(status.rows[2]?.requiredProofRefs).toEqual(['compiler-decision-proof-required']);
    expect(status.rows.every((row) => row.timerScheduled === false)).toBe(true);
  });

  it('rejects timer scheduling, adapter dispatch, and proof-count drift', () => {
    const status = buildStatus();

    expect(
      AppGameSourceGatedPolicyPreviewTimerStatusSchema.safeParse({
        ...status,
        timerScheduled: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerStatusSchema.safeParse({
        ...status,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerStatusSchema.safeParse({
        ...status,
        timerRuntimeProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

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
