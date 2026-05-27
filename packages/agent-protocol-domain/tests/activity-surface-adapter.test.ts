import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import {
  createActivityReadModelCommand,
  createActivityReportGenerateCommand,
  createActivityReportSaveCommand,
  parseActivityReadModelEvent,
  parseActivityReportDocumentEvent,
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
