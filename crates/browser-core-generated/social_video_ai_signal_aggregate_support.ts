/* generated support for crates/browser-core/src/social_video_ai_signal_aggregate.rs */

import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const BrowserAiConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));
export const BrowserAiRecommendedPolicyInputSchema = withParser(
  Schema.Literal(
    'allow-candidate',
    'warn-candidate',
    'limit-candidate',
    'parent-review-candidate',
    'block-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);
export const BrowserAiDegradedStateSchema = withParser(
  Schema.Literal('none', 'degraded', 'manual-required', 'unavailable')
);
export const BrowserCustodyLabelSchema = withParser(
  Schema.Literal(
    'child-device-local',
    'local-network-child-agent',
    'parent-cache',
    'parent-owned-export',
    'unavailable'
  )
);
export const BrowserSocialAiAnalysisIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialAiAnalysisId')
);
export const BrowserSocialFeedVideoRouteGatePlanIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialFeedVideoRouteGatePlanId')
);
export const BrowserSocialRiskBenefitSignalSetIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialRiskBenefitSignalSetId')
);
export const BrowserSocialVideoMetadataEvidenceIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialVideoMetadataEvidenceId')
);
