import { describe, expect, it } from 'vitest';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  AgentAppGameChildRuntimeTransportReceiptPayloadField,
  AgentAppGameChildRuntimeTransportReceiptReceiptContractRef,
  AgentAppGameChildRuntimeTransportReceiptSchemaVersion,
  AgentAppGameChildRuntimeTransportReceiptState,
  AgentAppGameChildRuntimeTransportReceiptTransportContractRef,
} from '@ocentra-parent/schema-domain/app-game-child-runtime-transport-receipt';
import { AgentEvent, type AgentEventEnvelope } from '../../src/contracts';
import { parseAgentAppGameChildRuntimeTransportReceiptEvent } from '../../src/app-game-child-runtime-transport-receipt';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const ChildRuntimeTransportReceiptReadModel = {
  schemaVersion: AgentAppGameChildRuntimeTransportReceiptSchemaVersion,
  readModelId: 'app-game-child-runtime-transport-receipt',
  generatedAt: '2026-06-08T23:15:00.000Z',
  sourceReadModelIds: ['app-game-child-device-runtime-writer'],
  custodyLabel: 'app-game-child-runtime-transport-receipt',
  capabilityStatus: 'app-game-child-runtime-transport-required',
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

describe('agent app-game child runtime transport receipt parser', () => {
  it('parses the child runtime transport receipt read-model event payload', () => {
    const parsed = parseAgentAppGameChildRuntimeTransportReceiptEvent(
      childRuntimeTransportReceiptEvent(JSON.stringify(ChildRuntimeTransportReceiptReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: ChildRuntimeTransportReceiptReadModel,
    });
  });

  it('rejects invalid child runtime transport receipt payloads and claim upgrades', () => {
    expect(
      parseAgentAppGameChildRuntimeTransportReceiptEvent({
        ...childRuntimeTransportReceiptEvent(JSON.stringify(ChildRuntimeTransportReceiptReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameChildRuntimeTransportReceiptEvent(childRuntimeTransportReceiptEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameChildRuntimeTransportReceiptEvent(
        childRuntimeTransportReceiptEvent(
          JSON.stringify({
            ...ChildRuntimeTransportReceiptReadModel,
            runtimeTransportExecuted: true,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentAppGameChildRuntimeTransportReceiptEvent(
        childRuntimeTransportReceiptEvent(
          JSON.stringify({
            ...ChildRuntimeTransportReceiptReadModel,
            transportRequiredCount: 0,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function childRuntimeTransportReceiptEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-child-runtime-transport-receipt-event',
    correlationId: 'app-game-child-runtime-transport-receipt-command',
    sentAt: '2026-06-08T23:15:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameChildRuntimeTransportReceiptReadModelReported,
    severity: 'info',
    payload: {
      [AgentAppGameChildRuntimeTransportReceiptPayloadField]: serializedReadModel,
    },
    snapshot: null,
  };
}

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
        : ['child-runtime-transport-not-executed'],
    requiredReceiptRefs:
      boundaryState === AgentAppGameChildRuntimeTransportReceiptState.TransportRequired
        ? [AgentAppGameChildRuntimeTransportReceiptReceiptContractRef]
        : ['child-runtime-transport-not-executed'],
    openGaps: ['child-runtime-transport-not-executed', 'child-runtime-receipt-not-ingested'],
    runtimeTransportExecuted: false,
    runtimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}
