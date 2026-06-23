import {
  AgentNetworkRemoteDeliveryStatusSchema,
  type AgentNetworkRemoteDeliveryStatus,
} from '@ocentra-parent/schema-domain/network-remote-delivery-status';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export type AgentNetworkRemoteDeliveryStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkRemoteDeliveryStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-remote-delivery-status'
        | 'invalid-remote-delivery-status-json'
        | 'invalid-remote-delivery-status';
    };

export function parseAgentNetworkRemoteDeliveryStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkRemoteDeliveryStatusParseResult {
  if (event.event !== AgentEvent.NetworkRemoteDeliveryStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-remote-delivery-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-remote-delivery-status-json' };
  }

  const parsed = AgentNetworkRemoteDeliveryStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-remote-delivery-status' };
  }

  return { ok: true, status: parsed.data };
}
