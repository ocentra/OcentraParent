import {
  ParentAssistantActionConfirmResultSchema,
  ParentAssistantActionPreviewResultSchema,
  ParentAssistantAnswerSchema,
  ParentAssistantProviderStatusSchema,
  ParentAssistantRunCancelResultSchema,
  ParentAssistantThreadResponseSchema,
} from '@ocentra-parent/parent-domain/parent-assistant';
import type {
  ParentAssistantActionConfirmResult,
  ParentAssistantAnswer,
  ParentAssistantProviderStatus,
  ParentAssistantRunCancelResult,
  ParentAssistantThreadResponse,
} from '@ocentra-parent/parent-domain/parent-assistant';
import type { ActivityReportDocument } from '@ocentra-parent/activity-domain/activity-surface';
import type { Infer } from '@ocentra-parent/schema-domain/effect';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentProtocolDefaults,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
} from './contracts';
import { AgentProtocolSchemaVersion, type AgentRoute } from './primitives';

export const ParentAssistantAdapterPayloadField = {
  ActionPreview: AgentProtocolDefaults.Field.ParentAssistantActionPreview,
  ActionConfirmResult: AgentProtocolDefaults.Field.ParentAssistantActionConfirmResult,
  ProviderRoute: AgentProtocolDefaults.Field.ParentAssistantProviderRoute,
  ProviderStatus: 'parentAssistantProviderStatus',
  RunCancelResult: 'parentAssistantRunCancelResult',
  ThreadResponse: 'parentAssistantThreadResponse',
} as const;

export type ParentAssistantRuntimeCommandKind =
  | 'answer-generate'
  | 'thread-list'
  | 'thread-create'
  | 'thread-open'
  | 'thread-archive'
  | 'message-send'
  | 'run-cancel'
  | 'quick-action'
  | 'action-preview'
  | 'action-confirm'
  | 'provider-status';
export type ParentAssistantAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

type ParentAssistantAdapterFailure = {
  readonly ok: false;
  readonly reason: ParentAssistantAdapterFailureReason;
};

export type ParentAssistantAdapterResult =
  | {
      readonly ok: true;
      readonly value: ParentAssistantAnswer;
    }
  | ParentAssistantAdapterFailure;

export type ParentAssistantThreadAdapterResult =
  | {
      readonly ok: true;
      readonly value: ParentAssistantThreadResponse;
    }
  | ParentAssistantAdapterFailure;

export type ParentAssistantProviderStatusAdapterResult =
  | {
      readonly ok: true;
      readonly value: ParentAssistantProviderStatus;
    }
  | ParentAssistantAdapterFailure;

export type ParentAssistantRunCancelAdapterResult =
  | {
      readonly ok: true;
      readonly value: ParentAssistantRunCancelResult;
    }
  | ParentAssistantAdapterFailure;

export type ParentAssistantActionConfirmAdapterResult =
  | {
      readonly ok: true;
      readonly value: ParentAssistantActionConfirmResult;
    }
  | ParentAssistantAdapterFailure;

export type ParentAssistantActionPreviewAdapterResult =
  | {
      readonly ok: true;
      readonly value: Infer<typeof ParentAssistantActionPreviewResultSchema>;
    }
  | ParentAssistantAdapterFailure;

export type ParentAssistantCommandPeerInput = {
  readonly peerId: string;
  readonly role: 'portal' | 'agent-service' | 'cloud-relay';
};

export type ParentAssistantCommandTargetInput = {
  readonly deviceId: string;
  readonly platform: string;
  readonly route: AgentRoute;
};

type ParentAssistantApiAuthorizationContextInput = {
  readonly authorizationState: 'authorized';
  readonly parentAuthorizationRequired: true;
  readonly evidenceCitationRequired: true;
  readonly custodyLabel: 'parent-authorized-api-ai';
  readonly retentionState: 'parent-authorized-no-default-retention';
  readonly deletionState: 'delete-provider-cache-on-parent-request';
};

export type CreateParentAssistantCommandInput = {
  readonly messageId: string;
  readonly sentAt: string;
  readonly source: ParentAssistantCommandPeerInput;
  readonly target: ParentAssistantCommandTargetInput;
  readonly requestId: string;
  readonly question: string;
  readonly threadId?: string;
  readonly runId?: string;
  readonly actionIntentId?: string;
  readonly previewId?: string;
  readonly actionAuditReason?: string;
  readonly rawAssistantProse?: string;
  readonly evidenceSummary?: string;
  readonly activityReport?: ActivityReportDocument;
  readonly apiAuthorizationContext?: ParentAssistantApiAuthorizationContextInput;
  readonly modelId?: string;
  readonly maxOutputTokens?: number;
  readonly timeoutMs?: number;
};

type ParentAssistantCommandPayload = Record<string, string | number | boolean | null>;
type ParentAssistantCommandPayloadValue = ParentAssistantCommandPayload[string];

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
    payload: commandPayload(kind, input),
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

export function parseParentAssistantThreadEvent(event: AgentEventEnvelope): ParentAssistantThreadAdapterResult {
  if (event.event !== AgentEvent.ParentAssistantThreadUpdated) {
    return adapterFailure('wrong-event');
  }

  const parsed = parseJsonPayload(event, ParentAssistantAdapterPayloadField.ThreadResponse);
  if (!parsed.ok) return parsed;
  const response = ParentAssistantThreadResponseSchema.safeParse(parsed.value);
  if (!response.success || response.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: response.data,
  };
}

export function parseParentAssistantProviderStatusEvent(
  event: AgentEventEnvelope
): ParentAssistantProviderStatusAdapterResult {
  if (event.event !== AgentEvent.ParentAssistantProviderDegraded) {
    return adapterFailure('wrong-event');
  }

  const parsed = parseJsonPayload(event, ParentAssistantAdapterPayloadField.ProviderStatus);
  if (!parsed.ok) return parsed;
  const status = ParentAssistantProviderStatusSchema.safeParse(parsed.value);
  if (!status.success || status.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: status.data,
  };
}

export function parseParentAssistantRunCancelEvent(event: AgentEventEnvelope): ParentAssistantRunCancelAdapterResult {
  if (event.event !== AgentEvent.ParentAssistantErrorReported) {
    return adapterFailure('wrong-event');
  }

  const parsed = parseJsonPayload(event, ParentAssistantAdapterPayloadField.RunCancelResult);
  if (!parsed.ok) return parsed;
  const result = ParentAssistantRunCancelResultSchema.safeParse(parsed.value);
  if (!result.success || result.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: result.data,
  };
}

export function parseParentAssistantActionConfirmEvent(
  event: AgentEventEnvelope
): ParentAssistantActionConfirmAdapterResult {
  if (event.event !== AgentEvent.ParentAssistantActionConfirmed) {
    return adapterFailure('wrong-event');
  }

  const parsed = parseJsonPayload(event, ParentAssistantAdapterPayloadField.ActionConfirmResult);
  if (!parsed.ok) return parsed;
  const result = ParentAssistantActionConfirmResultSchema.safeParse(parsed.value);
  if (!result.success || result.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: result.data,
  };
}

export function parseParentAssistantActionPreviewEvent(
  event: AgentEventEnvelope
): ParentAssistantActionPreviewAdapterResult {
  if (event.event !== AgentEvent.ParentAssistantActionPreviewed) {
    return adapterFailure('wrong-event');
  }

  const parsed = parseJsonPayload(event, ParentAssistantAdapterPayloadField.ActionPreview);
  if (!parsed.ok) return parsed;
  const result = ParentAssistantActionPreviewResultSchema.safeParse(parsed.value);
  if (!result.success || result.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: result.data,
  };
}

function commandPayload(
  kind: ParentAssistantRuntimeCommandKind,
  input: CreateParentAssistantCommandInput
): AgentCommandEnvelope['payload'] {
  const payload: ParentAssistantCommandPayload = {
    [AgentProtocolDefaults.Field.ParentAssistantRequestId]: input.requestId,
  };
  assignDefinedPayload(
    payload,
    AgentProtocolDefaults.Field.ParentAssistantQuestion,
    kind === 'action-confirm' ? undefined : input.question
  );
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary, input.evidenceSummary);
  assignDefinedPayload(
    payload,
    AgentProtocolDefaults.Field.ActivityReportDocument,
    input.activityReport === undefined ? undefined : JSON.stringify(input.activityReport)
  );
  assignApiAuthorizationPayload(payload, input.apiAuthorizationContext);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.ParentAssistantThreadId, input.threadId);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.ParentAssistantRunId, input.runId);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.ParentAssistantActionIntentId, input.actionIntentId);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.ParentAssistantActionPreviewId, input.previewId);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.ParentAssistantActionAuditReason, input.actionAuditReason);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.ParentAssistantActionRawProse, input.rawAssistantProse);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.LocalAiModelId, input.modelId);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.LocalAiMaxOutputTokens, input.maxOutputTokens);
  assignDefinedPayload(payload, AgentProtocolDefaults.Field.LocalAiTimeoutMs, input.timeoutMs);

  return payload;
}

function assignDefinedPayload(
  payload: ParentAssistantCommandPayload,
  field: string,
  value: ParentAssistantCommandPayloadValue | undefined
): void {
  if (value !== undefined) {
    payload[field] = value;
  }
}

function assignApiAuthorizationPayload(
  payload: ParentAssistantCommandPayload,
  context: ParentAssistantApiAuthorizationContextInput | undefined
): void {
  if (context === undefined) {
    return;
  }

  payload[AgentProtocolDefaults.Field.ParentAssistantApiAuthorizationState] = context.authorizationState;
  payload[AgentProtocolDefaults.Field.ParentAssistantApiCustodyLabel] = context.custodyLabel;
  payload[AgentProtocolDefaults.Field.ParentAssistantApiRetentionState] = context.retentionState;
  payload[AgentProtocolDefaults.Field.ParentAssistantApiDeletionState] = context.deletionState;
}

function commandForKind(kind: ParentAssistantRuntimeCommandKind): AgentCommandEnvelope['command'] {
  if (kind === 'thread-list') return AgentCommand.ParentAssistantThreadList;
  if (kind === 'thread-create') return AgentCommand.ParentAssistantThreadCreate;
  if (kind === 'thread-open') return AgentCommand.ParentAssistantThreadOpen;
  if (kind === 'thread-archive') return AgentCommand.ParentAssistantThreadArchive;
  if (kind === 'message-send') return AgentCommand.ParentAssistantMessageSend;
  if (kind === 'run-cancel') return AgentCommand.ParentAssistantRunCancel;
  if (kind === 'quick-action') return AgentCommand.ParentAssistantQuickActionStart;
  if (kind === 'action-preview') return AgentCommand.ParentAssistantActionPreview;
  if (kind === 'action-confirm') return AgentCommand.ParentAssistantActionConfirm;
  if (kind === 'provider-status') return AgentCommand.ParentAssistantProviderStatusGet;
  return AgentCommand.ParentAssistantAnswerGenerate;
}

function parseJsonPayload(
  event: AgentEventEnvelope,
  field: (typeof ParentAssistantAdapterPayloadField)[keyof typeof ParentAssistantAdapterPayloadField]
):
  | {
      readonly ok: true;
      readonly value: unknown;
    }
  | ParentAssistantAdapterFailure {
  const raw = event.payload[field];
  if (typeof raw !== 'string') {
    return adapterFailure('missing-json-field');
  }

  try {
    return {
      ok: true,
      value: JSON.parse(raw) as unknown,
    };
  } catch {
    return adapterFailure('invalid-json');
  }
}

function adapterFailure(reason: ParentAssistantAdapterFailureReason): ParentAssistantAdapterFailure {
  return {
    ok: false,
    reason,
  };
}
