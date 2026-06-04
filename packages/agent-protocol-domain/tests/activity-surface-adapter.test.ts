import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import {
  ActivitySurfaceAdapterCommandBuilder,
  ActivitySurfaceAdapterEventParser,
  ActivitySurfaceAdapterOperationId,
  ActivitySurfaceAdapterOperationManifest,
  createActivityDeviceRequest,
  createActivityFamilyRequest,
  createActivityReadModelCommand,
  createActivityReportGenerateCommand,
  createActivityReportHistoryCommand,
  createActivityReportSaveCommand,
  parseActivityReadModelEvent,
  parseActivityReportDocumentEvent,
  parseActivityReportHistoryEvent,
  parseActivityServiceUiSpineEvents,
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
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'activity-query-store-summary',
      rawChildEvidenceIncluded: false,
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

const FamilyReport = {
  ...Report,
  reportId: 'activity-report-family-local-20260527T201000Z',
  scope: {
    scopeKind: 'family',
    familyId: 'family-local',
    deviceId: null,
  },
  sourceStates: [
    {
      deviceId: 'local-dev-agent',
      reachabilityState: 'reachable',
      state: 'ready',
      reason: null,
      lastUpdatedAt: '2026-05-27T20:09:00Z',
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'activity-query-store-summary',
      rawChildEvidenceIncluded: false,
    },
    {
      deviceId: 'child-device-offline',
      reachabilityState: 'offline',
      state: 'offline',
      reason: 'Child source is offline for this report.',
      lastUpdatedAt: null,
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'family-fanout-source-state',
      rawChildEvidenceIncluded: false,
    },
  ],
} as const;

const LatestFamilyReport = {
  ...FamilyReport,
  reportId: 'activity-report-family-local-latest-20260527T201500Z',
  generatedAt: '2026-05-27T20:15:01Z',
  sourceStates: [
    {
      deviceId: 'local-dev-agent',
      reachabilityState: 'reachable',
      state: 'ready',
      reason: null,
      lastUpdatedAt: '2026-05-27T20:14:00Z',
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'activity-query-store-summary',
      rawChildEvidenceIncluded: false,
    },
    {
      deviceId: 'child-device-latest-offline',
      reachabilityState: 'offline',
      state: 'offline',
      reason: 'Latest child source is offline for this report.',
      lastUpdatedAt: null,
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'family-fanout-source-state',
      rawChildEvidenceIncluded: false,
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
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'activity-query-store-summary',
      rawChildEvidenceIncluded: false,
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
    custodyLabel: 'child-device-local-summary',
    sourceLabel: 'family-fanout-source-state',
    rawChildEvidenceIncluded: false,
  },
  {
    deviceId: 'child-device-error',
    reachabilityState: 'error',
    state: 'unavailable',
    reason: 'Child source returned an error.',
    lastUpdatedAt: null,
    custodyLabel: 'child-device-local-summary',
    sourceLabel: 'family-fanout-source-state',
    rawChildEvidenceIncluded: false,
  },
] as const;

describe('activity surface adapter boundary', () => {
  specifyAdapterManifest();
  specifyRequestCreation();
  specifyCommandCreation();
  specifyStorageUnavailableHistoryParsing();
  specifyReportEventParsing();
  specifyReadModelEventParsing();
  specifyServiceUiSpineParsing();
});

function specifyAdapterManifest() {
  it('exports the C-consumable operation manifest without Vite-owned product data', () => {
    expect(ActivitySurfaceAdapterOperationManifest.map((operation) => operation.operationId)).toEqual([
      ActivitySurfaceAdapterOperationId.GetDailyReport,
      ActivitySurfaceAdapterOperationId.GetWeeklyReport,
      ActivitySurfaceAdapterOperationId.GetMonthlyReport,
      ActivitySurfaceAdapterOperationId.SaveActivityReport,
      ActivitySurfaceAdapterOperationId.ListHistoricalReports,
      ActivitySurfaceAdapterOperationId.GetScreenActivity,
      ActivitySurfaceAdapterOperationId.GetAppUseActivity,
      ActivitySurfaceAdapterOperationId.GetBrowserActivity,
      ActivitySurfaceAdapterOperationId.GetGamesActivity,
      ActivitySurfaceAdapterOperationId.GetNetworkActivity,
    ]);

    const history = ActivitySurfaceAdapterOperationManifest.find(
      (operation) => operation.operationId === ActivitySurfaceAdapterOperationId.ListHistoricalReports
    );
    const network = ActivitySurfaceAdapterOperationManifest.find(
      (operation) => operation.operationId === ActivitySurfaceAdapterOperationId.GetNetworkActivity
    );

    expect(history?.command).toBe('agent.activity.report.history.list');
    expect(history?.successEvent).toBe('agent.activity.report.history.reported');
    expect(history?.payloadField).toBe('activityReports');
    expect(history?.responseKind).toBe('report-history');
    expect(history?.commandBuilder).toBe(ActivitySurfaceAdapterCommandBuilder.ReportHistory);
    expect(history?.eventParser).toBe(ActivitySurfaceAdapterEventParser.ReportHistory);
    expect(network?.command).toBe('agent.activity.network.read-model.get');
    expect(network?.successEvent).toBe('agent.activity.network.read-model.reported');
    expect(network?.readModelKind).toBe('network');
    expect(network?.commandBuilder).toBe(ActivitySurfaceAdapterCommandBuilder.ReadModel);
    expect(network?.eventParser).toBe(ActivitySurfaceAdapterEventParser.ReadModel);
    expect(
      ActivitySurfaceAdapterOperationManifest.every(
        (operation) =>
          operation.productDataOwner === 'rust-service-read-model' &&
          operation.uiConsumer === 'c-owned-activity-ui' &&
          operation.viteDataOwner === false &&
          operation.failureState === 'unavailable' &&
          operation.failureReasons.includes('wrong-event') &&
          operation.failureReasons.includes('missing-json-field') &&
          operation.commandBuilder.length > 0 &&
          operation.eventParser.length > 0 &&
          operation.unavailableState === 'unavailable'
      )
    ).toBe(true);
  });
}

function specifyRequestCreation() {
  it('creates family and device requests through the Activity request schema', () => {
    const family = createActivityFamilyRequest({
      familyId: 'family-local',
      requestedAt: Request.requestedAt,
      rangeStart: Request.rangeStart,
      rangeEnd: Request.rangeEnd,
    });
    const device = createActivityDeviceRequest({
      deviceId: 'child-device-2',
      requestedAt: Request.requestedAt,
      rangeStart: Request.rangeStart,
      rangeEnd: Request.rangeEnd,
    });

    expect(family.scope.scopeKind).toBe('family');
    expect(family.scope.familyId).toBe('family-local');
    expect(family.scope.deviceId).toBe(null);
    expect(device.scope.scopeKind).toBe('device');
    expect(device.scope.familyId).toBe(null);
    expect(device.scope.deviceId).toBe('child-device-2');
  });

  it('rejects malformed Activity requests before command creation', () => {
    expect(() =>
      createActivityFamilyRequest({
        familyId: '',
        requestedAt: Request.requestedAt,
        rangeStart: Request.rangeStart,
        rangeEnd: Request.rangeEnd,
      })
    ).toThrow();
    expect(() =>
      createActivityDeviceRequest({
        deviceId: '',
        requestedAt: Request.requestedAt,
        rangeStart: Request.rangeStart,
        rangeEnd: Request.rangeEnd,
      })
    ).toThrow();
  });
}

function specifyCommandCreation() {
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
}

function specifyReportEventParsing() {
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
}

function specifyReadModelEventParsing() {
  specifyReadModelAdapterFailureParsing();
  specifyAppUseReadModelEventParsing();
  specifyScreenReadModelEventParsing();
}

function specifyReadModelAdapterFailureParsing() {
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
}

function specifyAppUseReadModelEventParsing() {
  it('parses service-backed app-use rows with app-game source counts', () => {
    const parsed = parseActivityReadModelEvent(
      'app-use',
      eventEnvelope(AgentEvent.ActivityAppUseReadModelReported, {
        [AgentProtocolDefaults.Field.ActivityReadModel]: JSON.stringify({
          schemaVersion: ActivitySurfaceSchemaVersion,
          request: Request,
          state: 'ready',
          generatedAt: '2026-05-27T20:10:01Z',
          summary: 'Activity data is available from the local query store.',
          rows: [
            {
              rowId: 'foreground-evidence-window-4242',
              appName: 'ocentra-fixture.exe',
              deviceId: 'local-dev-agent',
              state: 'ready',
              productKind: 'nativeApp',
              classificationState: 'knownApp',
              inventoryState: 'installed',
              runtimeState: 'running',
              foregroundState: 'foreground',
              capabilityStatus: 'available',
              lastObservedAt: '2026-05-27T20:09:00Z',
              totalMs: 60000,
              launchCount: 1,
              inventoryRowCount: 1,
              runningRowCount: 1,
              foregroundRowCount: 1,
              dailyRollupCount: 1,
              evidenceClaimRowCount: 1,
              identityRowCount: 1,
              approvalAuthorityRowCount: 1,
              approvalActionResultRowCount: 1,
              platformAuthorityMatrixCount: 1,
              platformAuthorityRowCount: 1,
              aiClassifierResultRowCount: 1,
              sourceStatusRows: [
                {
                  sourceKind: 'osInstalledRecord',
                  state: 'ready',
                  rowCount: 1,
                  lastObservedAt: '2026-05-27T20:08:00Z',
                  capabilityStatus: 'available',
                  evidence: [],
                },
                {
                  sourceKind: 'foregroundWindow',
                  state: 'ready',
                  rowCount: 1,
                  lastObservedAt: '2026-05-27T20:09:00Z',
                  capabilityStatus: 'available',
                  evidence: [],
                },
              ],
              evidence: [],
            },
          ],
        }),
      })
    );

    const row = parsed.ok
      ? (parsed.value.rows[0] as { readonly foregroundState?: string; readonly dailyRollupCount?: number } | undefined)
      : undefined;
    expect(parsed.ok).toBe(true);
    expect(row?.foregroundState).toBe('foreground');
    expect(row?.dailyRollupCount).toBe(1);
    expect(parsed.ok ? parsed.value.rows[0]?.aiClassifierResultRowCount : null).toBe(1);
    expect(parsed.ok ? parsed.value.rows[0]?.sourceStatusRows[0]?.sourceKind : null).toBe('osInstalledRecord');
    expect(parsed.ok ? parsed.value.rows[0]?.sourceStatusRows[1]?.sourceKind : null).toBe('foregroundWindow');
  });
}

function specifyScreenReadModelEventParsing() {
  it('parses screen read-model events with capture, AI, policy, and deletion chain fields', () => {
    const parsed = parseActivityReadModelEvent(
      'screen',
      eventEnvelope(AgentEvent.ActivityScreenReadModelReported, {
        [AgentProtocolDefaults.Field.ActivityReadModel]: JSON.stringify(screenReadModel()),
      })
    );

    expect(parsed.ok).toBe(true);
    expect(parsed.ok ? parsed.value.rows[0]?.captureReason : null).toBe('nativeAppForegroundStart');
    expect(parsed.ok ? parsed.value.rows[0]?.providerKind : null).toBe('localVision');
    expect(parsed.ok ? parsed.value.rows[0]?.imageDeletionState : null).toBe('deleted');
    expect(parsed.ok ? parsed.value.rows[0]?.policyEligible : null).toBe(true);
  });
}

function specifyServiceUiSpineParsing() {
  it('builds a C-consumable service UI spine from service-owned browser and family events', () => {
    const spine = parseActivityServiceUiSpineEvents([
      eventEnvelope(AgentEvent.ActivityBrowserReadModelReported, {
        [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
        [AgentProtocolDefaults.Field.ActivityReadModelKind]: 'browser',
        [AgentProtocolDefaults.Field.Returned]: 1,
        [AgentProtocolDefaults.Field.ActivityReadModel]: JSON.stringify(browserReadModel()),
      }),
      eventEnvelope(AgentEvent.ActivityReportGenerated, {
        [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
        [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify(FamilyReport),
      }),
    ]);

    expect(spine.dataOwner).toBe('rust-service-read-model');
    expect(spine.uiConsumer).toBe('c-owned-activity-ui');
    expect(spine.viteDataOwner).toBe(false);
    expect(spine.currentState).toBe('ready');
    expect(spine.browser?.ok).toBe(true);
    expect(spine.browser?.ok ? spine.browser.value.rows[0]?.domainLabel : null).toBe('example.test');
    expect(spine.familyAggregation?.ok).toBe(true);
    expect(spine.familyAggregation?.ok ? spine.familyAggregation.value.offlineDeviceIds : null).toEqual([
      'child-device-offline',
    ]);
  });

  it('reports unavailable spine state when no service events have arrived', () => {
    const spine = parseActivityServiceUiSpineEvents([]);

    expect(spine.currentState).toBe('unavailable');
    expect(spine.browser).toBe(null);
    expect(spine.familyAggregation).toBe(null);
  });

  it('uses the latest matching service events for the portal-facing UI spine', () => {
    const spine = parseActivityServiceUiSpineEvents([
      eventEnvelope(AgentEvent.ActivityBrowserReadModelReported, {
        [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
        [AgentProtocolDefaults.Field.ActivityReadModelKind]: 'browser',
        [AgentProtocolDefaults.Field.Returned]: 1,
        [AgentProtocolDefaults.Field.ActivityReadModel]: JSON.stringify(browserReadModel('earlier.example')),
      }),
      eventEnvelope(AgentEvent.ActivityReportGenerated, {
        [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
        [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify(FamilyReport),
      }),
      eventEnvelope(AgentEvent.ActivityBrowserReadModelReported, {
        [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
        [AgentProtocolDefaults.Field.ActivityReadModelKind]: 'browser',
        [AgentProtocolDefaults.Field.Returned]: 1,
        [AgentProtocolDefaults.Field.ActivityReadModel]: JSON.stringify(browserReadModel('latest.example')),
      }),
      eventEnvelope(AgentEvent.ActivityReportSaved, {
        [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
        [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify(LatestFamilyReport),
      }),
    ]);

    expect(spine.browser?.ok ? spine.browser.value.rows[0]?.domainLabel : null).toBe('latest.example');
    expect(spine.familyAggregation?.ok ? spine.familyAggregation.value.offlineDeviceIds : null).toEqual([
      'child-device-latest-offline',
    ]);
  });
}

function specifyStorageUnavailableHistoryParsing() {
  it('parses storage-unavailable report history without promoting it to ready', () => {
    const parsed = parseActivityReportHistoryEvent(
      eventEnvelope(AgentEvent.ActivityReportHistoryReported, {
        [AgentProtocolDefaults.Field.ActivityReports]: JSON.stringify(storageUnavailableHistory()),
      })
    );

    expect(parsed.ok).toBe(true);
    expect(parsed.ok ? parsed.state : null).toBe('unavailable');
    expect(parsed.ok ? parsed.value.storageState : null).toBe('storage-unavailable');
    expect(parsed.ok ? parsed.value.reports.length : null).toBe(0);
  });
}

function browserReadModel(domainLabel = 'example.test') {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: Request,
    state: 'ready',
    generatedAt: '2026-05-27T20:10:01Z',
    summary: 'Activity data is available from the local query store.',
    rows: [
      {
        rowId: 'browser-evidence-row-1',
        domainLabel,
        deviceId: 'local-dev-agent',
        state: 'ready',
        visitCount: 1,
        totalMs: 0,
        evidenceDigest: null,
      },
    ],
  } as const;
}

function screenReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: Request,
    state: 'ready',
    generatedAt: '2026-05-27T20:10:01Z',
    summary: 'Screen summary is available from the local capture journal.',
    rows: [
      {
        rowId: 'screen-row-1',
        label: 'Visible activity summary',
        deviceId: 'local-dev-agent',
        state: 'ready',
        totalMs: 60000,
        foregroundMs: 60000,
        backgroundMs: 0,
        captureReason: 'nativeAppForegroundStart',
        captureScope: 'activeWindow',
        capabilityStatus: 'ready',
        queueJobId: 'screen-queue-job-1',
        modelRuntimeRef: 'local-vision-runtime-1',
        providerKind: 'localVision',
        primaryCategory: 'productivity',
        confidence: 0.91,
        imageDeletionState: 'deleted',
        policyEligible: true,
        imageDigest: 'sha256:screen-image-digest',
        custodyState: 'child-device-journal',
        evidence: [],
      },
    ],
  } as const;
}

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
        custodyLabel: 'parent-device-local-history',
        sourceLabel: 'saved-report-history',
        rawChildEvidenceIncluded: false,
      },
    ],
  } as const;
}

function storageUnavailableHistory() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: Request,
    state: 'unavailable',
    storageState: 'storage-unavailable',
    storageReason: 'Activity report storage is unavailable.',
    reports: [],
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
