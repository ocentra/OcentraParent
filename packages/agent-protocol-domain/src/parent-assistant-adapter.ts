import { ParentAssistantAnswerSchema } from '@ocentra-parent/parent-domain/parent-assistant';
import type { ParentAssistantAnswer } from '@ocentra-parent/parent-domain/parent-assistant';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentProtocolDefaults,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
} from './contracts';
import { AgentProtocolSchemaVersion, type AgentRoute } from './primitives';

export type ParentAssistantRuntimeCommandKind = 'answer-generate' | 'message-send' | 'quick-action' | 'action-preview';
export type ParentAssistantAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type ParentAssistantAdapterResult =
  | {
      readonly ok: true;
      readonly value: ParentAssistantAnswer;
    }
  | {
      readonly ok: false;
      readonly reason: ParentAssistantAdapterFailureReason;
    };

export type ParentAssistantCommandPeerInput = {
  readonly peerId: string;
  readonly role: 'portal' | 'agent-service' | 'cloud-relay';
};

export type ParentAssistantCommandTargetInput = {
  readonly deviceId: string;
  readonly platform: string;
  readonly route: AgentRoute;
};

export type CreateParentAssistantCommandInput = {
  readonly messageId: string;
  readonly sentAt: string;
  readonly source: ParentAssistantCommandPeerInput;
  readonly target: ParentAssistantCommandTargetInput;
  readonly requestId: string;
  readonly question: string;
  readonly evidenceSummary?: string;
  readonly modelId?: string;
  readonly maxOutputTokens?: number;
  readonly timeoutMs?: number;
};

export function createParentAssistantRuntimeCommand(
  kind: ParentAssistantRuntimeCommandKind,
  input: CreateParentAssistantCommandInput
): AgentCommandEnvelope {
  return AgentCommandEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    messageId: input.messageId,
    sentAt: input.sentAt,
    source: input.source,
    target: input.target,
    command: commandForKind(kind),
    payload: commandPayload(input),
  });
}

export function parseParentAssistantAnswerEvent(event: AgentEventEnvelope): ParentAssistantAdapterResult {
  if (event.event !== AgentEvent.ParentAssistantAnswerReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ParentAssistantAnswer];
  if (typeof raw !== 'string') {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = ParentAssistantAnswerSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function commandPayload(input: CreateParentAssistantCommandInput): AgentCommandEnvelope['payload'] {
  const payload: Record<string, string | number | boolean | null> = {
    [AgentProtocolDefaults.Field.ParentAssistantRequestId]: input.requestId,
    [AgentProtocolDefaults.Field.ParentAssistantQuestion]: input.question,
  };
  if (input.evidenceSummary !== undefined) {
    payload[AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary] = input.evidenceSummary;
  }
  if (input.modelId !== undefined) {
    payload[AgentProtocolDefaults.Field.LocalAiModelId] = input.modelId;
  }
  if (input.maxOutputTokens !== undefined) {
    payload[AgentProtocolDefaults.Field.LocalAiMaxOutputTokens] = input.maxOutputTokens;
  }
  if (input.timeoutMs !== undefined) {
    payload[AgentProtocolDefaults.Field.LocalAiTimeoutMs] = input.timeoutMs;
  }

  return payload;
}

function commandForKind(kind: ParentAssistantRuntimeCommandKind): AgentCommandEnvelope['command'] {
  if (kind === 'message-send') return AgentCommand.ParentAssistantMessageSend;
  if (kind === 'quick-action') return AgentCommand.ParentAssistantQuickActionStart;
  if (kind === 'action-preview') return AgentCommand.ParentAssistantActionPreview;
  return AgentCommand.ParentAssistantAnswerGenerate;
}

function adapterFailure(reason: ParentAssistantAdapterFailureReason): ParentAssistantAdapterResult {
  return {
    ok: false,
    reason,
  };
}
