import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import {
  createActivityReadModelCommand,
  createActivityReportGenerateCommand,
  createActivityReportHistoryCommand,
  createActivityReportSaveCommand,
  parseActivityReadModelEvent,
  parseActivityReportDocumentEvent,
  parseActivityReportHistoryEvent,
} from '../src/activity-surface-adapter';
import { AgentEvent, AgentProtocolDefaults } from '../src/contracts';

const Source = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const Target = {
  deviceId: 'local-dev-agent',
  platform: 'windows',
  route: 'localhost',
} as const;

const Request = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: {
    scopeKind: 'device',
    familyId: null,
    deviceId: 'local-dev-agent',
  },
  requestedAt: '2026-05-27T20:10:00Z',
  rangeStart: '2026-05-27T00:00:00Z',
  rangeEnd: '2026-05-27T20:10:00Z',
} as const;

const Report = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  reportId: 'activity-report-daily-local-20260527T201000Z',
  frequency: 'daily',
  scope: Request.scope,
  requestedAt: Request.requestedAt,
  rangeStart: Request.rangeStart,
  rangeEnd: Request.rangeEnd,
  generatedAt: '2026-05-27T20:10:01Z',
  savedMetadata: null,
  sourceStates: [
    {
      deviceId: 'local-dev-agent',
      reachabilityState: 'reachable',
      state: 'ready',
      reason: null,
      lastUpdatedAt: '2026-05-27T20:09:00Z',
    },
  ],
  sections: [
    {
      sectionKind: 'summary',
      title: 'Summary',
      state: 'ready',
      summary: 'Activity data is available from the local query store.',
      itemCount: 1,
      evidence: [],
    },
  ],
} as const;

const UnavailableReport = {
  ...Report,
  reportId: 'activity-report-daily-local-unavailable-20260527T201000Z',
  sourceStates: [
    {
      deviceId: 'local-dev-agent',
      reachabilityState: 'unreachable',
      state: 'unavailable',
      reason: 'Activity query store is unavailable.',
      lastUpdatedAt: null,
    },
  ],
  sections: [
    {
      sectionKind: 'summary',
      title: 'Summary',
      state: 'unavailable',
      summary: 'Activity query store is unavailable.',
      itemCount: 0,
      evidence: [],
    },
    {
      sectionKind: 'network',
      title: 'Network',
      state: 'unavailable',
      summary: 'Activity query store is unavailable.',
      itemCount: 0,
      evidence: [],
    },
  ],
} as const;

const FamilySources = [
  {
    deviceId: 'child-device-offline',
    reachabilityState: 'offline',
    state: 'offline',
    reason: 'Child source is offline for this report.',
    lastUpdatedAt: null,
  },
  {
    deviceId: 'child-device-error',
    reachabilityState: 'error',
    state: 'unavailable',
    reason: 'Child source returned an error.',
    lastUpdatedAt: null,
  },
] as const;

describe('activity surface adapter boundary', () => {
  it('creates report and read-model commands with family or device scope payloads', () => {
    const reportCommand = createActivityReportGenerateCommand('daily', commandInput());
    const readModelCommand = createActivityReadModelCommand('network', commandInput());

    expect(reportCommand.command).toBe('agent.activity.report.daily.generate');
    expect(readModelCommand.command).toBe('agent.activity.network.read-model.get');
    expect(reportCommand.payload[AgentProtocolDefaults.Field.ScopeKind]).toBe('device');
    expect(reportCommand.payload[AgentProtocolDefaults.Field.DeviceId]).toBe('local-dev-agent');
  });

  it('creates save command payloads with the typed report document JSON', () => {
    const command = createActivityReportSaveCommand({ ...commandInput(), report: Report });

    expect(command.command).toBe('agent.activity.report.save');
    expect(typeof command.payload[AgentProtocolDefaults.Field.ActivityReportDocument]).toBe('string');
  });

  it('creates family report commands with backend-owned source registry JSON', () => {
    const command = createActivityReportGenerateCommand('weekly', {
      ...commandInput(),
      familySources: FamilySources,
    });
    const sources = parsedFamilySources(command.payload[AgentProtocolDefaults.Field.ActivityFamilySources]);

    expect(command.command).toBe('agent.activity.report.weekly.generate');
    expect(sources[0]?.reachabilityState).toBe('offline');
    expect(sources[1]?.reachabilityState).toBe('error');
  });
});

describe('activity surface adapter event parsing', () => {
  it('creates and parses historical report list messages for the Activity UI handoff', () => {
    const command = createActivityReportHistoryCommand(commandInput());
    const parsed = parseActivityReportHistoryEvent(
      eventEnvelope(AgentEvent.ActivityReportHistoryReported, {
        [AgentProtocolDefaults.Field.ActivityReports]: JSON.stringify(historicalReportList()),
      })
    );

    expect(command.command).toBe('agent.activity.report.history.list');
    expect(parsed.ok).toBe(true);
    expect(parsed.ok ? parsed.value.storageState : null).toBe('saved');
    expect(parsed.ok ? parsed.value.reports[0]?.savedState : null).toBe('saved');
  });

  it('parses report document events and rejects wrong event names', () => {
    const parsed = parseActivityReportDocumentEvent(
      eventEnvelope(AgentEvent.ActivityReportGenerated, {
        [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify(Report),
      })
    );
    const rejected = parseActivityReportDocumentEvent(eventEnvelope(AgentEvent.HealthReported, {}));

    expect(parsed.ok).toBe(true);
    expect(parsed.ok ? parsed.value.reportId : null).toBe(Report.reportId);
    expect(rejected.ok).toBe(false);
  });

  it('parses report document state from service payload without overclaiming ready', () => {
    const parsed = parseActivityReportDocumentEvent(
      eventEnvelope(AgentEvent.ActivityReportGenerated, {
        [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'unavailable',
        [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify(UnavailableReport),
      })
    );
    const derived = parseActivityReportDocumentEvent(
      eventEnvelope(AgentEvent.ActivityReportGenerated, {
        [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify(UnavailableReport),
      })
    );

    expect(parsed.ok ? parsed.state : null).toBe('unavailable');
    expect(derived.ok ? derived.state : null).toBe('unavailable');
  });

  it('parses typed read-model events and reports missing JSON as unavailable adapter failure', () => {
    const parsed = parseActivityReadModelEvent(
      'app-use',
      eventEnvelope(AgentEvent.ActivityAppUseReadModelReported, {
        [AgentProtocolDefaults.Field.ActivityReadModel]: JSON.stringify({
          schemaVersion: ActivitySurfaceSchemaVersion,
          request: Request,
          state: 'empty',
          generatedAt: '2026-05-27T20:10:01Z',
          summary: 'No local activity rows are available for this request.',
          rows: [],
        }),
      })
    );
    const missing = parseActivityReadModelEvent(
      'app-use',
      eventEnvelope(AgentEvent.ActivityAppUseReadModelReported, {})
    );

    expect(parsed.ok).toBe(true);
    expect(parsed.ok ? parsed.state : null).toBe('empty');
    expect(missing.ok).toBe(false);
    expect(missing.ok ? null : missing.state).toBe('unavailable');
    expect(missing.ok ? null : missing.reason).toBe('missing-json-field');
  });
});

function commandInput() {
  return {
    messageId: 'cmd-activity-1',
    sentAt: '2026-05-27T20:10:00Z',
    source: Source,
    target: Target,
    request: Request,
  } as const;
}

function historicalReportList() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: Request,
    state: 'ready',
    storageState: 'saved',
    storageReason: null,
    reports: [
      {
        schemaVersion: ActivitySurfaceSchemaVersion,
        reportId: Report.reportId,
        fileName: 'activity-report-daily-local-20260527T201000Z.json',
        reportDate: Report.generatedAt,
        rangeStart: Report.rangeStart,
        rangeEnd: Report.rangeEnd,
        summary: 'Saved daily report',
        savedState: 'saved',
        savedAt: '2026-05-27T20:10:02Z',
        sourceStateSummary: {
          totalSources: 1,
          readySources: 1,
          offlineSources: 0,
          staleSources: 0,
          unavailableSources: 0,
          unreachableSources: 0,
          errorSources: 0,
        },
        parsedReport: Report,
      },
    ],
  } as const;
}

function parsedFamilySources(value: unknown) {
  return JSON.parse(String(value)) as Array<{
    readonly reachabilityState: string;
  }>;
}

function eventEnvelope(event: (typeof AgentEvent)[keyof typeof AgentEvent], payload: Record<string, unknown>) {
  return {
    schemaVersion: 1,
    eventId: 'activity-event-1',
    correlationId: 'cmd-activity-1',
    sentAt: '2026-05-27T20:10:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: Source,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  } as const;
}
