import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const BrowserGameDashboardUxSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-dashboard-ux-contract')
);

export const BrowserGameDashboardPanelIdSchema = withParser(brandedNonEmptyStringSchema('BrowserGameDashboardPanelId'));

export const BrowserGameDashboardPanelKindSchema = withParser(
  Schema.Literal(
    'detected-game-review',
    'unknown-game-approval-queue',
    'cloud-gaming-approval',
    'educational-game-allowlist',
    'game-time-budget-candidates',
    'mobile-native-capability-gaps',
    'manual-required-gaps'
  )
);

export const BrowserGameDashboardPanelStatusSchema = withParser(
  Schema.Literal('ready-for-review', 'manual-required', 'contract-only', 'unavailable')
);

export const BrowserGameDashboardPanelActionSchema = withParser(
  Schema.Literal(
    'review-detected-game',
    'open-parent-approval',
    'review-cloud-gaming',
    'review-educational-allowlist',
    'review-time-budget',
    'review-mobile-capability',
    'manual-review'
  )
);

export const BrowserGameDashboardPanelSeveritySchema = withParser(Schema.Literal('info', 'warning', 'critical'));

export const BrowserGameDashboardPanelReasonSchema = withParser(
  Schema.Literal(
    'detected-game-evidence-ready',
    'unknown-game-parent-review-needed',
    'cloud-gaming-manual-required',
    'educational-allowlist-contract-only',
    'time-budget-candidate-only',
    'mobile-native-proof-gap',
    'platform-proof-gap'
  )
);

export type BrowserGameDashboardPanelKind = Infer<typeof BrowserGameDashboardPanelKindSchema>;
export type BrowserGameDashboardPanelStatus = Infer<typeof BrowserGameDashboardPanelStatusSchema>;
