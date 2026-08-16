import type {
  ParentActivityNetworkFlowReadModelSnapshot,
  ParentActivityTrackingReadModelResultSnapshot,
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
