import {
  AgentNetworkLinuxNftablesLabStatusSchema,
  type AgentNetworkLinuxNftablesLabStatus,
} from '@ocentra-parent/schema-domain/agent-network-linux-nftables-status';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { mapJsonPayloadEventToStatus, parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

type AgentNetworkLinuxNftablesLabStatusFailureReason =
  | 'wrong-event'
  | 'missing-linux-nftables-lab-status'
  | 'invalid-linux-nftables-lab-status-json'
  | 'invalid-linux-nftables-lab-status';

export type AgentNetworkLinuxNftablesLabStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkLinuxNftablesLabStatus;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkLinuxNftablesLabStatusFailureReason;
    };

export function parseAgentNetworkLinuxNftablesLabStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkLinuxNftablesLabStatusParseResult {
  return mapJsonPayloadEventToStatus(
    parseJsonPayloadFieldEvent(
      event,
      AgentEvent.NetworkLinuxNftablesLabStatusReported,
      AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus,
      AgentNetworkLinuxNftablesLabStatusSchema
    ),
    {
      'missing-json-field': 'missing-linux-nftables-lab-status',
      'invalid-json': 'invalid-linux-nftables-lab-status-json',
      'invalid-payload': 'invalid-linux-nftables-lab-status',
    }
  );
}
