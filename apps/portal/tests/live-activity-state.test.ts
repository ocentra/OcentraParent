import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentNetworkRuntimeEventSchemaVersion,
  AgentNetworkRuntimeEventType,
} from '@ocentra-parent/agent-protocol-domain/network-runtime-events';
import { resolveLiveActivityState } from '../src/live-activity-state';

const NoClaimBoundary = {
  exactUrlAvailable: false,
  decryptedHttpsPayloadAvailable: false,
  messageContentAvailable: false,
  searchQueryAvailable: false,
  adapterActionExecuted: false,
} as const;

const FlowObserved = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  flowEventRef: 'event.network.flow.observed.1',
  observedAt: '2026-06-08T22:45:00Z',
  deviceRef: 'device.child.windows-1',
  flowEvidenceRef: 'evidence.network.flow.1',
  custody: 'child-device-query-store',
  evidenceGrade: 'A',
  claimBoundary: NoClaimBoundary,
} as const;

describe('portal live activity network service state', () => {
  it('resolves network service events through typed parsers', () => {
    const liveActivity = resolveLiveActivityState([
      eventWithPayload(AgentEvent.NetworkRuntimeEventChainStreamReported, {
        [AgentProtocolDefaults.Field.NetworkRuntimeStreamedEvents]: 1,
        [AgentProtocolDefaults.Field.NetworkRuntimeEventChainStream]: JSON.stringify([
          {
            eventType: AgentNetworkRuntimeEventType.NetworkFlowObserved,
            payload: FlowObserved,
          },
        ]),
      }),
      eventWithPayload(AgentEvent.NetworkRemoteDeliveryStatusReported, {
        [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: '{',
      }),
      eventWithPayload(AgentEvent.NetworkLiveCaptureStatusReported, {}),
      eventWithPayload(AgentEvent.NetworkLinuxNftablesLabStatusReported, {
        [AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus]: '{',
      }),
      eventWithPayload(AgentEvent.NetworkWindowsFirewallLabStatusReported, {
        [AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus]: '{',
      }),
      eventWithPayload(AgentEvent.NetworkWindowsWfpGateStatusReported, {
        [AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus]: '{',
      }),
    ]);

    expect(liveActivity.networkRuntimeEventChainEvent?.event).toBe('agent.network.runtime.event-chain.stream.reported');
    expect(liveActivity.networkRuntimeEventChainStream).toEqual({
      streamedEventCount: 1,
      invalidEventCount: 0,
      events: [
        {
          ok: true,
          eventType: 'network.flow.observed',
          value: FlowObserved,
        },
      ],
    });
    expect(liveActivity.networkRemoteDeliveryStatusEvent?.event).toBe('agent.network.remote-delivery.status.reported');
    expect(liveActivity.networkRemoteDeliveryStatusResult).toEqual({
      ok: false,
      reason: 'invalid-remote-delivery-status-json',
    });
    expect(liveActivity.networkLiveCaptureStatusEvent?.event).toBe('agent.network.live-capture.status.reported');
    expect(liveActivity.networkLiveCaptureStatusResult).toEqual({
      ok: false,
      reason: 'missing-live-capture-status',
    });
    expect(liveActivity.networkLinuxNftablesLabStatusEvent?.event).toBe(
      'agent.network.linux-nftables-lab.status.reported'
    );
    expect(liveActivity.networkLinuxNftablesLabStatusResult).toEqual({
      ok: false,
      reason: 'invalid-linux-nftables-lab-status-json',
    });
    expect(liveActivity.networkWindowsFirewallLabStatusEvent?.event).toBe(
      'agent.network.windows-firewall-lab.status.reported'
    );
    expect(liveActivity.networkWindowsFirewallLabStatusResult).toEqual({
      ok: false,
      reason: 'invalid-windows-firewall-lab-status-json',
    });
    expect(liveActivity.networkWindowsWfpGateStatusEvent?.event).toBe('agent.network.windows-wfp-gate.status.reported');
    expect(liveActivity.networkWindowsWfpGateStatusResult).toEqual({
      ok: false,
      reason: 'invalid-windows-wfp-gate-status-json',
    });
  });
});

function eventWithPayload(
  event: AgentEventEnvelope['event'],
  payload: AgentEventEnvelope['payload']
): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'portal-live-activity-network-event',
    correlationId: 'portal-live-activity-network-correlation',
    sentAt: '2026-06-08T22:45:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  });
}
