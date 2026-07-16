/* generated from crates/browser-core/src/social_schema_generated_values.rs */
import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const SocialDashboardUxSchemaVersionSchema = withParser(Schema.Literal('social-dashboard-ux-contract'));

export const SocialDashboardPanelIdSchema = withParser(brandedNonEmptyStringSchema('SocialDashboardPanelId'));

export const SocialDashboardPanelKindSchema = withParser(
  Schema.Literal(
    'account-approval-queue',
    'feed-video-gates',
    'native-app-capability',
    'connector-boundaries',
    'decision-memory',
    'settings-custody',
    'manual-required-gaps'
  )
);

export const SocialDashboardPanelStatusSchema = withParser(
  Schema.Literal('ready-for-review', 'manual-required', 'contract-only', 'unavailable')
);

export const SocialDashboardPanelActionSchema = withParser(
  Schema.Literal(
    'open-parent-approval',
    'review-feed-gate',
    'review-native-capability',
    'review-connector-boundary',
    'review-memory-entry',
    'review-settings-custody',
    'manual-review'
  )
);

export const SocialDashboardPanelSeveritySchema = withParser(Schema.Literal('info', 'warning', 'critical'));

export const SocialDashboardPanelReasonSchema = withParser(
  Schema.Literal(
    'parent-review-needed',
    'feed-video-gate-candidate',
    'native-app-manual-required',
    'connector-boundary-manual-required',
    'memory-contract-only',
    'settings-custody-runtime-gap',
    'platform-proof-gap'
  )
);

export type SocialDashboardPanelKind = Infer<typeof SocialDashboardPanelKindSchema>;
export type SocialDashboardPanelStatus = Infer<typeof SocialDashboardPanelStatusSchema>;
