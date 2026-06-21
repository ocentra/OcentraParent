import type { ActivityEvidenceRef } from './evidence-contracts';
import type {
  TrackingNearbyPlaceEvidence,
  TrackingParentDefinedPlace,
} from './tracking-geofence';
import type {
  TrackingLocalParentDefinedPlaceExportSnapshot,
  TrackingLocalParentDefinedPlaceStore,
  TrackingLocalParentDefinedPlaceTombstone,
  TrackingLocalPlaceStoreId,
} from './tracking-local-place-store-schemas';

export interface CreateTrackingLocalParentDefinedPlaceStoreInput {
  readonly storeId: TrackingLocalPlaceStoreId;
  readonly createdAt: TrackingLocalParentDefinedPlaceStore['createdAt'];
  readonly auditRefs: readonly TrackingLocalParentDefinedPlaceStore['auditRefs'][number][];
}

export interface TrackingLocalParentDefinedPlaceUpsertInput {
  readonly store: TrackingLocalParentDefinedPlaceStore;
  readonly place: TrackingParentDefinedPlace;
  readonly auditRefs: readonly TrackingLocalParentDefinedPlaceStore['auditRefs'][number][];
}

export interface TrackingLocalParentDefinedPlaceImportInput {
  readonly store: TrackingLocalParentDefinedPlaceStore;
  readonly importedAt: TrackingLocalParentDefinedPlaceStore['updatedAt'];
  readonly places: readonly TrackingParentDefinedPlace[];
  readonly auditRefs: readonly TrackingLocalParentDefinedPlaceStore['auditRefs'][number][];
}

export interface TrackingLocalParentDefinedPlaceDeleteInput {
  readonly store: TrackingLocalParentDefinedPlaceStore;
  readonly placeId: TrackingParentDefinedPlace['placeId'];
  readonly deletedAt: TrackingLocalParentDefinedPlaceStore['updatedAt'];
  readonly reasonCodes: readonly TrackingLocalParentDefinedPlaceTombstone['reasonCodes'][number][];
  readonly auditRefs: readonly TrackingLocalParentDefinedPlaceStore['auditRefs'][number][];
}

export interface TrackingLocalParentDefinedPlaceExportInput {
  readonly store: TrackingLocalParentDefinedPlaceStore;
  readonly exportedAt: TrackingLocalParentDefinedPlaceExportSnapshot['exportedAt'];
  readonly auditRefs: readonly TrackingLocalParentDefinedPlaceStore['auditRefs'][number][];
}

export interface TrackingLocalParentDefinedPlaceMatchInput {
  readonly store: TrackingLocalParentDefinedPlaceStore;
  readonly evidenceId: TrackingNearbyPlaceEvidence['evidenceId'];
  readonly observedAt: TrackingNearbyPlaceEvidence['observedAt'];
  readonly locationEvidenceId: TrackingNearbyPlaceEvidence['locationEvidenceId'];
  readonly placeId: TrackingParentDefinedPlace['placeId'];
  readonly queryRadiusMeters: TrackingNearbyPlaceEvidence['queryRadiusMeters'];
  readonly distanceMeters: NonNullable<TrackingNearbyPlaceEvidence['distanceMeters']>;
  readonly confidence: TrackingNearbyPlaceEvidence['confidence'];
  readonly reasonCodes: readonly TrackingNearbyPlaceEvidence['reasonCodes'][number][];
  readonly evidence: readonly ActivityEvidenceRef[];
}
