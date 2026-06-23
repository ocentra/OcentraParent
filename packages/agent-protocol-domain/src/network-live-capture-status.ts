import {
  AgentNetworkLiveCaptureStatusSchema,
  type AgentNetworkLiveCaptureStatus,
} from '@ocentra-parent/schema-domain/network-live-capture-status';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export type AgentNetworkLiveCaptureStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkLiveCaptureStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-live-capture-status'
        | 'invalid-live-capture-status-json'
        | 'invalid-live-capture-status';
    };

export function parseAgentNetworkLiveCaptureStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkLiveCaptureStatusParseResult {
  if (event.event !== AgentEvent.NetworkLiveCaptureStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkLiveCaptureStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-live-capture-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-live-capture-status-json' };
  }

  const parsed = AgentNetworkLiveCaptureStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-live-capture-status' };
  }

  return { ok: true, status: parsed.data };
}
