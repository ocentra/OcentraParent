import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from './effect';
import { ParentEvidenceReferenceIdSchema } from './family-reference-primitives';

export const BrowserGamePolicyEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser game policy evidence refs')
);
export const BrowserGamePolicyAnalysisRefsSchema = Schema.Array(NonEmptyStringSchema);
export const BrowserGamePolicyMobileCapabilityRefsSchema = Schema.Array(NonEmptyStringSchema);
export const BrowserGamePolicyParentRuleRefsSchema = Schema.Array(NonEmptyStringSchema);
export const BrowserGamePolicyScheduleRefsSchema = Schema.Array(NonEmptyStringSchema);

export const BrowserGamePolicyCompileRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGamePolicyCompileRequestId')
);
export const BrowserGamePolicyDecisionCandidateIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGamePolicyDecisionCandidateId')
);

export const BrowserGamePolicyTargetKindSchema = withParser(
  Schema.Literal(
    'browser-game-url',
    'game-portal',
    'cloud-gaming-session',
    'game-account-signup',
    'game-login',
    'game-purchase',
    'unblocked-game-site',
    'ugc-multiplayer-game',
    'educational-game',
    'unknown-game',
    'manual-required'
  )
);

export const BrowserGamePolicyActionCandidateSchema = withParser(
  Schema.Literal(
    'allow-candidate',
    'warn-candidate',
    'parent-review-candidate',
    'block-candidate',
    'time-limit-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);

export const BrowserGamePolicyReasonCodeSchema = withParser(
  Schema.Literal(
    'parent-rule-match',
    'browser-game-risk-high',
    'educational-benefit-present',
    'cloud-gaming-risk',
    'purchase-risk',
    'account-required-risk',
    'ugc-chat-risk',
    'unblocked-game-site-risk',
    'low-confidence',
    'manual-required',
    'missing-game-evidence',
    'degraded-analysis',
    'schedule-context',
    'unknown-evidence',
    'mobile-capability-manual-required'
  )
);

export const BrowserGamePolicyCompilerModeSchema = withParser(
  Schema.Literal('contract-only', 'manual-required', 'unavailable')
);
export const BrowserGamePolicyConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGamePolicyReasonCodesSchema = Schema.Array(BrowserGamePolicyReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser game policy reason codes')
);
