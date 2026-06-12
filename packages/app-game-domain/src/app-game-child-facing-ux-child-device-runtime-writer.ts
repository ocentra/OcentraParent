import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildDeviceDeliveryReadinessReadModelSchema,
  AppGameChildDeviceDeliveryReadinessStatus,
  type AppGameChildDeviceDeliveryReadinessReadModel,
  type AppGameChildDeviceDeliveryReadinessRow,
} from './app-game-child-facing-ux-child-device-delivery-readiness';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';

const ChildDeviceRuntimeWriterText = Schema.String.pipe(Schema.minLength(1));

export const AppGameChildDeviceRuntimeWriterState = {
  EnvelopeReady: 'writer-envelope-ready',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AppGameChildDeviceRuntimeWriterStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildDeviceRuntimeWriterState))
);

export const AppGameChildDeviceRuntimeWriterNonClaimSchema = withParser(
  Schema.Literal(
    'no-runtime-writer-execution',
    'no-child-runtime-transport',
    'no-child-runtime-receipt-ingestion',
    'no-provider-delivery-execution',
    'no-platform-delivery-channel',
    'no-adapter-dispatch',
    'no-platform-enforcement',
    'no-raw-private-source-rows'
  )
);

export const RequiredAppGameChildDeviceRuntimeWriterNonClaims = [
  'no-runtime-writer-execution',
  'no-child-runtime-transport',
  'no-child-runtime-receipt-ingestion',
  'no-provider-delivery-execution',
  'no-platform-delivery-channel',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

const ChildDeviceRuntimeWriterIdSchema = ChildDeviceRuntimeWriterText.pipe(
  Schema.brand('AppGameChildDeviceRuntimeWriterId')
);
const ChildDeviceRuntimeWriterRefSchema = ChildDeviceRuntimeWriterText.pipe(
  Schema.brand('AppGameChildDeviceRuntimeWriterReference')
);

const AppGameChildDeviceRuntimeWriterRowBaseSchema = Schema.Struct({
  runtimeWriterRowId: ChildDeviceRuntimeWriterRefSchema,
  sourceDeliveryReadinessRowId: ChildDeviceRuntimeWriterRefSchema,
  sourceDeliveryReadinessStatus: ChildDeviceRuntimeWriterRefSchema,
  writerEnvelopeState: AppGameChildDeviceRuntimeWriterStateSchema,
  childDeliveryTargetRefs: Schema.Array(ChildDeviceRuntimeWriterRefSchema),
  runtimeWriterAuditRefs: Schema.Array(ChildDeviceRuntimeWriterRefSchema),
  runtimeWriterExecuted: Schema.Literal(false),
  childRuntimeTransportAttached: Schema.Literal(false),
  childRuntimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
});

export const AppGameChildDeviceRuntimeWriterRowSchema = withParser(
  AppGameChildDeviceRuntimeWriterRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameChildDeviceRuntimeWriterRowIsHonest(row) ||
        'Expected child-device runtime writer rows to prepare envelopes without runtime writer execution, child transport, receipts, provider delivery, or platform channel claims'
    )
  )
);

const AppGameChildDeviceRuntimeWriterReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  runtimeWriterId: ChildDeviceRuntimeWriterIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceDeliveryReadinessId: ChildDeviceRuntimeWriterRefSchema,
  rows: Schema.Array(AppGameChildDeviceRuntimeWriterRowSchema),
  writerEnvelopeReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  nonClaims: Schema.Array(AppGameChildDeviceRuntimeWriterNonClaimSchema),
  runtimeWriterExecuted: Schema.Literal(false),
  childRuntimeTransportAttached: Schema.Literal(false),
  childRuntimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildDeviceRuntimeWriterReadModelSchema = withParser(
  AppGameChildDeviceRuntimeWriterReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameChildDeviceRuntimeWriterReadModelIsHonest(readModel) ||
        'Expected child-device runtime writer counts and non-claims to match envelope-ready, manual-required, and unavailable rows'
    )
  )
);

export type AppGameChildDeviceRuntimeWriterState = Infer<typeof AppGameChildDeviceRuntimeWriterStateSchema>;
export type AppGameChildDeviceRuntimeWriterRow = Infer<typeof AppGameChildDeviceRuntimeWriterRowSchema>;
export type AppGameChildDeviceRuntimeWriterReadModel = Infer<typeof AppGameChildDeviceRuntimeWriterReadModelSchema>;

export type AppGameChildDeviceRuntimeWriterOptions = {
  readonly generatedAt: string;
  readonly runtimeWriterId: string;
};

export function buildAppGameChildDeviceRuntimeWriterReadModel(
  options: AppGameChildDeviceRuntimeWriterOptions,
  sourceReadModel: AppGameChildDeviceDeliveryReadinessReadModel
): AppGameChildDeviceRuntimeWriterReadModel {
  const source = AppGameChildDeviceDeliveryReadinessReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map((row) => childDeviceRuntimeWriterRow(row));

  return AppGameChildDeviceRuntimeWriterReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    runtimeWriterId: options.runtimeWriterId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourceDeliveryReadinessId: source.readinessId,
    rows,
    writerEnvelopeReadyCount: countRows(rows, AppGameChildDeviceRuntimeWriterState.EnvelopeReady),
    manualRequiredCount: countRows(rows, AppGameChildDeviceRuntimeWriterState.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildDeviceRuntimeWriterState.Unavailable),
    nonClaims: RequiredAppGameChildDeviceRuntimeWriterNonClaims,
    runtimeWriterExecuted: false,
    childRuntimeTransportAttached: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function summarizeAppGameChildDeviceRuntimeWriter(readModel: AppGameChildDeviceRuntimeWriterReadModel) {
  return {
    writerEnvelopeReadyCount: readModel.writerEnvelopeReadyCount,
    manualRequiredCount: readModel.manualRequiredCount,
    unavailableCount: readModel.unavailableCount,
    rowCount: readModel.rows.length,
  } as const;
}

function childDeviceRuntimeWriterRow(row: AppGameChildDeviceDeliveryReadinessRow): AppGameChildDeviceRuntimeWriterRow {
  const writerEnvelopeState = writerStateFor(row);

  return AppGameChildDeviceRuntimeWriterRowSchema.parse({
    runtimeWriterRowId: `app-game-child-device-runtime-writer-${row.deliveryReadinessRowId}`,
    sourceDeliveryReadinessRowId: row.deliveryReadinessRowId,
    sourceDeliveryReadinessStatus: row.deliveryReadinessStatus,
    writerEnvelopeState,
    childDeliveryTargetRefs:
      writerEnvelopeState === AppGameChildDeviceRuntimeWriterState.EnvelopeReady
        ? row.requiredTransportRefs
        : row.openGaps,
    runtimeWriterAuditRefs: [`app-game-child-device-runtime-writer-audit-${row.deliveryReadinessRowId}`],
    runtimeWriterExecuted: false,
    childRuntimeTransportAttached: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  });
}

function writerStateFor(row: AppGameChildDeviceDeliveryReadinessRow): AppGameChildDeviceRuntimeWriterState {
  if (row.deliveryReadinessStatus === AppGameChildDeviceDeliveryReadinessStatus.TransportRequired) {
    return AppGameChildDeviceRuntimeWriterState.EnvelopeReady;
  }
  if (row.deliveryReadinessStatus === AppGameChildDeviceDeliveryReadinessStatus.Unavailable) {
    return AppGameChildDeviceRuntimeWriterState.Unavailable;
  }
  return AppGameChildDeviceRuntimeWriterState.ManualRequired;
}

function appGameChildDeviceRuntimeWriterRowIsHonest(
  row: Infer<typeof AppGameChildDeviceRuntimeWriterRowBaseSchema>
): boolean {
  return (
    row.childDeliveryTargetRefs.length > 0 &&
    row.runtimeWriterAuditRefs.length > 0 &&
    !row.runtimeWriterExecuted &&
    !row.childRuntimeTransportAttached &&
    !row.childRuntimeReceiptIngested &&
    !row.providerDeliveryExecuted &&
    !row.platformDeliveryChannelClaimed
  );
}

function appGameChildDeviceRuntimeWriterReadModelIsHonest(
  readModel: Infer<typeof AppGameChildDeviceRuntimeWriterReadModelBaseSchema>
): boolean {
  return (
    readModel.writerEnvelopeReadyCount ===
      countRows(readModel.rows, AppGameChildDeviceRuntimeWriterState.EnvelopeReady) &&
    readModel.manualRequiredCount === countRows(readModel.rows, AppGameChildDeviceRuntimeWriterState.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, AppGameChildDeviceRuntimeWriterState.Unavailable) &&
    RequiredAppGameChildDeviceRuntimeWriterNonClaims.every((claim) => readModel.nonClaims.includes(claim)) &&
    !readModel.runtimeWriterExecuted &&
    !readModel.childRuntimeTransportAttached &&
    !readModel.childRuntimeReceiptIngested &&
    !readModel.providerDeliveryExecuted &&
    !readModel.platformDeliveryChannelClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly writerEnvelopeState: AppGameChildDeviceRuntimeWriterState }>,
  state: AppGameChildDeviceRuntimeWriterState
): number {
  return rows.filter((row) => row.writerEnvelopeState === state).length;
}
