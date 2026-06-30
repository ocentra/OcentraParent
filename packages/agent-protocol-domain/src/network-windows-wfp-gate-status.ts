import {
  AgentNetworkWindowsWfpGateStatusSchema,
  type AgentNetworkWindowsWfpGateStatus,
} from '@ocentra-parent/schema-domain/agent-network-windows-wfp-status';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { mapJsonPayloadEventToStatus, parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

type AgentNetworkWindowsWfpGateStatusFailureReason =
  | 'wrong-event'
  | 'missing-windows-wfp-gate-status'
  | 'invalid-windows-wfp-gate-status-json'
  | 'invalid-windows-wfp-gate-status';

export type AgentNetworkWindowsWfpGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkWindowsWfpGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkWindowsWfpGateStatusFailureReason;
    };

export function parseAgentNetworkWindowsWfpGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkWindowsWfpGateStatusParseResult {
  return mapJsonPayloadEventToStatus(
    parseJsonPayloadFieldEvent(
      event,
      AgentEvent.NetworkWindowsWfpGateStatusReported,
      AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus,
      AgentNetworkWindowsWfpGateStatusSchema
    ),
    {
      'missing-json-field': 'missing-windows-wfp-gate-status',
      'invalid-json': 'invalid-windows-wfp-gate-status-json',
      'invalid-payload': 'invalid-windows-wfp-gate-status',
    }
  );
}
