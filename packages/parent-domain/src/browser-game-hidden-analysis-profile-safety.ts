import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  BrowserGameHiddenAnalysisCapabilityStateSchema,
  BrowserGameHiddenAnalysisConfidenceSchema,
  BrowserGameHiddenAnalysisEvidenceRefsSchema,
  BrowserGameHiddenAnalysisLoaderProofRefSchema,
  BrowserGameHiddenAnalysisLoaderRequestIdSchema,
  BrowserGameHiddenAnalysisLoaderResultIdSchema,
  BrowserGameHiddenAnalysisProfileDesignIdSchema,
  BrowserGameHiddenAnalysisProfileFingerprintSchema,
  BrowserGameHiddenAnalysisProfileKindSchema,
  BrowserGameHiddenAnalysisProfileSafetySchemaVersionSchema,
  BrowserGameHiddenAnalysisReasonCodesSchema,
  BrowserGameHiddenAnalysisStateSchema,
  BrowserGameHiddenAnalysisSummaryRefSchema,
} from './browser-game-hidden-analysis-profile-safety-values';
import {
  browserGameHiddenAnalysisLoaderRequestIsHonest,
  browserGameHiddenAnalysisLoaderResultIsHonest,
  browserGameHiddenAnalysisProfileDesignIsHonest,
  browserGameHiddenAnalysisResultForRequest,
} from './browser-game-hidden-analysis-profile-safety-guards';

const PositiveBrowserGameHiddenAnalysisIntegerSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value > 0 || 'Expected positive browser-game hidden analysis integer')
);
const OptionalBrowserGameHiddenAnalysisLoaderProofRefSchema = Schema.Union(
  BrowserGameHiddenAnalysisLoaderProofRefSchema,
  Schema.Null
);
const OptionalBrowserGameHiddenAnalysisSummaryRefSchema = Schema.Union(
  BrowserGameHiddenAnalysisSummaryRefSchema,
  Schema.Null
);

export const BrowserGameHiddenAnalysisProfileSafetyFlagsSchema = Schema.Struct({
  ocentraOwnedProfile: Schema.Boolean,
  separateFromChildVisibleProfile: Schema.Boolean,
  usesChildCookies: Schema.Boolean,
  usesChildSessionTokens: Schema.Boolean,
  sharesStorageWithChildProfile: Schema.Boolean,
  allowsAutoplayAudio: Schema.Boolean,
  allowsDownloads: Schema.Boolean,
  allowsFormSubmit: Schema.Boolean,
  claimsCaptchaAutomation: Schema.Boolean,
  claimsLoginBypass: Schema.Boolean,
  retainsRawPageBody: Schema.Boolean,
  retainsRawGamePayload: Schema.Boolean,
  retainsRawScreenFrame: Schema.Boolean,
  boundedRetention: Schema.Boolean,
});

export type BrowserGameHiddenAnalysisProfileSafetyFlags = Infer<
  typeof BrowserGameHiddenAnalysisProfileSafetyFlagsSchema
>;

const BrowserGameHiddenAnalysisProfileDesignBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameHiddenAnalysisProfileSafetySchemaVersionSchema,
  profileDesignId: BrowserGameHiddenAnalysisProfileDesignIdSchema,
  designedAt: ParentTimestampSchema,
  profileKind: BrowserGameHiddenAnalysisProfileKindSchema,
  profileFingerprint: BrowserGameHiddenAnalysisProfileFingerprintSchema,
  sourceEvidenceRefs: BrowserGameHiddenAnalysisEvidenceRefsSchema,
  state: BrowserGameHiddenAnalysisStateSchema,
  confidence: BrowserGameHiddenAnalysisConfidenceSchema,
  loaderProofRef: OptionalBrowserGameHiddenAnalysisLoaderProofRefSchema,
  summaryRef: OptionalBrowserGameHiddenAnalysisSummaryRefSchema,
  reasonCodes: BrowserGameHiddenAnalysisReasonCodesSchema,
  retentionTtlSeconds: PositiveBrowserGameHiddenAnalysisIntegerSchema,
  maxStructuredSummaryBytes: PositiveBrowserGameHiddenAnalysisIntegerSchema,
  safety: BrowserGameHiddenAnalysisProfileSafetyFlagsSchema,
  rawUrlStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  rawGamePayloadStored: Schema.Boolean,
  rawScreenFrameStored: Schema.Boolean,
  childCookiesOrSessionUsed: Schema.Boolean,
  browserInstrumentationClaimed: Schema.Boolean,
  hiddenNativeControlClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

export type BrowserGameHiddenAnalysisProfileDesignCandidate = Infer<
  typeof BrowserGameHiddenAnalysisProfileDesignBaseSchema
>;

export const BrowserGameHiddenAnalysisProfileDesignSchema = withParser(
  BrowserGameHiddenAnalysisProfileDesignBaseSchema.pipe(
    Schema.filter(
      (design) =>
        browserGameHiddenAnalysisProfileDesignIsHonest(design) ||
        'Expected browser-game hidden analysis profile design to stay isolated, bounded, and proof-backed'
    )
  )
);

const BrowserGameHiddenAnalysisLoaderRequestBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameHiddenAnalysisProfileSafetySchemaVersionSchema,
  loaderRequestId: BrowserGameHiddenAnalysisLoaderRequestIdSchema,
  requestedAt: ParentTimestampSchema,
  profileDesign: BrowserGameHiddenAnalysisProfileDesignSchema,
  capabilityState: BrowserGameHiddenAnalysisCapabilityStateSchema,
  policyAllowsHiddenAnalysis: Schema.Boolean,
});

export type BrowserGameHiddenAnalysisLoaderRequestCandidate = Infer<
  typeof BrowserGameHiddenAnalysisLoaderRequestBaseSchema
>;

export const BrowserGameHiddenAnalysisLoaderRequestSchema = withParser(
  BrowserGameHiddenAnalysisLoaderRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        browserGameHiddenAnalysisLoaderRequestIsHonest(request) ||
        'Expected browser-game hidden analysis loader request to preserve policy and profile proof boundaries'
    )
  )
);

const BrowserGameHiddenAnalysisLoaderResultBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameHiddenAnalysisProfileSafetySchemaVersionSchema,
  loaderResultId: BrowserGameHiddenAnalysisLoaderResultIdSchema,
  loaderRequestId: BrowserGameHiddenAnalysisLoaderRequestIdSchema,
  producedAt: ParentTimestampSchema,
  profileDesignId: BrowserGameHiddenAnalysisProfileDesignIdSchema,
  sourceEvidenceRefs: BrowserGameHiddenAnalysisEvidenceRefsSchema,
  state: BrowserGameHiddenAnalysisStateSchema,
  confidence: BrowserGameHiddenAnalysisConfidenceSchema,
  loaderProofRef: OptionalBrowserGameHiddenAnalysisLoaderProofRefSchema,
  summaryRef: OptionalBrowserGameHiddenAnalysisSummaryRefSchema,
  reasonCodes: BrowserGameHiddenAnalysisReasonCodesSchema,
  loadedByHiddenAdapter: Schema.Boolean,
  metadataOnly: Schema.Boolean,
  rawUrlStored: Schema.Boolean,
  rawPageBodyCaptured: Schema.Boolean,
  rawGamePayloadCaptured: Schema.Boolean,
  rawScreenFrameCaptured: Schema.Boolean,
  childCookiesOrSessionUsed: Schema.Boolean,
  browserInstrumentationClaimed: Schema.Boolean,
  hiddenNativeControlClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

export type BrowserGameHiddenAnalysisLoaderResultCandidate = Infer<
  typeof BrowserGameHiddenAnalysisLoaderResultBaseSchema
>;

export const BrowserGameHiddenAnalysisLoaderResultSchema = withParser(
  BrowserGameHiddenAnalysisLoaderResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        browserGameHiddenAnalysisLoaderResultIsHonest(result) ||
        'Expected browser-game hidden analysis loader result to stay proof-backed and no-capture'
    )
  )
);

export const decodeBrowserGameHiddenAnalysisProfileDesign = Schema.decodeUnknownSync(
  BrowserGameHiddenAnalysisProfileDesignSchema
);
export const decodeBrowserGameHiddenAnalysisLoaderRequest = Schema.decodeUnknownSync(
  BrowserGameHiddenAnalysisLoaderRequestSchema
);
export const decodeBrowserGameHiddenAnalysisLoaderResult = Schema.decodeUnknownSync(
  BrowserGameHiddenAnalysisLoaderResultSchema
);

export type BrowserGameHiddenAnalysisProfileDesign = Infer<typeof BrowserGameHiddenAnalysisProfileDesignSchema>;
export type BrowserGameHiddenAnalysisLoaderRequest = Infer<typeof BrowserGameHiddenAnalysisLoaderRequestSchema>;
export type BrowserGameHiddenAnalysisLoaderResult = Infer<typeof BrowserGameHiddenAnalysisLoaderResultSchema>;

export function planBrowserGameHiddenAnalysisProfileSafety(input: unknown): BrowserGameHiddenAnalysisLoaderResult {
  const request = decodeBrowserGameHiddenAnalysisLoaderRequest(input);
  const result = browserGameHiddenAnalysisResultForRequest(request);

  return decodeBrowserGameHiddenAnalysisLoaderResult(result);
}
