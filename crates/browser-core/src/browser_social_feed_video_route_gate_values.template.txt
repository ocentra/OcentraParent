/* generated from crates/browser-core/src/browser_generated_social_ts.rs */

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema } from '@ocentra-parent/schema-domain/evidence-primitives';

export const OptionalSocialFeedVideoRouteGateTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const SocialFeedVideoRouteGateSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social feed video route gate source evidence ids')
);

export const BrowserSocialFeedVideoRouteGateSchemaVersion = 1;

export const BrowserSocialFeedVideoRouteGatePlanIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialFeedVideoRouteGatePlanId')
);

export const BrowserSocialFeedVideoRouteGateTargetKindSchema = withParser(
  Schema.Literal('social-feed-route', 'social-short-video-route', 'social-video-route', 'manual-required')
);

export const BrowserSocialFeedVideoRouteGateActionSchema = withParser(
  Schema.Literal(
    'allow-route-candidate',
    'warn-route-candidate',
    'parent-review-candidate',
    'block-route-candidate',
    'limit-route-candidate',
    'manual-review-required',
    'unknown-route-warn-only'
  )
);

export const BrowserSocialFeedVideoRouteGateStateSchema = withParser(
  Schema.Literal('planned', 'manual-required', 'unavailable')
);

export const BrowserSocialFeedVideoRouteGateReasonSchema = withParser(
  Schema.Literal(
    'dynamic-feed-route',
    'short-video-route',
    'single-video-route',
    'metadata-available',
    'metadata-partial',
    'parent-policy-match',
    'schedule-limit-candidate',
    'policy-block-candidate',
    'parent-review-required',
    'manual-required',
    'unknown-evidence'
  )
);

export const BrowserSocialFeedVideoRouteGateReasonsSchema = Schema.Array(
  BrowserSocialFeedVideoRouteGateReasonSchema
).pipe(Schema.filter((value) => value.length > 0 || 'Expected social feed video route gate reasons'));

export type BrowserSocialFeedVideoRouteGateAction = Infer<typeof BrowserSocialFeedVideoRouteGateActionSchema>;
export type BrowserSocialFeedVideoRouteGateReason = Infer<typeof BrowserSocialFeedVideoRouteGateReasonSchema>;
export type BrowserSocialFeedVideoRouteGateState = Infer<typeof BrowserSocialFeedVideoRouteGateStateSchema>;
export type BrowserSocialFeedVideoRouteGateTargetKind = Infer<typeof BrowserSocialFeedVideoRouteGateTargetKindSchema>;
