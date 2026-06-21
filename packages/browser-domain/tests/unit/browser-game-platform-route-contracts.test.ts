import { describe, expect, it } from 'vitest';
import {
  BrowserGamePlatformRouteCatalogSchema,
  BrowserGamePlatformRouteContractSchema,
  type BrowserGamePlatformRouteCatalog,
  type BrowserGamePlatformRouteContract,
} from '@ocentra-parent/schema-domain/browser-game-platform-route-contracts';

describe('browser-game platform route contracts', () => {
  it(
    'accepts reviewed platform route contracts for catalog, play, purchase, and cloud-session surfaces',
    acceptsRoutes
  );
  it('accepts candidate, manual-required, and unavailable route states', acceptsFallbackStates);
  it('accepts route catalogs without claiming runtime behavior', acceptsCatalogs);
  it('rejects raw route data, parser, runtime, AI, policy, native, cloud-frame, and enforcement claims', rejectsClaims);
  it('rejects inconsistent reviewed rows, launch purposes, and catalog upgrades', rejectsInconsistentRows);
});

function acceptsRoutes() {
  expect(BrowserGamePlatformRouteContractSchema.safeParse(routeContract()).success).toBe(true);
  expect(
    BrowserGamePlatformRouteContractSchema.safeParse(
      routeContract({
        routeContractId: 'game-route-contract-catalog',
        routeSurfaceKind: 'catalog-route',
        managedBrowserRequired: false,
        childLaunchCandidate: false,
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGamePlatformRouteContractSchema.safeParse(
      routeContract({
        routeContractId: 'game-route-contract-purchase',
        routeSurfaceKind: 'purchase-route',
        managedBrowserRequired: false,
        childLaunchCandidate: false,
        accountOrPurchaseCandidate: true,
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGamePlatformRouteContractSchema.safeParse(
      routeContract({
        routeContractId: 'game-route-contract-cloud-session',
        platformKind: 'cloud-gaming-platform',
        routeSurfaceKind: 'cloud-session-route',
        cloudSessionCandidate: true,
      })
    ).success
  ).toBe(true);
}

function acceptsFallbackStates() {
  expect(
    BrowserGamePlatformRouteContractSchema.safeParse(
      routeContract({
        routeContractId: 'game-route-contract-candidate',
        status: 'candidate',
        confidence: 'medium',
      })
    ).success
  ).toBe(true);
  expect(BrowserGamePlatformRouteContractSchema.safeParse(manualRoute()).success).toBe(true);
  expect(
    BrowserGamePlatformRouteContractSchema.safeParse(
      manualRoute({
        routeContractId: 'game-route-contract-unavailable',
        status: 'unavailable',
        confidence: 'unknown',
        routeSourceKind: 'unavailable',
        custodyLabel: 'unavailable',
      })
    ).success
  ).toBe(true);
}

function acceptsCatalogs() {
  expect(BrowserGamePlatformRouteCatalogSchema.safeParse(routeCatalog()).success).toBe(true);
  expect(
    BrowserGamePlatformRouteCatalogSchema.safeParse(
      routeCatalog({
        status: 'manual-required',
        confidence: 'low',
        routes: [manualRoute()],
      })
    ).success
  ).toBe(true);
}

function rejectsClaims() {
  const invalidClaims = [
    { rawDomainStored: true },
    { rawUrlStored: true },
    { rawPathStored: true },
    { rawPageBodyStored: true },
    { runtimeDetectionClaimed: true },
    { urlParserClaimed: true },
    { aiClassificationClaimed: true },
    { policyDecisionClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidClaims) {
    expect(BrowserGamePlatformRouteContractSchema.safeParse(routeContract(invalid)).success).toBe(false);
    expect(BrowserGamePlatformRouteCatalogSchema.safeParse(routeCatalog(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentRows() {
  const invalidRoutes = [
    { platformKind: 'unknown-platform' },
    { routeSurfaceKind: 'unknown-route' },
    { routeSourceKind: 'unavailable' },
    { custodyLabel: 'manual-required' },
    { routeSurfaceKind: 'play-route', managedBrowserRequired: false },
    { childLaunchCandidate: true, routeSurfaceKind: 'catalog-route' },
    { accountOrPurchaseCandidate: true, routeSurfaceKind: 'play-route' },
    { cloudSessionCandidate: true, routeSurfaceKind: 'play-route' },
    { status: 'manual-required', confidence: 'high', routeSurfaceKind: 'unknown-route' },
  ];

  for (const invalid of invalidRoutes) {
    expect(BrowserGamePlatformRouteContractSchema.safeParse(routeContract(invalid)).success).toBe(false);
  }

  expect(
    BrowserGamePlatformRouteCatalogSchema.safeParse(
      routeCatalog({
        routes: [manualRoute()],
      })
    ).success
  ).toBe(false);
  expect(
    BrowserGamePlatformRouteCatalogSchema.safeParse(
      routeCatalog({
        status: 'manual-required',
        confidence: 'high',
        routes: [manualRoute()],
      })
    ).success
  ).toBe(false);
}

function routeContract(overrides = {}): BrowserGamePlatformRouteContract {
  return {
    routeContractId: 'game-route-contract-play',
    platformKind: 'browser-game-portal',
    routeSurfaceKind: 'play-route',
    routeSourceKind: 'platform-pattern-ref',
    custodyLabel: 'ref-only',
    routePatternRef: 'game-route-pattern-play-ref',
    sourceEvidenceRefs: ['game-route-evidence-platform', 'game-route-evidence-pattern'],
    confidence: 'high',
    status: 'reviewed',
    managedBrowserRequired: true,
    childLaunchCandidate: true,
    accountOrPurchaseCandidate: false,
    cloudSessionCandidate: false,
    rawDomainStored: false,
    rawUrlStored: false,
    rawPathStored: false,
    rawPageBodyStored: false,
    runtimeDetectionClaimed: false,
    urlParserClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function manualRoute(overrides = {}): BrowserGamePlatformRouteContract {
  return routeContract({
    routeContractId: 'game-route-contract-manual',
    platformKind: 'unknown-platform',
    routeSurfaceKind: 'unknown-route',
    routeSourceKind: 'manual-review-ref',
    custodyLabel: 'manual-required',
    confidence: 'low',
    status: 'manual-required',
    managedBrowserRequired: false,
    childLaunchCandidate: false,
    ...overrides,
  });
}

function routeCatalog(overrides = {}): BrowserGamePlatformRouteCatalog {
  return {
    schemaVersion: 'browser-game-platform-route-contract',
    catalogId: 'game-route-catalog-reviewed',
    generatedAt: '2026-06-03T11:35:00.000Z',
    sourceEvidenceRefs: ['game-route-catalog-evidence'],
    routes: [routeContract()],
    confidence: 'high',
    status: 'reviewed',
    rawDomainStored: false,
    rawUrlStored: false,
    rawPathStored: false,
    rawPageBodyStored: false,
    runtimeDetectionClaimed: false,
    urlParserClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
