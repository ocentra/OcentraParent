import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActorReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  SocialParentSensitivityCustodyMode,
  SocialParentSensitivityCustodyModeSchema,
  SocialParentSensitivityDecisionUse,
  SocialParentSensitivityDecisionUseSchema,
  type SocialParentSensitivityDecisionUseValue,
  SocialParentSensitivityLevel,
  SocialParentSensitivityLevelSchema,
  type SocialParentSensitivityLevelValue,
  SocialParentSensitivityNoClaim,
  SocialParentSensitivityNoClaimSchema,
  type SocialParentSensitivityNoClaimValue,
  SocialParentSensitivityReferenceSchema,
  SocialParentSensitivityRuntimeState,
  SocialParentSensitivityRuntimeStateSchema,
  SocialParentSensitivitySettingIdSchema,
  SocialParentSensitivitySettingKind,
  SocialParentSensitivitySettingKindSchema,
  type SocialParentSensitivitySettingKindValue,
  SocialParentSensitivityThreshold,
  SocialParentSensitivityThresholdSchema,
  type SocialParentSensitivityThresholdValue,
} from './social-parent-sensitivity-settings-values';

const SensitivityRefsSchema = Schema.Array(SocialParentSensitivityReferenceSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social sensitivity refs')
);
const SensitivityEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social sensitivity evidence refs')
);

const SocialParentSensitivitySettingBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  settingId: SocialParentSensitivitySettingIdSchema,
  family: FamilyReferenceSchema,
  childProfile: ChildProfileReferenceSchema,
  actor: ParentActorReferenceSchema,
  policyVersion: ParentPolicyVersionSchema,
  settingKind: SocialParentSensitivitySettingKindSchema,
  sensitivityLevel: SocialParentSensitivityLevelSchema,
  threshold: SocialParentSensitivityThresholdSchema,
  decisionUse: SocialParentSensitivityDecisionUseSchema,
  custodyMode: SocialParentSensitivityCustodyModeSchema,
  runtimeState: SocialParentSensitivityRuntimeStateSchema,
  sourcePrivacySummaryRefs: SensitivityRefsSchema,
  aiSignalAggregateRefs: SensitivityRefsSchema,
  dashboardPanelRefs: SensitivityRefsSchema,
  alertReportIntentRefs: Schema.Array(SocialParentSensitivityReferenceSchema),
  connectorAuthorizationRefs: Schema.Array(SocialParentSensitivityReferenceSchema),
  nativeCapabilityRefs: Schema.Array(SocialParentSensitivityReferenceSchema),
  scheduleContextRefs: Schema.Array(SocialParentSensitivityReferenceSchema),
  timeBudgetContextRefs: Schema.Array(SocialParentSensitivityReferenceSchema),
  evidenceReferences: SensitivityEvidenceRefsSchema,
  manualProofRequirements: Schema.Array(SocialParentSensitivityReferenceSchema),
  noClaimLabels: Schema.Array(SocialParentSensitivityNoClaimSchema),
  rawMessageContentAllowed: Schema.Boolean,
  rawVideoContentAllowed: Schema.Boolean,
  screenshotCaptureAllowed: Schema.Boolean,
  connectorTokenStored: Schema.Boolean,
  connectorApiCallClaimed: Schema.Boolean,
  runtimeSettingsUiClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  createdAt: ParentTimestampSchema,
});

type SocialParentSensitivitySettingCandidate = Infer<typeof SocialParentSensitivitySettingBaseSchema>;

export const SocialParentSensitivitySettingSchema = withParser(
  SocialParentSensitivitySettingBaseSchema.pipe(
    Schema.filter(
      (setting) =>
        socialSensitivityThresholdMatchesLevel(setting) ||
        'Expected social sensitivity threshold to match configured sensitivity level'
    )
  )
    .pipe(
      Schema.filter(
        (setting) =>
          socialSensitivityRefsAreCoherent(setting) ||
          'Expected social sensitivity settings to cite source privacy AI dashboard and evidence refs'
      )
    )
    .pipe(
      Schema.filter(
        (setting) =>
          socialSensitivityRuntimeStateIsHonest(setting) ||
          'Expected manual or unavailable social sensitivity settings to stay out of policy input'
      )
    )
    .pipe(
      Schema.filter(
        (setting) =>
          socialSensitivityCustodyIsHonest(setting) ||
          'Expected connector/native sensitivity settings to carry authorization or manual proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (setting) =>
          socialSensitivityHasNoUnsafeClaims(setting) ||
          'Expected social sensitivity settings to avoid raw content connector runtime final-policy and enforcement claims'
      )
    )
);

export type SocialParentSensitivitySetting = Infer<typeof SocialParentSensitivitySettingSchema>;

export {
  SocialParentSensitivityCustodyMode,
  SocialParentSensitivityDecisionUse,
  SocialParentSensitivityLevel,
  SocialParentSensitivityNoClaim,
  SocialParentSensitivityRuntimeState,
  SocialParentSensitivitySettingKind,
  SocialParentSensitivityThreshold,
};

export const decodeSocialParentSensitivitySetting = Schema.decodeUnknownSync(SocialParentSensitivitySettingSchema);

const thresholdByLevel: Record<SocialParentSensitivityLevelValue, SocialParentSensitivityThresholdValue> = {
  [SocialParentSensitivityLevel.Low]: SocialParentSensitivityThreshold.MediumConfidenceWarn,
  [SocialParentSensitivityLevel.Standard]: SocialParentSensitivityThreshold.LowConfidenceParentReview,
  [SocialParentSensitivityLevel.High]: SocialParentSensitivityThreshold.HighConfidenceBlockCandidate,
  [SocialParentSensitivityLevel.ManualOnly]: SocialParentSensitivityThreshold.ManualReviewOnly,
};

const RequiredNoClaimLabels = Object.values(SocialParentSensitivityNoClaim);

function socialSensitivityThresholdMatchesLevel(setting: SocialParentSensitivitySettingCandidate): boolean {
  return setting.threshold === thresholdByLevel[setting.sensitivityLevel];
}

function socialSensitivityRefsAreCoherent(setting: SocialParentSensitivitySettingCandidate): boolean {
  return (
    setting.sourcePrivacySummaryRefs.length > 0 &&
    setting.aiSignalAggregateRefs.length > 0 &&
    setting.dashboardPanelRefs.length > 0 &&
    setting.evidenceReferences.length > 0
  );
}

function socialSensitivityRuntimeStateIsHonest(setting: SocialParentSensitivitySettingCandidate): boolean {
  if (setting.runtimeState === SocialParentSensitivityRuntimeState.ContractOnly) {
    return (
      setting.decisionUse !== SocialParentSensitivityDecisionUse.ManualRequired &&
      setting.manualProofRequirements.length === 0
    );
  }
  return (
    setting.decisionUse === SocialParentSensitivityDecisionUse.ManualRequired &&
    setting.manualProofRequirements.length > 0
  );
}

function socialSensitivityCustodyIsHonest(setting: SocialParentSensitivitySettingCandidate): boolean {
  if (setting.settingKind === SocialParentSensitivitySettingKind.ConnectorDataUse) {
    return (
      setting.custodyMode === SocialParentSensitivityCustodyMode.ConnectorAuthorizationRefOnly &&
      setting.connectorAuthorizationRefs.length > 0
    );
  }
  if (setting.settingKind === SocialParentSensitivitySettingKind.NativeAppGapReview) {
    return setting.nativeCapabilityRefs.length > 0 || setting.manualProofRequirements.length > 0;
  }
  if (setting.decisionUse === SocialParentSensitivityDecisionUse.PolicyCandidateInput) {
    return (
      setting.custodyMode === SocialParentSensitivityCustodyMode.LocalRedactedRefsOnly &&
      setting.scheduleContextRefs.length > 0 &&
      setting.timeBudgetContextRefs.length > 0
    );
  }
  return (
    setting.custodyMode !== SocialParentSensitivityCustodyMode.ManualRequired ||
    setting.manualProofRequirements.length > 0
  );
}

function socialSensitivityHasNoUnsafeClaims(setting: SocialParentSensitivitySettingCandidate): boolean {
  const labels = new Set<SocialParentSensitivityNoClaimValue>(setting.noClaimLabels);
  return (
    RequiredNoClaimLabels.every((label) => labels.has(label)) &&
    !setting.rawMessageContentAllowed &&
    !setting.rawVideoContentAllowed &&
    !setting.screenshotCaptureAllowed &&
    !setting.connectorTokenStored &&
    !setting.connectorApiCallClaimed &&
    !setting.runtimeSettingsUiClaimed &&
    !setting.finalPolicyDecisionClaimed &&
    !setting.enforcementClaimed
  );
}

export type SocialParentSensitivityDecisionUseForPolicy = Extract<
  SocialParentSensitivityDecisionUseValue,
  'policy-candidate-input'
>;
export type SocialParentSensitivitySettingKindForConnector = Extract<
  SocialParentSensitivitySettingKindValue,
  'connector-data-use'
>;
