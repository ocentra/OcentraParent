import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyBrowserGameCloudPatternText = Schema.String.pipe(Schema.minLength(1));

export const BrowserGameCloudPatternLibrarySchemaVersionSchema = withParser(
  Schema.Literal('browser-game-cloud-pattern-library-contract')
);

export const BrowserGameCloudPatternLibraryIdSchema = withParser(
  NonEmptyBrowserGameCloudPatternText.pipe(Schema.brand('BrowserGameCloudPatternLibraryId'))
);

export const BrowserGameCloudPatternIdSchema = withParser(
  NonEmptyBrowserGameCloudPatternText.pipe(Schema.brand('BrowserGameCloudPatternId'))
);

export const BrowserGameCloudPatternFingerprintSchema = withParser(
  NonEmptyBrowserGameCloudPatternText.pipe(Schema.brand('BrowserGameCloudPatternFingerprint'))
);

export const BrowserGameCloudPatternFamilySchema = withParser(
  Schema.Literal(
    'cloud-gaming-platform',
    'cloud-pc-platform',
    'mobile-cloud-game-portal',
    'browser-embedded-cloud-game',
    'cloud-launcher-bridge',
    'unknown'
  )
);

export const BrowserGameCloudPatternRouteKindSchema = withParser(
  Schema.Literal(
    'cloud-home-route',
    'cloud-catalog-route',
    'cloud-title-route',
    'cloud-session-route',
    'cloud-account-route',
    'cloud-subscription-route',
    'cloud-launch-route',
    'cloud-support-route',
    'unknown-route'
  )
);

export const BrowserGameCloudPatternSignalKindSchema = withParser(
  Schema.Literal(
    'domain-ref',
    'path-pattern-ref',
    'streaming-session-route',
    'gamepad-api',
    'fullscreen-pointer-lock',
    'high-bandwidth-stream',
    'low-latency-network',
    'platform-title-metadata-ref',
    'platform-rating-metadata-ref',
    'subscription-prompt',
    'native-launcher-prompt',
    'unknown-signal'
  )
);

export const BrowserGameCloudPatternReviewStateSchema = withParser(
  Schema.Literal('reviewed', 'parent-review-required', 'manual-required', 'unavailable')
);

export const BrowserGameCloudPatternConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameCloudPatternEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game cloud pattern evidence refs')
);

export const BrowserGameCloudPatternRouteKindsSchema = Schema.Array(BrowserGameCloudPatternRouteKindSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game cloud pattern route kinds')
);

export const BrowserGameCloudPatternSignalKindsSchema = Schema.Array(BrowserGameCloudPatternSignalKindSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game cloud pattern signal kinds')
);

export type BrowserGameCloudPatternConfidence = Infer<typeof BrowserGameCloudPatternConfidenceSchema>;
export type BrowserGameCloudPatternFamily = Infer<typeof BrowserGameCloudPatternFamilySchema>;
export type BrowserGameCloudPatternReviewState = Infer<typeof BrowserGameCloudPatternReviewStateSchema>;
export type BrowserGameCloudPatternRouteKind = Infer<typeof BrowserGameCloudPatternRouteKindSchema>;
export type BrowserGameCloudPatternSignalKind = Infer<typeof BrowserGameCloudPatternSignalKindSchema>;
