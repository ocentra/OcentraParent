import {
  AgentNetworkWindowsWfpGateStatusSchema,
  type AgentNetworkWindowsWfpGateStatus,
} from '@ocentra-parent/schema-domain/agent-network-windows-wfp-status';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export type AgentNetworkWindowsWfpGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkWindowsWfpGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-windows-wfp-gate-status'
        | 'invalid-windows-wfp-gate-status-json'
        | 'invalid-windows-wfp-gate-status';
    };

export function parseAgentNetworkWindowsWfpGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkWindowsWfpGateStatusParseResult {
  if (event.event !== AgentEvent.NetworkWindowsWfpGateStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-windows-wfp-gate-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-windows-wfp-gate-status-json' };
  }

  const parsed = AgentNetworkWindowsWfpGateStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-windows-wfp-gate-status' };
  }

  return { ok: true, status: parsed.data };
}
