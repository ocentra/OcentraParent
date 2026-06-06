import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind, ActivityObserver } from '../src/kinds';
import {
  ActivityNetworkFlowDigestSchema,
  ActivityNetworkFlowObservationSchema,
  ActivityNetworkFlowReadModelSchema,
} from '../src/network-flow';
import { ActivityQuerySchemaVersion } from '../src/query';

const NetworkFlowObservationSample = {
  schemaVersion: ActivityQuerySchemaVersion,
  eventId: 'activity-event-network-1',
  observedAt: '2026-05-21T02:40:00Z',
  observer: ActivityObserver.WindowsNetwork,
  capabilityStatus: 'available',
  adapterId: 'windows-netstat-snapshot',
  protocol: 'tcp',
  tcpState: 'established',
  localEndpoint: {
    ip: '192.168.1.25',
    port: 51422,
  },
  destinationEndpoint: {
    ip: '93.184.216.34',
    port: 443,
  },
  destinationDomain: 'example.com',
  domainAttributionStatus: 'domain-observed',
  processAttributionStatus: 'process-attributed',
  processId: 4242,
  processName: 'chrome.exe',
  counters: {
    connectionCount: 1,
    bytesSent: null,
    bytesReceived: null,
    firstSeenAt: '2026-05-21T02:40:00Z',
    lastSeenAt: '2026-05-21T02:40:01Z',
  },
  evidence: [
    {
      evidenceId: 'journal-entry-network-1',
      kind: ActivityEvidenceKind.JournalEntry,
      digest: 'sha256:network-flow-digest',
      uri: null,
    },
  ],
} as const;

const NetworkFlowReadModelSample = {
  schemaVersion: ActivityQuerySchemaVersion,
  generatedAt: '2026-05-21T02:40:02Z',
  custody: 'child-device-query-store',
  limit: 25,
  returned: 1,
  activeRows: 1,
  tombstoneRows: 0,
  exportableRows: 1,
  capabilityStatus: 'available',
  latestEventId: 'activity-event-network-1',
  latestObservedAt: '2026-05-21T02:40:00Z',
  latestTombstoneEventId: null,
  latestTombstoneObservedAt: null,
  deletedEvidenceReferenceIds: [],
  rows: [NetworkFlowObservationSample],
} as const;

const NetworkFlowDeletedReadModelSample = {
  ...NetworkFlowReadModelSample,
  returned: 0,
  activeRows: 0,
  tombstoneRows: 1,
  exportableRows: 0,
  latestEventId: 'activity-event-network-retention-delete-1',
  latestObservedAt: '2026-05-21T02:41:00Z',
  latestTombstoneEventId: 'activity-event-network-retention-delete-1',
  latestTombstoneObservedAt: '2026-05-21T02:41:00Z',
  deletedEvidenceReferenceIds: ['activity-event-network-1'],
  rows: [],
} as const;

const NetworkFlowDigestSample = {
  schemaVersion: ActivityQuerySchemaVersion,
  generatedAt: '2026-05-21T02:40:02Z',
  custody: 'child-device-query-store',
  evidence: NetworkFlowObservationSample.evidence,
  topProcesses: [
    {
      key: 'process-4242',
      label: 'chrome.exe',
      connectionCount: 1,
      bytesSent: null,
      bytesReceived: null,
      evidenceIds: ['journal-entry-network-1'],
    },
  ],
  topDestinations: [
    {
      key: 'example.com',
      label: 'example.com:443',
      connectionCount: 1,
      bytesSent: null,
      bytesReceived: null,
      evidenceIds: ['journal-entry-network-1'],
    },
  ],
  unusualIndicators: [
    {
      kind: 'encrypted-content-unavailable',
      label: 'HTTPS payload was not decrypted or inspected.',
      observedAt: '2026-05-21T02:40:02Z',
      evidenceIds: ['journal-entry-network-1'],
    },
  ],
  runtimeDelivery: {
    observedRows: 1,
    deliveredRows: 1,
    failedRows: 0,
    publishReports: 11,
    storedEvents: 11,
    deadLetters: 0,
    manualRequiredRows: 1,
    enforcementCommandEvents: 0,
  },
} as const;

describe('network flow query contracts', () => {
  it('parses process-attributed network observations and read models', () => {
    const observation = ActivityNetworkFlowObservationSchema.parse(NetworkFlowObservationSample);
    const readModel = ActivityNetworkFlowReadModelSchema.parse(NetworkFlowReadModelSample);

    expect(observation.destinationDomain).toBe('example.com');
    expect(readModel.rows[0]?.processName).toBe('chrome.exe');
    expect(readModel.activeRows).toBe(1);
    expect(readModel.exportableRows).toBe(1);
    expect(readModel.runtimeDelivery).toBeNull();
  });

  it('parses network retention tombstone state without exposing deleted rows', () => {
    const readModel = ActivityNetworkFlowReadModelSchema.parse(NetworkFlowDeletedReadModelSample);

    expect(readModel.rows).toHaveLength(0);
    expect(readModel.tombstoneRows).toBe(1);
    expect(readModel.deletedEvidenceReferenceIds).toEqual(['activity-event-network-1']);
  });

  it('parses local AI digest references without packet or URL claims', () => {
    const digest = ActivityNetworkFlowDigestSchema.parse(NetworkFlowDigestSample);

    expect(digest.topProcesses[0]?.label).toBe('chrome.exe');
    expect(digest.unusualIndicators[0]?.kind).toBe('encrypted-content-unavailable');
    expect(digest.runtimeDelivery?.manualRequiredRows).toBe(1);
    expect(digest.runtimeDelivery?.enforcementCommandEvents).toBe(0);
  });

  it('keeps older network flow digests valid without runtime delivery', () => {
    const digest = ActivityNetworkFlowDigestSchema.parse({
      ...NetworkFlowDigestSample,
      runtimeDelivery: undefined,
    });

    expect(digest.runtimeDelivery).toBeNull();
  });

  it('rejects read models with untyped custody states', () => {
    const result = ActivityNetworkFlowReadModelSchema.safeParse({
      ...NetworkFlowReadModelSample,
      custody: 'raw-packet-capture',
    });

    expect(result.success).toBe(false);
  });
});

describe('network flow numeric bounds', () => {
  it('rejects negative endpoint ports, connection counts, and byte counters', () => {
    const negativePort = ActivityNetworkFlowObservationSchema.safeParse({
      ...NetworkFlowObservationSample,
      destinationEndpoint: {
        ...NetworkFlowObservationSample.destinationEndpoint,
        port: -1,
      },
    });
    const negativeConnectionCount = ActivityNetworkFlowObservationSchema.safeParse({
      ...NetworkFlowObservationSample,
      counters: {
        ...NetworkFlowObservationSample.counters,
        connectionCount: -1,
      },
    });
    const negativeBytesSent = ActivityNetworkFlowObservationSchema.safeParse({
      ...NetworkFlowObservationSample,
      counters: {
        ...NetworkFlowObservationSample.counters,
        bytesSent: -1,
      },
    });
    const negativeBytesReceived = ActivityNetworkFlowDigestSchema.safeParse({
      ...NetworkFlowDigestSample,
      topDestinations: [
        {
          ...NetworkFlowDigestSample.topDestinations[0],
          bytesReceived: -1,
        },
      ],
    });
    const negativeManualRows = ActivityNetworkFlowDigestSchema.safeParse({
      ...NetworkFlowDigestSample,
      runtimeDelivery: {
        ...NetworkFlowDigestSample.runtimeDelivery,
        manualRequiredRows: -1,
      },
    });

    expect(negativePort.success).toBe(false);
    expect(negativeConnectionCount.success).toBe(false);
    expect(negativeBytesSent.success).toBe(false);
    expect(negativeBytesReceived.success).toBe(false);
    expect(negativeManualRows.success).toBe(false);
  });

  it('rejects negative read-model counts and process identifiers', () => {
    const negativeLimit = ActivityNetworkFlowReadModelSchema.safeParse({
      ...NetworkFlowReadModelSample,
      limit: -1,
    });
    const negativeReturned = ActivityNetworkFlowReadModelSchema.safeParse({
      ...NetworkFlowReadModelSample,
      returned: -1,
    });
    const negativeActiveRows = ActivityNetworkFlowReadModelSchema.safeParse({
      ...NetworkFlowReadModelSample,
      activeRows: -1,
    });
    const negativeProcessId = ActivityNetworkFlowObservationSchema.safeParse({
      ...NetworkFlowObservationSample,
      processId: -1,
    });

    expect(negativeLimit.success).toBe(false);
    expect(negativeReturned.success).toBe(false);
    expect(negativeActiveRows.success).toBe(false);
    expect(negativeProcessId.success).toBe(false);
  });
});
