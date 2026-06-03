import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from './reference-primitives';

const NonEmptySocialPolicyText = Schema.String.pipe(Schema.minLength(1));

export const SocialPolicyEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy evidence refs')
);
export const SocialPolicySignalSetRefsSchema = Schema.Array(NonEmptySocialPolicyText);
export const SocialPolicyParentRuleRefsSchema = Schema.Array(NonEmptySocialPolicyText);
export const SocialPolicyScheduleRefsSchema = Schema.Array(NonEmptySocialPolicyText);

export const SocialParentPolicyCompileRequestIdSchema = withParser(
  NonEmptySocialPolicyText.pipe(Schema.brand('SocialParentPolicyCompileRequestId'))
);
export const SocialParentPolicyDecisionCandidateIdSchema = withParser(
  NonEmptySocialPolicyText.pipe(Schema.brand('SocialParentPolicyDecisionCandidateId'))
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

export const SocialParentPolicyReasonCodesSchema = Schema.Array(SocialParentPolicyReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy reason codes')
);
