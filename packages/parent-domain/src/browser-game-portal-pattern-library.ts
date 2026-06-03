import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  BrowserGamePortalFamilySchema,
  BrowserGamePortalPatternConfidenceSchema,
  BrowserGamePortalPatternEvidenceRefsSchema,
  BrowserGamePortalPatternFingerprintSchema,
  BrowserGamePortalPatternIdSchema,
  BrowserGamePortalPatternLibraryIdSchema,
  BrowserGamePortalPatternLibrarySchemaVersionSchema,
  BrowserGamePortalPatternReviewStateSchema,
  BrowserGamePortalRouteKindsSchema,
  BrowserGamePortalSignalKindsSchema,
} from './browser-game-portal-pattern-library-values';

const BrowserGamePortalPatternEntryBaseSchema = Schema.Struct({
  patternId: BrowserGamePortalPatternIdSchema,
  portalFamily: BrowserGamePortalFamilySchema,
  routeKinds: BrowserGamePortalRouteKindsSchema,
  signalKinds: BrowserGamePortalSignalKindsSchema,
  patternFingerprint: BrowserGamePortalPatternFingerprintSchema,
  sourceEvidenceRefs: BrowserGamePortalPatternEvidenceRefsSchema,
  confidence: BrowserGamePortalPatternConfidenceSchema,
  reviewState: BrowserGamePortalPatternReviewStateSchema,
  educationalCandidate: Schema.Boolean,
  ugcCandidate: Schema.Boolean,
  purchaseFlowCandidate: Schema.Boolean,
  cloudGamingCandidate: Schema.Boolean,
  rawDomainStored: Schema.Boolean,
  rawUrlStored: Schema.Boolean,
  rawPageTitleStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  runtimeDetectionClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGamePortalPatternEntryCandidate = Infer<typeof BrowserGamePortalPatternEntryBaseSchema>;

export const BrowserGamePortalPatternEntrySchema = withParser(
  BrowserGamePortalPatternEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        browserGamePortalPatternEntryIsHonest(entry) ||
        'Expected browser-game portal pattern entry to stay ref/hash backed'
    )
  )
);

const BrowserGamePortalPatternEntriesSchema = Schema.Array(BrowserGamePortalPatternEntrySchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game portal pattern entries')
);

const BrowserGamePortalPatternLibraryBaseSchema = Schema.Struct({
  schemaVersion: BrowserGamePortalPatternLibrarySchemaVersionSchema,
  libraryId: BrowserGamePortalPatternLibraryIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGamePortalPatternEvidenceRefsSchema,
  patterns: BrowserGamePortalPatternEntriesSchema,
  confidence: BrowserGamePortalPatternConfidenceSchema,
  reviewState: BrowserGamePortalPatternReviewStateSchema,
  rawDomainStored: Schema.Boolean,
  rawUrlStored: Schema.Boolean,
  rawPageTitleStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  runtimeDetectionClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGamePortalPatternLibraryCandidate = Infer<typeof BrowserGamePortalPatternLibraryBaseSchema>;

export const BrowserGamePortalPatternLibrarySchema = withParser(
  BrowserGamePortalPatternLibraryBaseSchema.pipe(
    Schema.filter(
      (library) =>
        browserGamePortalPatternLibraryIsHonest(library) ||
        'Expected browser-game portal pattern library to stay contract-only'
    )
  )
);

export const decodeBrowserGamePortalPatternLibrary = Schema.decodeUnknownSync(BrowserGamePortalPatternLibrarySchema);

export type BrowserGamePortalPatternEntry = Infer<typeof BrowserGamePortalPatternEntrySchema>;
export type BrowserGamePortalPatternLibrary = Infer<typeof BrowserGamePortalPatternLibrarySchema>;

function browserGamePortalPatternEntryIsHonest(entry: BrowserGamePortalPatternEntryCandidate): boolean {
  if (browserGamePortalPatternEntryClaimsAuthority(entry) || entry.cloudGamingCandidate) {
    return false;
  }
  if (entry.reviewState === 'reviewed') {
    return (
      entry.confidence !== 'unknown' &&
      entry.portalFamily !== 'unknown' &&
      !entry.routeKinds.includes('unknown-route') &&
      !entry.signalKinds.includes('unknown-signal')
    );
  }
  return (
    entry.confidence !== 'high' &&
    (entry.portalFamily === 'unknown' ||
      entry.routeKinds.includes('unknown-route') ||
      entry.signalKinds.includes('unknown-signal'))
  );
}

function browserGamePortalPatternLibraryIsHonest(library: BrowserGamePortalPatternLibraryCandidate): boolean {
  if (browserGamePortalPatternLibraryClaimsAuthority(library)) {
    return false;
  }
  if (library.reviewState === 'reviewed') {
    return library.confidence !== 'unknown' && library.patterns.every((entry) => entry.reviewState === 'reviewed');
  }
  return library.confidence !== 'high' && library.patterns.some((entry) => entry.reviewState !== 'reviewed');
}

function browserGamePortalPatternEntryClaimsAuthority(entry: BrowserGamePortalPatternEntryCandidate): boolean {
  return (
    entry.rawDomainStored ||
    entry.rawUrlStored ||
    entry.rawPageTitleStored ||
    entry.rawPageBodyStored ||
    entry.runtimeDetectionClaimed ||
    entry.aiClassificationClaimed ||
    entry.policyDecisionClaimed ||
    entry.enforcementClaimed
  );
}

function browserGamePortalPatternLibraryClaimsAuthority(library: BrowserGamePortalPatternLibraryCandidate): boolean {
  return (
    library.rawDomainStored ||
    library.rawUrlStored ||
    library.rawPageTitleStored ||
    library.rawPageBodyStored ||
    library.runtimeDetectionClaimed ||
    library.aiClassificationClaimed ||
    library.policyDecisionClaimed ||
    library.enforcementClaimed
  );
}
