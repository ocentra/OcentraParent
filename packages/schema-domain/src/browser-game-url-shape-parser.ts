import { type Infer, Schema, withParser } from './effect';
import { BrowserGameRouteSurfaceKindSchema } from './browser-game-platform-route-contract-values';
import {
  BrowserGameUrlHostShapeSchema,
  BrowserGameUrlPathDepthSchema,
  BrowserGameUrlProtocolShapeSchema,
  BrowserGameUrlShapeConfidenceSchema,
  BrowserGameUrlShapeFingerprintSchema,
  BrowserGameUrlShapeInputCustodySchema,
  BrowserGameUrlShapeParseStateSchema,
  BrowserGameUrlShapeReasonCodesSchema,
  BrowserGameUrlShapeResultIdSchema,
  BrowserGameUrlShapeSchemaVersionSchema,
} from './browser-game-url-shape-parser-values';

const OptionalBrowserGameUrlShapeFingerprintSchema = Schema.Union(BrowserGameUrlShapeFingerprintSchema, Schema.Null);

const BrowserGameUrlShapeParseResultBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameUrlShapeSchemaVersionSchema,
  parserResultId: BrowserGameUrlShapeResultIdSchema,
  inputCustody: BrowserGameUrlShapeInputCustodySchema,
  parseState: BrowserGameUrlShapeParseStateSchema,
  protocolShape: BrowserGameUrlProtocolShapeSchema,
  hostShape: BrowserGameUrlHostShapeSchema,
  pathDepth: BrowserGameUrlPathDepthSchema,
  routeSurfaceKind: BrowserGameRouteSurfaceKindSchema,
  routeShapeFingerprint: OptionalBrowserGameUrlShapeFingerprintSchema,
  hasQueryShape: Schema.Boolean,
  hasFragmentShape: Schema.Boolean,
  hasGameIdLikeSegment: Schema.Boolean,
  hasEmbedHint: Schema.Boolean,
  hasPlayHint: Schema.Boolean,
  hasAccountHint: Schema.Boolean,
  hasPurchaseHint: Schema.Boolean,
  hasCloudSessionHint: Schema.Boolean,
  reasonCodes: BrowserGameUrlShapeReasonCodesSchema,
  confidence: BrowserGameUrlShapeConfidenceSchema,
  rawUrlStored: Schema.Boolean,
  rawDomainStored: Schema.Boolean,
  rawPathStored: Schema.Boolean,
  rawQueryStored: Schema.Boolean,
  browserNavigationClaimed: Schema.Boolean,
  runtimeDetectionClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameUrlShapeParseResultCandidate = Infer<typeof BrowserGameUrlShapeParseResultBaseSchema>;

export const BrowserGameUrlShapeParseResultSchema = withParser(
  BrowserGameUrlShapeParseResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        browserGameUrlShapeParseResultIsHonest(result) ||
        'Expected browser-game URL shape parse result to stay redacted and non-authoritative'
    )
  )
);

export const decodeBrowserGameUrlShapeParseResult = Schema.decodeUnknownSync(BrowserGameUrlShapeParseResultSchema);

export type BrowserGameUrlShapeParseResult = Infer<typeof BrowserGameUrlShapeParseResultSchema>;

function browserGameUrlShapeParseResultIsHonest(result: BrowserGameUrlShapeParseResultCandidate): boolean {
  if (browserGameUrlShapeResultClaimsAuthority(result)) {
    return false;
  }
  if (result.parseState === 'parsed') {
    return (
      result.inputCustody === 'transient-parse-only' &&
      result.protocolShape === 'http-family' &&
      result.hostShape !== 'unknown' &&
      result.pathDepth !== 'unknown' &&
      result.routeSurfaceKind !== 'unknown-route' &&
      result.routeShapeFingerprint !== null &&
      result.confidence !== 'unknown'
    );
  }
  return (
    result.inputCustody !== 'transient-parse-only' &&
    result.routeShapeFingerprint === null &&
    result.confidence !== 'high' &&
    result.reasonCodes.some((reason) => reason === 'manual-required' || reason === 'unavailable')
  );
}

function browserGameUrlShapeResultClaimsAuthority(result: BrowserGameUrlShapeParseResultCandidate): boolean {
  return (
    result.rawUrlStored ||
    result.rawDomainStored ||
    result.rawPathStored ||
    result.rawQueryStored ||
    result.browserNavigationClaimed ||
    result.runtimeDetectionClaimed ||
    result.aiClassificationClaimed ||
    result.policyDecisionClaimed ||
    result.cloudFrameAnalysisClaimed ||
    result.nativeGameControlClaimed ||
    result.enforcementClaimed
  );
}
