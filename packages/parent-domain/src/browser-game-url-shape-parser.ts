import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BrowserGameRouteSurfaceKindSchema,
  type BrowserGameRouteSurfaceKind,
} from './browser-game-platform-route-contract-values';
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
  type BrowserGameUrlHostShape,
  type BrowserGameUrlPathDepth,
  type BrowserGameUrlProtocolShape,
  type BrowserGameUrlShapeConfidence,
  type BrowserGameUrlShapeReasonCode,
} from './browser-game-url-shape-parser-values';

const OptionalBrowserGameUrlShapeFingerprintSchema = Schema.Union(BrowserGameUrlShapeFingerprintSchema, Schema.Null);

type BrowserGameParsedUrl = {
  readonly protocol: string;
  readonly hostname: string;
  readonly pathname: string;
  readonly search: string;
  readonly hash: string;
};

type BrowserGameUrlConstructor = new (value: string) => BrowserGameParsedUrl;

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

export function parseBrowserGameUrlShape(input: unknown): BrowserGameUrlShapeParseResult {
  if (typeof input !== 'string' || input.trim().length === 0) {
    return manualBrowserGameUrlShapeResult('not-text-input');
  }

  const UrlConstructor = (globalThis as unknown as { readonly URL?: BrowserGameUrlConstructor }).URL;
  if (!UrlConstructor) {
    return manualBrowserGameUrlShapeResult('unavailable');
  }

  let parsedUrl: BrowserGameParsedUrl;
  try {
    parsedUrl = new UrlConstructor(input);
  } catch {
    return manualBrowserGameUrlShapeResult('invalid-url');
  }

  const protocolShape = protocolShapeFor(parsedUrl);
  if (protocolShape !== 'http-family') {
    return manualBrowserGameUrlShapeResult('unsupported-protocol', protocolShape);
  }

  const segments = pathSegmentsFor(parsedUrl);
  const routeHints = routeHintsFor(segments);
  const pathDepth = pathDepthFor(segments);
  const routeSurfaceKind = routeSurfaceKindFor(segments, routeHints);
  const hostShape = hostShapeFor(parsedUrl);
  const hasGameIdLikeSegment = segments.some(segmentLooksLikeGameId);
  const reasonCodes = reasonCodesFor(routeSurfaceKind, routeHints);
  const confidence = confidenceFor(routeSurfaceKind, hostShape, pathDepth);

  return decodeBrowserGameUrlShapeParseResult({
    schemaVersion: 'browser-game-url-shape-parser-contract',
    parserResultId: 'browser-game-url-shape-parser-result',
    inputCustody: 'transient-parse-only',
    parseState: 'parsed',
    protocolShape,
    hostShape,
    pathDepth,
    routeSurfaceKind,
    routeShapeFingerprint: fingerprintFor({
      protocolShape,
      hostShape,
      pathDepth,
      routeSurfaceKind,
      hasGameIdLikeSegment,
      hasQueryShape: parsedUrl.search.length > 0,
      hasFragmentShape: parsedUrl.hash.length > 0,
      ...routeHints,
    }),
    hasQueryShape: parsedUrl.search.length > 0,
    hasFragmentShape: parsedUrl.hash.length > 0,
    hasGameIdLikeSegment,
    ...routeHints,
    reasonCodes,
    confidence,
    rawUrlStored: false,
    rawDomainStored: false,
    rawPathStored: false,
    rawQueryStored: false,
    browserNavigationClaimed: false,
    runtimeDetectionClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
  });
}

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

function manualBrowserGameUrlShapeResult(
  reasonCode: BrowserGameUrlShapeReasonCode,
  protocolShape: BrowserGameUrlProtocolShape = 'unknown'
): BrowserGameUrlShapeParseResult {
  return decodeBrowserGameUrlShapeParseResult({
    schemaVersion: 'browser-game-url-shape-parser-contract',
    parserResultId: 'browser-game-url-shape-parser-result',
    inputCustody: 'manual-required',
    parseState: 'manual-required',
    protocolShape,
    hostShape: 'unknown',
    pathDepth: 'unknown',
    routeSurfaceKind: 'unknown-route',
    routeShapeFingerprint: null,
    hasQueryShape: false,
    hasFragmentShape: false,
    hasGameIdLikeSegment: false,
    hasEmbedHint: false,
    hasPlayHint: false,
    hasAccountHint: false,
    hasPurchaseHint: false,
    hasCloudSessionHint: false,
    reasonCodes: [reasonCode, 'manual-required'],
    confidence: 'low',
    rawUrlStored: false,
    rawDomainStored: false,
    rawPathStored: false,
    rawQueryStored: false,
    browserNavigationClaimed: false,
    runtimeDetectionClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
  });
}

function protocolShapeFor(parsedUrl: BrowserGameParsedUrl): BrowserGameUrlProtocolShape {
  if (parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:') {
    return 'http-family';
  }
  return parsedUrl.protocol.length > 0 ? 'non-http' : 'missing';
}

function hostShapeFor(parsedUrl: BrowserGameParsedUrl): BrowserGameUrlHostShape {
  const hostname = parsedUrl.hostname.toLowerCase();
  if (hostname === 'localhost') {
    return 'localhost-like';
  }
  if (/^\d{1,3}(?:\.\d{1,3}){3}$/.test(hostname)) {
    return 'ip-like';
  }
  return hostname.includes('.') ? 'domain-like' : 'unknown';
}

function pathSegmentsFor(parsedUrl: BrowserGameParsedUrl) {
  return parsedUrl.pathname
    .split('/')
    .map((segment) => segment.trim().toLowerCase())
    .filter((segment) => segment.length > 0);
}

function pathDepthFor(segments: ReadonlyArray<unknown>): BrowserGameUrlPathDepth {
  if (segments.length === 0) {
    return 'root';
  }
  if (segments.length === 1) {
    return 'one-segment';
  }
  if (segments.length === 2) {
    return 'two-segments';
  }
  return 'three-or-more-segments';
}

function routeHintsFor(segments: ReadonlyArray<unknown>) {
  const normalizedSegments = segments.filter((segment): segment is string => typeof segment === 'string');
  return {
    hasEmbedHint: normalizedSegments.some((segment) => segment === 'embed' || segment === 'iframe'),
    hasPlayHint: normalizedSegments.some((segment) => segment === 'play' || segment === 'launch'),
    hasAccountHint: normalizedSegments.some(
      (segment) => segment === 'account' || segment === 'login' || segment === 'signup'
    ),
    hasPurchaseHint: normalizedSegments.some(
      (segment) => segment === 'buy' || segment === 'store' || segment === 'checkout'
    ),
    hasCloudSessionHint: normalizedSegments.some(
      (segment) => segment === 'cloud' || segment === 'stream' || segment === 'session'
    ),
  };
}

function routeSurfaceKindFor(
  segments: ReadonlyArray<unknown>,
  routeHints: ReturnType<typeof routeHintsFor>
): BrowserGameRouteSurfaceKind {
  if (routeHints.hasCloudSessionHint) {
    return 'cloud-session-route';
  }
  if (routeHints.hasEmbedHint) {
    return 'embed-route';
  }
  if (routeHints.hasPlayHint) {
    return 'play-route';
  }
  if (routeHints.hasPurchaseHint) {
    return 'purchase-route';
  }
  if (routeHints.hasAccountHint) {
    return 'account-route';
  }
  if (segments.length === 0) {
    return 'home-route';
  }
  if (segments.length <= 2) {
    return 'catalog-route';
  }
  return 'game-detail-route';
}

function reasonCodesFor(
  routeSurfaceKind: BrowserGameRouteSurfaceKind,
  routeHints: ReturnType<typeof routeHintsFor>
): [BrowserGameUrlShapeReasonCode, ...BrowserGameUrlShapeReasonCode[]] {
  if (routeHints.hasCloudSessionHint || routeSurfaceKind === 'cloud-session-route') {
    return ['cloud-session-hint'];
  }
  if (routeHints.hasEmbedHint || routeSurfaceKind === 'embed-route') {
    return ['embed-route-hint'];
  }
  if (routeHints.hasPlayHint || routeSurfaceKind === 'play-route') {
    return ['game-route-hint'];
  }
  if (routeHints.hasPurchaseHint || routeSurfaceKind === 'purchase-route') {
    return ['purchase-route-hint'];
  }
  if (routeHints.hasAccountHint || routeSurfaceKind === 'account-route') {
    return ['account-route-hint'];
  }
  return ['catalog-route-hint'];
}

function confidenceFor(
  routeSurfaceKind: BrowserGameRouteSurfaceKind,
  hostShape: BrowserGameUrlHostShape,
  pathDepth: BrowserGameUrlPathDepth
): BrowserGameUrlShapeConfidence {
  if (hostShape === 'unknown' || pathDepth === 'unknown' || routeSurfaceKind === 'unknown-route') {
    return 'unknown';
  }
  if (routeSurfaceKind === 'home-route' || routeSurfaceKind === 'catalog-route') {
    return 'medium';
  }
  return 'high';
}

function segmentLooksLikeGameId(segment: unknown): boolean {
  return typeof segment === 'string' && segment.length >= 4 && (/\d/.test(segment) || segment.includes('-'));
}

function fingerprintFor(parts: {
  protocolShape: BrowserGameUrlProtocolShape;
  hostShape: BrowserGameUrlHostShape;
  pathDepth: BrowserGameUrlPathDepth;
  routeSurfaceKind: BrowserGameRouteSurfaceKind;
  hasGameIdLikeSegment: boolean;
  hasQueryShape: boolean;
  hasFragmentShape: boolean;
  hasEmbedHint: boolean;
  hasPlayHint: boolean;
  hasAccountHint: boolean;
  hasPurchaseHint: boolean;
  hasCloudSessionHint: boolean;
}) {
  return [
    'url-shape',
    parts.protocolShape,
    parts.hostShape,
    parts.pathDepth,
    parts.routeSurfaceKind,
    parts.hasGameIdLikeSegment ? 'game-id-like' : 'no-game-id',
    parts.hasQueryShape ? 'query' : 'no-query',
    parts.hasFragmentShape ? 'fragment' : 'no-fragment',
    parts.hasEmbedHint ? 'embed' : 'no-embed',
    parts.hasPlayHint ? 'play' : 'no-play',
    parts.hasAccountHint ? 'account' : 'no-account',
    parts.hasPurchaseHint ? 'purchase' : 'no-purchase',
    parts.hasCloudSessionHint ? 'cloud-session' : 'no-cloud-session',
  ].join(':');
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
