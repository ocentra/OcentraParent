pub fn tracking_runtime_generated_typescript() -> String {
    r###"import { ActivityEvidenceKind } from '@ocentra-parent/schema-domain/evidence-kinds';
import type { TrackingLocationEvidence } from '@ocentra-parent/schema-domain/tracking-evidence';
import {
  type TrackingExpectedPlaceExceptionState,
  type TrackingExpectedPlaceDecision,
  type TrackingExpectedPlaceSchedule,
  type TrackingGeofenceRule,
  type TrackingGeofenceTransition,
} from '@ocentra-parent/schema-domain/tracking-geofence';
import { TrackingReasonCodeSchema } from '@ocentra-parent/schema-domain/tracking-primitives';
import type {
  TrackingExpectedPlaceEvaluationInput,
  TrackingGeofenceEvaluationInput,
} from '@ocentra-parent/schema-domain/tracking-runtime';

const EarthRadiusMeters = 6_371_008.8;
const HalfCircleDegrees = 180;
const RadiansPerDegree = Math.PI / HalfCircleDegrees;
type TrackingReasonCode = ReturnType<typeof TrackingReasonCodeSchema.parse>;
const MillisecondsPerSecond = 1_000;
const ManualReviewCapabilityStatuses: ReadonlySet<TrackingLocationEvidence['capabilityStatus']> = new Set([
  'stale',
  'last-known',
  'offline-last-known-only',
  'permission-required',
  'background-permission-required',
  'approximate-only',
  'manual-required',
  'unavailable',
  'adapter-error',
  'disabled-by-parent',
] as const);

export function evaluateTrackingGeofenceTransitionGenerated(input: TrackingGeofenceEvaluationInput) {
  const distanceMeters = distanceFromRule(input.rule, input.location);
  const reasonCodes: TrackingReasonCode[] = [];
  let transition: TrackingGeofenceTransition['transition'] = 'ambiguous';

  if (!input.rule.enabled) {
    reasonCodes.push(reasonCode('geofence-rule-disabled'));
  } else if (input.location.capabilityStatus !== 'live' && input.location.capabilityStatus !== 'recent') {
    reasonCodes.push(reasonCode('fresh-location-required'));
  } else if (input.location.coordinate === null || distanceMeters === null) {
    reasonCodes.push(reasonCode('precise-location-required'));
  } else if (input.location.accuracyMeters === null || input.location.accuracyMeters > input.rule.minAccuracyMeters) {
    reasonCodes.push(reasonCode('location-accuracy-below-rule-threshold'));
  } else {
    const inside = locationInsideRule(input.rule, input.location, distanceMeters);
    transition = inside ? (input.wasInside ? 'dwell' : 'enter') : input.wasInside ? 'exit' : 'ambiguous';
    reasonCodes.push(reasonCode(inside ? 'inside-geofence-with-accuracy' : 'outside-geofence-with-accuracy'));
  }

  return {
    schemaVersion: input.rule.schemaVersion,
    transitionId: input.transitionId,
    observedAt: input.observedAt,
    ruleId: input.rule.ruleId,
    geofenceId: input.rule.geofenceId,
    locationEvidenceId: input.location.evidenceId,
    transition,
    capabilityStatus: input.location.capabilityStatus,
    distanceMeters,
    reasonCodes,
    evidence: [
      ...input.location.evidence,
      {
        evidenceId: input.location.evidenceId,
        kind: ActivityEvidenceKind.LocalDbRow,
        digest: null,
        uri: null,
      },
    ],
  };
}

export function evaluateTrackingExpectedPlaceDecisionGenerated(input: TrackingExpectedPlaceEvaluationInput) {
  const observedAt = Date.parse(input.observedAt);
  const activeWindow = input.schedule.windows.find((window) => {
    return observedAt >= Date.parse(window.startsAt) && observedAt <= Date.parse(window.endsAt);
  });

  const reasonCodes: TrackingReasonCode[] = [];
  let outcome: TrackingExpectedPlaceDecision['outcome'] = 'unknown';
  let exceptionState: TrackingExpectedPlaceDecision['exceptionState'] = null;
  let exceptionAuditRef: TrackingExpectedPlaceDecision['exceptionAuditRef'] = null;

  if (!input.schedule.enabled) {
    outcome = 'manual-required';
    reasonCodes.push(reasonCode('expected-place-schedule-disabled'));
  } else if (capabilityRequiresManualReview(input.location.capabilityStatus)) {
    outcome = 'manual-required';
    reasonCodes.push(reasonCode('fresh-location-required'));
  } else if (input.schedule.activeException !== null) {
    exceptionState = input.schedule.activeException.state;
    exceptionAuditRef = input.schedule.activeException.auditRef;
    reasonCodes.push(reasonCode(reasonCodeForExpectedPlaceException(exceptionState)));
  } else if (activeWindow === undefined) {
    reasonCodes.push(reasonCode('outside-expected-place-window'));
  } else if (lateGraceIsActive(input.schedule, activeWindow, observedAt, input.transition.transition)) {
    reasonCodes.push(reasonCode('expected-place-late-grace-active'));
  } else if (earlyExitGraceIsActive(input.schedule, activeWindow, observedAt, input.transition.transition)) {
    reasonCodes.push(reasonCode('expected-place-early-exit-grace-active'));
  } else if (input.transition.transition === 'enter' || input.transition.transition === 'dwell') {
    outcome = 'where-expected';
    reasonCodes.push(reasonCode('inside-expected-place-window'));
  } else if (input.transition.transition === 'exit') {
    outcome = 'left-expected-place';
    reasonCodes.push(reasonCode('exited-expected-place-window'));
  } else if (input.transition.transition === 'missed-arrival') {
    outcome = 'late-arrival';
    reasonCodes.push(reasonCode('missed-expected-place-arrival'));
  } else {
    reasonCodes.push(reasonCode('expected-place-ambiguous'));
  }

  return {
    schemaVersion: input.schedule.schemaVersion,
    decisionId: input.decisionId,
    observedAt: input.observedAt,
    scheduleId: input.schedule.scheduleId,
    ruleId: input.schedule.ruleId,
    locationEvidenceId: input.location.evidenceId,
    outcome,
    distanceToleranceMeters: input.schedule.distanceToleranceMeters,
    lateGraceSeconds: input.schedule.lateGraceSeconds,
    earlyExitGraceSeconds: input.schedule.earlyExitGraceSeconds,
    exceptionState,
    exceptionAuditRef,
    reasonCodes,
    evidence: input.transition.evidence,
  };
}

function distanceFromRule(
  rule: TrackingGeofenceRule,
  location: TrackingLocationEvidence
): TrackingGeofenceTransition['distanceMeters'] {
  if (location.coordinate === null) {
    return null;
  }

  if (rule.shape.kind === 'circle') {
    if (rule.shape.center === null) {
      return null;
    }
    return distanceMeters(
      location.coordinate.latitude,
      location.coordinate.longitude,
      rule.shape.center.latitude,
      rule.shape.center.longitude
    );
  }

  return pointInRulePolygon(rule, location) ? 0 : null;
}

function locationInsideRule(
  rule: TrackingGeofenceRule,
  location: TrackingLocationEvidence,
  distanceMeters: TrackingGeofenceTransition['distanceMeters']
) {
  if (rule.shape.kind === 'circle') {
    return rule.shape.radiusMeters !== null && distanceMeters !== null && distanceMeters <= rule.shape.radiusMeters;
  }

  return pointInRulePolygon(rule, location);
}

function pointInRulePolygon(rule: TrackingGeofenceRule, location: TrackingLocationEvidence) {
  if (rule.shape.kind !== 'polygon' || location.coordinate === null) {
    return false;
  }

  let inside = false;
  const { latitude, longitude } = location.coordinate;
  for (let index = 0, previous = rule.shape.polygon.length - 1; index < rule.shape.polygon.length; previous = index++) {
    const currentPoint = rule.shape.polygon[index];
    const previousPoint = rule.shape.polygon[previous];
    if (currentPoint === undefined || previousPoint === undefined) {
      continue;
    }
    const currentLongitudeCrosses = currentPoint.longitude > longitude;
    const previousLongitudeCrosses = previousPoint.longitude > longitude;
    const intersects =
      currentLongitudeCrosses !== previousLongitudeCrosses &&
      latitude <
        ((previousPoint.latitude - currentPoint.latitude) * (longitude - currentPoint.longitude)) /
          (previousPoint.longitude - currentPoint.longitude) +
          currentPoint.latitude;
    if (intersects) {
      inside = !inside;
    }
  }
  return inside;
}

function distanceMeters(startLatitude: number, startLongitude: number, endLatitude: number, endLongitude: number) {
  const startLatitudeRadians = startLatitude * RadiansPerDegree;
  const endLatitudeRadians = endLatitude * RadiansPerDegree;
  const deltaLatitude = (endLatitude - startLatitude) * RadiansPerDegree;
  const deltaLongitude = (endLongitude - startLongitude) * RadiansPerDegree;
  const arc =
    Math.sin(deltaLatitude / 2) * Math.sin(deltaLatitude / 2) +
    Math.cos(startLatitudeRadians) *
      Math.cos(endLatitudeRadians) *
      Math.sin(deltaLongitude / 2) *
      Math.sin(deltaLongitude / 2);
  return Math.round(EarthRadiusMeters * 2 * Math.atan2(Math.sqrt(arc), Math.sqrt(1 - arc)));
}

function capabilityRequiresManualReview(capabilityStatus: TrackingLocationEvidence['capabilityStatus']) {
  return ManualReviewCapabilityStatuses.has(capabilityStatus);
}

function reasonCodeForExpectedPlaceException(exceptionState: TrackingExpectedPlaceExceptionState) {
  if (exceptionState === 'holiday-mode') {
    return 'expected-place-holiday-exception-active';
  }

  return 'expected-place-trip-exception-active';
}

function lateGraceIsActive(
  schedule: TrackingExpectedPlaceSchedule,
  activeWindow: TrackingExpectedPlaceSchedule['windows'][number],
  observedAt: number,
  transition: TrackingGeofenceTransition['transition']
) {
  if (transition !== 'missed-arrival' || schedule.lateGraceSeconds === 0) {
    return false;
  }

  const startsAt = Date.parse(activeWindow.startsAt);
  return observedAt >= startsAt && observedAt <= startsAt + schedule.lateGraceSeconds * MillisecondsPerSecond;
}

function earlyExitGraceIsActive(
  schedule: TrackingExpectedPlaceSchedule,
  activeWindow: TrackingExpectedPlaceSchedule['windows'][number],
  observedAt: number,
  transition: TrackingGeofenceTransition['transition']
) {
  if (transition !== 'exit' || schedule.earlyExitGraceSeconds === 0) {
    return false;
  }

  const endsAt = Date.parse(activeWindow.endsAt);
  return observedAt >= endsAt - schedule.earlyExitGraceSeconds * MillisecondsPerSecond && observedAt < endsAt;
}

function reasonCode(value: unknown) {
  return TrackingReasonCodeSchema.parse(value);
}
"###
        .to_owned()
}

pub fn tracking_retention_runtime_generated_typescript() -> String {
    r###"export function applyTrackingRetentionDeleteGenerated(input: {
  readonly readModel: {
    readonly capabilityStatus: string;
    readonly locationRows: readonly { readonly evidenceId: string }[];
    readonly deviceStatusRows: readonly { readonly lastLocationEvidenceId: string | null }[];
    readonly geofenceTransitions: readonly { readonly locationEvidenceId: string }[];
    readonly expectedPlaceDecisions: readonly { readonly locationEvidenceId: string }[];
    readonly nearbyPlaceRows: readonly { readonly locationEvidenceId: string }[];
    readonly timeline: readonly { readonly rowId: string }[];
  };
  readonly generatedAt: string;
  readonly deletedEvidenceIds: readonly string[];
}) {
  const deleted = new Set(input.deletedEvidenceIds);
  const locationRows = input.readModel.locationRows.filter((row) => !deleted.has(row.evidenceId));
  const deviceStatusRows = input.readModel.deviceStatusRows.filter(
    (row) => row.lastLocationEvidenceId === null || !deleted.has(row.lastLocationEvidenceId)
  );
  const geofenceTransitions = input.readModel.geofenceTransitions.filter((row) => !deleted.has(row.locationEvidenceId));
  const expectedPlaceDecisions = input.readModel.expectedPlaceDecisions.filter(
    (row) => !deleted.has(row.locationEvidenceId)
  );
  const nearbyPlaceRows = input.readModel.nearbyPlaceRows.filter((row) => !deleted.has(row.locationEvidenceId));
  const timeline = input.readModel.timeline.filter((row) => !deleted.has(row.rowId));

  return {
    beforeLocationRows: input.readModel.locationRows.length,
    afterLocationRows: locationRows.length,
    deletedEvidenceIds: input.deletedEvidenceIds,
    readModel: {
      ...input.readModel,
      generatedAt: input.generatedAt,
      returned: timeline.length,
      locationRows,
      deviceStatusRows,
      geofenceTransitions,
      expectedPlaceDecisions,
      nearbyPlaceRows,
      timeline,
      capabilityStatus: locationRows.length === 0 ? 'stale' : input.readModel.capabilityStatus,
    },
  };
}

export function applyTrackingRetentionExportGenerated(input: {
  readonly readModel: {
    readonly capabilityStatus: string;
    readonly locationRows: readonly { readonly custodyLabel: string; readonly retentionMode: string }[];
    readonly deviceStatusRows: readonly { readonly custodyLabel: string; readonly retentionMode: string }[];
    readonly timeline: readonly unknown[];
  };
  readonly generatedAt: string;
  readonly policy: {
    readonly exportAllowed: boolean;
    readonly custodyLabel: string;
    readonly mode: string;
    readonly remoteSyncDefault: string;
  };
}) {
  const locationRows = input.policy.exportAllowed
    ? input.readModel.locationRows.map((row) => ({
        ...row,
        custodyLabel: input.policy.custodyLabel,
        retentionMode: input.policy.mode,
      }))
    : [];
  const deviceStatusRows = input.policy.exportAllowed
    ? input.readModel.deviceStatusRows.map((row) => ({
        ...row,
        custodyLabel: input.policy.custodyLabel,
        retentionMode: input.policy.mode,
      }))
    : [];
  const timeline = input.policy.exportAllowed ? input.readModel.timeline : [];

  return {
    exportAllowed: input.policy.exportAllowed,
    sourceLocationRows: input.readModel.locationRows.length,
    exportedLocationRows: locationRows.length,
    custodyLabel: input.policy.custodyLabel,
    retentionMode: input.policy.mode,
    remoteSyncDefault: input.policy.remoteSyncDefault,
    readModel: {
      ...input.readModel,
      generatedAt: input.generatedAt,
      custodyLabel: input.policy.custodyLabel,
      capabilityStatus: input.policy.exportAllowed ? input.readModel.capabilityStatus : 'unavailable',
      returned: timeline.length,
      locationRows,
      deviceStatusRows,
      retentionPolicies: [input.policy],
      timeline,
    },
  };
}
"###
        .to_owned()
}

pub fn tracking_local_place_store_generated_typescript() -> String {
    r###"import { ActivityEvidenceRefSchema } from '@ocentra-parent/schema-domain/evidence-contracts';
import {
  TrackingNearbyPlaceEvidenceSchema,
  TrackingParentDefinedPlaceSchema,
  TrackingPlaceRiskCategorySchema,
  type TrackingParentDefinedPlace,
} from '@ocentra-parent/schema-domain/tracking-geofence';
import {
  TrackingLocalParentDefinedPlaceMatchSchema,
  TrackingLocalParentDefinedPlaceMutationReceiptSchema,
  TrackingLocalParentDefinedPlaceStoreSchema,
  TrackingLocalParentDefinedPlaceTombstoneSchema,
  TrackingLocalPlacePolicySignalSchema,
  type TrackingLocalParentDefinedPlaceStore,
  type TrackingLocalParentDefinedPlaceTombstone,
  type TrackingLocalPlaceMutationKind,
  type TrackingLocalPlacePolicySignal,
} from '@ocentra-parent/schema-domain/tracking-local-place-store-schemas';
import type {
  CreateTrackingLocalParentDefinedPlaceStoreInput,
  TrackingLocalParentDefinedPlaceDeleteInput,
  TrackingLocalParentDefinedPlaceExportInput,
  TrackingLocalParentDefinedPlaceImportInput,
  TrackingLocalParentDefinedPlaceMatchInput,
  TrackingLocalParentDefinedPlaceUpsertInput,
} from '@ocentra-parent/schema-domain/tracking-local-place-store';
import {
  TrackingEvidenceSchemaVersion,
  TrackingPlaceIdSchema,
  TrackingProviderRefSchema,
} from '@ocentra-parent/schema-domain/tracking-primitives';

export function createTrackingLocalParentDefinedPlaceStoreGenerated(
  input: CreateTrackingLocalParentDefinedPlaceStoreInput
) {
  return {
    schemaVersion: TrackingEvidenceSchemaVersion,
    storeId: input.storeId,
    createdAt: input.createdAt,
    updatedAt: input.createdAt,
    storageBoundary: 'parent-device-local',
    remoteSyncDefault: 'disabled',
    ocentraHostedDefaultStorage: false,
    places: [],
    tombstones: [],
    auditRefs: input.auditRefs,
  };
}

export function upsertTrackingLocalParentDefinedPlaceGenerated(input: TrackingLocalParentDefinedPlaceUpsertInput) {
  const places = input.store.places.filter((place) => place.placeId !== input.place.placeId);
  places.push(TrackingParentDefinedPlaceSchema.parse(input.place));
  return mutationReceiptGenerated(input.store, places, input.store.tombstones, input.place.updatedAt, input.auditRefs);
}

export function importTrackingLocalParentDefinedPlacesGenerated(input: TrackingLocalParentDefinedPlaceImportInput) {
  const importedPlaces = input.places.map((place) => TrackingParentDefinedPlaceSchema.parse(place));
  const importedPlaceIds = new Set(importedPlaces.map((place) => place.placeId));
  const existingPlaces = input.store.places.filter((place) => !importedPlaceIds.has(place.placeId));
  return mutationReceiptGenerated(
    input.store,
    [...existingPlaces, ...importedPlaces],
    input.store.tombstones,
    input.importedAt,
    input.auditRefs,
    'import'
  );
}

export function deleteTrackingLocalParentDefinedPlaceGenerated(input: TrackingLocalParentDefinedPlaceDeleteInput) {
  const placeId = TrackingPlaceIdSchema.parse(input.placeId);
  const tombstone = TrackingLocalParentDefinedPlaceTombstoneSchema.parse({
    placeId,
    deletedAt: input.deletedAt,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
  });
  return mutationReceiptGenerated(
    input.store,
    input.store.places.filter((place) => place.placeId !== placeId),
    [...input.store.tombstones.filter((entry) => entry.placeId !== placeId), tombstone],
    input.deletedAt,
    input.auditRefs,
    'delete'
  );
}

export function exportTrackingLocalParentDefinedPlaceStoreGenerated(input: TrackingLocalParentDefinedPlaceExportInput) {
  return {
    schemaVersion: TrackingEvidenceSchemaVersion,
    storeId: input.store.storeId,
    exportedAt: input.exportedAt,
    custodyLabel: 'parent-owned-export',
    remoteSyncDefault: 'disabled',
    ocentraHostedDefaultStorage: false,
    places: input.store.places,
    tombstones: input.store.tombstones,
    auditRefs: input.auditRefs,
  };
}

export function buildTrackingLocalParentDefinedPlaceMatchGenerated(input: TrackingLocalParentDefinedPlaceMatchInput) {
  const place = input.store.places.find((candidate) => candidate.placeId === input.placeId);
  if (place === undefined) {
    throw new Error('tracking local parent-defined place match needs an existing place');
  }
  const match = TrackingLocalParentDefinedPlaceMatchSchema.parse({
    placeId: place.placeId,
    placeKind: place.placeKind,
    distanceMeters: input.distanceMeters,
    queryRadiusMeters: input.queryRadiusMeters,
    ambiguityState: 'clear',
    policySignal: policySignalForPlace(place),
    reasonCodes: input.reasonCodes,
  });
  const nearbyPlaceEvidence = TrackingNearbyPlaceEvidenceSchema.parse({
    schemaVersion: TrackingEvidenceSchemaVersion,
    evidenceId: input.evidenceId,
    observedAt: input.observedAt,
    locationEvidenceId: input.locationEvidenceId,
    providerKind: 'parent-defined',
    providerRef: TrackingProviderRefSchema.parse(input.store.storeId),
    queryRadiusMeters: input.queryRadiusMeters,
    distanceMeters: input.distanceMeters,
    category: categoryForPlace(place),
    confidence: input.confidence,
    ambiguityState: match.ambiguityState,
    reasonCodes: input.reasonCodes,
    evidence: input.evidence.map((entry) => ActivityEvidenceRefSchema.parse(entry)),
  });
  return { match, nearbyPlaceEvidence };
}

function mutationReceiptGenerated(
  store: TrackingLocalParentDefinedPlaceStore,
  places: readonly TrackingParentDefinedPlace[],
  tombstones: readonly TrackingLocalParentDefinedPlaceTombstone[],
  updatedAt: TrackingLocalParentDefinedPlaceStore['updatedAt'],
  auditRefs: readonly TrackingLocalParentDefinedPlaceStore['auditRefs'][number][],
  operation: TrackingLocalPlaceMutationKind = places.some((place) =>
    store.places.some((existing) => existing.placeId === place.placeId)
  )
    ? 'update'
    : 'create'
) {
  const nextStore = TrackingLocalParentDefinedPlaceStoreSchema.parse({
    ...store,
    updatedAt,
    places,
    tombstones,
    auditRefs: [...store.auditRefs, ...auditRefs],
  });
  return TrackingLocalParentDefinedPlaceMutationReceiptSchema.parse({
    schemaVersion: TrackingEvidenceSchemaVersion,
    storeId: store.storeId,
    operation,
    beforePlaceCount: store.places.length,
    afterPlaceCount: nextStore.places.length,
    remoteSyncDefault: 'disabled',
    ocentraHostedDefaultStorage: false,
    auditRefs,
    store: nextStore,
  });
}

function policySignalForPlace(place: TrackingParentDefinedPlace): TrackingLocalPlacePolicySignal {
  if (place.placeKind === 'safe-zone') return TrackingLocalPlacePolicySignalSchema.parse('safe-zone-context');
  if (place.placeKind === 'restricted-zone') {
    return TrackingLocalPlacePolicySignalSchema.parse('restricted-zone-attention');
  }
  if (place.placeKind === 'home' || place.placeKind === 'school' || place.placeKind === 'activity') {
    return TrackingLocalPlacePolicySignalSchema.parse('expected-place-context');
  }
  return TrackingLocalPlacePolicySignalSchema.parse('custom-place-context');
}

function categoryForPlace(place: TrackingParentDefinedPlace) {
  if (place.placeKind === 'home') return TrackingPlaceRiskCategorySchema.parse('home');
  if (place.placeKind === 'school') return TrackingPlaceRiskCategorySchema.parse('school');
  if (place.placeKind === 'restricted-zone') return TrackingPlaceRiskCategorySchema.parse('unknown');
  if (place.placeKind === 'custom') return TrackingPlaceRiskCategorySchema.parse('unknown');
  return TrackingPlaceRiskCategorySchema.parse('park');
}
"###
        .to_owned()
}

pub fn tracking_poi_provider_adapter_generated_typescript() -> String {
    r##########"import {
  GooglePlacesNearbySearchResponseSchema,
  TrackingGooglePlacesNearbyRequestSchema,
  TrackingGooglePlacesNearbySearchInputSchema,
  TrackingPoiAmbiguityState,
  TrackingPoiCandidateSchema,
  TrackingPoiCategory,
  TrackingPoiProviderId,
  TrackingPoiProviderParityRowSchema,
  TrackingPoiProviderParityStatus,
  TrackingPoiProviderReadModelSchema,
  TrackingPoiProviderStatus,
  type GooglePlacesNearbySearchResponse,
  type TrackingGooglePlacesNearbySearchInput,
  type TrackingPoiProviderParityRow,
  type TrackingPoiProviderReadModel,
} from '@ocentra-parent/schema-domain/tracking-poi-provider-adapter';
import { withParser } from '@ocentra-parent/schema-domain/effect';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';

type GooglePlace = GooglePlacesNearbySearchResponse['places'][number];
type TrackingPoiAmbiguityStateValue = TrackingPoiProviderReadModel['candidates'][number]['ambiguityState'];
type TrackingPoiCategoryValue = TrackingPoiProviderReadModel['candidates'][number]['category'];

const TrackingPolicyAuditRefParsedSchema = withParser(TrackingPolicyAuditRefSchema);

export function buildGooglePlacesNearbySearchRequestGenerated(input: TrackingGooglePlacesNearbySearchInput) {
  const parsed = TrackingGooglePlacesNearbySearchInputSchema.parse(input);

  return TrackingGooglePlacesNearbyRequestSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    provider: parsed.provider,
    status: TrackingPoiProviderStatus.RequestReady,
    requestId: parsed.requestId,
    method: 'POST',
    endpointRef: 'places.googleapis.com/v1/places:searchNearby',
    fieldMaskHeader: parsed.fieldMask.join(','),
    body: {
      includedTypes: parsed.includedTypes,
      maxResultCount: parsed.maxResultCount,
      locationRestriction: {
        circle: {
          center: {
            latitude: parsed.center.latitude,
            longitude: parsed.center.longitude,
          },
          radius: parsed.radiusMeters,
        },
      },
    },
    credentialsStored: false,
    liveProviderRequestClaimed: false,
    reasonCodes: ['google-places-nearby-request-ready'],
    auditRefs: parsed.auditRefs,
  });
}

export function buildGooglePlacesNearbyReadModelGenerated(
  input: TrackingGooglePlacesNearbySearchInput,
  response: GooglePlacesNearbySearchResponse
) {
  const parsedInput = TrackingGooglePlacesNearbySearchInputSchema.parse(input);
  const parsedResponse = GooglePlacesNearbySearchResponseSchema.parse(response);
  const request = buildGooglePlacesNearbySearchRequestGenerated(parsedInput);
  const ambiguityState = ambiguityFor(parsedInput.center.accuracyMeters, parsedResponse.places.length);
  const candidates = parsedResponse.places.map((place) => candidateFor(parsedInput, place, ambiguityState));

  return TrackingPoiProviderReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    provider: parsedInput.provider,
    status: candidates.length > 0 ? TrackingPoiProviderStatus.ResponseMapped : TrackingPoiProviderStatus.ManualRequired,
    generatedAt: parsedInput.requestedAt,
    request,
    candidates,
    radiusMeters: parsedInput.radiusMeters,
    fieldMask: parsedInput.fieldMask,
    providerFailureReason: candidates.length > 0 ? null : 'nearby-place-no-candidates',
    locationRestrictionApplied: true,
    wildcardFieldMaskRejected: true,
    credentialsStored: false,
    liveProviderRequestClaimed: false,
    exactPlaceClaimed: false,
    physicalDeviceProofClaimed: false,
    reasonCodes:
      candidates.length > 0
        ? ['google-places-response-mapped', 'nearby-place-ambiguity-preserved']
        : ['nearby-place-no-candidates'],
    auditRefs: parsedInput.auditRefs,
  });
}

export function buildGooglePlacesProviderFailureReadModelGenerated(
  input: TrackingGooglePlacesNearbySearchInput,
  reason: string
) {
  const parsed = TrackingGooglePlacesNearbySearchInputSchema.parse(input);

  return TrackingPoiProviderReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    provider: parsed.provider,
    status: TrackingPoiProviderStatus.ProviderUnavailable,
    generatedAt: parsed.requestedAt,
    request: buildGooglePlacesNearbySearchRequestGenerated(parsed),
    candidates: [],
    radiusMeters: parsed.radiusMeters,
    fieldMask: parsed.fieldMask,
    providerFailureReason: reason,
    locationRestrictionApplied: true,
    wildcardFieldMaskRejected: true,
    credentialsStored: false,
    liveProviderRequestClaimed: false,
    exactPlaceClaimed: false,
    physicalDeviceProofClaimed: false,
    reasonCodes: ['google-places-provider-unavailable', reason],
    auditRefs: parsed.auditRefs,
  });
}

export function buildTrackingPoiProviderParityRowsGenerated(readModel: TrackingPoiProviderReadModel) {
  const parsed = TrackingPoiProviderReadModelSchema.parse(readModel);
  const sourceProofRef = parsed.auditRefs[0] ?? auditRef('nearby-place-provider-proof');

  return [
    providerParityRow({
      schemaVersion: TrackingPolicySchemaVersion,
      provider: TrackingPoiProviderId.GooglePlacesNearby,
      status: TrackingPoiProviderParityStatus.RequestMapped,
      generatedAt: parsed.generatedAt,
      sourceProofRef,
      providerTermsReviewRequired: false,
      providerCredentialsRequired: true,
      boundedLocationRestrictionRequired: true,
      ambiguityPreservedRequired: true,
      exactPlaceClaimed: false,
      liveProviderRequestClaimed: false,
      credentialsStored: false,
      physicalDeviceProofClaimed: false,
      reasonCodes: [
        reasonCode('google-places-provider-contract-ready'),
        reasonCode('nearby-place-ambiguity-preserved'),
      ],
      auditRefs: parsed.auditRefs,
    }),
    providerParityRow({
      schemaVersion: TrackingPolicySchemaVersion,
      provider: TrackingPoiProviderId.AppleMapKitSearch,
      status: TrackingPoiProviderParityStatus.ManualRequired,
      generatedAt: parsed.generatedAt,
      sourceProofRef,
      providerTermsReviewRequired: true,
      providerCredentialsRequired: true,
      boundedLocationRestrictionRequired: true,
      ambiguityPreservedRequired: true,
      exactPlaceClaimed: false,
      liveProviderRequestClaimed: false,
      credentialsStored: false,
      physicalDeviceProofClaimed: false,
      reasonCodes: [
        reasonCode('apple-mapkit-provider-parity-manual-required'),
        reasonCode('provider-runtime-not-proved'),
      ],
      auditRefs: [auditRef('apple-mapkit-provider-parity-required'), ...parsed.auditRefs],
    }),
    providerParityRow({
      schemaVersion: TrackingPolicySchemaVersion,
      provider: TrackingPoiProviderId.OpenStreetMapNominatim,
      status: TrackingPoiProviderParityStatus.ManualRequired,
      generatedAt: parsed.generatedAt,
      sourceProofRef,
      providerTermsReviewRequired: true,
      providerCredentialsRequired: false,
      boundedLocationRestrictionRequired: true,
      ambiguityPreservedRequired: true,
      exactPlaceClaimed: false,
      liveProviderRequestClaimed: false,
      credentialsStored: false,
      physicalDeviceProofClaimed: false,
      reasonCodes: [reasonCode('osm-provider-parity-manual-required'), reasonCode('provider-runtime-not-proved')],
      auditRefs: [auditRef('osm-provider-parity-required'), ...parsed.auditRefs],
    }),
  ] as const satisfies readonly TrackingPoiProviderParityRow[];
}

function providerParityRow(input: TrackingPoiProviderParityRow): TrackingPoiProviderParityRow {
  return TrackingPoiProviderParityRowSchema.parse(input);
}

function reasonCode(value: string) {
  return TrackingPolicyReasonCodeSchema.parse(value);
}

function auditRef(value: string) {
  return TrackingPolicyAuditRefParsedSchema.parse(value);
}

function candidateFor(
  input: TrackingGooglePlacesNearbySearchInput,
  place: GooglePlace,
  ambiguityState: TrackingPoiAmbiguityStateValue
) {
  const distanceMeters = distanceBetweenMeters(input.center, place.location);
  return TrackingPoiCandidateSchema.parse({
    providerPlaceId: place.id,
    providerResourceName: place.name,
    displayName: place.displayName.text,
    primaryType: place.primaryType,
    category: categoryFor([place.primaryType, ...place.types]),
    distanceMeters,
    confidence: confidenceFor(distanceMeters, input.radiusMeters, ambiguityState),
    ambiguityState,
    evidenceReferenceId: input.center.evidenceReferenceId,
    reasonCodes: ['google-places-candidate-mapped', `google-primary-type-${place.primaryType}`],
  });
}

function ambiguityFor(accuracyMeters: number, candidateCount: number): TrackingPoiAmbiguityStateValue {
  if (candidateCount === 0) {
    return TrackingPoiAmbiguityState.NoCandidates;
  }
  if (accuracyMeters > 75) {
    return TrackingPoiAmbiguityState.LowAccuracy;
  }
  if (candidateCount > 1) {
    return TrackingPoiAmbiguityState.MultipleCandidates;
  }
  return TrackingPoiAmbiguityState.SingleCandidate;
}

function confidenceFor(
  distanceMeters: number,
  radiusMeters: number,
  ambiguityState: TrackingPoiAmbiguityStateValue
): number {
  const distanceScore = Math.max(0.1, 1 - distanceMeters / radiusMeters);
  const ambiguityPenalty =
    ambiguityState === TrackingPoiAmbiguityState.SingleCandidate
      ? 1
      : ambiguityState === TrackingPoiAmbiguityState.MultipleCandidates
        ? 0.72
        : ambiguityState === TrackingPoiAmbiguityState.LowAccuracy
          ? 0.5
          : 0.2;

  return Number(Math.max(0.1, Math.min(0.95, distanceScore * ambiguityPenalty)).toFixed(2));
}

function categoryFor(types: readonly string[]): TrackingPoiCategoryValue {
  if (types.some((type) => type.includes('school'))) {
    return TrackingPoiCategory.School;
  }
  if (types.some((type) => type.includes('restaurant') || type.includes('cafe') || type.includes('food'))) {
    return TrackingPoiCategory.Food;
  }
  if (types.some((type) => type.includes('store') || type.includes('shopping'))) {
    return TrackingPoiCategory.Store;
  }
  if (types.some((type) => type.includes('transit') || type.includes('bus') || type.includes('train'))) {
    return TrackingPoiCategory.Transit;
  }
  if (types.some((type) => type.includes('hospital') || type.includes('doctor') || type.includes('pharmacy'))) {
    return TrackingPoiCategory.Healthcare;
  }
  if (types.some((type) => type.includes('bar') || type.includes('casino') || type.includes('liquor'))) {
    return TrackingPoiCategory.Sensitive;
  }
  return TrackingPoiCategory.Unknown;
}

function distanceBetweenMeters(
  from: { readonly latitude: number; readonly longitude: number },
  to: { readonly latitude: number; readonly longitude: number }
): number {
  const earthRadiusMeters = 6_371_000;
  const fromLatitude = radians(from.latitude);
  const toLatitude = radians(to.latitude);
  const latitudeDelta = radians(to.latitude - from.latitude);
  const longitudeDelta = radians(to.longitude - from.longitude);
  const haversine =
    Math.sin(latitudeDelta / 2) ** 2 +
    Math.cos(fromLatitude) * Math.cos(toLatitude) * Math.sin(longitudeDelta / 2) ** 2;

  return Math.round(earthRadiusMeters * 2 * Math.atan2(Math.sqrt(haversine), Math.sqrt(1 - haversine)));
}

function radians(value: number): number {
  return (value * Math.PI) / 180;
}
"##########
        .to_owned()
}

pub fn tracking_policy_compiler_runtime_proof_generated_typescript() -> String {
    r##########"import {
  TrackingAlertIntentSchema,
  TrackingChildCheckInRequestSchema,
  TrackingEscalationChainSchema,
  TrackingPolicyDecisionSchema,
  TrackingTemporaryLiveTrackingGrantSchema,
} from '@ocentra-parent/schema-domain/tracking-location-policy';
import {
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';
import type {
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingEscalationChain,
  TrackingPolicyDecision,
  TrackingPolicyRule,
  TrackingTemporaryLiveTrackingGrant,
} from '@ocentra-parent/schema-domain/tracking-location-policy-types';
import {
  TrackingPolicyCompilerRuntimeProofRequestSchema,
  TrackingPolicyCompilerRuntimeProofResultSchema,
  type TrackingPolicyCompilerRuntimeProofRequest,
  type TrackingPolicyCompilerRuntimeProofResult,
} from '@ocentra-parent/schema-domain/tracking-policy-compiler-runtime-proof';

export function compileTrackingPolicyRuntimeProofDecisionGenerated(
  input: TrackingPolicyCompilerRuntimeProofRequest
) {
  const request = TrackingPolicyCompilerRuntimeProofRequestSchema.parse(input);
  const finalActionSource = finalActionSourceFor(request);
  const action = actionFor(request, finalActionSource);
  const reasonCodes = reasonCodesFor(request, action, finalActionSource);
  const alertIntentId = trackingPolicyCompilerActionNeedsAlertGenerated(action) ? request.alertId : null;
  const decision = TrackingPolicyDecisionSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId: request.decisionId,
    decidedAt: request.decidedAt,
    ruleId: request.rule.ruleId,
    action,
    dryRun: request.compilerMode === 'dry-run',
    evidenceReferences: request.evidenceReferences,
    aiAnalysisId: request.aiAnalysis?.analysisId ?? null,
    alertIntentId,
    reasonCodes,
    auditRefs: request.auditRefs,
  });

  return TrackingPolicyCompilerRuntimeProofResultSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    requestId: request.requestId,
    requestedAction: request.requestedAction,
    finalActionSource,
    decision,
    alertIntent: alertFor(request, decision, action, reasonCodes),
    childCheckInRequest: childCheckInFor(request, action),
    escalationChain: escalationFor(request, action),
    temporaryLiveGrant: liveGrantFor(request, action),
    parentPolicyFinalAuthority: true,
    aiFinalAuthority: false,
    runtimeEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    platformAdapterClaimed: false,
    physicalDeviceClaimed: false,
    productionWorkerClaimed: false,
  });
}

export function trackingPolicyCompilerActionNeedsAlertGenerated(action: TrackingPolicyRule['action']): boolean {
  return action === 'notify-parent' || action === 'request-parent-acknowledgement' || action === 'escalate';
}

function actionFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  finalActionSource: TrackingPolicyCompilerRuntimeProofResult['finalActionSource']
): TrackingPolicyDecision['action'] {
  if (finalActionSource === 'disabled-rule') return 'no-action';
  if (finalActionSource === 'manual-required') return 'manual-required';
  if (request.requestedAction === 'suppress' && request.rule.action === 'no-action') return 'no-action';
  return request.rule.action;
}

function finalActionSourceFor(
  request: TrackingPolicyCompilerRuntimeProofRequest
): TrackingPolicyCompilerRuntimeProofResult['finalActionSource'] {
  if (!request.rule.enabled) return 'disabled-rule';
  if (
    request.platformManualRequired ||
    (request.rule.requiresFreshEvidence && !request.freshEvidenceAvailable) ||
    (request.rule.requiresParentConfirmation && !request.parentConfirmationReceived)
  ) {
    return 'manual-required';
  }
  return 'parent-policy-rule';
}

function alertFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  decision: TrackingPolicyDecision,
  action: TrackingPolicyDecision['action'],
  reasonCodes: readonly TrackingPolicyDecision['reasonCodes'][number][]
): TrackingAlertIntent | null {
  if (!trackingPolicyCompilerActionNeedsAlertGenerated(action) || request.alertId === null) return null;
  return TrackingAlertIntentSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    alertId: request.alertId,
    createdAt: request.decidedAt,
    severity: alertSeverityFor(request, action),
    policyDecisionId: decision.decisionId,
    evidenceReferences: request.evidenceReferences,
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: request.auditRefs,
    acknowledgementId: null,
    reasonCodes,
  });
}

function childCheckInFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingChildCheckInRequest | null {
  if (action !== 'ask-child-check-in' || request.checkInId === null) return null;
  return TrackingChildCheckInRequestSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId: request.checkInId,
    requestedAt: request.decidedAt,
    state: 'sent',
    relatedAlertId: request.alertId,
    includeLocationIfPermitted: true,
    expiresAt: request.followUpExpiresAt,
    evidenceReferences: request.evidenceReferences,
    auditRefs: request.auditRefs,
  });
}

function escalationFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingEscalationChain | null {
  if (action !== 'escalate' || request.alertId === null || request.escalationId === null) return null;
  return TrackingEscalationChainSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    escalationId: request.escalationId,
    alertId: request.alertId,
    state: 'waiting-for-parent',
    startedAt: request.decidedAt,
    nextActionAt: request.followUpExpiresAt,
    steps: ['notify-parent', 'ask-child-check-in', 'manual-review'],
    auditRefs: request.auditRefs,
  });
}

function liveGrantFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingTemporaryLiveTrackingGrant | null {
  if (
    action !== 'start-temporary-live-tracking' ||
    request.liveTrackingGrantId === null ||
    request.liveTrackingDurationSeconds === null
  ) {
    return null;
  }
  return TrackingTemporaryLiveTrackingGrantSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    grantId: request.liveTrackingGrantId,
    state: 'requested',
    requestedAt: request.decidedAt,
    expiresAt: request.followUpExpiresAt,
    durationSeconds: request.liveTrackingDurationSeconds,
    parentApproved: request.parentConfirmationReceived,
    childDisclosureRequired: true,
    auditRefs: request.auditRefs,
  });
}

function alertSeverityFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingAlertIntent['severity'] {
  if (request.requestedAction === 'critical-alert') return 'critical';
  if (action === 'escalate') return 'urgent';
  return request.alertSeverity ?? 'watch';
}

function reasonCodesFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action'],
  finalActionSource: TrackingPolicyCompilerRuntimeProofResult['finalActionSource']
) {
  const reasonCodes = [
    ...request.rule.reasonCodes,
    ...request.reasonCodes,
    reasonCode('parent-policy-final-authority'),
    reasonCodeForAction(action),
  ];
  if (finalActionSource === 'disabled-rule') reasonCodes.push(reasonCode('tracking-rule-disabled'));
  if (finalActionSource === 'manual-required') reasonCodes.push(reasonCode('tracking-manual-required'));
  if (candidateDiffersFromAction(request, action)) reasonCodes.push(reasonCode('parent-policy-overrode-candidate'));
  if (request.aiAnalysis !== null) reasonCodes.push(reasonCode('ai-evidence-not-final-authority'));
  return [...new Set(reasonCodes)];
}

function candidateDiffersFromAction(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): boolean {
  if (request.requestedAction === 'critical-alert') return action !== 'notify-parent' && action !== 'escalate';
  if (request.requestedAction === 'suppress') return action !== 'no-action';
  return request.requestedAction !== action;
}

function reasonCodeForAction(action: TrackingPolicyDecision['action']) {
  return reasonCode(`tracking-policy-action-${action}`);
}

function reasonCode(value: string) {
  return TrackingPolicyReasonCodeSchema.parse(value);
}
"##########
        .to_owned()
}

pub fn tracking_control_catalog_schema_generated_typescript() -> String {
    include_str!("tracking_control_catalog_schema.ts.txt").to_owned()
}

pub fn tracking_control_catalog_metadata_generated_typescript() -> String {
    include_str!("tracking_control_catalog_metadata.ts.txt").to_owned()
}

pub fn tracking_control_catalog_generated_typescript() -> String {
    include_str!("tracking_control_catalog.ts.txt").to_owned()
}
