import type {
  BrowserAiParentExplanationSection,
  BrowserAiParentExplanationState,
} from '@ocentra-parent/schema-domain/browser-ai-parent-explanation-values';
import { BrowserAiParentExplanationTextToken } from '@ocentra-parent/schema-domain/browser-ai-parent-explanation-values';
import { includesAll } from './browser-ai-schema-shared';

type ParentExplanationShape = {
  readonly state: BrowserAiParentExplanationState;
  readonly sections: readonly BrowserAiParentExplanationSection[];
  readonly sourceEvidenceIds: readonly unknown[];
  readonly aiAnalysis: {
    readonly degradedState: string;
    readonly sourceEvidenceIds: readonly unknown[];
  };
  readonly policyDecision: {
    readonly fallbackUsed: boolean;
    readonly evaluatorMode: string;
    readonly outcome: string;
    readonly sourceEvidenceIds: readonly unknown[];
  };
  readonly postAnalysisActionPlan: {
    readonly actionPlanId: string;
    readonly childAlreadyEngaged: boolean;
    readonly sourceEvidenceIds: readonly unknown[];
  };
  readonly childUxSnapshot: {
    readonly state: string;
    readonly postAnalysisActionPlan: { readonly actionPlanId: string } | null;
  };
  readonly evidenceVisible: boolean;
  readonly modelRuntimeVisible: boolean;
  readonly promptVersionVisible: boolean;
  readonly policyRuleVisible: boolean;
  readonly actionVisible: boolean;
  readonly childExperienceVisible: boolean;
  readonly childSawPageVisible: boolean;
  readonly degradedStateVisible: boolean;
  readonly manualFallbackVisible: boolean;
  readonly auditTrailVisible: boolean;
  readonly rawPageContentIncluded: boolean;
  readonly rawPromptTextIncluded: boolean;
  readonly portalEvaluatedClaimed: boolean;
  readonly policyAuthorityClaimed: boolean;
  readonly directEnforcementClaimed: boolean;
};

const RequiredSectionGroups = [
  ['summary', 'evidence', 'ai-analysis'],
  ['policy-decision', 'action-taken', 'audit'],
] as const satisfies ReadonlyArray<readonly BrowserAiParentExplanationSection[]>;

export function browserAiParentExplanationBundleIsConsistent(value: ParentExplanationShape) {
  return (
    !claimsAuthorityOrRawContent(value) &&
    RequiredSectionGroups.every((sectionGroup) => includesAll(value.sections, sectionGroup)) &&
    requiredVisibilityIsPresent(value) &&
    degradedAndManualFallbackVisibilityIsHonest(value) &&
    childExperienceVisibilityIsHonest(value) &&
    linkedRecordsShareEvidenceAndAction(value) &&
    browserAiParentExplanationStateIsReady(value.state, value)
  );
}

export function browserAiParentExplanationStateIsReady(
  state: BrowserAiParentExplanationState,
  value: ParentExplanationShape
) {
  const stateRules = {
    ready: value.aiAnalysis.degradedState === 'none' && !value.policyDecision.fallbackUsed,
    preview: value.policyDecision.evaluatorMode === 'dry_run',
    degraded: value.degradedStateVisible,
    manual_required: value.manualFallbackVisible,
    unavailable: value.degradedStateVisible && value.manualFallbackVisible,
  } as const satisfies Record<BrowserAiParentExplanationState, boolean>;
  return stateRules[state];
}

export function browserAiParentExplanationPrimaryTokenForState(state: BrowserAiParentExplanationState) {
  return state === 'ready' || state === 'preview'
    ? BrowserAiParentExplanationTextToken.Title
    : BrowserAiParentExplanationTextToken.Degraded;
}

function claimsAuthorityOrRawContent(value: ParentExplanationShape) {
  return (
    value.rawPageContentIncluded ||
    value.rawPromptTextIncluded ||
    value.portalEvaluatedClaimed ||
    value.policyAuthorityClaimed ||
    value.directEnforcementClaimed
  );
}

function requiredVisibilityIsPresent(value: ParentExplanationShape) {
  return (
    value.evidenceVisible &&
    value.modelRuntimeVisible &&
    value.promptVersionVisible &&
    value.policyRuleVisible &&
    value.actionVisible &&
    value.childExperienceVisible &&
    value.auditTrailVisible
  );
}

function degradedAndManualFallbackVisibilityIsHonest(value: ParentExplanationShape) {
  const hasDegradedState =
    value.aiAnalysis.degradedState !== 'none' ||
    value.policyDecision.fallbackUsed ||
    value.childUxSnapshot.state === 'unavailable';
  const hasManualState =
    value.policyDecision.outcome === 'unknown' ||
    value.childUxSnapshot.state === 'manual_required' ||
    value.childUxSnapshot.state === 'unavailable';
  return (!hasDegradedState || value.degradedStateVisible) && (!hasManualState || value.manualFallbackVisible);
}

function childExperienceVisibilityIsHonest(value: ParentExplanationShape) {
  return !value.postAnalysisActionPlan.childAlreadyEngaged || value.childSawPageVisible;
}

function linkedRecordsShareEvidenceAndAction(value: ParentExplanationShape) {
  return (
    includesAll(value.sourceEvidenceIds, value.aiAnalysis.sourceEvidenceIds) &&
    includesAll(value.sourceEvidenceIds, value.policyDecision.sourceEvidenceIds) &&
    includesAll(value.sourceEvidenceIds, value.postAnalysisActionPlan.sourceEvidenceIds) &&
    (value.childUxSnapshot.postAnalysisActionPlan === null ||
      value.childUxSnapshot.postAnalysisActionPlan.actionPlanId === value.postAnalysisActionPlan.actionPlanId)
  );
}
