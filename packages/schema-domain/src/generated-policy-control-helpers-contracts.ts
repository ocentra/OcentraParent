export const GeneratedPolicyActionValues = ['allow', 'warn', 'block', 'time-limit', 'ask-parent', 'unknown'] as const;
export const GeneratedPolicyTargetTypeValues = [
  'app',
  'process',
  'window',
  'domain',
  'site',
  'category',
  'video',
  'channel',
  'activity-type',
  'device',
] as const;
export const GeneratedPolicyScheduleDayValues = [
  'monday',
  'tuesday',
  'wednesday',
  'thursday',
  'friday',
  'saturday',
  'sunday',
] as const;
export const GeneratedPolicyDecisionHandoffStateValues = [
  'not-requested',
  'disabled',
  'pending',
  'handed-off',
] as const;
export const GeneratedPermissionRequestStateValues = ['open', 'approved', 'denied', 'expired', 'cancelled'] as const;
export const GeneratedPolicyPreviewConfirmationStateValues = ['confirmation-required', 'confirmed'] as const;
export const GeneratedPolicyScheduleBoundaryStateValues = [
  'within-window',
  'outside-window',
  'dst-gap',
  'dst-overlap',
  'clock-skew',
  'exception-active',
  'expired',
] as const;
export const GeneratedPolicyScheduleDstTransitionValues = ['spring-forward', 'fall-back'] as const;
export const GeneratedPolicyScheduleDstResolutionValues = [
  'skip-forward',
  'first-occurrence',
  'second-occurrence',
  'manual-required',
] as const;
export const GeneratedPolicyScheduleClockSourceValues = ['child-device', 'trusted-service', 'manual-required'] as const;
export const GeneratedPolicyScheduleOfflineRecoveryStateValues = [
  'not-needed',
  'recovered-from-device',
  'recomputed-from-journal',
  'manual-required',
] as const;
export const GeneratedPolicyScheduleBudgetResetKindValues = ['daily', 'weekly', 'monthly'] as const;
export const GeneratedPolicyScheduleBudgetCarryoverModeValues = [
  'discard-unused',
  'carry-forward',
  'cap-carryover',
] as const;
export const GeneratedPolicyScheduleOfflineRecoveryValues = [
  'resume-remaining',
  'recompute-from-journal',
  'manual-required',
] as const;
export const GeneratedPolicyPreviewOriginValues = ['parent-preview', 'assistant-preview'] as const;
export const GeneratedPolicyPreviewBudgetBoundaryStateValues = [
  'within-budget',
  'bonus-time-active',
  'bonus-time-expiring',
  'manual-required',
  'expired',
] as const;
export const GeneratedPolicyAuthoritySourceValues = [
  'parent-policy',
  'local-ai-result',
  'tracking-signal',
  'activity-evidence',
] as const;
export const GeneratedPolicyAuthorityStateValues = ['authorized', 'evidence-only', 'dry-run'] as const;
export const GeneratedPolicyApprovalOriginValues = ['child-request', 'assistant-draft'] as const;
export const GeneratedPolicyApprovalKindValues = ['ask-parent', 'temporary-override', 'bonus-time'] as const;
export const GeneratedPolicyApprovalStateValues = [
  'pending',
  'approved',
  'denied',
  'modified',
  'expired-request',
  'replay-rejected',
  'preview-only',
] as const;
export const GeneratedPolicyOverrideTypeValues = ['temporary-allow', 'temporary-block', 'bonus-time'] as const;
export const GeneratedPolicyOverrideStateValues = ['active', 'expired', 'revoked'] as const;
export const GeneratedPolicyCompilerDomainValues = [
  'app-game',
  'browser',
  'network',
  'tracking',
  'screen',
  'ai',
  'enforcement',
  'notification-ask-parent',
] as const;
export const GeneratedPolicyCompilerRuleStatusValues = ['ready', 'manual-required', 'unsupported'] as const;
export const GeneratedPolicyCompilerCapabilityStateValues = ['supported', 'manual-required', 'unsupported'] as const;
export const GeneratedPolicyCompilerSourceStatusValues = [
  'draft',
  'preview',
  'confirmed',
  'queued',
  'delivered',
  'acknowledged',
  'active',
  'partially-active',
  'rejected',
  'superseded',
  'rolled-back',
  'stale',
  'expired',
  'manual-required',
] as const;
export const GeneratedPolicyCompilerTargetKindValues = [
  'child-profile',
  'device',
  'app',
  'site',
  'category',
  'resource',
] as const;
export const GeneratedPolicyCompilerNoClaimLabelValues = [
  'compiled-artifact-not-source-truth',
  'runtime-mutation-not-claimed',
  'enforcement-not-claimed',
  'ui-delivery-not-claimed',
  'platform-support-not-claimed',
] as const;
export const GeneratedAppGameCategoryRiskPolicyRouteFamilyValues = [
  'nativeApp',
  'nativeGame',
  'riskCandidate',
  'gameContext',
] as const;
export const GeneratedAppGameCategoryRiskPolicyRouteSourceKindValues = [
  'catalog',
  'storeMetadata',
  'launcherManifest',
  'parentLabel',
  'localAi',
  'processMetadata',
  'executableName',
  'managedDevice',
  'manualReview',
] as const;
export const GeneratedAppGameCategoryRiskPolicyCandidateActionValues = [
  'observe',
  'warn',
  'askParent',
  'manualReview',
] as const;
export const GeneratedAppGameCategoryRiskPolicyRoutingStateValues = ['compile-ready', 'manual-required'] as const;

export type GeneratedPolicyAction = (typeof GeneratedPolicyActionValues)[number];
export type GeneratedPolicyTargetType = (typeof GeneratedPolicyTargetTypeValues)[number];
export type GeneratedPolicyScheduleDay = (typeof GeneratedPolicyScheduleDayValues)[number];
export type GeneratedPolicyDecisionHandoffState = (typeof GeneratedPolicyDecisionHandoffStateValues)[number];
export type GeneratedPermissionRequestState = (typeof GeneratedPermissionRequestStateValues)[number];
export type GeneratedPolicyPreviewConfirmationState = (typeof GeneratedPolicyPreviewConfirmationStateValues)[number];
export type GeneratedPolicyScheduleBoundaryState = (typeof GeneratedPolicyScheduleBoundaryStateValues)[number];
export type GeneratedPolicyScheduleDstTransition = (typeof GeneratedPolicyScheduleDstTransitionValues)[number];
export type GeneratedPolicyScheduleDstResolution = (typeof GeneratedPolicyScheduleDstResolutionValues)[number];
export type GeneratedPolicyScheduleClockSource = (typeof GeneratedPolicyScheduleClockSourceValues)[number];
export type GeneratedPolicyScheduleOfflineRecoveryState =
  (typeof GeneratedPolicyScheduleOfflineRecoveryStateValues)[number];
export type GeneratedPolicyScheduleBudgetResetKind = (typeof GeneratedPolicyScheduleBudgetResetKindValues)[number];
export type GeneratedPolicyScheduleBudgetCarryoverMode =
  (typeof GeneratedPolicyScheduleBudgetCarryoverModeValues)[number];
export type GeneratedPolicyScheduleOfflineRecovery = (typeof GeneratedPolicyScheduleOfflineRecoveryValues)[number];
export type GeneratedPolicyPreviewOrigin = (typeof GeneratedPolicyPreviewOriginValues)[number];
export type GeneratedPolicyPreviewBudgetBoundaryState =
  (typeof GeneratedPolicyPreviewBudgetBoundaryStateValues)[number];
export type GeneratedPolicyAuthoritySource = (typeof GeneratedPolicyAuthoritySourceValues)[number];
export type GeneratedPolicyAuthorityState = (typeof GeneratedPolicyAuthorityStateValues)[number];
export type GeneratedPolicyApprovalOrigin = (typeof GeneratedPolicyApprovalOriginValues)[number];
export type GeneratedPolicyApprovalKind = (typeof GeneratedPolicyApprovalKindValues)[number];
export type GeneratedPolicyApprovalState = (typeof GeneratedPolicyApprovalStateValues)[number];
export type GeneratedPolicyOverrideType = (typeof GeneratedPolicyOverrideTypeValues)[number];
export type GeneratedPolicyOverrideState = (typeof GeneratedPolicyOverrideStateValues)[number];
export type GeneratedPolicyCompilerDomain = (typeof GeneratedPolicyCompilerDomainValues)[number];
export type GeneratedPolicyCompilerRuleStatus = (typeof GeneratedPolicyCompilerRuleStatusValues)[number];
export type GeneratedPolicyCompilerCapabilityState = (typeof GeneratedPolicyCompilerCapabilityStateValues)[number];
export type GeneratedPolicyCompilerSourceStatus = (typeof GeneratedPolicyCompilerSourceStatusValues)[number];
export type GeneratedPolicyCompilerTargetKind = (typeof GeneratedPolicyCompilerTargetKindValues)[number];
export type GeneratedPolicyCompilerNoClaimLabel = (typeof GeneratedPolicyCompilerNoClaimLabelValues)[number];
export type GeneratedAppGameCategoryRiskPolicyRouteFamily =
  (typeof GeneratedAppGameCategoryRiskPolicyRouteFamilyValues)[number];
export type GeneratedAppGameCategoryRiskPolicyRouteSourceKind =
  (typeof GeneratedAppGameCategoryRiskPolicyRouteSourceKindValues)[number];
export type GeneratedAppGameCategoryRiskPolicyCandidateAction =
  (typeof GeneratedAppGameCategoryRiskPolicyCandidateActionValues)[number];
export type GeneratedAppGameCategoryRiskPolicyRoutingState =
  (typeof GeneratedAppGameCategoryRiskPolicyRoutingStateValues)[number];

export const GeneratedPolicyActionStrictnessRank = {
  allow: 0,
  warn: 10,
  unknown: 20,
  'ask-parent': 30,
  'time-limit': 40,
  block: 50,
} as const satisfies Readonly<Record<GeneratedPolicyAction, number>>;
