import {
  AgentNetworkLinuxNftablesLabStatusSchema,
  type AgentNetworkLinuxNftablesLabStatus,
} from '@ocentra-parent/schema-domain/agent-network-linux-nftables-status';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

export type AgentNetworkLinuxNftablesLabStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkLinuxNftablesLabStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-linux-nftables-lab-status'
        | 'invalid-linux-nftables-lab-status-json'
        | 'invalid-linux-nftables-lab-status';
    };

export function parseAgentNetworkLinuxNftablesLabStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkLinuxNftablesLabStatusParseResult {
  if (event.event !== AgentEvent.NetworkLinuxNftablesLabStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-linux-nftables-lab-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-linux-nftables-lab-status-json' };
  }

  const parsed = AgentNetworkLinuxNftablesLabStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-linux-nftables-lab-status' };
  }

  return { ok: true, status: parsed.data };
}
