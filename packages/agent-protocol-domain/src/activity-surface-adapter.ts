import {
  ActivityAppUseReadModelSchema,
  ActivityBrowserReadModelSchema,
  ActivityGamesReadModelSchema,
  ActivityHistoricalReportListSchema,
  ActivityNetworkReadModelSchema,
  ActivityReadModelStateSchema,
  ActivityReportDocumentSchema,
  ActivityScreenReadModelSchema,
} from '@ocentra-parent/activity-domain/activity-surface';
import type {
  ActivityAppUseReadModel,
  ActivityBrowserReadModel,
  ActivityGamesReadModel,
  ActivityHistoricalReportList,
  ActivityNetworkReadModel,
  ActivityReportDocument,
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

export type ActivitySurfaceReportFrequency = 'daily' | 'weekly' | 'monthly';
export type ActivitySurfaceReadModelKind = 'screen' | 'app-use' | 'browser' | 'games' | 'network';
export type ActivitySurfaceAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type ActivitySurfaceAdapterResult<TValue> =
  | {
      readonly ok: true;
      readonly state: ReturnType<typeof ActivityReadModelStateSchema.parse>;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
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

export type CreateActivitySurfaceCommandInput = {
  readonly messageId: string;
  readonly sentAt: string;
  readonly source: ActivitySurfaceCommandPeerInput;
  readonly target: ActivitySurfaceCommandTargetInput;
  readonly request: ActivitySurfaceRequest;
  readonly report?: ActivityReportDocument;
};

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

  return parsePayloadJson(event, AgentProtocolDefaults.Field.ActivityReportDocument, ActivityReportDocumentSchema);
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

  return payload;
}

function parsePayloadJson<TValue>(
  event: AgentEventEnvelope,
  field: string,
  schema: ActivitySurfaceSchemaParser<TValue>
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
    state: ActivityReadModelStateSchema.parse((value as { readonly state?: unknown }).state ?? 'ready'),
    value,
  };
}

function adapterFailure(reason: ActivitySurfaceAdapterFailureReason): ActivitySurfaceAdapterResult<never> {
  return {
    ok: false,
    reason,
  };
}

function reportCommandForFrequency(frequency: ActivitySurfaceReportFrequency): AgentCommandEnvelope['command'] {
  if (frequency === 'weekly') return AgentCommand.ActivityReportWeeklyGenerate;
  if (frequency === 'monthly') return AgentCommand.ActivityReportMonthlyGenerate;
  return AgentCommand.ActivityReportDailyGenerate;
}

function readModelCommandForKind(kind: ActivitySurfaceReadModelKind): AgentCommandEnvelope['command'] {
  if (kind === 'screen') return AgentCommand.ActivityScreenReadModelGet;
  if (kind === 'app-use') return AgentCommand.ActivityAppUseReadModelGet;
  if (kind === 'browser') return AgentCommand.ActivityBrowserReadModelGet;
  if (kind === 'games') return AgentCommand.ActivityGamesReadModelGet;
  return AgentCommand.ActivityNetworkReadModelGet;
}

function readModelEventForKind(kind: ActivitySurfaceReadModelKind): AgentEventEnvelope['event'] {
  if (kind === 'screen') return AgentEvent.ActivityScreenReadModelReported;
  if (kind === 'app-use') return AgentEvent.ActivityAppUseReadModelReported;
  if (kind === 'browser') return AgentEvent.ActivityBrowserReadModelReported;
  if (kind === 'games') return AgentEvent.ActivityGamesReadModelReported;
  return AgentEvent.ActivityNetworkReadModelReported;
}

function readModelSchemaForKind(
  kind: ActivitySurfaceReadModelKind
): ActivitySurfaceSchemaParser<ActivitySurfaceReadModel> {
  if (kind === 'screen') return ActivityScreenReadModelSchema;
  if (kind === 'app-use') return ActivityAppUseReadModelSchema;
  if (kind === 'browser') return ActivityBrowserReadModelSchema;
  if (kind === 'games') return ActivityGamesReadModelSchema;
  return ActivityNetworkReadModelSchema;
}
