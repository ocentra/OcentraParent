import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const BrowserGameUgcRiskSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-ugc-multiplayer-chat-risk-contract')
);

export const BrowserGameUgcRiskAssessmentIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameUgcRiskAssessmentId')
);

export const BrowserGameUgcRiskRowIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameUgcRiskRowId')
);

export const BrowserGameUgcPlatformSurfaceKindSchema = withParser(
  Schema.Literal(
    'ugc-game-page',
    'experience-page',
    'multiplayer-lobby',
    'profile-friends-messages',
    'launch-prompt',
    'web-to-app-launch',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameUgcRiskKindSchema = withParser(
  Schema.Literal(
    'unknown-player-contact',
    'chat-contact',
    'voice-contact',
    'ugc-world',
    'unsafe-user-created-experience',
    'off-platform-contact',
    'virtual-currency',
    'in-game-purchase',
    'age-rating-missing',
    'web-to-app-launch-risk',
    'manual-required',
    'unknown-risk'
  )
);

export const BrowserGameUgcRiskEvidenceKindSchema = withParser(
  Schema.Literal(
    'managed-route',
    'platform-metadata',
    'parent-rule',
    'approved-experience',
    'chat-control-capability',
    'purchase-control-capability',
    'mobile-capability',
    'public-risk-context',
    'manual-required'
  )
);

export const BrowserGameUgcRiskSeveritySchema = withParser(Schema.Literal('low', 'medium', 'high', 'unknown'));
export const BrowserGameUgcRiskConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));
export const BrowserGameUgcRiskStateSchema = withParser(Schema.Literal('candidate', 'manual-required', 'unavailable'));

export const BrowserGameUgcRecommendedControlSchema = withParser(
  Schema.Literal(
    'approved-experience-only-candidate',
    'parent-review-candidate',
    'block-chat-candidate',
    'time-limit-candidate',
    'purchase-approval-candidate',
    'block-unknown-ugc-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);

export const BrowserGameUgcRiskDegradedStateSchema = withParser(
  Schema.Literal('none', 'degraded', 'manual-required', 'unavailable')
);

export const BrowserGameUgcRiskUncertaintyReasonSchema = withParser(
  Schema.Literal(
    'missing-managed-route',
    'missing-platform-metadata',
    'missing-capability-proof',
    'low-confidence',
    'conflicting-evidence',
    'manual-required'
  )
);

export const BrowserGameUgcRiskEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game UGC risk evidence refs')
);

export type BrowserGameUgcPlatformSurfaceKind = Infer<typeof BrowserGameUgcPlatformSurfaceKindSchema>;
export type BrowserGameUgcRecommendedControl = Infer<typeof BrowserGameUgcRecommendedControlSchema>;
export type BrowserGameUgcRiskConfidence = Infer<typeof BrowserGameUgcRiskConfidenceSchema>;
export type BrowserGameUgcRiskKind = Infer<typeof BrowserGameUgcRiskKindSchema>;
export type BrowserGameUgcRiskSeverity = Infer<typeof BrowserGameUgcRiskSeveritySchema>;

