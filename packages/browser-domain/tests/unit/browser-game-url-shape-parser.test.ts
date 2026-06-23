import { describe, expect, it } from 'vitest';
import {
  type BrowserGameUrlShapeParseResult,
  BrowserGameUrlShapeParseResultSchema,
} from '@ocentra-parent/schema-domain/browser-game-url-shape-parser';
import { parseBrowserGameUrlShape } from '../../src/browser-game-url-shape-evaluator';

describe('browser-game URL shape parser contracts', () => {
  it('parses play URL shape without storing raw URL/domain/path/query values', parsesPlayShape);
  it('parses embed, purchase, account, and cloud-session route shapes', parsesOtherRouteShapes);
  it('returns manual-required states for invalid, non-text, or non-http input', returnsManualRequiredStates);
  it(
    'rejects raw storage, navigation, runtime, AI, policy, cloud-frame, native, and enforcement claims',
    rejectsClaims
  );
  it('rejects inconsistent parsed/manual states and dishonest upgrades', rejectsInconsistentStates);
});

function parsesPlayShape() {
  const result = parseBrowserGameUrlShape('https://games.example.test/catalog/space-runner-42/play?token=secret#frag');

  expect(result.parseState).toBe('parsed');
  expect(result.routeSurfaceKind).toBe('play-route');
  expect(result.inputCustody).toBe('transient-parse-only');
  expect(result.hasGameIdLikeSegment).toBe(true);
  expect(result.hasQueryShape).toBe(true);
  expect(result.hasFragmentShape).toBe(true);
  expect(result.rawUrlStored).toBe(false);
  expect(result.rawDomainStored).toBe(false);
  expect(result.rawPathStored).toBe(false);
  expect(result.rawQueryStored).toBe(false);
  expect(result.routeShapeFingerprint ?? '').not.toContain('example');
  expect(result.routeShapeFingerprint ?? '').not.toContain('space-runner');
  expect(BrowserGameUrlShapeParseResultSchema.safeParse(result).success).toBe(true);
}

function parsesOtherRouteShapes() {
  expect(parseBrowserGameUrlShape('https://games.example.test/embed/abc-123').routeSurfaceKind).toBe('embed-route');
  expect(parseBrowserGameUrlShape('https://games.example.test/store/abc-123').routeSurfaceKind).toBe('purchase-route');
  expect(parseBrowserGameUrlShape('https://games.example.test/account/login').routeSurfaceKind).toBe('account-route');
  expect(parseBrowserGameUrlShape('https://cloud.example.test/cloud/session/abc-123').routeSurfaceKind).toBe(
    'cloud-session-route'
  );
}

function returnsManualRequiredStates() {
  expect(parseBrowserGameUrlShape(42).parseState).toBe('manual-required');
  expect(parseBrowserGameUrlShape('not a url').reasonCodes).toContain('invalid-url');
  expect(parseBrowserGameUrlShape('file:///games/example').reasonCodes).toContain('unsupported-protocol');
}

function rejectsClaims() {
  const valid = parseBrowserGameUrlShape('https://games.example.test/catalog/space-runner-42/play');
  const invalidClaims = [
    { rawUrlStored: true },
    { rawDomainStored: true },
    { rawPathStored: true },
    { rawQueryStored: true },
    { browserNavigationClaimed: true },
    { runtimeDetectionClaimed: true },
    { aiClassificationClaimed: true },
    { policyDecisionClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { nativeGameControlClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidClaims) {
    expect(BrowserGameUrlShapeParseResultSchema.safeParse({ ...valid, ...invalid }).success).toBe(false);
  }
}

function rejectsInconsistentStates() {
  const parsed = parseBrowserGameUrlShape('https://games.example.test/catalog/space-runner-42/play');
  const manual = parseBrowserGameUrlShape('not a url');
  const invalidResults: BrowserGameUrlShapeParseResult[] = [
    { ...parsed, routeShapeFingerprint: null },
    { ...parsed, routeSurfaceKind: 'unknown-route' },
    { ...parsed, protocolShape: 'non-http' },
    { ...parsed, inputCustody: 'manual-required' },
    { ...manual, routeShapeFingerprint: 'manual-fingerprint' },
    { ...manual, inputCustody: 'transient-parse-only' },
    { ...manual, confidence: 'high' },
    { ...manual, reasonCodes: ['invalid-url'] },
  ];

  for (const invalid of invalidResults) {
    expect(BrowserGameUrlShapeParseResultSchema.safeParse(invalid).success).toBe(false);
  }
}
