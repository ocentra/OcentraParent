import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  BrowserGamePlatformKindSchema,
  BrowserGamePlatformRouteCatalogIdSchema,
  BrowserGamePlatformRouteCatalogSchemaVersionSchema,
  BrowserGamePlatformRouteConfidenceSchema,
  BrowserGamePlatformRouteContractIdSchema,
  BrowserGamePlatformRouteEvidenceRefsSchema,
  BrowserGamePlatformRouteStatusSchema,
  BrowserGameRouteCustodyLabelSchema,
  BrowserGameRoutePatternRefSchema,
  BrowserGameRouteSourceKindSchema,
  BrowserGameRouteSurfaceKindSchema,
} from './browser-game-platform-route-contract-values';

const BrowserGamePlayableRouteSurfaceSchema = Schema.Literal('play-route', 'embed-route', 'cloud-session-route');

const BrowserGamePlatformRouteContractBaseSchema = Schema.Struct({
  routeContractId: BrowserGamePlatformRouteContractIdSchema,
  platformKind: BrowserGamePlatformKindSchema,
  routeSurfaceKind: BrowserGameRouteSurfaceKindSchema,
  routeSourceKind: BrowserGameRouteSourceKindSchema,
  custodyLabel: BrowserGameRouteCustodyLabelSchema,
  routePatternRef: BrowserGameRoutePatternRefSchema,
  sourceEvidenceRefs: BrowserGamePlatformRouteEvidenceRefsSchema,
  confidence: BrowserGamePlatformRouteConfidenceSchema,
  status: BrowserGamePlatformRouteStatusSchema,
  managedBrowserRequired: Schema.Boolean,
  childLaunchCandidate: Schema.Boolean,
  accountOrPurchaseCandidate: Schema.Boolean,
  cloudSessionCandidate: Schema.Boolean,
  rawDomainStored: Schema.Boolean,
  rawUrlStored: Schema.Boolean,
  rawPathStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  runtimeDetectionClaimed: Schema.Boolean,
  urlParserClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGamePlatformRouteContractCandidate = Infer<typeof BrowserGamePlatformRouteContractBaseSchema>;

export const BrowserGamePlatformRouteContractSchema = withParser(
  BrowserGamePlatformRouteContractBaseSchema.pipe(
    Schema.filter(
      (route) =>
        browserGamePlatformRouteContractIsHonest(route) ||
        'Expected browser-game platform route contract to stay ref-backed and non-executing'
    )
  )
);

const BrowserGamePlatformRouteContractsSchema = Schema.Array(BrowserGamePlatformRouteContractSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game platform route contracts')
);

const BrowserGamePlatformRouteCatalogBaseSchema = Schema.Struct({
  schemaVersion: BrowserGamePlatformRouteCatalogSchemaVersionSchema,
  catalogId: BrowserGamePlatformRouteCatalogIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGamePlatformRouteEvidenceRefsSchema,
  routes: BrowserGamePlatformRouteContractsSchema,
  confidence: BrowserGamePlatformRouteConfidenceSchema,
  status: BrowserGamePlatformRouteStatusSchema,
  rawDomainStored: Schema.Boolean,
  rawUrlStored: Schema.Boolean,
  rawPathStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  runtimeDetectionClaimed: Schema.Boolean,
  urlParserClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGamePlatformRouteCatalogCandidate = Infer<typeof BrowserGamePlatformRouteCatalogBaseSchema>;

export const BrowserGamePlatformRouteCatalogSchema = withParser(
  BrowserGamePlatformRouteCatalogBaseSchema.pipe(
    Schema.filter(
      (catalog) =>
        browserGamePlatformRouteCatalogIsHonest(catalog) ||
        'Expected browser-game platform route catalog to remain contract-only'
    )
  )
);

export const decodeBrowserGamePlatformRouteCatalog = Schema.decodeUnknownSync(BrowserGamePlatformRouteCatalogSchema);

export type BrowserGamePlatformRouteCatalog = Infer<typeof BrowserGamePlatformRouteCatalogSchema>;
export type BrowserGamePlatformRouteContract = Infer<typeof BrowserGamePlatformRouteContractSchema>;

type BrowserGamePlatformRouteStatusValidator = (route: BrowserGamePlatformRouteContractCandidate) => boolean;

const BrowserGamePlatformRouteStatusValidators = {
  reviewed: reviewedPlatformRouteIsHonest,
  candidate: candidatePlatformRouteIsHonest,
  'manual-required': degradedPlatformRouteIsHonest,
  unavailable: degradedPlatformRouteIsHonest,
} satisfies Record<BrowserGamePlatformRouteContractCandidate['status'], BrowserGamePlatformRouteStatusValidator>;

function browserGamePlatformRouteContractIsHonest(route: BrowserGamePlatformRouteContractCandidate): boolean {
  if (browserGamePlatformRouteContractClaimsAuthority(route) || browserGamePlatformRouteHasInconsistentPurpose(route)) {
    return false;
  }
  return BrowserGamePlatformRouteStatusValidators[route.status](route);
}

function reviewedPlatformRouteIsHonest(route: BrowserGamePlatformRouteContractCandidate): boolean {
  return (
    route.confidence !== 'unknown' &&
    route.platformKind !== 'unknown-platform' &&
    route.routeSurfaceKind !== 'unknown-route' &&
    route.routeSourceKind !== 'unavailable' &&
    routeHasReviewedCustody(route)
  );
}

function candidatePlatformRouteIsHonest(route: BrowserGamePlatformRouteContractCandidate): boolean {
  return (
    route.confidence !== 'high' &&
    route.platformKind !== 'unknown-platform' &&
    route.routeSurfaceKind !== 'unknown-route' &&
    route.routeSourceKind !== 'unavailable' &&
    route.custodyLabel !== 'unavailable'
  );
}

function degradedPlatformRouteIsHonest(route: BrowserGamePlatformRouteContractCandidate): boolean {
  return route.confidence !== 'high' && routeHasDegradedMarker(route);
}

function routeHasReviewedCustody(route: BrowserGamePlatformRouteContractCandidate): boolean {
  return route.custodyLabel === 'ref-only' || route.custodyLabel === 'hash-only';
}

function routeHasDegradedMarker(route: BrowserGamePlatformRouteContractCandidate): boolean {
  return (
    route.platformKind === 'unknown-platform' ||
    route.routeSurfaceKind === 'unknown-route' ||
    route.routeSourceKind === 'manual-review-ref' ||
    route.routeSourceKind === 'unavailable' ||
    route.custodyLabel === 'manual-required' ||
    route.custodyLabel === 'unavailable'
  );
}

function browserGamePlatformRouteCatalogIsHonest(catalog: BrowserGamePlatformRouteCatalogCandidate): boolean {
  if (browserGamePlatformRouteCatalogClaimsAuthority(catalog)) {
    return false;
  }
  if (catalog.status === 'reviewed') {
    return catalog.confidence !== 'unknown' && catalog.routes.every((route) => route.status === 'reviewed');
  }
  return catalog.confidence !== 'high' && catalog.routes.some((route) => route.status !== 'reviewed');
}

function browserGamePlatformRouteHasInconsistentPurpose(route: BrowserGamePlatformRouteContractCandidate): boolean {
  if (Schema.is(BrowserGamePlayableRouteSurfaceSchema)(route.routeSurfaceKind) && !route.managedBrowserRequired) {
    return true;
  }
  if (route.childLaunchCandidate && !Schema.is(BrowserGamePlayableRouteSurfaceSchema)(route.routeSurfaceKind)) {
    return true;
  }
  if (
    route.accountOrPurchaseCandidate &&
    route.routeSurfaceKind !== 'account-route' &&
    route.routeSurfaceKind !== 'purchase-route'
  ) {
    return true;
  }
  return route.cloudSessionCandidate && route.routeSurfaceKind !== 'cloud-session-route';
}

function browserGamePlatformRouteContractClaimsAuthority(route: BrowserGamePlatformRouteContractCandidate): boolean {
  return (
    route.rawDomainStored ||
    route.rawUrlStored ||
    route.rawPathStored ||
    route.rawPageBodyStored ||
    route.runtimeDetectionClaimed ||
    route.urlParserClaimed ||
    route.aiClassificationClaimed ||
    route.policyDecisionClaimed ||
    route.nativeGameControlClaimed ||
    route.cloudFrameAnalysisClaimed ||
    route.enforcementClaimed
  );
}

function browserGamePlatformRouteCatalogClaimsAuthority(catalog: BrowserGamePlatformRouteCatalogCandidate): boolean {
  return (
    catalog.rawDomainStored ||
    catalog.rawUrlStored ||
    catalog.rawPathStored ||
    catalog.rawPageBodyStored ||
    catalog.runtimeDetectionClaimed ||
    catalog.urlParserClaimed ||
    catalog.aiClassificationClaimed ||
    catalog.policyDecisionClaimed ||
    catalog.nativeGameControlClaimed ||
    catalog.cloudFrameAnalysisClaimed ||
    catalog.enforcementClaimed
  );
}
