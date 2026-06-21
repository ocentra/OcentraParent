import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const BrowserGamePortalPatternLibrarySchemaVersionSchema = withParser(
  Schema.Literal('browser-game-portal-pattern-library-contract')
);

export const BrowserGamePortalPatternLibraryIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGamePortalPatternLibraryId')
);

export const BrowserGamePortalPatternIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGamePortalPatternId')
);

export const BrowserGamePortalPatternFingerprintSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGamePortalPatternFingerprint')
);

export const BrowserGamePortalFamilySchema = withParser(
  Schema.Literal(
    'known-game-portal',
    'educational-game-portal',
    'ugc-game-platform',
    'indie-game-marketplace',
    'classic-game-archive',
    'school-game-portal',
    'unknown'
  )
);

export const BrowserGamePortalRouteKindSchema = withParser(
  Schema.Literal(
    'home-route',
    'catalog-route',
    'game-detail-route',
    'play-route',
    'embed-route',
    'account-route',
    'purchase-route',
    'download-route',
    'unknown-route'
  )
);

export const BrowserGamePortalSignalKindSchema = withParser(
  Schema.Literal(
    'domain-ref',
    'path-pattern-ref',
    'game-id-segment',
    'catalog-grid',
    'iframe-embed',
    'account-prompt',
    'purchase-prompt',
    'launcher-download',
    'unknown-signal'
  )
);

export const BrowserGamePortalPatternReviewStateSchema = withParser(
  Schema.Literal('reviewed', 'parent-review-required', 'manual-required', 'unavailable')
);

export const BrowserGamePortalPatternConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGamePortalPatternEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game portal pattern evidence refs')
);

export const BrowserGamePortalRouteKindsSchema = Schema.Array(BrowserGamePortalRouteKindSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game portal route kinds')
);

export const BrowserGamePortalSignalKindsSchema = Schema.Array(BrowserGamePortalSignalKindSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game portal signal kinds')
);

export type BrowserGamePortalFamily = Infer<typeof BrowserGamePortalFamilySchema>;
export type BrowserGamePortalPatternConfidence = Infer<typeof BrowserGamePortalPatternConfidenceSchema>;
export type BrowserGamePortalPatternReviewState = Infer<typeof BrowserGamePortalPatternReviewStateSchema>;
export type BrowserGamePortalRouteKind = Infer<typeof BrowserGamePortalRouteKindSchema>;
export type BrowserGamePortalSignalKind = Infer<typeof BrowserGamePortalSignalKindSchema>;

