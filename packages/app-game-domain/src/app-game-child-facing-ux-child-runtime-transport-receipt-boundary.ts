import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildDeviceRuntimeWriterReadModelSchema,
  AppGameChildDeviceRuntimeWriterState,
  type AppGameChildDeviceRuntimeWriterReadModel,
  type AppGameChildDeviceRuntimeWriterRow,
} from './app-game-child-facing-ux-child-device-runtime-writer';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';

export const AppGameChildRuntimeTransportReceiptBoundaryState = {
  TransportRequired: 'child-runtime-transport-required',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AppGameChildRuntimeTransportReceiptBoundaryStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildRuntimeTransportReceiptBoundaryState))
);

export const AppGameChildRuntimeTransportReceiptBoundaryNonClaimSchema = withParser(
  Schema.Literal(
    'no-child-runtime-transport-execution',
    'no-child-runtime-receipt-ingestion',
    'no-provider-delivery-execution',
    'no-platform-delivery-channel',
    'no-adapter-dispatch',
    'no-platform-enforcement',
    'no-raw-private-source-rows'
  )
);

export const RequiredAppGameChildRuntimeTransportReceiptBoundaryNonClaims = [
  'no-child-runtime-transport-execution',
  'no-child-runtime-receipt-ingestion',
  'no-provider-delivery-execution',
  'no-platform-delivery-channel',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

const ChildRuntimeTransportReceiptBoundaryIdSchema = brandedNonEmptyStringSchema('AppGameChildRuntimeTransportReceiptBoundaryId');
const ChildRuntimeTransportReceiptBoundaryRefSchema = brandedNonEmptyStringSchema('AppGameChildRuntimeTransportReceiptBoundaryReference');

const AppGameChildRuntimeTransportReceiptBoundaryRowBaseSchema = Schema.Struct({
  boundaryRowId: ChildRuntimeTransportReceiptBoundaryRefSchema,
  sourceRuntimeWriterRowId: ChildRuntimeTransportReceiptBoundaryRefSchema,
  sourceWriterEnvelopeState: ChildRuntimeTransportReceiptBoundaryRefSchema,
  boundaryState: AppGameChildRuntimeTransportReceiptBoundaryStateSchema,
  requiredTransportRefs: Schema.Array(ChildRuntimeTransportReceiptBoundaryRefSchema),
  requiredReceiptRefs: Schema.Array(ChildRuntimeTransportReceiptBoundaryRefSchema),
  openGaps: Schema.Array(ChildRuntimeTransportReceiptBoundaryRefSchema),
  runtimeTransportExecuted: Schema.Literal(false),
  runtimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
});

export const AppGameChildRuntimeTransportReceiptBoundaryRowSchema = withParser(
  AppGameChildRuntimeTransportReceiptBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameChildRuntimeTransportReceiptBoundaryRowIsHonest(row) ||
        'Expected child runtime transport receipt boundary rows to preserve required transport/receipt refs without execution, receipt ingestion, provider delivery, or platform channel claims'
    )
  )
);

const AppGameChildRuntimeTransportReceiptBoundaryReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  boundaryId: ChildRuntimeTransportReceiptBoundaryIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceRuntimeWriterId: ChildRuntimeTransportReceiptBoundaryRefSchema,
  rows: Schema.Array(AppGameChildRuntimeTransportReceiptBoundaryRowSchema),
  transportRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  nonClaims: Schema.Array(AppGameChildRuntimeTransportReceiptBoundaryNonClaimSchema),
  runtimeTransportExecuted: Schema.Literal(false),
  runtimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildRuntimeTransportReceiptBoundaryReadModelSchema = withParser(
  AppGameChildRuntimeTransportReceiptBoundaryReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameChildRuntimeTransportReceiptBoundaryReadModelIsHonest(readModel) ||
        'Expected child runtime transport receipt boundary counts and non-claims to match transport-required, manual-required, and unavailable rows'
    )
  )
);

export type AppGameChildRuntimeTransportReceiptBoundaryState = Infer<
  typeof AppGameChildRuntimeTransportReceiptBoundaryStateSchema
>;
export type AppGameChildRuntimeTransportReceiptBoundaryRow = Infer<
  typeof AppGameChildRuntimeTransportReceiptBoundaryRowSchema
>;
export type AppGameChildRuntimeTransportReceiptBoundaryReadModel = Infer<
  typeof AppGameChildRuntimeTransportReceiptBoundaryReadModelSchema
>;

export type AppGameChildRuntimeTransportReceiptBoundaryOptions = {
  readonly generatedAt: string;
  readonly boundaryId: string;
  readonly receiptContractRefs: readonly string[];
};

export function buildAppGameChildRuntimeTransportReceiptBoundaryReadModel(
  options: AppGameChildRuntimeTransportReceiptBoundaryOptions,
  sourceReadModel: AppGameChildDeviceRuntimeWriterReadModel
): AppGameChildRuntimeTransportReceiptBoundaryReadModel {
  const source = AppGameChildDeviceRuntimeWriterReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map((row) => childRuntimeTransportReceiptBoundaryRow(options, row));

  return AppGameChildRuntimeTransportReceiptBoundaryReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    boundaryId: options.boundaryId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourceRuntimeWriterId: source.runtimeWriterId,
    rows,
    transportRequiredCount: countRows(rows, AppGameChildRuntimeTransportReceiptBoundaryState.TransportRequired),
    manualRequiredCount: countRows(rows, AppGameChildRuntimeTransportReceiptBoundaryState.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildRuntimeTransportReceiptBoundaryState.Unavailable),
    nonClaims: RequiredAppGameChildRuntimeTransportReceiptBoundaryNonClaims,
    runtimeTransportExecuted: false,
    runtimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function summarizeAppGameChildRuntimeTransportReceiptBoundary(
  readModel: AppGameChildRuntimeTransportReceiptBoundaryReadModel
) {
  return {
    transportRequiredCount: readModel.transportRequiredCount,
    manualRequiredCount: readModel.manualRequiredCount,
    unavailableCount: readModel.unavailableCount,
    rowCount: readModel.rows.length,
  } as const;
}

function childRuntimeTransportReceiptBoundaryRow(
  options: AppGameChildRuntimeTransportReceiptBoundaryOptions,
  row: AppGameChildDeviceRuntimeWriterRow
): AppGameChildRuntimeTransportReceiptBoundaryRow {
  const boundaryState = boundaryStateFor(row);
  const openGaps = openGapsFor(boundaryState);

  return AppGameChildRuntimeTransportReceiptBoundaryRowSchema.parse({
    boundaryRowId: `app-game-child-runtime-transport-receipt-boundary-${row.runtimeWriterRowId}`,
    sourceRuntimeWriterRowId: row.runtimeWriterRowId,
    sourceWriterEnvelopeState: row.writerEnvelopeState,
    boundaryState,
    requiredTransportRefs:
      boundaryState === AppGameChildRuntimeTransportReceiptBoundaryState.TransportRequired
        ? row.childDeliveryTargetRefs
        : openGaps,
    requiredReceiptRefs:
      boundaryState === AppGameChildRuntimeTransportReceiptBoundaryState.TransportRequired
        ? options.receiptContractRefs
        : openGaps,
    openGaps,
    runtimeTransportExecuted: false,
    runtimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  });
}

function boundaryStateFor(row: AppGameChildDeviceRuntimeWriterRow): AppGameChildRuntimeTransportReceiptBoundaryState {
  if (row.writerEnvelopeState === AppGameChildDeviceRuntimeWriterState.EnvelopeReady) {
    return AppGameChildRuntimeTransportReceiptBoundaryState.TransportRequired;
  }
  if (row.writerEnvelopeState === AppGameChildDeviceRuntimeWriterState.Unavailable) {
    return AppGameChildRuntimeTransportReceiptBoundaryState.Unavailable;
  }
  return AppGameChildRuntimeTransportReceiptBoundaryState.ManualRequired;
}

function openGapsFor(state: AppGameChildRuntimeTransportReceiptBoundaryState): readonly string[] {
  if (state === AppGameChildRuntimeTransportReceiptBoundaryState.Unavailable) {
    return ['source-unavailable', 'child-runtime-transport-not-executed'];
  }
  if (state === AppGameChildRuntimeTransportReceiptBoundaryState.ManualRequired) {
    return ['manual-proof-required', 'child-runtime-transport-not-executed'];
  }
  return [
    'child-runtime-transport-not-executed',
    'child-runtime-receipt-not-ingested',
    'provider-delivery-not-executed',
    'platform-delivery-channel-not-proved',
  ];
}

function appGameChildRuntimeTransportReceiptBoundaryRowIsHonest(
  row: Infer<typeof AppGameChildRuntimeTransportReceiptBoundaryRowBaseSchema>
): boolean {
  return (
    row.requiredTransportRefs.length > 0 &&
    row.requiredReceiptRefs.length > 0 &&
    row.openGaps.some((gap) => gap === 'child-runtime-transport-not-executed') &&
    !row.runtimeTransportExecuted &&
    !row.runtimeReceiptIngested &&
    !row.providerDeliveryExecuted &&
    !row.platformDeliveryChannelClaimed
  );
}

function appGameChildRuntimeTransportReceiptBoundaryReadModelIsHonest(
  readModel: Infer<typeof AppGameChildRuntimeTransportReceiptBoundaryReadModelBaseSchema>
): boolean {
  return (
    readModel.transportRequiredCount ===
      countRows(readModel.rows, AppGameChildRuntimeTransportReceiptBoundaryState.TransportRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameChildRuntimeTransportReceiptBoundaryState.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, AppGameChildRuntimeTransportReceiptBoundaryState.Unavailable) &&
    RequiredAppGameChildRuntimeTransportReceiptBoundaryNonClaims.every((claim) =>
      readModel.nonClaims.includes(claim)
    ) &&
    !readModel.runtimeTransportExecuted &&
    !readModel.runtimeReceiptIngested &&
    !readModel.providerDeliveryExecuted &&
    !readModel.platformDeliveryChannelClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly boundaryState: AppGameChildRuntimeTransportReceiptBoundaryState }>,
  state: AppGameChildRuntimeTransportReceiptBoundaryState
): number {
  return rows.filter((row) => row.boundaryState === state).length;
}

