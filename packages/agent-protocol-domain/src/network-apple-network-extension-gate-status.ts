import {
  AgentNetworkAppleNetworkExtensionGateStatusSchema,
  type AgentNetworkAppleNetworkExtensionGateStatus,
} from '@ocentra-parent/schema-domain/agent-network-apple-extension-status';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { mapJsonPayloadEventToStatus, parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

type AgentNetworkAppleNetworkExtensionGateStatusFailureReason =
  | 'wrong-event'
  | 'missing-apple-network-extension-gate-status'
  | 'invalid-apple-network-extension-gate-status-json'
  | 'invalid-apple-network-extension-gate-status';

export type AgentNetworkAppleNetworkExtensionGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkAppleNetworkExtensionGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkAppleNetworkExtensionGateStatusFailureReason;
    };

export function parseAgentNetworkAppleNetworkExtensionGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkAppleNetworkExtensionGateStatusParseResult {
  return mapJsonPayloadEventToStatus(
    parseJsonPayloadFieldEvent(
      event,
      AgentEvent.NetworkAppleNetworkExtensionGateStatusReported,
      AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus,
      AgentNetworkAppleNetworkExtensionGateStatusSchema
    ),
    {
      'missing-json-field': 'missing-apple-network-extension-gate-status',
      'invalid-json': 'invalid-apple-network-extension-gate-status-json',
      'invalid-payload': 'invalid-apple-network-extension-gate-status',
    }
  );
}
