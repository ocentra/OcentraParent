import { type Infer, Schema, withParser } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
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
  BrowserGameHiddenAnalysisReasonCodeSchema,
  BrowserGameHiddenAnalysisStateSchema,
  BrowserGameHiddenAnalysisSummaryRefSchema,
} from './browser-game-hidden-analysis-profile-safety-values';

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

function browserGameHiddenAnalysisProfileDesignIsHonest(
  design: BrowserGameHiddenAnalysisProfileDesignCandidate
): boolean {
  if (profileDesignClaimsUnsafeAuthority(design) || profileSafetyIsInvalid(design.safety)) {
    return false;
  }
  if (design.state === 'metadata-only' || design.state === 'analysis-ready') {
    return (
      design.loaderProofRef !== null &&
      design.summaryRef !== null &&
      design.reasonCodes.includes('loader-proof-required') &&
      design.confidence !== 'unknown'
    );
  }
  if (design.state === 'queued' || design.state === 'loading') {
    return design.loaderProofRef === null && design.summaryRef === null && design.confidence !== 'high';
  }
  return degradedDesignIsHonest(design);
}

function browserGameHiddenAnalysisLoaderRequestIsHonest(
  request: BrowserGameHiddenAnalysisLoaderRequestCandidate
): boolean {
  if (!request.policyAllowsHiddenAnalysis) {
    return request.capabilityState === 'disabled-by-policy';
  }
  if (request.capabilityState === 'available') {
    return request.profileDesign.state === 'queued' || request.profileDesign.state === 'loading';
  }
  return request.profileDesign.state !== 'analysis-ready';
}

function browserGameHiddenAnalysisLoaderResultIsHonest(
  result: BrowserGameHiddenAnalysisLoaderResultCandidate
): boolean {
  if (loaderResultClaimsUnsafeAuthority(result)) {
    return false;
  }
  if (hiddenAnalysisLoadedStateIsReady(result.state)) {
    return loadedHiddenAnalysisResultIsHonest(result);
  }
  if (hiddenAnalysisPendingStateIsReady(result.state)) {
    return pendingHiddenAnalysisResultIsHonest(result);
  }
  return degradedResultIsHonest(result);
}

function degradedDesignIsHonest(design: BrowserGameHiddenAnalysisProfileDesignCandidate): boolean {
  return (
    design.confidence !== 'high' &&
    design.loaderProofRef === null &&
    design.summaryRef === null &&
    design.reasonCodes.some((reason) => degradedReasons.has(reason))
  );
}

function degradedResultIsHonest(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return (
    result.confidence !== 'high' &&
    !result.loadedByHiddenAdapter &&
    result.loaderProofRef === null &&
    result.summaryRef === null &&
    result.reasonCodes.some((reason) => degradedReasons.has(reason))
  );
}

function hiddenAnalysisLoadedStateIsReady(state: BrowserGameHiddenAnalysisLoaderResultCandidate['state']): boolean {
  return state === 'metadata-only' || state === 'analysis-ready';
}

function hiddenAnalysisPendingStateIsReady(state: BrowserGameHiddenAnalysisLoaderResultCandidate['state']): boolean {
  return state === 'queued' || state === 'loading';
}

function loadedHiddenAnalysisResultIsHonest(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return (
    result.loadedByHiddenAdapter &&
    result.loaderProofRef !== null &&
    result.summaryRef !== null &&
    result.reasonCodes.includes('loader-proof-required') &&
    result.confidence !== 'unknown'
  );
}

function pendingHiddenAnalysisResultIsHonest(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return (
    !result.loadedByHiddenAdapter &&
    result.loaderProofRef === null &&
    result.summaryRef === null &&
    result.confidence !== 'high'
  );
}

function profileSafetyIsInvalid(safety: BrowserGameHiddenAnalysisProfileSafetyFlags): boolean {
  return (
    RequiredProfileSafetyFields.some((field) => safety[field] !== true) ||
    UnsafeProfileSafetyFields.some((field) => safety[field] === true)
  );
}

function profileDesignClaimsUnsafeAuthority(design: BrowserGameHiddenAnalysisProfileDesignCandidate): boolean {
  return ProfileDesignUnsafeAuthorityFields.some((field) => design[field] === true);
}

function loaderResultClaimsUnsafeAuthority(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return LoaderResultUnsafeAuthorityFields.some((field) => result[field] === true);
}

const RequiredProfileSafetyFields = [
  'ocentraOwnedProfile',
  'separateFromChildVisibleProfile',
  'boundedRetention',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisProfileSafetyFlags>;

const UnsafeProfileSafetyFields = [
  'usesChildCookies',
  'usesChildSessionTokens',
  'sharesStorageWithChildProfile',
  'allowsAutoplayAudio',
  'allowsDownloads',
  'allowsFormSubmit',
  'claimsCaptchaAutomation',
  'claimsLoginBypass',
  'retainsRawPageBody',
  'retainsRawGamePayload',
  'retainsRawScreenFrame',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisProfileSafetyFlags>;

const ProfileDesignUnsafeAuthorityFields = [
  'rawUrlStored',
  'rawPageBodyStored',
  'rawGamePayloadStored',
  'rawScreenFrameStored',
  'childCookiesOrSessionUsed',
  'browserInstrumentationClaimed',
  'hiddenNativeControlClaimed',
  'aiClassificationClaimed',
  'finalPolicyDecisionClaimed',
  'uiRenderedClaimed',
  'cloudFrameAnalysisClaimed',
  'nativeGameControlClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisProfileDesignCandidate>;

const LoaderResultUnsafeAuthorityFields = [
  'rawUrlStored',
  'rawPageBodyCaptured',
  'rawGamePayloadCaptured',
  'rawScreenFrameCaptured',
  'childCookiesOrSessionUsed',
  'browserInstrumentationClaimed',
  'hiddenNativeControlClaimed',
  'aiClassificationClaimed',
  'finalPolicyDecisionClaimed',
  'uiRenderedClaimed',
  'cloudFrameAnalysisClaimed',
  'nativeGameControlClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisLoaderResultCandidate>;

const degradedReasons = new Set<Infer<typeof BrowserGameHiddenAnalysisReasonCodeSchema>>([
  'policy-disabled',
  'profile-proof-missing',
  'manual-required',
  'timeout',
  'platform-restricted',
  'unsupported-content',
  'unavailable',
]);
