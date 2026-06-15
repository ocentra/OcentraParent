import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import {
  TrackingAmbiguityStateSchema,
  TrackingGeofencePlaceKindSchema,
  TrackingParentDefinedPlaceSchema,
} from './tracking-geofence';
import {
  TrackingAuditRefSchema,
  TrackingEvidenceSchemaVersion,
  TrackingNonNegativeNumberSchema,
  TrackingPlaceIdSchema,
  TrackingReasonCodeSchema,
} from './tracking-primitives';

const TrackingLocalPlaceStoreTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingLocalPlaceStoreIdSchema = withParser(
  TrackingLocalPlaceStoreTextSchema.pipe(Schema.brand('TrackingLocalPlaceStoreId'))
);

export const TrackingLocalPlaceMutationKindSchema = withParser(Schema.Literal('create', 'update', 'import', 'delete'));

export const TrackingLocalPlacePolicySignalSchema = withParser(
  Schema.Literal('safe-zone-context', 'restricted-zone-attention', 'expected-place-context', 'custom-place-context')
);

export const TrackingLocalParentDefinedPlaceTombstoneSchema = withParser(
  Schema.Struct({
    placeId: TrackingPlaceIdSchema,
    deletedAt: ActivityTimestampSchema,
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

export const TrackingLocalParentDefinedPlaceStoreSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    storeId: TrackingLocalPlaceStoreIdSchema,
    createdAt: ActivityTimestampSchema,
    updatedAt: ActivityTimestampSchema,
    storageBoundary: Schema.Literal('parent-device-local'),
    remoteSyncDefault: Schema.Literal('disabled'),
    ocentraHostedDefaultStorage: Schema.Literal(false),
    places: Schema.Array(TrackingParentDefinedPlaceSchema),
    tombstones: Schema.Array(TrackingLocalParentDefinedPlaceTombstoneSchema),
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

export const TrackingLocalParentDefinedPlaceExportSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    storeId: TrackingLocalPlaceStoreIdSchema,
    exportedAt: ActivityTimestampSchema,
    custodyLabel: Schema.Literal('parent-owned-export'),
    remoteSyncDefault: Schema.Literal('disabled'),
    ocentraHostedDefaultStorage: Schema.Literal(false),
    places: Schema.Array(TrackingParentDefinedPlaceSchema),
    tombstones: Schema.Array(TrackingLocalParentDefinedPlaceTombstoneSchema),
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

export const TrackingLocalParentDefinedPlaceMutationReceiptSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    storeId: TrackingLocalPlaceStoreIdSchema,
    operation: TrackingLocalPlaceMutationKindSchema,
    beforePlaceCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    afterPlaceCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    remoteSyncDefault: Schema.Literal('disabled'),
    ocentraHostedDefaultStorage: Schema.Literal(false),
    auditRefs: Schema.Array(TrackingAuditRefSchema),
    store: TrackingLocalParentDefinedPlaceStoreSchema,
  })
);

export const TrackingLocalParentDefinedPlaceMatchSchema = withParser(
  Schema.Struct({
    placeId: TrackingPlaceIdSchema,
    placeKind: TrackingGeofencePlaceKindSchema,
    distanceMeters: TrackingNonNegativeNumberSchema,
    queryRadiusMeters: TrackingNonNegativeNumberSchema,
    ambiguityState: TrackingAmbiguityStateSchema,
    policySignal: TrackingLocalPlacePolicySignalSchema,
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
  })
);

export type TrackingLocalPlaceStoreId = Infer<typeof TrackingLocalPlaceStoreIdSchema>;
export type TrackingLocalPlaceMutationKind = Infer<typeof TrackingLocalPlaceMutationKindSchema>;
export type TrackingLocalPlacePolicySignal = Infer<typeof TrackingLocalPlacePolicySignalSchema>;
export type TrackingLocalParentDefinedPlaceTombstone = Infer<typeof TrackingLocalParentDefinedPlaceTombstoneSchema>;
export type TrackingLocalParentDefinedPlaceStore = Infer<typeof TrackingLocalParentDefinedPlaceStoreSchema>;
export type TrackingLocalParentDefinedPlaceExportSnapshot = Infer<
  typeof TrackingLocalParentDefinedPlaceExportSnapshotSchema
>;
export type TrackingLocalParentDefinedPlaceMutationReceipt = Infer<
  typeof TrackingLocalParentDefinedPlaceMutationReceiptSchema
>;
export type TrackingLocalParentDefinedPlaceMatch = Infer<typeof TrackingLocalParentDefinedPlaceMatchSchema>;
