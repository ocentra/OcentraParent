import {
  AgentNetworkWindowsFirewallLabStatusSchema,
  type AgentNetworkWindowsFirewallLabStatus,
} from '@ocentra-parent/schema-domain/agent-network-windows-firewall-status';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

export type AgentNetworkWindowsFirewallLabStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkWindowsFirewallLabStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-windows-firewall-lab-status'
        | 'invalid-windows-firewall-lab-status-json'
        | 'invalid-windows-firewall-lab-status';
    };

export function parseAgentNetworkWindowsFirewallLabStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkWindowsFirewallLabStatusParseResult {
  if (event.event !== AgentEvent.NetworkWindowsFirewallLabStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-windows-firewall-lab-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-windows-firewall-lab-status-json' };
  }

  const parsed = AgentNetworkWindowsFirewallLabStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-windows-firewall-lab-status' };
  }

  return { ok: true, status: parsed.data };
}
