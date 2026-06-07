import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  TrackingRetentionLocalServiceStateProofSchema,
  type TrackingRetentionLocalServiceStateRow,
} from './tracking-retention-local-service-state-proof';
import {
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';

const TrackingRetentionDurableSettingsTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRetentionDurableSettingsProofIdSchema = TrackingRetentionDurableSettingsTextSchema.pipe(
  Schema.brand('TrackingRetentionDurableSettingsProofId')
);

export const TrackingRetentionDurableSettingsStoreRefSchema = TrackingRetentionDurableSettingsTextSchema.pipe(
  Schema.brand('TrackingRetentionDurableSettingsStoreRef')
);

export const TrackingRetentionDurableSettingsRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    durableProofId: TrackingRetentionDurableSettingsProofIdSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    generatedAt: ParentTimestampSchema,
    sourceLocalServiceStateProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceMutationProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    localServiceStateRevision: Schema.Number.pipe(Schema.int(), Schema.positive()),
    localServiceStateSnapshotRef: TrackingRetentionDurableSettingsStoreRefSchema,
    durableStoreRef: TrackingRetentionDurableSettingsStoreRefSchema,
    durableSettingsPersisted: Schema.Literal(false),
    durablePersistenceRequired: Schema.Literal(true),
    durabilityFailureVisible: Schema.Literal(true),
    productSettingsWritable: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Durable settings rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) => row.sourceMutationProofRefs.length > 0 || 'Durable settings rows need source mutation refs'
      )
    )
);

export const TrackingRetentionDurableSettingsProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-durable-settings-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionDurableSettingsRowSchema),
    proofClaims: Schema.Struct({
      localServiceStateReadbackClaimed: Schema.Literal(true),
      durablePersistenceRequirementVisible: Schema.Literal(true),
      durabilityFailureVisible: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      durableSettingsPersisted: Schema.Literal(false),
      productSettingsWritable: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length > 0 || 'Durable settings proof needs at least one row'))
);

export type TrackingRetentionDurableSettingsRow = Infer<typeof TrackingRetentionDurableSettingsRowSchema>;
export type TrackingRetentionDurableSettingsProof = Infer<typeof TrackingRetentionDurableSettingsProofSchema>;

export function buildTrackingRetentionDurableSettingsProof(
  generatedAt: string,
  sourceLocalServiceStateProofRef: string,
  localServiceStateProof: unknown
): TrackingRetentionDurableSettingsProof {
  const parsedLocalServiceStateProof = TrackingRetentionLocalServiceStateProofSchema.parse(localServiceStateProof);
  return TrackingRetentionDurableSettingsProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-durable-settings-proof',
    generatedAt,
    rows: parsedLocalServiceStateProof.rows.map((row) =>
      durableSettingsRow(generatedAt, sourceLocalServiceStateProofRef, row)
    ),
    proofClaims: {
      localServiceStateReadbackClaimed: true,
      durablePersistenceRequirementVisible: true,
      durabilityFailureVisible: true,
    },
    productClaims: {
      durableSettingsPersisted: false,
      productSettingsWritable: false,
      platformRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
      productClaimReady: false,
    },
  });
}

function durableSettingsRow(
  generatedAt: string,
  sourceLocalServiceStateProofRef: string,
  row: TrackingRetentionLocalServiceStateRow
): TrackingRetentionDurableSettingsRow {
  return TrackingRetentionDurableSettingsRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    durableProofId: `${String(row.stateProofId)}-durable-settings-required`,
    settingsKind: row.settingsKind,
    generatedAt,
    sourceLocalServiceStateProofRef,
    sourceReadModelProofRefs: row.sourceReadModelProofRefs,
    sourceMutationProofRefs: row.sourceMutationProofRefs,
    auditRefs: [`${String(row.stateProofId)}-durable-settings-audit`],
    localServiceStateRevision: row.localServiceStateRevision,
    localServiceStateSnapshotRef: row.localServiceStateSnapshotRef,
    durableStoreRef: 'retention-settings-durable-store-required',
    durableSettingsPersisted: false,
    durablePersistenceRequired: true,
    durabilityFailureVisible: true,
    productSettingsWritable: false,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}
