import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  TrackingRetentionDurableSettingsProofSchema,
  type TrackingRetentionDurableSettingsRow,
} from './tracking-retention-durable-settings-proof';
import {
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';

export const TrackingRetentionProductReadinessProofIdSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionProductReadinessProofId'
);

export const TrackingRetentionProductBlockerSchema = Schema.Literal(
  'writable-product-settings-execution',
  'platform-runtime-retention-enforcement',
  'child-device-runtime-delivery',
  'provider-delivery',
  'notification-receipt-ingestion',
  'physical-device-proof',
  'authority-enrollment-proof',
  'production-worker-hardening'
);

export const TrackingRetentionProductReadinessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    readinessProofId: TrackingRetentionProductReadinessProofIdSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    generatedAt: ParentTimestampSchema,
    sourceDurableSettingsProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceLocalServiceStateProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceMutationProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    localServiceStateRevision: Schema.Number.pipe(Schema.int(), Schema.positive()),
    durableSettingsStoreRef: NonEmptyStringSchema,
    durableSettingsPersisted: Schema.Literal(true),
    productReadinessBlockers: Schema.Array(TrackingRetentionProductBlockerSchema),
    localDurableSettingsReady: Schema.Literal(true),
    productSettingsWritable: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Readiness rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) => row.productReadinessBlockers.length >= ProductReadinessBlockers.length || 'Readiness blockers missing'
      )
    )
);

export const TrackingRetentionProductReadinessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-product-readiness-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionProductReadinessRowSchema),
    proofClaims: Schema.Struct({
      localDurableSettingsReady: Schema.Literal(true),
      productReadinessBlockersEnumerated: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      localDurableSettingsReady: Schema.Literal(true),
      productSettingsWritable: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length > 0 || 'Product readiness proof needs at least one row'))
);

export type TrackingRetentionProductReadinessRow = Infer<typeof TrackingRetentionProductReadinessRowSchema>;
export type TrackingRetentionProductReadinessProof = Infer<typeof TrackingRetentionProductReadinessProofSchema>;

export const ProductReadinessBlockers = [
  'writable-product-settings-execution',
  'platform-runtime-retention-enforcement',
  'child-device-runtime-delivery',
  'provider-delivery',
  'notification-receipt-ingestion',
  'physical-device-proof',
  'authority-enrollment-proof',
  'production-worker-hardening',
] as const;

export const RequiredTrackingRetentionRuntimeProductBlockers = [
  'writable-product-settings-execution',
  'platform-runtime-retention-enforcement',
] as const;

export const RequiredTrackingRetentionRuntimeArtifactRefs = [
  'tracking-retention/product-settings-writable-execution.json',
  'tracking-retention/platform-runtime-retention-enforcement.json',
] as const;

export function buildTrackingRetentionProductReadinessProof(
  generatedAt: string,
  sourceDurableSettingsProofRef: string,
  durableSettingsProof: unknown
): TrackingRetentionProductReadinessProof {
  const parsedDurableProof = TrackingRetentionDurableSettingsProofSchema.parse(durableSettingsProof);
  return TrackingRetentionProductReadinessProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-product-readiness-proof',
    generatedAt,
    rows: parsedDurableProof.rows.map((row) => readinessRow(generatedAt, sourceDurableSettingsProofRef, row)),
    proofClaims: {
      localDurableSettingsReady: true,
      productReadinessBlockersEnumerated: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      localDurableSettingsReady: true,
      productSettingsWritable: false,
      platformRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function readinessRow(
  generatedAt: string,
  sourceDurableSettingsProofRef: string,
  row: TrackingRetentionDurableSettingsRow
): TrackingRetentionProductReadinessRow {
  return TrackingRetentionProductReadinessRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    readinessProofId: `${String(row.durableProofId)}-product-readiness-blocked`,
    settingsKind: row.settingsKind,
    generatedAt,
    sourceDurableSettingsProofRef,
    sourceLocalServiceStateProofRef: row.sourceLocalServiceStateProofRef,
    sourceReadModelProofRefs: row.sourceReadModelProofRefs,
    sourceMutationProofRefs: row.sourceMutationProofRefs,
    auditRefs: [`${String(row.durableProofId)}-product-readiness-audit`],
    localServiceStateRevision: row.localServiceStateRevision,
    durableSettingsStoreRef: String(row.durableSettingsStoreRef),
    durableSettingsPersisted: true,
    productReadinessBlockers: [...ProductReadinessBlockers],
    localDurableSettingsReady: true,
    productSettingsWritable: false,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}
