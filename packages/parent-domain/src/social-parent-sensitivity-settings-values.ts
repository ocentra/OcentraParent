import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptySocialSensitivityText = Schema.String.pipe(Schema.minLength(1));

export const SocialParentSensitivitySettingKind = {
  HighRiskAlert: 'high-risk-alert',
  FeedVideoReview: 'feed-video-review',
  AccountFlowReview: 'account-flow-review',
  ConnectorDataUse: 'connector-data-use',
  NativeAppGapReview: 'native-app-gap-review',
  WeeklySummary: 'weekly-summary',
} as const;

export const SocialParentSensitivityLevel = {
  Low: 'low',
  Standard: 'standard',
  High: 'high',
  ManualOnly: 'manual-only',
} as const;

export const SocialParentSensitivityDecisionUse = {
  PolicyCandidateInput: 'policy-candidate-input',
  AlertCandidateInput: 'alert-candidate-input',
  ExplanationOnly: 'explanation-only',
  ManualRequired: 'manual-required',
} as const;

export const SocialParentSensitivityCustodyMode = {
  LocalRedactedRefsOnly: 'local-redacted-refs-only',
  ParentProvidedRefsOnly: 'parent-provided-refs-only',
  ConnectorAuthorizationRefOnly: 'connector-authorization-ref-only',
  ManualRequired: 'manual-required',
} as const;

export const SocialParentSensitivityRuntimeState = {
  ContractOnly: 'contract-only',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialParentSensitivityThreshold = {
  LowConfidenceParentReview: 'low-confidence-parent-review',
  MediumConfidenceWarn: 'medium-confidence-warn',
  HighConfidenceBlockCandidate: 'high-confidence-block-candidate',
  ManualReviewOnly: 'manual-review-only',
} as const;

export const SocialParentSensitivityNoClaim = {
  RawContent: 'raw-content-not-claimed',
  ConnectorApi: 'connector-api-not-claimed',
  RuntimeSettings: 'runtime-settings-not-claimed',
  FinalPolicy: 'final-policy-not-claimed',
  Enforcement: 'enforcement-not-claimed',
} as const;

export const SocialParentSensitivitySettingIdSchema = withParser(
  NonEmptySocialSensitivityText.pipe(Schema.brand('SocialParentSensitivitySettingId'))
);
export const SocialParentSensitivityReferenceSchema = withParser(
  NonEmptySocialSensitivityText.pipe(Schema.brand('SocialParentSensitivityReference'))
);
export const SocialParentSensitivitySettingKindSchema = withParser(
  Schema.Literal(...Object.values(SocialParentSensitivitySettingKind))
);
export const SocialParentSensitivityLevelSchema = withParser(
  Schema.Literal(...Object.values(SocialParentSensitivityLevel))
);
export const SocialParentSensitivityDecisionUseSchema = withParser(
  Schema.Literal(...Object.values(SocialParentSensitivityDecisionUse))
);
export const SocialParentSensitivityCustodyModeSchema = withParser(
  Schema.Literal(...Object.values(SocialParentSensitivityCustodyMode))
);
export const SocialParentSensitivityRuntimeStateSchema = withParser(
  Schema.Literal(...Object.values(SocialParentSensitivityRuntimeState))
);
export const SocialParentSensitivityThresholdSchema = withParser(
  Schema.Literal(...Object.values(SocialParentSensitivityThreshold))
);
export const SocialParentSensitivityNoClaimSchema = withParser(
  Schema.Literal(...Object.values(SocialParentSensitivityNoClaim))
);

export type SocialParentSensitivitySettingKindValue = Infer<typeof SocialParentSensitivitySettingKindSchema>;
export type SocialParentSensitivityLevelValue = Infer<typeof SocialParentSensitivityLevelSchema>;
export type SocialParentSensitivityDecisionUseValue = Infer<typeof SocialParentSensitivityDecisionUseSchema>;
export type SocialParentSensitivityCustodyModeValue = Infer<typeof SocialParentSensitivityCustodyModeSchema>;
export type SocialParentSensitivityRuntimeStateValue = Infer<typeof SocialParentSensitivityRuntimeStateSchema>;
export type SocialParentSensitivityThresholdValue = Infer<typeof SocialParentSensitivityThresholdSchema>;
export type SocialParentSensitivityNoClaimValue = Infer<typeof SocialParentSensitivityNoClaimSchema>;
