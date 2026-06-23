import { type ScreenAnalysisParentSetting } from '@ocentra-parent/schema-domain/screen-evidence-settings';
import {
  ScreenSettingsGetRequestSchema,
  ScreenSettingsReplaceRequestSchema,
  ScreenSettingsSchemaVersion,
  ScreenSettingsUpdateRequestSchema,
  ScreenSettingsUpdateResponseSchema,
  type ScreenSettingsGetRequest,
  type ScreenSettingsReplaceRequest,
  type ScreenSettingsUpdateKind,
  type ScreenSettingsUpdateRequest,
  type ScreenSettingsUpdateResponse,
} from '@ocentra-parent/schema-domain/agent-screen-settings';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  AgentProtocolSchemaVersion,
  type AgentPeerRole,
  type AgentRoute,
} from '@ocentra-parent/schema-domain/event-primitives';

export type ScreenSettingsAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-request'
  | 'invalid-payload';

export type ScreenSettingsAdapterResult =
  | {
      readonly ok: true;
      readonly value: ScreenSettingsUpdateResponse;
    }
  | {
      readonly ok: false;
      readonly reason: ScreenSettingsAdapterFailureReason;
    };

export type ScreenSettingsCommandPeerInput = {
  readonly peerId: string;
  readonly role: AgentPeerRole;
};

export type ScreenSettingsCommandTargetInput = {
  readonly deviceId: string;
  readonly platform: string;
  readonly route: AgentRoute;
};

export type CreateScreenSettingsCommandInput = {
  readonly messageId: string;
  readonly sentAt: string;
  readonly source: ScreenSettingsCommandPeerInput;
  readonly target: ScreenSettingsCommandTargetInput;
  readonly request: ScreenSettingsUpdateRequest;
};

const ScreenSettingsRequestIdPrefix = 'screen-settings-request-';

export function createScreenSettingsCommand(input: CreateScreenSettingsCommandInput): AgentCommandEnvelope {
  const parsedRequest = ScreenSettingsUpdateRequestSchema.safeParse(input.request);
  if (!parsedRequest.success) {
    throw new Error('invalid screen settings request');
  }
  return AgentCommandEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    messageId: input.messageId,
    sentAt: input.sentAt,
    source: input.source,
    target: input.target,
    command: commandForKind(parsedRequest.data.kind),
    payload: createScreenSettingsCommandPayload(parsedRequest.data),
  });
}

export function createScreenSettingsCommandPayload(request: ScreenSettingsUpdateRequest): AgentProtocolLogFields {
  const parsedRequest = ScreenSettingsUpdateRequestSchema.safeParse(request);
  if (!parsedRequest.success) {
    throw new Error('invalid screen settings request');
  }
  return {
    [AgentProtocolDefaults.Field.ScreenSettingsRequest]: JSON.stringify(parsedRequest.data),
    [AgentProtocolDefaults.Field.ScreenSettingsUpdateKind]: parsedRequest.data.kind,
  };
}

export function createScreenSettingsGetRequest(requestId: string): ScreenSettingsGetRequest {
  return ScreenSettingsGetRequestSchema.parse({
    schemaVersion: ScreenSettingsSchemaVersion,
    requestId,
    kind: 'get',
  });
}

export function createScreenSettingsPortalRequestId(sequence: number): string {
  return `${ScreenSettingsRequestIdPrefix}${sequence}`;
}

export function createScreenSettingsReplaceRequest(input: {
  readonly requestId: string;
  readonly baseSettingVersion: number | null;
  readonly setting: ScreenAnalysisParentSetting;
}): ScreenSettingsReplaceRequest {
  return ScreenSettingsReplaceRequestSchema.parse({
    schemaVersion: ScreenSettingsSchemaVersion,
    requestId: input.requestId,
    kind: 'replace',
    baseSettingVersion: input.baseSettingVersion,
    setting: input.setting,
  });
}

export function parseScreenSettingsUpdateEvent(event: AgentEventEnvelope): ScreenSettingsAdapterResult {
  if (!screenSettingsEventNames().includes(event.event)) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ScreenSettingsResponse];
  if (typeof raw !== 'string') {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = ScreenSettingsUpdateResponseSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function commandForKind(kind: ScreenSettingsUpdateKind): AgentCommandEnvelope['command'] {
  if (kind === 'replace') return AgentCommand.ScreenSettingsReplace;
  return AgentCommand.ScreenSettingsGet;
}

function screenSettingsEventNames(): AgentEventName[] {
  return [
    AgentEvent.ScreenSettingsReported,
    AgentEvent.ScreenSettingsReplaceAccepted,
    AgentEvent.ScreenSettingsReplaceRejected,
  ];
}

function adapterFailure(reason: ScreenSettingsAdapterFailureReason): ScreenSettingsAdapterResult {
  return {
    ok: false,
    reason,
  };
}
