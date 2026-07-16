const passthroughSchema = <T>() => ({
  parse: (value: unknown) => value as T,
});

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
export type AgentAppGameChildRuntimeTransportReceiptState =
  (typeof AgentAppGameChildRuntimeTransportReceiptState)[keyof typeof AgentAppGameChildRuntimeTransportReceiptState];
export const AgentAppGameChildRuntimeTransportReceiptStateSchema =
  passthroughSchema<AgentAppGameChildRuntimeTransportReceiptState>();

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
  stateValues: [...Object.values(AgentAppGameChildRuntimeTransportReceiptState)],
  productMeanings: ['native-app', 'native-game'],
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

export interface AgentAppGameChildRuntimeTransportReceiptRow {
  schemaVersion: number;
  rowId: string;
  sourceRuntimeWriterRowId: string;
  boundaryState: AgentAppGameChildRuntimeTransportReceiptState;
  productMeanings: readonly ['native-app', 'native-game'] | readonly string[];
  requiredTransportRefs: readonly string[];
  requiredReceiptRefs: readonly string[];
  openGaps: readonly string[];
  runtimeTransportExecuted: boolean;
  runtimeReceiptIngested: boolean;
  providerDeliveryExecuted: boolean;
  platformDeliveryChannelClaimed: boolean;
}

export interface AgentAppGameChildRuntimeTransportReceiptReadModel {
  schemaVersion: number;
  readModelId: string;
  generatedAt: string;
  sourceReadModelIds: readonly string[];
  custodyLabel: string;
  capabilityStatus: string;
  returned: number;
  transportRequiredCount: number;
  manualRequiredCount: number;
  unavailableCount: number;
  runtimeTransportExecuted: boolean;
  runtimeReceiptIngested: boolean;
  providerDeliveryExecuted: boolean;
  platformDeliveryChannelClaimed: boolean;
  adapterDispatchClaimed: boolean;
  platformEnforcementClaimed: boolean;
  rawPrivateSourceRowsIncluded: boolean;
  rows: readonly AgentAppGameChildRuntimeTransportReceiptRow[];
}

export const AgentAppGameChildRuntimeTransportReceiptRowSchema =
  passthroughSchema<AgentAppGameChildRuntimeTransportReceiptRow>();
export const AgentAppGameChildRuntimeTransportReceiptReadModelSchema =
  passthroughSchema<AgentAppGameChildRuntimeTransportReceiptReadModel>();
