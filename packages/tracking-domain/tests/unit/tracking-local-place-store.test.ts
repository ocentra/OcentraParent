import { describe, expect, it } from 'vitest';
import { EvidenceRef, LocationEvidence } from './tracking-fixtures';
import { TrackingParentDefinedPlaceSchema } from '@ocentra-parent/schema-domain/tracking-geofence';
import { TrackingLocalParentDefinedPlaceStoreSchema } from '@ocentra-parent/schema-domain/tracking-local-place-store-schemas';
import {
  buildTrackingLocalParentDefinedPlaceMatch,
  createTrackingLocalParentDefinedPlaceStore,
  deleteTrackingLocalParentDefinedPlace,
  exportTrackingLocalParentDefinedPlaceStore,
  importTrackingLocalParentDefinedPlaces,
  upsertTrackingLocalParentDefinedPlace,
} from '../../src/tracking-local-place-store';

const Store = createTrackingLocalParentDefinedPlaceStore({
  storeId: 'parent-local-place-store',
  createdAt: '2026-06-03T01:00:00.000Z',
  auditRefs: ['parent-local-place-store-created'],
});

const HomePlace = place('home', 'Home', 'home', '2026-06-03T01:00:00.000Z', 'parent-defined-home-created');
const RestrictedPlace = place(
  'restricted-lot',
  'Restricted lot',
  'restricted-zone',
  '2026-06-03T01:05:00.000Z',
  'parent-defined-restricted-zone-created'
);
const SafePlace = place(
  'safe-library',
  'Safe library',
  'safe-zone',
  '2026-06-03T01:10:00.000Z',
  'parent-defined-safe-zone-imported'
);
type PlaceUpdatedAt = '2026-06-03T01:00:00.000Z' | '2026-06-03T01:05:00.000Z' | '2026-06-03T01:10:00.000Z';
type PlaceAuditRef =
  | 'parent-defined-home-created'
  | 'parent-defined-restricted-zone-created'
  | 'parent-defined-safe-zone-imported';

describe('tracking local parent-defined place store', () => {
  registersCreateUpdateProof();
  registersImportExportProof();
  registersDeleteProof();
  registersNearbyPolicySignalProof();
  registersStorageBoundaryNegativeProof();
});

function registersCreateUpdateProof() {
  it('creates and updates a parent-owned local place store without Ocentra-hosted storage', () => {
    const createReceipt = upsertTrackingLocalParentDefinedPlace({
      store: Store,
      place: HomePlace,
      auditRefs: ['parent-defined-home-upserted'],
    });
    const updateReceipt = upsertTrackingLocalParentDefinedPlace({
      store: createReceipt.store,
      place: {
        ...HomePlace,
        updatedAt: '2026-06-03T01:15:00.000Z',
        auditRefs: ['parent-defined-home-radius-updated'],
      },
      auditRefs: ['parent-defined-home-updated'],
    });

    expect(createReceipt.operation).toBe('create');
    expect(createReceipt.afterPlaceCount).toBe(1);
    expect(createReceipt.remoteSyncDefault).toBe('disabled');
    expect(createReceipt.ocentraHostedDefaultStorage).toBe(false);
    expect(updateReceipt.operation).toBe('update');
    expect(updateReceipt.afterPlaceCount).toBe(1);
    expect(updateReceipt.store.storageBoundary).toBe('parent-device-local');
  });
}

function registersImportExportProof() {
  it('imports safe and restricted places, exports a parent-owned snapshot, and preserves local custody', () => {
    const created = upsertTrackingLocalParentDefinedPlace({
      store: Store,
      place: HomePlace,
      auditRefs: ['parent-defined-home-upserted'],
    });
    const imported = importTrackingLocalParentDefinedPlaces({
      store: created.store,
      importedAt: '2026-06-03T01:20:00.000Z',
      places: [RestrictedPlace, SafePlace],
      auditRefs: ['parent-defined-place-imported'],
    });
    const exported = exportTrackingLocalParentDefinedPlaceStore({
      store: imported.store,
      exportedAt: '2026-06-03T01:25:00.000Z',
      auditRefs: ['parent-defined-place-exported'],
    });

    expect(imported.operation).toBe('import');
    expect(imported.afterPlaceCount).toBe(3);
    expect(exported.custodyLabel).toBe('parent-owned-export');
    expect(exported.remoteSyncDefault).toBe('disabled');
    expect(exported.ocentraHostedDefaultStorage).toBe(false);
    expect(exported.places.map((entry) => entry.placeId)).toEqual(['home', 'restricted-lot', 'safe-library']);
  });
}

function registersDeleteProof() {
  it('deletes a parent-defined place with an auditable tombstone', () => {
    const imported = importTrackingLocalParentDefinedPlaces({
      store: Store,
      importedAt: '2026-06-03T01:20:00.000Z',
      places: [HomePlace, RestrictedPlace],
      auditRefs: ['parent-defined-place-imported'],
    });
    const deleted = deleteTrackingLocalParentDefinedPlace({
      store: imported.store,
      placeId: 'restricted-lot',
      deletedAt: '2026-06-03T01:30:00.000Z',
      reasonCodes: ['parent-requested-place-delete'],
      auditRefs: ['parent-defined-restricted-zone-deleted'],
    });

    expect(deleted.operation).toBe('delete');
    expect(deleted.beforePlaceCount).toBe(2);
    expect(deleted.afterPlaceCount).toBe(1);
    expect(deleted.store.places[0]?.placeId).toBe('home');
    expect(deleted.store.tombstones[0]?.reasonCodes).toEqual(['parent-requested-place-delete']);
  });
}

function registersNearbyPolicySignalProof() {
  it('builds parent-defined nearby-place evidence and distinguishes safe from restricted policy signals', () => {
    const imported = importTrackingLocalParentDefinedPlaces({
      store: Store,
      importedAt: '2026-06-03T01:20:00.000Z',
      places: [RestrictedPlace, SafePlace],
      auditRefs: ['parent-defined-place-imported'],
    });
    const restricted = buildTrackingLocalParentDefinedPlaceMatch(matchInput(imported.store, 'restricted-lot', 42));
    const safe = buildTrackingLocalParentDefinedPlaceMatch(matchInput(imported.store, 'safe-library', 38));

    expect(restricted.match.policySignal).toBe('restricted-zone-attention');
    expect(restricted.nearbyPlaceEvidence.providerKind).toBe('parent-defined');
    expect(restricted.nearbyPlaceEvidence.providerRef).toBe('parent-local-place-store');
    expect(safe.match.policySignal).toBe('safe-zone-context');
    expect(safe.nearbyPlaceEvidence.ambiguityState).toBe('clear');
  });
}

function registersStorageBoundaryNegativeProof() {
  it('rejects local place stores that try to become Ocentra-hosted default storage', () => {
    const result = TrackingLocalParentDefinedPlaceStoreSchema.safeParse({
      ...Store,
      remoteSyncDefault: 'enabled',
      ocentraHostedDefaultStorage: true,
    });

    expect(result.success).toBe(false);
  });
}

function matchInput(store: typeof Store, placeId: 'restricted-lot' | 'safe-library', distanceMeters: number) {
  const safe = placeId === 'safe-library';
  return {
    store,
    evidenceId: safe ? 'nearby-parent-defined-safe-place' : 'nearby-parent-defined-restricted-place',
    observedAt: '2026-06-03T02:01:00.000Z',
    locationEvidenceId: LocationEvidence.evidenceId,
    placeId,
    queryRadiusMeters: 250,
    distanceMeters,
    confidence: safe ? 0.88 : 0.9,
    reasonCodes: [safe ? 'parent-defined-safe-zone-match' : 'parent-defined-restricted-zone-match'],
    evidence: [EvidenceRef],
  };
}

function place(
  placeId: 'home' | 'restricted-lot' | 'safe-library',
  label: 'Home' | 'Restricted lot' | 'Safe library',
  placeKind: 'home' | 'restricted-zone' | 'safe-zone',
  updatedAt: PlaceUpdatedAt,
  auditRef: PlaceAuditRef
) {
  return TrackingParentDefinedPlaceSchema.parse({
    schemaVersion: 1,
    placeId,
    label,
    placeKind,
    shape: {
      kind: 'circle',
      center: {
        latitude: 43.6532,
        longitude: -79.3832,
      },
      radiusMeters: 150,
      polygon: [],
    },
    createdAt: '2026-06-03T01:00:00.000Z',
    updatedAt,
    auditRefs: [auditRef],
  });
}
