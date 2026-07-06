/* generated from crates/browser-core/src/social_schema_generated_values.rs */
import { Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { ParentEvidenceReferenceIdSchema } from './family-reference-primitives';

export const SocialPolicyEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy evidence refs')
);
export const SocialPolicySignalSetRefsSchema = Schema.Array(NonEmptyStringSchema);
export const SocialPolicyParentRuleRefsSchema = Schema.Array(NonEmptyStringSchema);
export const SocialPolicyScheduleRefsSchema = Schema.Array(NonEmptyStringSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy schedule refs')
);
export const SocialPolicyTimeBudgetRefsSchema = Schema.Array(NonEmptyStringSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy time budget refs')
);

export const SocialParentPolicyCompileRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialParentPolicyCompileRequestId')
);
export const SocialParentPolicyDecisionCandidateIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialParentPolicyDecisionCandidateId')
);

export const SocialParentPolicyTargetKindSchema = withParser(
  Schema.Literal(
    'social-account-signup',
    'social-login',
    'social-account-switch',
    'social-feed',
    'social-short-video',
    'social-video',
    'social-messaging',
    'social-upload-post',
    'social-livestream',
    'social-bypass',
    'manual-required'
  )
);
export const SocialParentPolicyActionCandidateSchema = withParser(
  Schema.Literal(
    'allow-candidate',
    'warn-candidate',
    'parent-review-candidate',
    'block-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);
export const SocialParentPolicyReasonCodeSchema = withParser(
  Schema.Literal(
    'parent-rule-match',
    'social-risk-high',
    'social-benefit-present',
    'secondary-account-risk',
    'feed-risk',
    'messaging-risk',
    'video-safety-risk',
    'low-confidence',
    'manual-required',
    'missing-signal-proof',
    'degraded-analysis',
    'schedule-context',
    'unknown-evidence'
  )
);
export const SocialParentPolicyCompilerModeSchema = withParser(
  Schema.Literal('contract-only', 'manual-required', 'unavailable')
);
export const SocialParentPolicyConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));
export const SocialParentPolicyScheduleStateSchema = withParser(
  Schema.Literal('inside-allowed-window', 'outside-allowed-window', 'manual-required', 'unavailable')
);
export const SocialParentPolicyTimeBudgetStateSchema = withParser(
  Schema.Literal('budget-available', 'budget-low', 'budget-exhausted', 'manual-required', 'unavailable')
);

export const SocialParentPolicyReasonCodesSchema = Schema.Array(SocialParentPolicyReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy reason codes')
);
