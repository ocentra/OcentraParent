import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  RequiredTrackingRetentionRuntimeArtifactPlan,
  type TrackingRetentionRuntimeArtifactInventory,
} from './tracking-retention-runtime-artifact-gate-proof';
import { TrackingRetentionSettingsProofRefSchema } from './tracking-retention-settings-read-model-proof';
import {
  TrackingRetentionProductSettingsWritableExecutionProofSchema,
  type TrackingRetentionProductSettingsWritableExecutionRow,
} from './tracking-retention-product-settings-writable-execution-proof';

const TrackingRetentionAppliedSettingsRuntimeBridgeTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRetentionAppliedSettingsRuntimeBridgeRowIdSchema =
  TrackingRetentionAppliedSettingsRuntimeBridgeTextSchema.pipe(
    Schema.brand('TrackingRetentionAppliedSettingsRuntimeBridgeRowId')
  );

export const TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRefSchema =
  TrackingRetentionAppliedSettingsRuntimeBridgeTextSchema.pipe(
    Schema.brand('TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRef')
  );

export const TrackingRetentionAppliedSettingsRuntimeBridgeStatusSchema = Schema.Literal(
  'local-applied-settings-observed',
  'platform-runtime-missing'
);

export const TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingRetentionAppliedSettingsRuntimeBridgeRowIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceWritableExecutionProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceLocalServiceStateProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceWriteCommandProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceRuntimeArtifactPlanRef: TrackingRetentionSettingsProofRefSchema,
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    localServiceStateRevision: Schema.Number.pipe(Schema.int(), Schema.positive()),
    localServiceStateSnapshotRef: TrackingRetentionAppliedSettingsRuntimeBridgeTextSchema,
    durableSettingsStoreRef: TrackingRetentionAppliedSettingsRuntimeBridgeTextSchema,
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolved: Schema.Boolean,
    parentExportPrepared: Schema.Boolean,
    requiredRuntimeArtifacts: Schema.Array(TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRefSchema),
    presentRuntimeArtifacts: Schema.Array(TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRefSchema),
    missingRuntimeArtifacts: Schema.Array(TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRefSchema),
    bridgeStatus: TrackingRetentionAppliedSettingsRuntimeBridgeStatusSchema,
    localAppliedSettingsObserved: Schema.Literal(true),
    writableExecutionArtifactPresent: Schema.Literal(true),
    platformRuntimeRetentionEnforcementPresent: Schema.Literal(false),
    platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
    productionRetentionWriteResultClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Applied settings bridge rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredRuntimeArtifacts.length ===
            row.presentRuntimeArtifacts.length + row.missingRuntimeArtifacts.length ||
          'Applied settings bridge rows must classify every retention runtime artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.presentRuntimeArtifacts.some(
            (artifact) => String(artifact) === 'tracking-retention/product-settings-writable-execution.json'
          ) || 'Applied settings bridge rows must include the local writable execution artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.missingRuntimeArtifacts.some(
            (artifact) => String(artifact) === 'tracking-retention/platform-runtime-retention-enforcement.json'
          ) || 'Applied settings bridge rows must keep platform runtime enforcement missing'
      )
    )
);

export const TrackingRetentionAppliedSettingsRuntimeBridgeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-applied-settings-runtime-bridge-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema),
    proofClaims: Schema.Struct({
      writableExecutionProofConsumed: Schema.Literal(true),
      localAppliedSettingsObserved: Schema.Literal(true),
      localDurableSettingsPersisted: Schema.Literal(true),
      runtimeArtifactAccountingUpdated: Schema.Literal(true),
      platformRuntimeRetentionEnforcementMissing: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    runtimeArtifactInventory: Schema.Struct({
      requiredArtifacts: Schema.Array(TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRefSchema),
      presentArtifacts: Schema.Array(TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRefSchema),
      missingArtifacts: Schema.Array(TrackingRetentionAppliedSettingsRuntimeBridgeArtifactRefSchema),
      artifactSetComplete: Schema.Literal(false),
    }),
    productClaims: Schema.Struct({
      platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
      productionRetentionWriteResultClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(Schema.filter((proof) => proof.rows.length > 0 || 'Applied settings bridge proof needs rows'))
    .pipe(
      Schema.filter(
        (proof) =>
          proof.runtimeArtifactInventory.requiredArtifacts.length ===
            proof.runtimeArtifactInventory.presentArtifacts.length +
              proof.runtimeArtifactInventory.missingArtifacts.length ||
          'Applied settings bridge proof must classify every runtime artifact'
      )
    )
);

export type TrackingRetentionAppliedSettingsRuntimeBridgeProof = Infer<
  typeof TrackingRetentionAppliedSettingsRuntimeBridgeProofSchema
>;
export type TrackingRetentionAppliedSettingsRuntimeBridgeRow = Infer<
  typeof TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema
>;

export const TrackingRetentionAppliedSettingsRuntimeBridgeProofRef =
  'output/tracking-plan-proof/tracking-retention-applied-settings-runtime-bridge-proof/proof.json' as const;

export function buildTrackingRetentionAppliedSettingsRuntimeBridgeProof(
  generatedAt: string,
  sourceWritableExecutionProofRef: string,
  writableExecutionProof: unknown
): TrackingRetentionAppliedSettingsRuntimeBridgeProof {
  const parsedWritableExecutionProof =
    TrackingRetentionProductSettingsWritableExecutionProofSchema.parse(writableExecutionProof);
  const inventory = runtimeArtifactInventory(parsedWritableExecutionProof.rows);
  return TrackingRetentionAppliedSettingsRuntimeBridgeProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-applied-settings-runtime-bridge-proof',
    generatedAt,
    rows: parsedWritableExecutionProof.rows.map((row) =>
      runtimeBridgeRow(generatedAt, sourceWritableExecutionProofRef, row, inventory)
    ),
    proofClaims: {
      writableExecutionProofConsumed: true,
      localAppliedSettingsObserved: true,
      localDurableSettingsPersisted: true,
      runtimeArtifactAccountingUpdated: true,
      platformRuntimeRetentionEnforcementMissing: true,
      noProductReadyClaim: true,
    },
    runtimeArtifactInventory: {
      requiredArtifacts: [...RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts],
      presentArtifacts: inventory.presentArtifacts,
      missingArtifacts: missingRuntimeArtifacts(inventory),
      artifactSetComplete: false,
    },
    productClaims: {
      platformRuntimeRetentionEnforcementClaimed: false,
      productionRetentionWriteResultClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function runtimeBridgeRow(
  generatedAt: string,
  sourceWritableExecutionProofRef: string,
  row: TrackingRetentionProductSettingsWritableExecutionRow,
  inventory: TrackingRetentionRuntimeArtifactInventory
): TrackingRetentionAppliedSettingsRuntimeBridgeRow {
  return TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `${String(row.rowId)}-runtime-bridge`,
    generatedAt,
    sourceWritableExecutionProofRef,
    sourceLocalServiceStateProofRef: row.sourceLocalServiceStateProofRef,
    sourceWriteCommandProofRef: row.sourceWriteCommandProofRef,
    sourceRuntimeArtifactPlanRef: RequiredTrackingRetentionRuntimeArtifactPlan.sourceProductReadinessProofRef,
    auditRefs: [`${String(row.rowId)}-runtime-bridge-audit`],
    localServiceStateRevision: row.localServiceStateRevision,
    localServiceStateSnapshotRef: row.localServiceStateSnapshotRef,
    durableSettingsStoreRef: row.durableSettingsStoreRef,
    appliedRetentionWindowHours: row.appliedRetentionWindowHours,
    appliedDeleteAfterAlertResolved: row.appliedDeleteAfterAlertResolved,
    parentExportPrepared: row.parentExportPrepared,
    requiredRuntimeArtifacts: [...RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts],
    presentRuntimeArtifacts: [...inventory.presentArtifacts],
    missingRuntimeArtifacts: missingRuntimeArtifacts(inventory),
    bridgeStatus: 'platform-runtime-missing',
    localAppliedSettingsObserved: true,
    writableExecutionArtifactPresent: true,
    platformRuntimeRetentionEnforcementPresent: false,
    platformRuntimeRetentionEnforcementClaimed: false,
    productionRetentionWriteResultClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function runtimeArtifactInventory(
  rows: readonly TrackingRetentionProductSettingsWritableExecutionRow[]
): TrackingRetentionRuntimeArtifactInventory {
  const presentArtifacts = new Set<string>();
  for (const row of rows) {
    if (row.outputArtifactRef === 'tracking-retention/product-settings-writable-execution.json') {
      presentArtifacts.add(row.outputArtifactRef);
    }
  }
  return { presentArtifacts: [...presentArtifacts] };
}

function missingRuntimeArtifacts(inventory: TrackingRetentionRuntimeArtifactInventory): string[] {
  const presentArtifacts = new Set(inventory.presentArtifacts);
  return RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts.filter(
    (artifact) => !presentArtifacts.has(artifact)
  );
}
