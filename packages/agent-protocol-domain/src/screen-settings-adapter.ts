import {
  ScreenAnalysisParentSettingSchema,
  type ScreenAnalysisParentSetting,
} from '@ocentra-parent/activity-domain/screen-evidence-settings';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentProtocolDefaults,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from './contracts';
import { AgentProtocolSchemaVersion, type AgentRoute } from './primitives';

export const ScreenSettingsUpdateKindSchema = withParser(Schema.Literal('get', 'replace'));
export const ScreenSettingsUpdateStatusSchema = withParser(Schema.Literal('accepted', 'rejected'));
export const ScreenSettingsRejectionReasonSchema = withParser(
  Schema.Literal(
    'storage-unavailable',
    'invalid-setting',
    'stale-revision',
    'raw-retention-forbidden',
    'disabled-setting-inconsistent',
    'policy-mode-inconsistent',
    'strict-mode-inconsistent',
    'trigger-mode-inconsistent',
    'ocr-mode-inconsistent'
  )
);

export const ScreenSettingsGetRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    requestId: Schema.String,
    kind: Schema.Literal('get'),
  })
);

export const ScreenSettingsReplaceRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    requestId: Schema.String,
    kind: Schema.Literal('replace'),
    baseSettingVersion: Schema.Union(Schema.Number, Schema.Null),
    setting: ScreenAnalysisParentSettingSchema,
  })
);

export const ScreenSettingsUpdateRequestSchema = withParser(
  Schema.Union(ScreenSettingsGetRequestSchema, ScreenSettingsReplaceRequestSchema)
);

export const ScreenSettingsUpdateResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    requestId: Schema.String,
    kind: ScreenSettingsUpdateKindSchema,
    status: ScreenSettingsUpdateStatusSchema,
    setting: Schema.Union(ScreenAnalysisParentSettingSchema, Schema.Null),
    auditEventId: Schema.Union(Schema.String, Schema.Null),
    rejectionReason: Schema.Union(ScreenSettingsRejectionReasonSchema, Schema.Null),
    message: Schema.Union(Schema.String, Schema.Null),
  })
);

export type ScreenSettingsUpdateKind = Infer<typeof ScreenSettingsUpdateKindSchema>;
export const ScreenSettingsUpdateKindValue = {
  Get: ScreenSettingsUpdateKindSchema.parse('get'),
  Replace: ScreenSettingsUpdateKindSchema.parse('replace'),
} as const;
export const ScreenSettingsUpdateStatus = {
  Accepted: ScreenSettingsUpdateStatusSchema.parse('accepted'),
  Rejected: ScreenSettingsUpdateStatusSchema.parse('rejected'),
} as const;
export type ScreenSettingsGetRequest = Infer<typeof ScreenSettingsGetRequestSchema>;
export type ScreenSettingsReplaceRequest = Infer<typeof ScreenSettingsReplaceRequestSchema>;
export type ScreenSettingsUpdateRequest = Infer<typeof ScreenSettingsUpdateRequestSchema>;
export type ScreenSettingsUpdateResponse = Infer<typeof ScreenSettingsUpdateResponseSchema>;

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
  readonly role: 'portal' | 'agent-service' | 'cloud-relay';
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
    schemaVersion: 1,
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
    schemaVersion: 1,
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
