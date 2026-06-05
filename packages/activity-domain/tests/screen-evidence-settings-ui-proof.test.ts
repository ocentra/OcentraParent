import { describe, expect, it } from 'vitest';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenEvidenceRemoteBoundarySettingSchema,
  ScreenEvidenceSettingsUiProofSchema,
  screenEvidenceSettingsWritableUiProof,
} from '../src/screen-evidence';

describe('screen evidence settings UI proof', () => {
  specifyWritableIntentProof();
  specifyDisabledIntent();
  specifyObserveOnlyIntent();
  specifyStrictDryRunIntent();
});

function specifyWritableIntentProof() {
  it('builds disabled, observe-only, and strict dry-run parent setting intents from real schemas', () => {
    const proof = screenEvidenceSettingsWritableUiProof();

    expect(ScreenEvidenceSettingsUiProofSchema.safeParse(proof).success).toBe(true);
    expect(proof.title).toBe('Writable screen-analysis settings proof');
    expect(proof.defaultIntentKey).toBe('disabledLocalSummary');
    expect(proof.intents.map((intent) => intent.intentKey)).toEqual([
      'disabledLocalSummary',
      'observeOnlyLocalSummary',
      'strictDryRunLocalSummary',
    ]);
    expect(proof.intents.map((intent) => ScreenAnalysisParentSettingSchema.safeParse(intent.setting).success)).toEqual([
      true,
      true,
      true,
    ]);
    expect(
      proof.intents.map(
        (intent) => ScreenEvidenceRemoteBoundarySettingSchema.safeParse(intent.remoteBoundarySetting).success
      )
    ).toEqual([true, true, true]);
  });
}

function specifyDisabledIntent() {
  it('keeps disabled intent unable to capture or drive policy', () => {
    const disabled = screenEvidenceSettingsWritableUiProof().intents[0]?.setting;

    expect(disabled?.screenAnalysisEnabled).toBe(false);
    expect(disabled?.cadenceCaptureEnabled).toBe(false);
    expect(disabled?.triggerCaptureEnabled).toBe(false);
    expect(disabled?.strictModeEnabled).toBe(false);
    expect(disabled?.policyUseEnabled).toBe(false);
    expect(disabled?.retainRawImage).toBe(false);
  });
}

function specifyObserveOnlyIntent() {
  it('keeps observe-only summaries out of policy handoff', () => {
    const observeOnly = screenEvidenceSettingsWritableUiProof().intents[1]?.setting;

    expect(observeOnly?.screenAnalysisEnabled).toBe(true);
    expect(observeOnly?.analysisMode).toBe('observeOnly');
    expect(observeOnly?.cadenceSeconds).toBe(300);
    expect(observeOnly?.enabledTriggers).toEqual(['foregroundAppChange', 'policyAmbiguity']);
    expect(observeOnly?.policyUseEnabled).toBe(false);
    expect(observeOnly?.retainRawImage).toBe(false);
  });
}

function specifyStrictDryRunIntent() {
  it('requires strict dry-run to use one-minute cadence, explicit triggers, and no raw retention', () => {
    const strictDryRun = screenEvidenceSettingsWritableUiProof().intents[2]?.setting;
    const remoteBoundary = screenEvidenceSettingsWritableUiProof().intents[2]?.remoteBoundarySetting;

    expect(strictDryRun?.analysisMode).toBe('policyDryRun');
    expect(strictDryRun?.cadenceSeconds).toBe(60);
    expect(strictDryRun?.strictModeEnabled).toBe(true);
    expect(strictDryRun?.policyUseEnabled).toBe(true);
    expect(strictDryRun?.enabledTriggers).toEqual([
      'foregroundAppChange',
      'managedBrowserUrlChange',
      'appGameForegroundStart',
      'policyAmbiguity',
    ]);
    expect(strictDryRun?.retainRawImage).toBe(false);
    expect(remoteBoundary?.rawScreenshotRetentionMode).toBe('disabled');
    expect(remoteBoundary?.liveViewMode).toBe('disabled');
    expect(remoteBoundary?.rawScreenshotRemoteUploadEnabled).toBe(false);
  });
}
