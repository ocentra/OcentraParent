import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import { AppGameSourceGatedPolicyPreviewTimerHandoffSchema } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-handoff';
import { AppGameSourceGatedPolicyPreviewTimerHandoffState } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-handoff-rules';
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
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-read-model',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
} as const;

describe('app/game source-gated policy preview timer handoff', () => {
  it('marks preview-ready rows as future timer sequencing candidates', () => {
    const handoff = buildHandoff();

    expect(handoff.timerSequenceCandidateCount).toBe(1);
    expect(handoff.sourceManualBlockedCount).toBe(1);
    expect(handoff.compilerManualBlockedCount).toBe(1);
    expect(handoff.rows.map((row) => row.timerHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing,
      AppGameSourceGatedPolicyPreviewTimerHandoffState.SourceManualRequiredBeforeTimer,
      AppGameSourceGatedPolicyPreviewTimerHandoffState.CompilerManualRequiredBeforeTimer,
    ]);
  });

  it('keeps manual rows out of future timer runtime sequencing', () => {
    const handoff = buildHandoff();

    expect(handoff.rows[0]?.timerRuntimeRequired).toBe(true);
    expect(handoff.rows[0]?.manualProofRequired).toBe(false);
    expect(handoff.rows[1]?.timerRuntimeRequired).toBe(false);
    expect(handoff.rows[1]?.manualProofRequired).toBe(true);
    expect(handoff.rows[2]?.timerRuntimeRequired).toBe(false);
    expect(handoff.rows[2]?.manualProofRequired).toBe(true);
  });

  it('rejects timer runtime, adapter dispatch, and row-count overclaims', () => {
    const handoff = buildHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerHandoffSchema.safeParse({
        ...handoff,
        timerRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerHandoffSchema.safeParse({
        ...handoff,
        timerSequenceCandidateCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildHandoff() {
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
