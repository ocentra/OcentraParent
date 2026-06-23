import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const ChildRuntimeTransportReceiptCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGameChildRuntimeTransportReceiptSchemaVersion = 1;
export const AgentAppGameChildRuntimeTransportReceiptPayloadField =
  'appGameChildRuntimeTransportReceiptReadModel' as const;
export const AgentAppGameChildRuntimeTransportReceiptReadModelId = 'app-game-child-runtime-transport-receipt' as const;
export const AgentAppGameChildRuntimeTransportReceiptSourceRuntimeWriterRef =
  'app-game-child-device-runtime-writer' as const;
export const AgentAppGameChildRuntimeTransportReceiptCustodyLabel = 'app-game-child-runtime-transport-receipt' as const;
export const AgentAppGameChildRuntimeTransportReceiptCapabilityStatus =
  'app-game-child-runtime-transport-required' as const;
export const AgentAppGameChildRuntimeTransportReceiptTransportContractRef =
  'child-runtime-transport-contract-ref' as const;
export const AgentAppGameChildRuntimeTransportReceiptReceiptContractRef =
  'child-runtime-delivery-receipt-contract-ref' as const;
export const AgentAppGameChildRuntimeTransportReceiptGapTransportNotExecuted =
  'child-runtime-transport-not-executed' as const;
export const AgentAppGameChildRuntimeTransportReceiptGapReceiptNotIngested =
  'child-runtime-receipt-not-ingested' as const;
export const AgentAppGameChildRuntimeTransportReceiptGapProviderNotExecuted = 'provider-delivery-not-executed' as const;
export const AgentAppGameChildRuntimeTransportReceiptGapPlatformChannelNotProved =
  'platform-delivery-channel-not-proved' as const;

export const AgentAppGameChildRuntimeTransportReceiptState = {
  TransportRequired: 'child-runtime-transport-required',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AgentAppGameChildRuntimeTransportReceiptStateValues = [
  AgentAppGameChildRuntimeTransportReceiptState.TransportRequired,
  AgentAppGameChildRuntimeTransportReceiptState.ManualRequired,
  AgentAppGameChildRuntimeTransportReceiptState.Unavailable,
] as const;

export const AgentAppGameChildRuntimeTransportReceiptProductMeanings = ['native-app', 'native-game'] as const;

export const AgentAppGameChildRuntimeTransportReceiptCanonicalGaps = [
  AgentAppGameChildRuntimeTransportReceiptGapTransportNotExecuted,
  AgentAppGameChildRuntimeTransportReceiptGapReceiptNotIngested,
  AgentAppGameChildRuntimeTransportReceiptGapProviderNotExecuted,
  AgentAppGameChildRuntimeTransportReceiptGapPlatformChannelNotProved,
] as const;

export const AgentAppGameChildRuntimeTransportReceiptCanonicalRefs = [
  AgentAppGameChildRuntimeTransportReceiptTransportContractRef,
  AgentAppGameChildRuntimeTransportReceiptReceiptContractRef,
] as const;

export const AgentAppGameChildRuntimeTransportReceiptParityManifest = {
  schemaVersion: AgentAppGameChildRuntimeTransportReceiptSchemaVersion,
  payloadField: AgentAppGameChildRuntimeTransportReceiptPayloadField,
  readModelId: AgentAppGameChildRuntimeTransportReceiptReadModelId,
  sourceRuntimeWriterRef: AgentAppGameChildRuntimeTransportReceiptSourceRuntimeWriterRef,
  custodyLabel: AgentAppGameChildRuntimeTransportReceiptCustodyLabel,
  capabilityStatus: AgentAppGameChildRuntimeTransportReceiptCapabilityStatus,
  stateValues: [...AgentAppGameChildRuntimeTransportReceiptStateValues],
  productMeanings: [...AgentAppGameChildRuntimeTransportReceiptProductMeanings],
  canonicalRefs: [...AgentAppGameChildRuntimeTransportReceiptCanonicalRefs],
  canonicalGaps: [...AgentAppGameChildRuntimeTransportReceiptCanonicalGaps],
  rowFields: [
    'schemaVersion',
    'rowId',
    'sourceRuntimeWriterRowId',
    'boundaryState',
    'productMeanings',
    'requiredTransportRefs',
    'requiredReceiptRefs',
    'openGaps',
    'runtimeTransportExecuted',
    'runtimeReceiptIngested',
    'providerDeliveryExecuted',
    'platformDeliveryChannelClaimed',
  ],
  readModelFields: [
    'schemaVersion',
    'readModelId',
    'generatedAt',
    'sourceReadModelIds',
    'custodyLabel',
    'capabilityStatus',
    'returned',
    'transportRequiredCount',
    'manualRequiredCount',
    'unavailableCount',
    'runtimeTransportExecuted',
    'runtimeReceiptIngested',
    'providerDeliveryExecuted',
    'platformDeliveryChannelClaimed',
    'adapterDispatchClaimed',
    'platformEnforcementClaimed',
    'rawPrivateSourceRowsIncluded',
    'rows',
  ],
} as const;

const AgentAppGameChildRuntimeTransportReceiptStateLiteralSchema = Schema.Literal(
  ...AgentAppGameChildRuntimeTransportReceiptStateValues
);

export const AgentAppGameChildRuntimeTransportReceiptStateSchema = withParser(
  AgentAppGameChildRuntimeTransportReceiptStateLiteralSchema
);

const AgentAppGameChildRuntimeTransportReceiptProductMeaningLiteralSchema = Schema.Literal(
  ...AgentAppGameChildRuntimeTransportReceiptProductMeanings
);

const AgentAppGameChildRuntimeTransportReceiptRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AgentAppGameChildRuntimeTransportReceiptSchemaVersion),
  rowId: NonEmptyStringSchema,
  sourceRuntimeWriterRowId: NonEmptyStringSchema,
  boundaryState: AgentAppGameChildRuntimeTransportReceiptStateLiteralSchema,
  productMeanings: Schema.Array(AgentAppGameChildRuntimeTransportReceiptProductMeaningLiteralSchema),
  requiredTransportRefs: Schema.Array(NonEmptyStringSchema),
  requiredReceiptRefs: Schema.Array(NonEmptyStringSchema),
  openGaps: Schema.Array(NonEmptyStringSchema),
  runtimeTransportExecuted: Schema.Literal(false),
  runtimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
});

type AgentAppGameChildRuntimeTransportReceiptRowCandidate = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptRowBaseSchema
>;

const AgentAppGameChildRuntimeTransportReceiptRowSchemaInternal =
  AgentAppGameChildRuntimeTransportReceiptRowBaseSchema.pipe(
    Schema.filter(
      (row: AgentAppGameChildRuntimeTransportReceiptRowCandidate) =>
        childRuntimeTransportReceiptRowIsHonest(row) ||
        'Expected app/game child runtime transport receipt rows to keep delivery and receipt execution unclaimed'
    )
  );

export const AgentAppGameChildRuntimeTransportReceiptRowSchema = withParser(
  AgentAppGameChildRuntimeTransportReceiptRowSchemaInternal
);

const AgentAppGameChildRuntimeTransportReceiptReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AgentAppGameChildRuntimeTransportReceiptSchemaVersion),
  readModelId: Schema.Literal(AgentAppGameChildRuntimeTransportReceiptReadModelId),
  generatedAt: NonEmptyStringSchema,
  sourceReadModelIds: Schema.Array(NonEmptyStringSchema),
  custodyLabel: Schema.Literal(AgentAppGameChildRuntimeTransportReceiptCustodyLabel),
  capabilityStatus: Schema.Literal(AgentAppGameChildRuntimeTransportReceiptCapabilityStatus),
  returned: ChildRuntimeTransportReceiptCount,
  transportRequiredCount: ChildRuntimeTransportReceiptCount,
  manualRequiredCount: ChildRuntimeTransportReceiptCount,
  unavailableCount: ChildRuntimeTransportReceiptCount,
  runtimeTransportExecuted: Schema.Literal(false),
  runtimeReceiptIngested: Schema.Literal(false),
  providerDeliveryExecuted: Schema.Literal(false),
  platformDeliveryChannelClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  rows: Schema.Array(AgentAppGameChildRuntimeTransportReceiptRowSchemaInternal),
});

type AgentAppGameChildRuntimeTransportReceiptReadModelCandidate = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptReadModelBaseSchema
>;

const AgentAppGameChildRuntimeTransportReceiptReadModelSchemaInternal =
  AgentAppGameChildRuntimeTransportReceiptReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel: AgentAppGameChildRuntimeTransportReceiptReadModelCandidate) =>
        childRuntimeTransportReceiptCountsMatch(readModel) ||
        'Expected app/game child runtime transport receipt counts to match status rows'
    )
  );
type AgentAppGameChildRuntimeTransportReceiptStateValue =
  (typeof AgentAppGameChildRuntimeTransportReceiptState)[keyof typeof AgentAppGameChildRuntimeTransportReceiptState];

export const AgentAppGameChildRuntimeTransportReceiptReadModelSchema = withParser(
  AgentAppGameChildRuntimeTransportReceiptReadModelSchemaInternal
);

export type AgentAppGameChildRuntimeTransportReceiptRow = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptRowSchema
>;
export type AgentAppGameChildRuntimeTransportReceiptReadModel = Infer<
  typeof AgentAppGameChildRuntimeTransportReceiptReadModelSchema
>;

function childRuntimeTransportReceiptRowIsHonest(row: AgentAppGameChildRuntimeTransportReceiptRowCandidate): boolean {
  return (
    row.productMeanings.includes('native-app') &&
    row.productMeanings.includes('native-game') &&
    row.requiredTransportRefs.length > 0 &&
    row.requiredReceiptRefs.length > 0 &&
    row.openGaps.some((gap: string) => gap === AgentAppGameChildRuntimeTransportReceiptGapTransportNotExecuted) &&
    !row.runtimeTransportExecuted &&
    !row.runtimeReceiptIngested &&
    !row.providerDeliveryExecuted &&
    !row.platformDeliveryChannelClaimed
  );
}

function childRuntimeTransportReceiptCountsMatch(
  readModel: AgentAppGameChildRuntimeTransportReceiptReadModelCandidate
): boolean {
  return (
    readModel.returned === readModel.rows.length &&
    readModel.transportRequiredCount ===
      countRows(readModel.rows, AgentAppGameChildRuntimeTransportReceiptState.TransportRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AgentAppGameChildRuntimeTransportReceiptState.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, AgentAppGameChildRuntimeTransportReceiptState.Unavailable) &&
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
  rows: readonly AgentAppGameChildRuntimeTransportReceiptRowCandidate[],
  state: AgentAppGameChildRuntimeTransportReceiptStateValue
): number {
  return rows.filter((row) => row.boundaryState === state).length;
}
