import type {
  ParentActivityNetworkFlowReadModelSnapshot,
  ParentActivityTrackingReadModelResultSnapshot,
  ParentAgentActivityHistoricalReportList,
  ParentAgentActivityReportDocument,
  ParentRouteEventSnapshot,
} from '../../generated/parent-ui-bridge';
import {
  ParentAgentActivityReadModelState,
  ParentAgentActivityReportCustodyLabel,
  ParentAgentActivityReportFrequency,
  ParentAgentActivityReportSectionKind,
  ParentAgentActivityReportSourceLabel,
  ParentAgentActivitySavedReportState,
  ParentAgentActivitySurfaceScopeKind,
  ParentAgentEvent,
  ParentAgentProtocolField,
} from '../../generated/parent-ui-bridge';

const NoClaimBoundary = {
  exactUrlAvailable: false,
  decryptedHttpsPayloadAvailable: false,
  messageContentAvailable: false,
  searchQueryAvailable: false,
  adapterActionExecuted: false,
} as const;

export const FlowObserved = {
  schemaVersion: 1,
  flowEventRef: 'event.network.flow.observed.1',
  observedAt: '2026-06-08T22:45:00Z',
  deviceRef: 'device.child.windows-1',
  flowEvidenceRef: 'evidence.network.flow.1',
  custody: 'child-device-query-store',
  evidenceGrade: 'A',
  claimBoundary: NoClaimBoundary,
} as const;

export function networkFlowReadModelSnapshot(
  destinationDomain = 'example-network.test'
): ParentActivityNetworkFlowReadModelSnapshot {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-23T00:00:00Z',
    custody: 'child-device-query-store',
    limit: 10,
    returned: 1,
    activeRows: 1,
    tombstoneRows: 0,
    exportableRows: 1,
    capabilityStatus: 'available',
    latestEventId: 'activity-network-flow-1',
    latestObservedAt: '2026-06-23T00:00:00Z',
    latestTombstoneEventId: null,
    latestTombstoneObservedAt: null,
    deletedEvidenceReferenceIds: [],
    rows: [
      {
        schemaVersion: 1,
        eventId: 'activity-network-flow-1',
        observedAt: '2026-06-23T00:00:00Z',
        observer: 'windows-network',
        capabilityStatus: 'available',
        adapterId: 'windows-network-snapshot',
        protocol: 'tcp',
        tcpState: 'established',
        localEndpoint: {
          ip: '127.0.0.1',
          port: 4242,
        },
        destinationEndpoint: {
          ip: '203.0.113.10',
          port: 443,
        },
        destinationDomain,
        domainAttributionStatus: 'domain-observed',
        processAttributionStatus: 'process-attributed',
        processId: 4242,
        processName: 'notepad.exe',
        counters: {
          connectionCount: 1,
          bytesSent: null,
          bytesReceived: null,
          firstSeenAt: '2026-06-23T00:00:00Z',
          lastSeenAt: '2026-06-23T00:00:00Z',
        },
        evidence: [],
      },
    ],
  };
}

export function activityTrackingReadModelResultSnapshot(
  deviceId = 'child-device-1'
): ParentActivityTrackingReadModelResultSnapshot {
  return {
    ok: true,
    reason: null,
    value: {
      schemaVersion: 1,
      generatedAt: '2026-06-23T00:00:00Z',
      custodyLabel: 'child-device-query-store',
      limit: 10,
      returned: 1,
      activeRows: 1,
      tombstoneRows: 0,
      capabilityStatus: 'available',
      latestEventId: 'evt-rust-tracking',
      latestObservedAt: '2026-06-23T00:00:00Z',
      latestActiveEventId: 'evt-rust-tracking',
      latestActiveObservedAt: '2026-06-23T00:00:00Z',
      latestTombstoneEventId: null,
      latestTombstoneObservedAt: null,
      activeKindCounts: [],
      activeDeviceCounts: [
        {
          value: deviceId,
          count: 1,
        },
      ],
      activeCapabilityStatusCounts: [
        {
          value: 'available',
          count: 1,
        },
      ],
      deletedEvidenceReferenceIds: [],
      rows: [
        {
          schemaVersion: 1,
          eventId: 'evt-rust-tracking',
          observedAt: '2026-06-23T00:00:00Z',
          deviceId,
          platform: 'windows',
          observer: 'tracking-read-model',
          kind: 'device-status',
          subjectKind: 'device',
          subjectId: deviceId,
          subjectDisplayName: 'Child Laptop',
          capabilityStatus: 'available',
          queryVisibility: 'parent-visible',
          deletedAt: null,
          evidenceReferenceIds: [],
          deletedEvidenceReferenceIds: [],
          evidence: [],
        },
      ],
    },
  };
}

export function activityReportDocumentSnapshot(): ParentAgentActivityReportDocument {
  return {
    schemaVersion: 1,
    reportId: 'activity-report-1',
    frequency: ParentAgentActivityReportFrequency.Daily,
    scope: {
      scopeKind: ParentAgentActivitySurfaceScopeKind.Family,
      familyId: 'family-1',
      deviceId: null,
    },
    requestedAt: '2026-06-23T00:00:00Z',
    rangeStart: '2026-06-22T00:00:00Z',
    rangeEnd: '2026-06-23T00:00:00Z',
    generatedAt: '2026-06-23T00:00:01Z',
    savedMetadata: {
      reportId: 'activity-report-1',
      fileName: 'activity-report-1.json',
      savedState: ParentAgentActivitySavedReportState.Saved,
      savedAt: '2026-06-23T00:00:02Z',
      storageReason: null,
      custodyLabel: ParentAgentActivityReportCustodyLabel.ParentDeviceLocalReportJson,
      sourceLabel: ParentAgentActivityReportSourceLabel.SavedReportJson,
      rawChildEvidenceIncluded: false,
    },
    sourceStates: [],
    sections: [
      {
        sectionKind: ParentAgentActivityReportSectionKind.Summary,
        title: 'Daily activity summary',
        state: ParentAgentActivityReadModelState.Ready,
        summary: 'The local service generated this report.',
        itemCount: 0,
        evidence: [],
      },
    ],
  };
}

export function activityReportHistorySnapshot(
  report = activityReportDocumentSnapshot()
): ParentAgentActivityHistoricalReportList {
  return {
    schemaVersion: 1,
    request: {
      schemaVersion: 1,
      scope: report.scope,
      requestedAt: '2026-06-23T00:00:03Z',
      rangeStart: report.rangeStart,
      rangeEnd: report.rangeEnd,
    },
    state: ParentAgentActivityReadModelState.Ready,
    storageState: ParentAgentActivitySavedReportState.Saved,
    storageReason: null,
    reports: [
      {
        schemaVersion: 1,
        reportId: report.reportId,
        fileName: 'activity-report-1.json',
        reportDate: '2026-06-23',
        rangeStart: report.rangeStart,
        rangeEnd: report.rangeEnd,
        summary: 'The local service generated this report.',
        savedState: ParentAgentActivitySavedReportState.Saved,
        savedAt: '2026-06-23T00:00:02Z',
        sourceStateSummary: {
          totalSources: 0,
          readySources: 0,
          offlineSources: 0,
          staleSources: 0,
          unavailableSources: 0,
          unreachableSources: 0,
          errorSources: 0,
        },
        parsedReport: report,
        custodyLabel: ParentAgentActivityReportCustodyLabel.ParentDeviceLocalHistory,
        sourceLabel: ParentAgentActivityReportSourceLabel.SavedReportHistory,
        rawChildEvidenceIncluded: false,
      },
    ],
  };
}

export function activityReportEventSnapshots(): readonly ParentRouteEventSnapshot[] {
  const report = activityReportDocumentSnapshot();
  const history = activityReportHistorySnapshot(report);
  return [
    activityReportRouteEvent(ParentAgentEvent.ActivityReportSaved, 'evt-activity-report-saved', {
      [ParentAgentProtocolField.ActivitySurfaceState]: ParentAgentActivityReadModelState.Ready,
      [ParentAgentProtocolField.ActivityReportDocument]: JSON.stringify(report),
    }),
    activityReportRouteEvent(ParentAgentEvent.ActivityReportHistoryReported, 'evt-activity-report-history', {
      [ParentAgentProtocolField.ActivitySurfaceState]: ParentAgentActivityReadModelState.Ready,
      [ParentAgentProtocolField.ActivityReports]: JSON.stringify(history),
    }),
  ];
}

export function malformedActivityReportEventSnapshot(): ParentRouteEventSnapshot {
  return activityReportRouteEvent(ParentAgentEvent.ActivityReportGenerated, 'evt-activity-report-malformed', {
    [ParentAgentProtocolField.ActivitySurfaceState]: ParentAgentActivityReadModelState.Ready,
    [ParentAgentProtocolField.ActivityReportDocument]: '{',
  });
}

function activityReportRouteEvent(
  event: string,
  eventId: string,
  payload: Readonly<Record<string, string>>
): ParentRouteEventSnapshot {
  return {
    event,
    eventId,
    correlationId: `correlation-${eventId}`,
    sentAt: '2026-06-23T00:00:04Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload,
  };
}
