/* generated from crates/browser-core/src/browser_game_url_shape_evaluator.rs */

import { confidenceFor } from './browser_game_url_shape_evaluator_confidence';
import { fingerprintFor } from './browser_game_url_shape_evaluator_fingerprint';
import { hostShapeFor, protocolShapeFor } from './browser_game_url_shape_evaluator_parse';
import {
  pathDepthFor,
  pathSegmentsFor,
  routeHintsFor,
  segmentLooksLikeGameId,
} from './browser_game_url_shape_evaluator_route_hints';
import { reasonCodesFor } from './browser_game_url_shape_evaluator_reason_codes';
import { routeSurfaceKindFor } from './browser_game_url_shape_evaluator_route_surface';

export type BrowserGameParsedUrl = {
  readonly protocol: string;
  readonly hostname: string;
  readonly pathname: string;
  readonly search: string;
  readonly hash: string;
};

export type BrowserGameUrlConstructor = new (value: string) => BrowserGameParsedUrl;

export function browserGameUrlShapeParseResultTemplate(input: unknown) {
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

  return {
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
  } as const;
}

function manualBrowserGameUrlShapeResult(reasonCode: string, protocolShape: string = 'unknown') {
  return {
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
  } as const;
}
