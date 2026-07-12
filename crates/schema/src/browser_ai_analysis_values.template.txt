/* generated from crates/schema/src/browser_generated_values_ts.rs */

import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const BrowserAiConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));
export const BrowserAiDegradedStateSchema = withParser(
  Schema.Literal('none', 'degraded', 'manual-required', 'unavailable')
);
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
