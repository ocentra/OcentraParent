import { describe, expect, it } from 'vitest';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenEvidenceRemoteBoundarySettingSchema,
} from '@ocentra-parent/schema-domain/screen-evidence-settings';
import {
  ScreenEvidenceSettingsUiProofSchema,
  screenEvidenceSettingsWritableUiProof,
} from '@ocentra-parent/schema-domain/screen-evidence-settings-ui-proof';

describe('screen evidence settings UI proof', () => {
  specifyWritableIntentProof();
  specifyDisabledIntent();
  specifyObserveOnlyIntent();
  specifyStrictDryRunIntent();
  specifyApprovedRawRetentionIntent();
});

function specifyWritableIntentProof() {
  it('builds disabled, observe-only, strict dry-run, and approved raw-retention intents from real schemas', () => {
    const proof = screenEvidenceSettingsWritableUiProof();

    expect(ScreenEvidenceSettingsUiProofSchema.safeParse(proof).success).toBe(true);
    expect(proof.title).toBe('Writable screen settings proof');
    expect(proof.note).toContain('submit it to the child service command path');
    expect(proof.serviceCommandHeading).toBe('Service command');
    expect(proof.serviceApplyActionLabel).toBe('Save selected screen setting');
    expect(proof.serviceRefreshActionLabel).toBe('Refresh persisted screen setting');
    expect(proof.serviceAcceptedStatus).toBe('service accepted persisted setting');
    expect(proof.defaultIntentKey).toBe('disabledLocalSummary');
    expect(proof.intents.map((intent) => intent.intentKey)).toEqual([
      'disabledLocalSummary',
      'observeOnlyLocalSummary',
      'strictDryRunLocalSummary',
      'approvedRawRetentionLocalTtl',
    ]);
    expect(proof.intents.map((intent) => ScreenAnalysisParentSettingSchema.safeParse(intent.setting).success)).toEqual([
      true,
      true,
      true,
      true,
    ]);
    expect(
      proof.intents.map(
        (intent) => ScreenEvidenceRemoteBoundarySettingSchema.safeParse(intent.remoteBoundarySetting).success
      )
    ).toEqual([true, true, true, true]);
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

function specifyApprovedRawRetentionIntent() {
  it('requires approved local short TTL retention to stay local and deletion-bound', () => {
    const rawRetention = screenEvidenceSettingsWritableUiProof().intents[3]?.setting;
    const remoteBoundary = screenEvidenceSettingsWritableUiProof().intents[3]?.remoteBoundarySetting;

    expect(rawRetention?.retainRawImage).toBe(true);
    expect(rawRetention?.temporaryImageTtlSeconds).toBe(120);
    expect(rawRetention?.deleteAfterSuccess).toBe(true);
    expect(rawRetention?.deleteAfterExpiry).toBe(true);
    expect(remoteBoundary?.rawScreenshotRetentionMode).toBe('parentApprovedLocalShortTtl');
    expect(remoteBoundary?.rawScreenshotRemoteUploadEnabled).toBe(false);
    expect(remoteBoundary?.liveViewMode).toBe('disabled');
  });
}
