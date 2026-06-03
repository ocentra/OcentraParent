import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { BrowserGameCloudPlatformSchema } from './browser-game-cloud-gaming-gate-values';
import { ParentTimestampSchema } from './reference-primitives';
import {
  BrowserGameCloudPatternConfidenceSchema,
  BrowserGameCloudPatternEvidenceRefsSchema,
  BrowserGameCloudPatternFamilySchema,
  BrowserGameCloudPatternFingerprintSchema,
  BrowserGameCloudPatternIdSchema,
  BrowserGameCloudPatternLibraryIdSchema,
  BrowserGameCloudPatternLibrarySchemaVersionSchema,
  BrowserGameCloudPatternReviewStateSchema,
  BrowserGameCloudPatternRouteKindsSchema,
  BrowserGameCloudPatternSignalKindsSchema,
} from './browser-game-cloud-pattern-library-values';

const BrowserGameCloudPatternEntryBaseSchema = Schema.Struct({
  patternId: BrowserGameCloudPatternIdSchema,
  platform: BrowserGameCloudPlatformSchema,
  cloudFamily: BrowserGameCloudPatternFamilySchema,
  routeKinds: BrowserGameCloudPatternRouteKindsSchema,
  signalKinds: BrowserGameCloudPatternSignalKindsSchema,
  patternFingerprint: BrowserGameCloudPatternFingerprintSchema,
  sourceEvidenceRefs: BrowserGameCloudPatternEvidenceRefsSchema,
  confidence: BrowserGameCloudPatternConfidenceSchema,
  reviewState: BrowserGameCloudPatternReviewStateSchema,
  sessionCandidate: Schema.Boolean,
  titleMetadataCandidate: Schema.Boolean,
  ratingMetadataCandidate: Schema.Boolean,
  subscriptionOrAccountCandidate: Schema.Boolean,
  nativeLauncherPromptCandidate: Schema.Boolean,
  rawCloudDomainStored: Schema.Boolean,
  rawCloudUrlStored: Schema.Boolean,
  rawCloudTitleStored: Schema.Boolean,
  rawStreamFrameStored: Schema.Boolean,
  runtimeDetectionClaimed: Schema.Boolean,
  cloudStreamFrameAnalysisClaimed: Schema.Boolean,
  perGameCloudTitleCertaintyClaimed: Schema.Boolean,
  nativeLauncherControlClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameCloudPatternEntryCandidate = Infer<typeof BrowserGameCloudPatternEntryBaseSchema>;

export const BrowserGameCloudPatternEntrySchema = withParser(
  BrowserGameCloudPatternEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        browserGameCloudPatternEntryIsHonest(entry) ||
        'Expected browser-game cloud pattern entry to stay ref/fingerprint backed'
    )
  )
);

const BrowserGameCloudPatternEntriesSchema = Schema.Array(BrowserGameCloudPatternEntrySchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game cloud pattern entries')
);

const BrowserGameCloudPatternLibraryBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameCloudPatternLibrarySchemaVersionSchema,
  libraryId: BrowserGameCloudPatternLibraryIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGameCloudPatternEvidenceRefsSchema,
  patterns: BrowserGameCloudPatternEntriesSchema,
  confidence: BrowserGameCloudPatternConfidenceSchema,
  reviewState: BrowserGameCloudPatternReviewStateSchema,
  rawCloudDomainStored: Schema.Boolean,
  rawCloudUrlStored: Schema.Boolean,
  rawCloudTitleStored: Schema.Boolean,
  rawStreamFrameStored: Schema.Boolean,
  runtimeDetectionClaimed: Schema.Boolean,
  cloudStreamFrameAnalysisClaimed: Schema.Boolean,
  perGameCloudTitleCertaintyClaimed: Schema.Boolean,
  nativeLauncherControlClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameCloudPatternLibraryCandidate = Infer<typeof BrowserGameCloudPatternLibraryBaseSchema>;

export const BrowserGameCloudPatternLibrarySchema = withParser(
  BrowserGameCloudPatternLibraryBaseSchema.pipe(
    Schema.filter(
      (library) =>
        browserGameCloudPatternLibraryIsHonest(library) ||
        'Expected browser-game cloud pattern library to remain contract-only metadata'
    )
  )
);

export const decodeBrowserGameCloudPatternLibrary = Schema.decodeUnknownSync(BrowserGameCloudPatternLibrarySchema);

export type BrowserGameCloudPatternEntry = Infer<typeof BrowserGameCloudPatternEntrySchema>;
export type BrowserGameCloudPatternLibrary = Infer<typeof BrowserGameCloudPatternLibrarySchema>;

function browserGameCloudPatternEntryIsHonest(entry: BrowserGameCloudPatternEntryCandidate): boolean {
  if (browserGameCloudPatternEntryClaimsAuthority(entry) || browserGameCloudPatternEntryHasInconsistentSignals(entry)) {
    return false;
  }
  if (entry.reviewState === 'reviewed') {
    return (
      entry.confidence !== 'unknown' &&
      entry.platform !== 'unknown-cloud-gaming' &&
      entry.cloudFamily !== 'unknown' &&
      !entry.routeKinds.includes('unknown-route') &&
      !entry.signalKinds.includes('unknown-signal')
    );
  }
  return (
    entry.confidence !== 'high' &&
    (entry.platform === 'unknown-cloud-gaming' ||
      entry.cloudFamily === 'unknown' ||
      entry.routeKinds.includes('unknown-route') ||
      entry.signalKinds.includes('unknown-signal'))
  );
}

function browserGameCloudPatternLibraryIsHonest(library: BrowserGameCloudPatternLibraryCandidate): boolean {
  if (browserGameCloudPatternLibraryClaimsAuthority(library)) {
    return false;
  }
  if (library.reviewState === 'reviewed') {
    return library.confidence !== 'unknown' && library.patterns.every((entry) => entry.reviewState === 'reviewed');
  }
  return library.confidence !== 'high' && library.patterns.some((entry) => entry.reviewState !== 'reviewed');
}

function browserGameCloudPatternEntryHasInconsistentSignals(entry: BrowserGameCloudPatternEntryCandidate): boolean {
  if (
    entry.sessionCandidate &&
    (!entry.routeKinds.includes('cloud-session-route') || !entry.signalKinds.includes('streaming-session-route'))
  ) {
    return true;
  }
  if (entry.titleMetadataCandidate && !entry.signalKinds.includes('platform-title-metadata-ref')) {
    return true;
  }
  if (entry.ratingMetadataCandidate && !entry.signalKinds.includes('platform-rating-metadata-ref')) {
    return true;
  }
  if (
    entry.subscriptionOrAccountCandidate &&
    !(
      entry.routeKinds.includes('cloud-account-route') ||
      entry.routeKinds.includes('cloud-subscription-route') ||
      entry.signalKinds.includes('subscription-prompt')
    )
  ) {
    return true;
  }
  return entry.nativeLauncherPromptCandidate && !entry.signalKinds.includes('native-launcher-prompt');
}

function browserGameCloudPatternEntryClaimsAuthority(entry: BrowserGameCloudPatternEntryCandidate): boolean {
  return (
    entry.rawCloudDomainStored ||
    entry.rawCloudUrlStored ||
    entry.rawCloudTitleStored ||
    entry.rawStreamFrameStored ||
    entry.runtimeDetectionClaimed ||
    entry.cloudStreamFrameAnalysisClaimed ||
    entry.perGameCloudTitleCertaintyClaimed ||
    entry.nativeLauncherControlClaimed ||
    entry.nativeGameControlClaimed ||
    entry.policyDecisionClaimed ||
    entry.enforcementClaimed
  );
}

function browserGameCloudPatternLibraryClaimsAuthority(library: BrowserGameCloudPatternLibraryCandidate): boolean {
  return (
    library.rawCloudDomainStored ||
    library.rawCloudUrlStored ||
    library.rawCloudTitleStored ||
    library.rawStreamFrameStored ||
    library.runtimeDetectionClaimed ||
    library.cloudStreamFrameAnalysisClaimed ||
    library.perGameCloudTitleCertaintyClaimed ||
    library.nativeLauncherControlClaimed ||
    library.nativeGameControlClaimed ||
    library.policyDecisionClaimed ||
    library.enforcementClaimed
  );
}
