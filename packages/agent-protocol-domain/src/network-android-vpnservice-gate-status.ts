import {
  AgentNetworkAndroidVpnServiceGateStatusSchema,
  type AgentNetworkAndroidVpnServiceGateStatus,
} from '@ocentra-parent/schema-domain/agent-network-android-vpnservice-status';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { mapJsonPayloadEventToStatus, parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

type AgentNetworkAndroidVpnServiceGateStatusFailureReason =
  | 'wrong-event'
  | 'missing-android-vpn-service-gate-status'
  | 'invalid-android-vpn-service-gate-status-json'
  | 'invalid-android-vpn-service-gate-status';

export type AgentNetworkAndroidVpnServiceGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkAndroidVpnServiceGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkAndroidVpnServiceGateStatusFailureReason;
    };

export function parseAgentNetworkAndroidVpnServiceGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkAndroidVpnServiceGateStatusParseResult {
  return mapJsonPayloadEventToStatus(
    parseJsonPayloadFieldEvent(
      event,
      AgentEvent.NetworkAndroidVpnServiceGateStatusReported,
      AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus,
      AgentNetworkAndroidVpnServiceGateStatusSchema
    ),
    {
      'missing-json-field': 'missing-android-vpn-service-gate-status',
      'invalid-json': 'invalid-android-vpn-service-gate-status-json',
      'invalid-payload': 'invalid-android-vpn-service-gate-status',
    }
  );
}
