/* generated support for crates/browser-core/src/social_applied_schedule_time_budget_proof.rs */

import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';

export const SocialPolicyScheduleRefsSchema = Schema.Array(NonEmptyStringSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy schedule refs')
);
export const SocialPolicyTimeBudgetRefsSchema = Schema.Array(NonEmptyStringSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social policy time budget refs')
);

export const SocialParentPolicyDecisionCandidateIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialParentPolicyDecisionCandidateId')
);
export const SocialParentPolicyScheduleStateSchema = withParser(
  Schema.Literal('inside-allowed-window', 'outside-allowed-window', 'manual-required', 'unavailable')
);
export const SocialParentPolicyTimeBudgetStateSchema = withParser(
  Schema.Literal('budget-available', 'budget-low', 'budget-exhausted', 'manual-required', 'unavailable')
);

export const PolicyCompilerCapabilityState = {
  Supported: 'supported',
  ManualRequired: 'manual-required',
  Unsupported: 'unsupported',
} as const;
