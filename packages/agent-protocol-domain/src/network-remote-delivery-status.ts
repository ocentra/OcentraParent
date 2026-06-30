import {
  AgentNetworkRemoteDeliveryStatusSchema,
  type AgentNetworkRemoteDeliveryStatus,
} from '@ocentra-parent/schema-domain/network-remote-delivery-status';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { mapJsonPayloadEventToStatus, parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

type AgentNetworkRemoteDeliveryStatusFailureReason =
  | 'wrong-event'
  | 'missing-remote-delivery-status'
  | 'invalid-remote-delivery-status-json'
  | 'invalid-remote-delivery-status';

export type AgentNetworkRemoteDeliveryStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkRemoteDeliveryStatus;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkRemoteDeliveryStatusFailureReason;
    };

export function parseAgentNetworkRemoteDeliveryStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkRemoteDeliveryStatusParseResult {
  return mapJsonPayloadEventToStatus(
    parseJsonPayloadFieldEvent(
      event,
      AgentEvent.NetworkRemoteDeliveryStatusReported,
      AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus,
      AgentNetworkRemoteDeliveryStatusSchema
    ),
    {
      'missing-json-field': 'missing-remote-delivery-status',
      'invalid-json': 'invalid-remote-delivery-status-json',
      'invalid-payload': 'invalid-remote-delivery-status',
    }
  );
}
