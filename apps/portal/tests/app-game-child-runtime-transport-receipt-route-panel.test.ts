import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  AgentProtocolSchemaVersion,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { createAppGameChildRuntimeTransportReceiptPanelIntent } from '@ocentra-parent/portal-domain/app-game-child-runtime-transport-receipt-panel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { shouldRenderAppGameChildRuntimeTransportReceiptRoute } from '../src/AppGameChildRuntimeTransportReceiptRoutePanel';

const AppGameSchemaVersion = 1;

const ChildRuntimeTransportReceiptReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-child-runtime-transport-receipt',
  generatedAt: '2026-06-08T20:55:00.000Z',
  sourceReadModelIds: ['app-game-child-device-runtime-writer'],
  custodyLabel: 'app-game-child-runtime-transport-receipt',
  capabilityStatus: 'child-runtime-transport-required',
  returned: 2,
  transportRequiredCount: 1,
  manualRequiredCount: 1,
  unavailableCount: 0,
  runtimeTransportExecuted: false,
  runtimeReceiptIngested: false,
  providerDeliveryExecuted: false,
  platformDeliveryChannelClaimed: false,
  adapterDispatchClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
  rows: [
    childRuntimeTransportReceiptRow(
      'app-game-child-runtime-transport-receipt-warning',
      'child-runtime-transport-required'
    ),
    childRuntimeTransportReceiptRow('app-game-child-runtime-transport-receipt-apple', 'manual-required'),
  ],
} as const;

describe('app-game child runtime transport receipt portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', () => {
    expect(shouldRenderAppGameChildRuntimeTransportReceiptRoute(PortalRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameChildRuntimeTransportReceiptRoute(PortalRoute.Overview)).toBe(false);
  });

  it('uses the latest service-backed child runtime transport receipt event for the route intent', () => {
    const event = childRuntimeTransportReceiptEvent(JSON.stringify(ChildRuntimeTransportReceiptReadModel));
    const liveActivity = resolveLiveActivityState([event]);

    expect(liveActivity.appGameChildRuntimeTransportReceiptReadModel).toMatchObject({
      ok: true,
      value: {
        returned: 2,
        transportRequiredCount: 1,
        runtimeTransportExecuted: false,
        runtimeReceiptIngested: false,
      },
    });

    const intent = createAppGameChildRuntimeTransportReceiptPanelIntent(
      liveActivity.appGameChildRuntimeTransportReceiptReadModel?.ok === true
        ? liveActivity.appGameChildRuntimeTransportReceiptReadModel.value
        : null
    );

    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Transport rows', value: '2' }),
        expect.objectContaining({ label: 'Transport-required rows', value: '1' }),
        expect.objectContaining({ label: 'Manual-required rows', value: '1' }),
        expect.objectContaining({ label: 'Runtime transport', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Runtime receipt', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows.map((row) => row.title)).toEqual([
      'app-game-child-runtime-transport-receipt-warning',
      'app-game-child-runtime-transport-receipt-apple',
    ]);
  });
});

function childRuntimeTransportReceiptRow(rowId: string, boundaryState: string) {
  return {
    schemaVersion: AppGameSchemaVersion,
    rowId,
    sourceRuntimeWriterRowId: `${rowId}-source-writer`,
    boundaryState,
    productMeanings: ['native-app', 'native-game'],
    requiredTransportRefs: [`${rowId}-transport-ref`],
    requiredReceiptRefs: [`${rowId}-receipt-ref`],
    openGaps: [
      'child-runtime-transport-not-executed',
      'child-runtime-receipt-not-ingested',
      'provider-delivery-not-executed',
    ],
    runtimeTransportExecuted: false,
    runtimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}

function childRuntimeTransportReceiptEvent(serializedReadModel: string): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-child-runtime-transport-receipt-event',
    correlationId: 'app-game-child-runtime-transport-receipt-command',
    sentAt: '2026-06-08T20:55:01.000Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityAppGameChildRuntimeTransportReceiptReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameChildRuntimeTransportReceiptReadModel]: serializedReadModel,
    },
    snapshot: null,
  });
}
