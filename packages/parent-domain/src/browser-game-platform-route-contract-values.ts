import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from './reference-primitives';

const NonEmptyBrowserGamePlatformRouteText = Schema.String.pipe(Schema.minLength(1));

export const BrowserGamePlatformRouteCatalogSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-platform-route-contract')
);

export const BrowserGamePlatformRouteCatalogIdSchema = withParser(
  NonEmptyBrowserGamePlatformRouteText.pipe(Schema.brand('BrowserGamePlatformRouteCatalogId'))
);

export const BrowserGamePlatformRouteContractIdSchema = withParser(
  NonEmptyBrowserGamePlatformRouteText.pipe(Schema.brand('BrowserGamePlatformRouteContractId'))
);

export const BrowserGameRoutePatternRefSchema = withParser(
  NonEmptyBrowserGamePlatformRouteText.pipe(Schema.brand('BrowserGameRoutePatternRef'))
);

export const BrowserGamePlatformKindSchema = withParser(
  Schema.Literal(
    'browser-game-portal',
    'educational-game-site',
    'ugc-game-platform',
    'cloud-gaming-platform',
    'cloud-pc-platform',
    'classic-game-archive',
    'school-game-platform',
    'unknown-platform'
  )
);

export const BrowserGameRouteSurfaceKindSchema = withParser(
  Schema.Literal(
    'home-route',
    'catalog-route',
    'game-detail-route',
    'play-route',
    'embed-route',
    'account-route',
    'purchase-route',
    'cloud-session-route',
    'download-route',
    'support-route',
    'unknown-route'
  )
);

export const BrowserGameRouteSourceKindSchema = withParser(
  Schema.Literal(
    'managed-browser-evidence-ref',
    'parent-curated-ref',
    'school-curated-ref',
    'platform-pattern-ref',
    'manual-review-ref',
    'unavailable'
  )
);

export const BrowserGameRouteCustodyLabelSchema = withParser(
  Schema.Literal('ref-only', 'hash-only', 'manual-required', 'unavailable')
);

export const BrowserGamePlatformRouteStatusSchema = withParser(
  Schema.Literal('reviewed', 'candidate', 'manual-required', 'unavailable')
);

export const BrowserGamePlatformRouteConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGamePlatformRouteEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game platform route evidence refs')
);

export type BrowserGamePlatformKind = Infer<typeof BrowserGamePlatformKindSchema>;
export type BrowserGamePlatformRouteConfidence = Infer<typeof BrowserGamePlatformRouteConfidenceSchema>;
export type BrowserGamePlatformRouteStatus = Infer<typeof BrowserGamePlatformRouteStatusSchema>;
export type BrowserGameRouteCustodyLabel = Infer<typeof BrowserGameRouteCustodyLabelSchema>;
export type BrowserGameRouteSourceKind = Infer<typeof BrowserGameRouteSourceKindSchema>;
export type BrowserGameRouteSurfaceKind = Infer<typeof BrowserGameRouteSurfaceKindSchema>;
