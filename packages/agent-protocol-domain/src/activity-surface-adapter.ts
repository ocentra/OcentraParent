import {
  ActivityAppUseReadModelSchema,
  ActivityBrowserReadModelSchema,
  ActivityGamesReadModelSchema,
  ActivityHistoricalReportListSchema,
  ActivityNetworkReadModelSchema,
  ActivityReadModelStateSchema,
  ActivityReportDocumentSchema,
  ActivityScreenReadModelSchema,
  ActivitySurfaceRequestSchema,
  ActivitySurfaceSchemaVersion,
} from '@ocentra-parent/activity-domain/activity-surface';
import type {
  ActivityAppUseReadModel,
  ActivityBrowserReadModel,
  ActivityGamesReadModel,
  ActivityHistoricalReportList,
  ActivityNetworkReadModel,
  ActivityReportDocument,
  ActivityReportSourceState,
  ActivityScreenReadModel,
  ActivitySurfaceRequest,
} from '@ocentra-parent/activity-domain/activity-surface';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentProtocolDefaults,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
} from './contracts';
import { AgentProtocolSchemaVersion, type AgentRoute } from './primitives';

export * from './activity-surface-adapter-manifest';

export type ActivitySurfaceReportFrequency = 'daily' | 'weekly' | 'monthly';
export const ActivitySurfaceReadModelKindName = {
  Screen: 'screen',
  AppUse: 'app-use',
  Browser: 'browser',
  Games: 'games',
  Network: 'network',
} as const;

export type ActivitySurfaceReadModelKind =
  (typeof ActivitySurfaceReadModelKindName)[keyof typeof ActivitySurfaceReadModelKindName];
export type ActivitySurfaceAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';
type ActivitySurfaceAdapterState = ReturnType<typeof ActivityReadModelStateSchema.parse>;

export type ActivitySurfaceAdapterResult<TValue> =
  | {
      readonly ok: true;
      readonly state: ActivitySurfaceAdapterState;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
      readonly state: ActivitySurfaceAdapterState;
      readonly reason: ActivitySurfaceAdapterFailureReason;
    };

type ActivitySurfaceSchemaParser<TValue> = {
  readonly safeParse: (input: unknown) => { readonly success: boolean; readonly data?: TValue };
};

type ActivitySurfaceReadModel =
  | ActivityScreenReadModel
  | ActivityAppUseReadModel
  | ActivityBrowserReadModel
  | ActivityGamesReadModel
  | ActivityNetworkReadModel;

export type ActivitySurfaceCommandPeerInput = {
  readonly peerId: string;
  readonly role: 'portal' | 'agent-service' | 'cloud-relay';
};

export type ActivitySurfaceCommandTargetInput = {
  readonly deviceId: string;
  readonly platform: string;
  readonly route: AgentRoute;
};

export type CreateActivityFamilyRequestInput = {
  readonly familyId: unknown;
  readonly requestedAt: unknown;
  readonly rangeStart: unknown;
  readonly rangeEnd: unknown;
};

export type CreateActivityDeviceRequestInput = {
  readonly deviceId: unknown;
  readonly requestedAt: unknown;
  readonly rangeStart: unknown;
  readonly rangeEnd: unknown;
};

export type CreateActivitySurfaceCommandInput = {
  readonly messageId: string;
  readonly sentAt: string;
  readonly source: ActivitySurfaceCommandPeerInput;
  readonly target: ActivitySurfaceCommandTargetInput;
  readonly request: ActivitySurfaceRequest;
  readonly report?: ActivityReportDocument;
  readonly familySources?: readonly ActivityReportSourceState[];
};

export function createActivityFamilyRequest(input: CreateActivityFamilyRequestInput): ActivitySurfaceRequest {
  return ActivitySurfaceRequestSchema.parse({
    schemaVersion: ActivitySurfaceSchemaVersion,
    scope: {
      scopeKind: 'family',
      familyId: input.familyId,
      deviceId: null,
    },
    requestedAt: input.requestedAt,
    rangeStart: input.rangeStart,
    rangeEnd: input.rangeEnd,
  });
}

export function createActivityDeviceRequest(input: CreateActivityDeviceRequestInput): ActivitySurfaceRequest {
  return ActivitySurfaceRequestSchema.parse({
    schemaVersion: ActivitySurfaceSchemaVersion,
    scope: {
      scopeKind: 'device',
      familyId: null,
      deviceId: input.deviceId,
    },
    requestedAt: input.requestedAt,
    rangeStart: input.rangeStart,
    rangeEnd: input.rangeEnd,
  });
}

export function createActivityReportGenerateCommand(
  frequency: ActivitySurfaceReportFrequency,
  input: CreateActivitySurfaceCommandInput
): AgentCommandEnvelope {
  return createActivityCommand(reportCommandForFrequency(frequency), input);
}

export function createActivityReportSaveCommand(input: CreateActivitySurfaceCommandInput): AgentCommandEnvelope {
  return createActivityCommand(AgentCommand.ActivityReportSave, input);
}

export function createActivityReportHistoryCommand(input: CreateActivitySurfaceCommandInput): AgentCommandEnvelope {
  return createActivityCommand(AgentCommand.ActivityReportHistoryList, input);
}

export function createActivityReadModelCommand(
  kind: ActivitySurfaceReadModelKind,
  input: CreateActivitySurfaceCommandInput
): AgentCommandEnvelope {
  return createActivityCommand(readModelCommandForKind(kind), input);
}

export function parseActivityReportDocumentEvent(
  event: AgentEventEnvelope
): ActivitySurfaceAdapterResult<ActivityReportDocument> {
  if (event.event !== AgentEvent.ActivityReportGenerated && event.event !== AgentEvent.ActivityReportSaved) {
    return adapterFailure('wrong-event');
  }

  return parsePayloadJson(
    event,
    AgentProtocolDefaults.Field.ActivityReportDocument,
    ActivityReportDocumentSchema,
    reportDocumentState
  );
}

export function parseActivityReportHistoryEvent(
  event: AgentEventEnvelope
): ActivitySurfaceAdapterResult<ActivityHistoricalReportList> {
  if (event.event !== AgentEvent.ActivityReportHistoryReported) {
    return adapterFailure('wrong-event');
  }

  return parsePayloadJson(event, AgentProtocolDefaults.Field.ActivityReports, ActivityHistoricalReportListSchema);
}

export function parseActivityReadModelEvent(
  kind: ActivitySurfaceReadModelKind,
  event: AgentEventEnvelope
): ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> {
  if (event.event !== readModelEventForKind(kind)) {
    return adapterFailure('wrong-event');
  }

  return parsePayloadJson(event, AgentProtocolDefaults.Field.ActivityReadModel, readModelSchemaForKind(kind));
}

function createActivityCommand(
  command: AgentCommandEnvelope['command'],
  input: CreateActivitySurfaceCommandInput
): AgentCommandEnvelope {
  return AgentCommandEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    messageId: input.messageId,
    sentAt: input.sentAt,
    source: input.source,
    target: input.target,
    command,
    payload: commandPayload(input),
  });
}

function commandPayload(input: CreateActivitySurfaceCommandInput): AgentCommandEnvelope['payload'] {
  const payload: Record<string, string | number | boolean | null> = {
    [AgentProtocolDefaults.Field.ScopeKind]: input.request.scope.scopeKind,
    [AgentProtocolDefaults.Field.RangeStart]: input.request.rangeStart,
    [AgentProtocolDefaults.Field.RangeEnd]: input.request.rangeEnd,
    [AgentProtocolDefaults.Field.RequestedAt]: input.request.requestedAt,
  };
  if (input.request.scope.familyId !== null) {
    payload[AgentProtocolDefaults.Field.FamilyId] = input.request.scope.familyId;
  }
  if (input.request.scope.deviceId !== null) {
    payload[AgentProtocolDefaults.Field.DeviceId] = input.request.scope.deviceId;
  }
  if (input.report !== undefined) {
    payload[AgentProtocolDefaults.Field.ActivityReportDocument] = JSON.stringify(input.report);
  }
  if (input.familySources !== undefined) {
    payload[AgentProtocolDefaults.Field.ActivityFamilySources] = JSON.stringify(input.familySources);
  }

  return payload;
}

function parsePayloadJson<TValue>(
  event: AgentEventEnvelope,
  field: string,
  schema: ActivitySurfaceSchemaParser<TValue>,
  stateFromValue: (value: TValue, event: AgentEventEnvelope) => ActivitySurfaceAdapterState = payloadState
): ActivitySurfaceAdapterResult<TValue> {
  const raw = event.payload[field];
  if (typeof raw !== 'string') {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = schema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  const value = parsed.data;
  return {
    ok: true,
    state: stateFromValue(value, event),
    value,
  };
}

function payloadState(value: unknown): ActivitySurfaceAdapterState {
  return ActivityReadModelStateSchema.parse((value as { readonly state?: unknown }).state ?? 'ready');
}

function reportDocumentState(report: ActivityReportDocument, event: AgentEventEnvelope): ActivitySurfaceAdapterState {
  const serviceState = event.payload[AgentProtocolDefaults.Field.ActivitySurfaceState];
  if (typeof serviceState === 'string') {
    const parsed = ActivityReadModelStateSchema.safeParse(serviceState);
    if (parsed.success && parsed.data !== undefined) {
      return parsed.data;
    }
  }

  if (report.sections.some((section) => section.state === 'ready')) {
    return ActivityReadModelStateSchema.parse('ready');
  }

  return ActivityReadModelStateSchema.parse(report.sections[0]?.state ?? 'empty');
}

function adapterFailure(reason: ActivitySurfaceAdapterFailureReason): ActivitySurfaceAdapterResult<never> {
  return {
    ok: false,
    state: ActivityReadModelStateSchema.parse('unavailable'),
    reason,
  };
}

function reportCommandForFrequency(frequency: ActivitySurfaceReportFrequency): AgentCommandEnvelope['command'] {
  if (frequency === 'weekly') return AgentCommand.ActivityReportWeeklyGenerate;
  if (frequency === 'monthly') return AgentCommand.ActivityReportMonthlyGenerate;
  return AgentCommand.ActivityReportDailyGenerate;
}

function readModelCommandForKind(kind: ActivitySurfaceReadModelKind): AgentCommandEnvelope['command'] {
  if (kind === ActivitySurfaceReadModelKindName.Screen) return AgentCommand.ActivityScreenReadModelGet;
  if (kind === ActivitySurfaceReadModelKindName.AppUse) return AgentCommand.ActivityAppUseReadModelGet;
  if (kind === ActivitySurfaceReadModelKindName.Browser) return AgentCommand.ActivityBrowserReadModelGet;
  if (kind === ActivitySurfaceReadModelKindName.Games) return AgentCommand.ActivityGamesReadModelGet;
  return AgentCommand.ActivityNetworkReadModelGet;
}

function readModelEventForKind(kind: ActivitySurfaceReadModelKind): AgentEventEnvelope['event'] {
  if (kind === ActivitySurfaceReadModelKindName.Screen) return AgentEvent.ActivityScreenReadModelReported;
  if (kind === ActivitySurfaceReadModelKindName.AppUse) return AgentEvent.ActivityAppUseReadModelReported;
  if (kind === ActivitySurfaceReadModelKindName.Browser) return AgentEvent.ActivityBrowserReadModelReported;
  if (kind === ActivitySurfaceReadModelKindName.Games) return AgentEvent.ActivityGamesReadModelReported;
  return AgentEvent.ActivityNetworkReadModelReported;
}

function readModelSchemaForKind(
  kind: ActivitySurfaceReadModelKind
): ActivitySurfaceSchemaParser<ActivitySurfaceReadModel> {
  if (kind === ActivitySurfaceReadModelKindName.Screen) return ActivityScreenReadModelSchema;
  if (kind === ActivitySurfaceReadModelKindName.AppUse) return ActivityAppUseReadModelSchema;
  if (kind === ActivitySurfaceReadModelKindName.Browser) return ActivityBrowserReadModelSchema;
  if (kind === ActivitySurfaceReadModelKindName.Games) return ActivityGamesReadModelSchema;
  return ActivityNetworkReadModelSchema;
}
