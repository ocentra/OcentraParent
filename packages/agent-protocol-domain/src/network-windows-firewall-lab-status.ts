import {
  AgentNetworkWindowsFirewallLabStatusSchema,
  type AgentNetworkWindowsFirewallLabStatus,
} from '@ocentra-parent/schema-domain/agent-network-windows-firewall-status';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { mapJsonPayloadEventToStatus, parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

type AgentNetworkWindowsFirewallLabStatusFailureReason =
  | 'wrong-event'
  | 'missing-windows-firewall-lab-status'
  | 'invalid-windows-firewall-lab-status-json'
  | 'invalid-windows-firewall-lab-status';

export type AgentNetworkWindowsFirewallLabStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkWindowsFirewallLabStatus;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkWindowsFirewallLabStatusFailureReason;
    };

export function parseAgentNetworkWindowsFirewallLabStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkWindowsFirewallLabStatusParseResult {
  return mapJsonPayloadEventToStatus(
    parseJsonPayloadFieldEvent(
      event,
      AgentEvent.NetworkWindowsFirewallLabStatusReported,
      AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus,
      AgentNetworkWindowsFirewallLabStatusSchema
    ),
    {
      'missing-json-field': 'missing-windows-firewall-lab-status',
      'invalid-json': 'invalid-windows-firewall-lab-status-json',
      'invalid-payload': 'invalid-windows-firewall-lab-status',
    }
  );
}
