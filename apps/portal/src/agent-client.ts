import {
  AgentCommandEnvelopeSchema,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  AgentProtocolSchemaVersion,
  decodeAgentMessageId,
  decodeAgentTimestamp,
  decodeSerializedAgentMessage,
  type AgentCommandEnvelope,
  type AgentCommandName,
  type AgentMessageTarget,
  type AgentEventEnvelope,
  type AgentMessageId,
  type AgentProtocolLogFields,
  type SerializedAgentMessage,
} from '@ocentra-parent/agent-protocol-domain/contracts';

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

  return JSON.parse(decodeSerializedAgentMessage(data)) as unknown;
}

function createMessageId(): AgentMessageId {
  const randomId = globalThis.crypto?.randomUUID?.();
  if (randomId !== undefined) {
    return decodeAgentMessageId(`${AgentProtocolDefaults.MessageIdPrefix}${randomId}`);
  }
  return decodeAgentMessageId(`${AgentProtocolDefaults.MessageIdPrefix}${Date.now()}`);
}
