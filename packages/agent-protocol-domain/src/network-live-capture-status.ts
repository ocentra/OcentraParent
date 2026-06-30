import {
  AgentNetworkLiveCaptureStatusSchema,
  type AgentNetworkLiveCaptureStatus,
} from '@ocentra-parent/schema-domain/network-live-capture-status';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { mapJsonPayloadEventToStatus, parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

type AgentNetworkLiveCaptureStatusFailureReason =
  | 'wrong-event'
  | 'missing-live-capture-status'
  | 'invalid-live-capture-status-json'
  | 'invalid-live-capture-status';

export type AgentNetworkLiveCaptureStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkLiveCaptureStatus;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkLiveCaptureStatusFailureReason;
    };

export function parseAgentNetworkLiveCaptureStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkLiveCaptureStatusParseResult {
  return mapJsonPayloadEventToStatus(
    parseJsonPayloadFieldEvent(
      event,
      AgentEvent.NetworkLiveCaptureStatusReported,
      AgentProtocolDefaults.Field.NetworkLiveCaptureStatus,
      AgentNetworkLiveCaptureStatusSchema
    ),
    {
      'missing-json-field': 'missing-live-capture-status',
      'invalid-json': 'invalid-live-capture-status-json',
      'invalid-payload': 'invalid-live-capture-status',
    }
  );
}
