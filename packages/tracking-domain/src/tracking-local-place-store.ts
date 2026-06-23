import { ActivityEvidenceRefSchema } from '@ocentra-parent/schema-domain/evidence-contracts';
import {
  TrackingNearbyPlaceEvidenceSchema,
  TrackingParentDefinedPlaceSchema,
  TrackingPlaceRiskCategorySchema,
  type TrackingNearbyPlaceEvidence,
  type TrackingParentDefinedPlace,
} from '@ocentra-parent/schema-domain/tracking-geofence';
import {
  TrackingLocalParentDefinedPlaceExportSnapshotSchema,
  TrackingLocalParentDefinedPlaceMatchSchema,
  TrackingLocalParentDefinedPlaceMutationReceiptSchema,
  TrackingLocalParentDefinedPlaceStoreSchema,
  TrackingLocalParentDefinedPlaceTombstoneSchema,
  TrackingLocalPlacePolicySignalSchema,
  type TrackingLocalParentDefinedPlaceExportSnapshot,
  type TrackingLocalParentDefinedPlaceMatch,
  type TrackingLocalParentDefinedPlaceMutationReceipt,
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

export function createTrackingLocalParentDefinedPlaceStore(
  input: CreateTrackingLocalParentDefinedPlaceStoreInput
): TrackingLocalParentDefinedPlaceStore {
  return TrackingLocalParentDefinedPlaceStoreSchema.parse({
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
  });
}

export function upsertTrackingLocalParentDefinedPlace(
  input: TrackingLocalParentDefinedPlaceUpsertInput
): TrackingLocalParentDefinedPlaceMutationReceipt {
  const places = input.store.places.filter((place) => place.placeId !== input.place.placeId);
  places.push(TrackingParentDefinedPlaceSchema.parse(input.place));
  return mutationReceipt(input.store, places, input.store.tombstones, input.place.updatedAt, input.auditRefs);
}

export function importTrackingLocalParentDefinedPlaces(
  input: TrackingLocalParentDefinedPlaceImportInput
): TrackingLocalParentDefinedPlaceMutationReceipt {
  const importedPlaces = input.places.map((place) => TrackingParentDefinedPlaceSchema.parse(place));
  const importedPlaceIds = new Set(importedPlaces.map((place) => place.placeId));
  const existingPlaces = input.store.places.filter((place) => !importedPlaceIds.has(place.placeId));
  return mutationReceipt(
    input.store,
    [...existingPlaces, ...importedPlaces],
    input.store.tombstones,
    input.importedAt,
    input.auditRefs,
    'import'
  );
}

export function deleteTrackingLocalParentDefinedPlace(
  input: TrackingLocalParentDefinedPlaceDeleteInput
): TrackingLocalParentDefinedPlaceMutationReceipt {
  const placeId = TrackingPlaceIdSchema.parse(input.placeId);
  const tombstone = TrackingLocalParentDefinedPlaceTombstoneSchema.parse({
    placeId,
    deletedAt: input.deletedAt,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
  });
  return mutationReceipt(
    input.store,
    input.store.places.filter((place) => place.placeId !== placeId),
    [...input.store.tombstones.filter((entry) => entry.placeId !== placeId), tombstone],
    input.deletedAt,
    input.auditRefs,
    'delete'
  );
}

export function exportTrackingLocalParentDefinedPlaceStore(
  input: TrackingLocalParentDefinedPlaceExportInput
): TrackingLocalParentDefinedPlaceExportSnapshot {
  return TrackingLocalParentDefinedPlaceExportSnapshotSchema.parse({
    schemaVersion: TrackingEvidenceSchemaVersion,
    storeId: input.store.storeId,
    exportedAt: input.exportedAt,
    custodyLabel: 'parent-owned-export',
    remoteSyncDefault: 'disabled',
    ocentraHostedDefaultStorage: false,
    places: input.store.places,
    tombstones: input.store.tombstones,
    auditRefs: input.auditRefs,
  });
}

export function buildTrackingLocalParentDefinedPlaceMatch(input: TrackingLocalParentDefinedPlaceMatchInput): {
  readonly match: TrackingLocalParentDefinedPlaceMatch;
  readonly nearbyPlaceEvidence: TrackingNearbyPlaceEvidence;
} {
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

function mutationReceipt(
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
