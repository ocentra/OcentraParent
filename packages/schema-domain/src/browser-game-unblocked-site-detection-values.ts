import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const BrowserGameUnblockedSiteDetectionSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-unblocked-site-detection-contract')
);

export const BrowserGameUnblockedSiteDetectionIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameUnblockedSiteDetectionId')
);

export const BrowserGameUnblockedSiteSignalIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameUnblockedSiteSignalId')
);

export const BrowserGameUnblockedSiteSurfaceKindSchema = withParser(
  Schema.Literal(
    'managed-browser-route',
    'managed-browser-page',
    'search-intent',
    'portal-index',
    'iframe-embed',
    'unmanaged-browser-bypass',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameUnblockedSiteSignalKindSchema = withParser(
  Schema.Literal(
    'unblocked-domain-keyword',
    'game-portal-index',
    'proxy-or-mirror-route',
    'external-game-iframe',
    'hidden-game-origin',
    'search-query-intent',
    'school-bypass-language',
    'managed-browser-game-proof',
    'unmanaged-browser-process-only',
    'unknown-signal'
  )
);

export const BrowserGameUnblockedSiteClassificationKindSchema = withParser(
  Schema.Literal(
    'unblocked-game-site',
    'game-portal-bypass',
    'hidden-origin-game-embed',
    'unmanaged-browser-game-bypass',
    'unknown-game-portal',
    'unknown'
  )
);

export const BrowserGameUnblockedSiteDetectionStateSchema = withParser(
  Schema.Literal('candidate', 'manual-required', 'unavailable')
);

export const BrowserGameUnblockedSiteConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameUnblockedSiteActionCandidateSchema = withParser(
  Schema.Literal(
    'block-during-school-candidate',
    'parent-review-candidate',
    'allow-specific-game-candidate',
    'block-unknown-iframe-candidate',
    'bypass-evidence-only-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);

export const BrowserGameUnblockedSiteReasonCodeSchema = withParser(
  Schema.Literal(
    'domain-keyword-match',
    'portal-index-detected',
    'proxy-or-mirror-route',
    'external-game-iframe',
    'hidden-game-origin',
    'school-bypass-portal',
    'search-intent-unblocked-games',
    'unmanaged-browser-process-only',
    'missing-managed-route-proof',
    'manual-required',
    'unavailable-proof'
  )
);

export const BrowserGameUnblockedSiteReasonCodesSchema = Schema.Array(BrowserGameUnblockedSiteReasonCodeSchema);

export const BrowserGameUnblockedSiteEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game unblocked-site evidence refs')
);

export type BrowserGameUnblockedSiteActionCandidate = Infer<typeof BrowserGameUnblockedSiteActionCandidateSchema>;
export type BrowserGameUnblockedSiteClassificationKind = Infer<typeof BrowserGameUnblockedSiteClassificationKindSchema>;
export type BrowserGameUnblockedSiteConfidence = Infer<typeof BrowserGameUnblockedSiteConfidenceSchema>;
export type BrowserGameUnblockedSiteDetectionState = Infer<typeof BrowserGameUnblockedSiteDetectionStateSchema>;
export type BrowserGameUnblockedSiteReasonCode = Infer<typeof BrowserGameUnblockedSiteReasonCodeSchema>;
export type BrowserGameUnblockedSiteSignalKind = Infer<typeof BrowserGameUnblockedSiteSignalKindSchema>;
export type BrowserGameUnblockedSiteSurfaceKind = Infer<typeof BrowserGameUnblockedSiteSurfaceKindSchema>;
