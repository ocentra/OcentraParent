import * as TrackingContracts from '@ocentra-parent/schema-domain/agent-tracking-retention-settings-write-command';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { parseJsonPayloadFieldEvent } from './protocol-event-payload.js';

export type AgentTrackingRetentionSettingsWriteResultFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export const AgentTrackingRetentionSettingsWriteResultParseState = {
  Parsed: 'parsed',
  Failed: 'failed',
} as const;

export type AgentTrackingRetentionSettingsWriteResultParseState =
  (typeof AgentTrackingRetentionSettingsWriteResultParseState)[keyof typeof AgentTrackingRetentionSettingsWriteResultParseState];

export type AgentTrackingRetentionSettingsWriteResultParseResult =
  | {
      readonly parseState: typeof AgentTrackingRetentionSettingsWriteResultParseState.Parsed;
      readonly value: TrackingContracts.AgentTrackingRetentionSettingsWriteResult;
    }
  | {
      readonly parseState: typeof AgentTrackingRetentionSettingsWriteResultParseState.Failed;
      readonly reason: AgentTrackingRetentionSettingsWriteResultFailureReason;
    };

export function parseAgentTrackingRetentionSettingsWriteResultEvent(
  event: AgentEventEnvelope
): AgentTrackingRetentionSettingsWriteResultParseResult {
  const parsed = parseJsonPayloadFieldEvent(
    event,
    AgentEvent.ActivityTrackingRetentionSettingsWriteReported,
    AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsWriteResult,
    TrackingContracts.AgentTrackingRetentionSettingsWriteResultSchema
  );
  if (!parsed.ok) {
    return adapterFailure(parsed.reason);
  }

  return {
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Parsed,
    value: parsed.value,
  };
}

function adapterFailure(
  reason: AgentTrackingRetentionSettingsWriteResultFailureReason
): AgentTrackingRetentionSettingsWriteResultParseResult {
  return {
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason,
  };
}
