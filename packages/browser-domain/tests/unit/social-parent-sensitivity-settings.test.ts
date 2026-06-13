import { describe, expect, it } from 'vitest';
import {
  SocialParentSensitivityCustodyMode,
  SocialParentSensitivityDecisionUse,
  SocialParentSensitivityLevel,
  SocialParentSensitivityNoClaim,
  SocialParentSensitivityRuntimeState,
  SocialParentSensitivitySettingKind,
  SocialParentSensitivitySettingSchema,
  SocialParentSensitivityThreshold,
} from '../../src/social-parent-sensitivity-settings';
import { ParentActorRole, ParentContractSchemaVersion, ParentEvidenceReferenceKind } from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-06T08:21:00Z';
const RequiredNoClaimLabels = [
  SocialParentSensitivityNoClaim.RawContent,
  SocialParentSensitivityNoClaim.ConnectorApi,
  SocialParentSensitivityNoClaim.RuntimeSettings,
  SocialParentSensitivityNoClaim.FinalPolicy,
  SocialParentSensitivityNoClaim.Enforcement,
] as const;

const BaseSetting = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  settingId: 'social-sensitivity-high-risk-alert',
  family: { familyId: 'family-social-sensitivity' },
  childProfile: {
    childProfileId: 'child-social-sensitivity',
    displayName: 'Study profile',
  },
  actor: {
    actorId: 'parent-social-sensitivity',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-social-sensitivity-v1',
  settingKind: SocialParentSensitivitySettingKind.HighRiskAlert,
  sensitivityLevel: SocialParentSensitivityLevel.Standard,
  threshold: SocialParentSensitivityThreshold.LowConfidenceParentReview,
  decisionUse: SocialParentSensitivityDecisionUse.PolicyCandidateInput,
  custodyMode: SocialParentSensitivityCustodyMode.LocalRedactedRefsOnly,
  runtimeState: SocialParentSensitivityRuntimeState.ContractOnly,
  sourcePrivacySummaryRefs: ['social-source-privacy-summary'],
  aiSignalAggregateRefs: ['social-ai-signal-aggregate'],
  dashboardPanelRefs: ['panel-feed-video-gates'],
  alertReportIntentRefs: ['social-alert-report-high-risk'],
  connectorAuthorizationRefs: [],
  nativeCapabilityRefs: [],
  scheduleContextRefs: ['schedule-social-school-night'],
  timeBudgetContextRefs: ['time-budget-social-daily'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-sensitivity-source',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  manualProofRequirements: [],
  noClaimLabels: RequiredNoClaimLabels,
  rawMessageContentAllowed: false,
  rawVideoContentAllowed: false,
  screenshotCaptureAllowed: false,
  connectorTokenStored: false,
  connectorApiCallClaimed: false,
  runtimeSettingsUiClaimed: false,
  finalPolicyDecisionClaimed: false,
  enforcementClaimed: false,
  createdAt: Timestamp,
} as const;

describe('social parent sensitivity settings contracts', () => {
  acceptsPolicyCandidateSensitivitySettings();
  acceptsConnectorAndNativeManualBoundaries();
  rejectsUnsafeRawConnectorRuntimeAndPolicyClaims();
  rejectsMismatchedThresholdAndMissingRefs();
  keepsManualAndUnavailableRowsOutOfPolicyInput();
});

function acceptsPolicyCandidateSensitivitySettings() {
  it('accepts a redacted local sensitivity setting as policy candidate input', () => {
    const parsed = SocialParentSensitivitySettingSchema.parse(BaseSetting);

    expect(parsed.settingKind).toBe(SocialParentSensitivitySettingKind.HighRiskAlert);
    expect(parsed.threshold).toBe(SocialParentSensitivityThreshold.LowConfidenceParentReview);
    expect(parsed.decisionUse).toBe(SocialParentSensitivityDecisionUse.PolicyCandidateInput);
    expect(parsed.scheduleContextRefs).toEqual(['schedule-social-school-night']);
    expect(parsed.timeBudgetContextRefs).toEqual(['time-budget-social-daily']);
  });
}

function acceptsConnectorAndNativeManualBoundaries() {
  it('accepts connector ref-only and native manual-required sensitivity rows without runtime claims', () => {
    const connector = SocialParentSensitivitySettingSchema.parse({
      ...BaseSetting,
      settingId: 'social-sensitivity-connector-data-use',
      settingKind: SocialParentSensitivitySettingKind.ConnectorDataUse,
      sensitivityLevel: SocialParentSensitivityLevel.Low,
      threshold: SocialParentSensitivityThreshold.MediumConfidenceWarn,
      decisionUse: SocialParentSensitivityDecisionUse.AlertCandidateInput,
      custodyMode: SocialParentSensitivityCustodyMode.ConnectorAuthorizationRefOnly,
      connectorAuthorizationRefs: ['social-connector-authorization-ref'],
      scheduleContextRefs: [],
      timeBudgetContextRefs: [],
    });
    const nativeManual = SocialParentSensitivitySettingSchema.parse({
      ...BaseSetting,
      settingId: 'social-sensitivity-native-app-gap',
      settingKind: SocialParentSensitivitySettingKind.NativeAppGapReview,
      sensitivityLevel: SocialParentSensitivityLevel.ManualOnly,
      threshold: SocialParentSensitivityThreshold.ManualReviewOnly,
      decisionUse: SocialParentSensitivityDecisionUse.ManualRequired,
      custodyMode: SocialParentSensitivityCustodyMode.ManualRequired,
      runtimeState: SocialParentSensitivityRuntimeState.ManualRequired,
      nativeCapabilityRefs: ['social-android-native-capability-row'],
      scheduleContextRefs: [],
      timeBudgetContextRefs: [],
      manualProofRequirements: ['android-native-social-app-device-proof-required'],
    });

    expect(connector.connectorAuthorizationRefs).toEqual(['social-connector-authorization-ref']);
    expect(nativeManual.runtimeState).toBe(SocialParentSensitivityRuntimeState.ManualRequired);
  });
}

function rejectsUnsafeRawConnectorRuntimeAndPolicyClaims() {
  it('rejects raw content connector token API UI final-policy and enforcement claims', () => {
    for (const invalidSetting of [
      { ...BaseSetting, settingId: 'social-sensitivity-raw-message', rawMessageContentAllowed: true },
      { ...BaseSetting, settingId: 'social-sensitivity-raw-video', rawVideoContentAllowed: true },
      { ...BaseSetting, settingId: 'social-sensitivity-screenshot', screenshotCaptureAllowed: true },
      { ...BaseSetting, settingId: 'social-sensitivity-token', connectorTokenStored: true },
      { ...BaseSetting, settingId: 'social-sensitivity-connector-api', connectorApiCallClaimed: true },
      { ...BaseSetting, settingId: 'social-sensitivity-runtime-ui', runtimeSettingsUiClaimed: true },
      { ...BaseSetting, settingId: 'social-sensitivity-final-policy', finalPolicyDecisionClaimed: true },
      { ...BaseSetting, settingId: 'social-sensitivity-enforcement', enforcementClaimed: true },
      {
        ...BaseSetting,
        settingId: 'social-sensitivity-missing-no-claim',
        noClaimLabels: [SocialParentSensitivityNoClaim.RawContent],
      },
    ]) {
      expect(SocialParentSensitivitySettingSchema.safeParse(invalidSetting).success).toBe(false);
    }
  });
}

function rejectsMismatchedThresholdAndMissingRefs() {
  it('rejects mismatched thresholds missing evidence refs and policy input without schedule or budget refs', () => {
    for (const invalidSetting of [
      {
        ...BaseSetting,
        settingId: 'social-sensitivity-wrong-threshold',
        threshold: SocialParentSensitivityThreshold.HighConfidenceBlockCandidate,
      },
      { ...BaseSetting, settingId: 'social-sensitivity-missing-source', sourcePrivacySummaryRefs: [] },
      { ...BaseSetting, settingId: 'social-sensitivity-missing-ai', aiSignalAggregateRefs: [] },
      { ...BaseSetting, settingId: 'social-sensitivity-missing-dashboard', dashboardPanelRefs: [] },
      { ...BaseSetting, settingId: 'social-sensitivity-missing-evidence', evidenceReferences: [] },
      { ...BaseSetting, settingId: 'social-sensitivity-missing-schedule', scheduleContextRefs: [] },
      { ...BaseSetting, settingId: 'social-sensitivity-missing-budget', timeBudgetContextRefs: [] },
    ]) {
      expect(SocialParentSensitivitySettingSchema.safeParse(invalidSetting).success).toBe(false);
    }
  });
}

function keepsManualAndUnavailableRowsOutOfPolicyInput() {
  it('rejects manual-required and unavailable rows that pretend to be policy candidate input', () => {
    for (const invalidSetting of [
      {
        ...BaseSetting,
        settingId: 'social-sensitivity-manual-policy-input',
        sensitivityLevel: SocialParentSensitivityLevel.ManualOnly,
        threshold: SocialParentSensitivityThreshold.ManualReviewOnly,
        runtimeState: SocialParentSensitivityRuntimeState.ManualRequired,
        manualProofRequirements: ['social-sensitivity-manual-proof'],
      },
      {
        ...BaseSetting,
        settingId: 'social-sensitivity-unavailable-policy-input',
        runtimeState: SocialParentSensitivityRuntimeState.Unavailable,
        manualProofRequirements: ['social-sensitivity-runtime-proof-unavailable'],
      },
    ]) {
      expect(SocialParentSensitivitySettingSchema.safeParse(invalidSetting).success).toBe(false);
    }
  });
}
