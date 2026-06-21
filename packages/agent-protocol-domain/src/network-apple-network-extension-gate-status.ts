import {
  AgentNetworkAppleNetworkExtensionGateStatusSchema,
  type AgentNetworkAppleNetworkExtensionGateStatus,
} from '@ocentra-parent/schema-domain/agent-network-apple-extension-status';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

export type AgentNetworkAppleNetworkExtensionGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkAppleNetworkExtensionGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-apple-network-extension-gate-status'
        | 'invalid-apple-network-extension-gate-status-json'
        | 'invalid-apple-network-extension-gate-status';
    };

export function parseAgentNetworkAppleNetworkExtensionGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkAppleNetworkExtensionGateStatusParseResult {
  if (event.event !== AgentEvent.NetworkAppleNetworkExtensionGateStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-apple-network-extension-gate-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-apple-network-extension-gate-status-json' };
  }

  const parsed = AgentNetworkAppleNetworkExtensionGateStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-apple-network-extension-gate-status' };
  }

  return { ok: true, status: parsed.data };
}
