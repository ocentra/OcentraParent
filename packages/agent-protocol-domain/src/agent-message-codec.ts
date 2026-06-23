import {
  AgentCommandEnvelopeSchema,
  AgentEventEnvelopeSchema,
  decodeAgentMessageId,
  decodeAgentTimestamp,
  decodeSerializedAgentMessage,
  type AgentCommandEnvelope,
  type AgentCommandName,
  type AgentEventEnvelope,
  type AgentProtocolLogFields,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  AgentProtocolSchemaVersion,
  type AgentMessageId,
  type AgentMessageTarget,
  type SerializedAgentMessage,
} from '@ocentra-parent/schema-domain/event-primitives';

type AgentMessageCodecGlobal = typeof globalThis & {
  readonly crypto?: {
    readonly randomUUID?: () => string;
  };
};

export function createAgentCommand(
  command: AgentCommandName,
  payload: AgentProtocolLogFields = {},
  target: AgentMessageTarget = AgentProtocolDefaults.Target.LocalhostWindowsAgent
): AgentCommandEnvelope {
  return AgentCommandEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    messageId: createMessageId(),
    sentAt: decodeAgentTimestamp(new Date().toISOString()),
    source: AgentProtocolDefaults.Peer.PortalDev,
    target,
    command,
    payload,
  });
}

export function serializeAgentCommand(command: AgentCommandEnvelope): SerializedAgentMessage {
  return decodeSerializedAgentMessage(JSON.stringify(command));
}

export function parseAgentEventMessage(data: unknown): AgentEventEnvelope {
  const payload = parseAgentEventInput(data);
  return AgentEventEnvelopeSchema.parse(payload);
}

function parseAgentEventInput(data: unknown): unknown {
  if (typeof data !== AgentProtocolDefaults.Primitive.String) {
    return data;
  }

  return JSON.parse(String(decodeSerializedAgentMessage(data))) as unknown;
}

function createMessageId(): AgentMessageId {
  const runtimeGlobal = globalThis as AgentMessageCodecGlobal;
  const randomId = runtimeGlobal.crypto?.randomUUID?.();
  if (randomId !== undefined) {
    return decodeAgentMessageId(`${AgentProtocolDefaults.MessageIdPrefix}${randomId}`);
  }
  return decodeAgentMessageId(`${AgentProtocolDefaults.MessageIdPrefix}${Date.now()}`);
}
