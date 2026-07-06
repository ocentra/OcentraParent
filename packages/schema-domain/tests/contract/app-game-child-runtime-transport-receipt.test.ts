import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  AgentAppGameChildRuntimeTransportReceiptCapabilityStatus,
  AgentAppGameChildRuntimeTransportReceiptCustodyLabel,
  AgentAppGameChildRuntimeTransportReceiptGapReceiptNotIngested,
  AgentAppGameChildRuntimeTransportReceiptGapTransportNotExecuted,
  AgentAppGameChildRuntimeTransportReceiptParityManifest,
  AgentAppGameChildRuntimeTransportReceiptReadModelId,
  AgentAppGameChildRuntimeTransportReceiptReadModelSchema,
  AgentAppGameChildRuntimeTransportReceiptReceiptContractRef,
  AgentAppGameChildRuntimeTransportReceiptSchemaVersion,
  AgentAppGameChildRuntimeTransportReceiptSourceRuntimeWriterRef,
  AgentAppGameChildRuntimeTransportReceiptState,
  AgentAppGameChildRuntimeTransportReceiptTransportContractRef,
} from '../../src/generated-app-game-child-runtime-transport-receipt';

const ChildRuntimeTransportReceiptReadModel = {
  schemaVersion: AgentAppGameChildRuntimeTransportReceiptSchemaVersion,
  readModelId: AgentAppGameChildRuntimeTransportReceiptReadModelId,
  generatedAt: '2026-06-08T23:15:00.000Z',
  sourceReadModelIds: [AgentAppGameChildRuntimeTransportReceiptSourceRuntimeWriterRef],
  custodyLabel: AgentAppGameChildRuntimeTransportReceiptCustodyLabel,
  capabilityStatus: AgentAppGameChildRuntimeTransportReceiptCapabilityStatus,
  returned: 3,
  transportRequiredCount: 1,
  manualRequiredCount: 1,
  unavailableCount: 1,
  runtimeTransportExecuted: false,
  runtimeReceiptIngested: false,
  providerDeliveryExecuted: false,
  platformDeliveryChannelClaimed: false,
  adapterDispatchClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
  rows: [
    childRuntimeRow('limit-reached', AgentAppGameChildRuntimeTransportReceiptState.TransportRequired),
    childRuntimeRow('manual-required', AgentAppGameChildRuntimeTransportReceiptState.ManualRequired),
    childRuntimeRow('unavailable', AgentAppGameChildRuntimeTransportReceiptState.Unavailable),
  ],
} as const;

describe('schema-domain app-game child runtime transport receipt contract', () => {
  it('parses the canonical read-model sample', () => {
    expect(
      AgentAppGameChildRuntimeTransportReceiptReadModelSchema.parse(ChildRuntimeTransportReceiptReadModel)
    ).toEqual(ChildRuntimeTransportReceiptReadModel);
  });

  it('matches the Rust parity manifest', () => {
    const rustSource = readFileSync(
      new URL('../../../../crates/agent-protocol/src/app_game_child_runtime_transport_receipt.rs', import.meta.url),
      'utf8'
    );
    const match = rustSource.match(/APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_PARITY_MANIFEST: &str = r#"(.*?)"#;/su);

    expect(match).not.toBeNull();
    expect(JSON.parse(match![1])).toEqual(AgentAppGameChildRuntimeTransportReceiptParityManifest);
  });
});

function childRuntimeRow(suffix: string, boundaryState: string) {
  return {
    schemaVersion: AgentAppGameChildRuntimeTransportReceiptSchemaVersion,
    rowId: `app-game-child-runtime-transport-receipt-${suffix}`,
    sourceRuntimeWriterRowId: `app-game-child-device-runtime-writer-${suffix}`,
    boundaryState,
    productMeanings: ['native-app', 'native-game'],
    requiredTransportRefs:
      boundaryState === AgentAppGameChildRuntimeTransportReceiptState.TransportRequired
        ? [AgentAppGameChildRuntimeTransportReceiptTransportContractRef]
        : [AgentAppGameChildRuntimeTransportReceiptGapTransportNotExecuted],
    requiredReceiptRefs:
      boundaryState === AgentAppGameChildRuntimeTransportReceiptState.TransportRequired
        ? [AgentAppGameChildRuntimeTransportReceiptReceiptContractRef]
        : [AgentAppGameChildRuntimeTransportReceiptGapTransportNotExecuted],
    openGaps: [
      AgentAppGameChildRuntimeTransportReceiptGapTransportNotExecuted,
      AgentAppGameChildRuntimeTransportReceiptGapReceiptNotIngested,
    ],
    runtimeTransportExecuted: false,
    runtimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}
