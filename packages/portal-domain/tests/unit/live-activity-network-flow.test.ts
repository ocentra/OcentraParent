import { describe, expect, it } from 'vitest';
import { networkEvidenceDrawerSummary } from '../../src/network-evidence-drawer';
import { NetworkEvidenceDrawerProof } from './fixtures/network-evidence-drawer-proof-fixture';

type NetworkFlowReadModel = NonNullable<Parameters<typeof networkEvidenceDrawerSummary>[0]>;
type NetworkFlowObservation = NetworkFlowReadModel['rows'][number];

describe('portal live activity network flow', () => {
  it('projects the parent network evidence drawer with truthful surfaced refs and unsupported gaps', () => {
    expectPopulatedNetworkEvidenceDrawerSummary(
      networkEvidenceDrawerSummary(
        networkFlowReadModel(),
        networkEvidenceDrawerSummaryContext({
          aiAuditRef: 'event.ai.analysis.completed.1',
          policyDecisionRef: 'event.policy.decision.completed.1',
          interventionResultRef: 'event.enforcement.result.observed.1',
          networkEvidenceGrade: 'A',
        })
      )
    );
  });

  it('keeps empty network flow read models visible without inventing destinations', () => {
    const summary = networkEvidenceDrawerSummary(
      networkFlowReadModel({
        returned: 0,
        activeRows: 0,
        exportableRows: 0,
        capabilityStatus: 'no-network-observations',
        latestEventId: null,
        latestObservedAt: null,
        rows: [],
      })
    );

    expectEmptyNetworkEvidenceDrawerSummary(summary);
  });

  it('renders deleted and degraded network status from service-backed read models', () => {
    const summary = networkEvidenceDrawerSummary(
      networkFlowReadModel({
        returned: 0,
        activeRows: 0,
        tombstoneRows: 1,
        exportableRows: 0,
        capabilityStatus: 'adapter-error',
        latestEventId: null,
        latestObservedAt: null,
        latestTombstoneEventId: 'activity-network-flow-deleted',
        latestTombstoneObservedAt: '2026-05-21T02:05:00Z',
        deletedEvidenceReferenceIds: ['network-evidence-1'],
        rows: [],
      })
    );

    expectDeletedNetworkEvidenceDrawerSummary(summary);
  });

  it('uses supplied evidence summary refs when no runtime refs were streamed', () => {
    const summary = networkEvidenceDrawerSummary(
      networkFlowReadModel(),
      networkEvidenceDrawerSummaryContext({
        aiAuditRef: 'local-ai-result.network.preview.1',
        policyDecisionRef: 'policy-decision.network.preview.1',
        networkEvidenceGrade: 'A',
      })
    );

    expectSuppliedReferenceNetworkEvidenceDrawerSummary(summary);
  });
});

function expectPopulatedNetworkEvidenceDrawerSummary(summary: ReturnType<typeof networkEvidenceDrawerSummary>) {
  expect(summary.evidenceId).toBe(NetworkEvidenceDrawerProof.eventId);
  expect(summary.sourceAdapter).toBe('windows-network-snapshot');
  expect(summary.sourceQuality).toBe('available');
  expect(summary.platformState).toBe('child-device-query-store | available');
  expect(summary.readModelRows).toBe('1 | 1 | 0 | 1');
  expect(summary.localEndpoint).toBe('127.0.0.1 | 4242');
  expect(summary.remoteEndpoint).toBe('203.0.113.10 | 443');
  expect(summary.domainEvidenceRef).toBe(NetworkEvidenceDrawerProof.expected.domainEvidenceRef);
  expect(summary.processRef).toBe(NetworkEvidenceDrawerProof.expected.processRef);
  expect(summary.evidenceReferences).toBe(
    `${NetworkEvidenceDrawerProof.evidenceId} | ${NetworkEvidenceDrawerProof.journalEvidenceId}`
  );
  expect(summary.exactUrlClaim).toBe('Not reported');
  expect(summary.aiAuditRef).toBe('event.ai.analysis.completed.1');
  expect(summary.policyDecisionRef).toBe('event.policy.decision.completed.1');
  expect(summary.interventionResultRef).toBe('event.enforcement.result.observed.1');
  expect(summary.evidenceGrade).toBe('A');
  expect(summary.retentionState).toBe('0 | 1');
  expect(summary.deletedEvidenceReferences).toBe('Not reported');
  expect(summary.degradedState).toBe('available | domain-observed | process-attributed');
}

function expectEmptyNetworkEvidenceDrawerSummary(summary: ReturnType<typeof networkEvidenceDrawerSummary>) {
  expect(summary.evidenceReferences).toBe('Not reported');
  expect(summary.exactUrlClaim).toBe('Not reported');
  expect(summary.platformState).toBe('child-device-query-store | no-network-observations');
  expect(summary.readModelRows).toBe('0 | 0 | 0 | 0');
  expect(summary.degradedState).toBe('no-network-observations');
}

function expectDeletedNetworkEvidenceDrawerSummary(summary: ReturnType<typeof networkEvidenceDrawerSummary>) {
  expect(summary.sourceQuality).toBe('adapter-error');
  expect(summary.platformState).toBe('child-device-query-store | adapter-error');
  expect(summary.readModelRows).toBe('0 | 0 | 1 | 0');
  expect(summary.retentionState).toBe('activity-network-flow-deleted | 2026-05-21T02:05:00Z | network-evidence-1');
  expect(summary.deletedEvidenceReferences).toBe('network-evidence-1');
  expect(summary.degradedState).toBe('adapter-error');
  expect(summary.policyDecisionRef).toBe('Not reported');
  expect(summary.interventionResultRef).toBe('Not reported');
}

function expectSuppliedReferenceNetworkEvidenceDrawerSummary(summary: ReturnType<typeof networkEvidenceDrawerSummary>) {
  expect(summary.aiAuditRef).toBe('local-ai-result.network.preview.1');
  expect(summary.policyDecisionRef).toBe('policy-decision.network.preview.1');
  expect(summary.interventionResultRef).toBe('Not reported');
  expect(summary.evidenceGrade).toBe('A');
}

function networkEvidenceDrawerSummaryContext(
  networkEvidenceSummary: {
    readonly aiAuditRef?: string;
    readonly policyDecisionRef?: string;
    readonly networkEvidenceGrade?: string;
    readonly interventionResultRef?: string;
  } | null = null
) {
  return {
    networkEvidenceSummary,
  };
}

function networkFlowReadModel(
  overrides: {
    readonly returned?: number;
    readonly activeRows?: number;
    readonly tombstoneRows?: number;
    readonly exportableRows?: number;
    readonly capabilityStatus?: 'available' | 'no-network-observations' | 'adapter-error';
    readonly latestEventId?: string | null;
    readonly latestObservedAt?: string | null;
    readonly latestTombstoneEventId?: string | null;
    readonly latestTombstoneObservedAt?: string | null;
    readonly deletedEvidenceReferenceIds?: readonly string[];
    readonly rows?: readonly NetworkFlowObservation[];
  } = {}
): NetworkFlowReadModel {
  return {
    custody: 'child-device-query-store',
    returned: overrides.returned ?? 1,
    activeRows: overrides.activeRows ?? 1,
    tombstoneRows: overrides.tombstoneRows ?? 0,
    exportableRows: overrides.exportableRows ?? 1,
    capabilityStatus: overrides.capabilityStatus ?? 'available',
    latestTombstoneEventId: overrides.latestTombstoneEventId ?? null,
    latestTombstoneObservedAt: overrides.latestTombstoneObservedAt ?? null,
    deletedEvidenceReferenceIds: overrides.deletedEvidenceReferenceIds ?? [],
    rows: overrides.rows ?? [networkFlowObservation()],
  };
}

function networkFlowObservation(): NetworkFlowObservation {
  return {
    eventId: NetworkEvidenceDrawerProof.eventId,
    observedAt: '2026-05-21T02:00:00Z',
    capabilityStatus: NetworkEvidenceDrawerProof.fields.capabilityStatus,
    adapterId: NetworkEvidenceDrawerProof.fields.adapterId,
    protocol: NetworkEvidenceDrawerProof.fields.networkProtocol,
    tcpState: NetworkEvidenceDrawerProof.fields.tcpState,
    localEndpoint: {
      ip: NetworkEvidenceDrawerProof.fields.localIp,
      port: NetworkEvidenceDrawerProof.fields.localPort,
    },
    destinationEndpoint: {
      ip: NetworkEvidenceDrawerProof.fields.destinationIp,
      port: NetworkEvidenceDrawerProof.fields.destinationPort,
    },
    destinationDomain: NetworkEvidenceDrawerProof.fields.destinationDomain,
    domainAttributionStatus: NetworkEvidenceDrawerProof.fields.domainAttributionStatus,
    processAttributionStatus: NetworkEvidenceDrawerProof.fields.processAttributionStatus,
    processId: NetworkEvidenceDrawerProof.fields.pid,
    processName: NetworkEvidenceDrawerProof.fields.processName,
    counters: {
      connectionCount: 1,
      bytesSent: null,
      bytesReceived: null,
      firstSeenAt: '2026-05-21T02:00:00Z',
      lastSeenAt: '2026-05-21T02:00:00Z',
    },
    evidence: [
      {
        evidenceId: NetworkEvidenceDrawerProof.evidenceId,
        kind: 'local-db-row',
        digest: NetworkEvidenceDrawerProof.evidenceDigest,
        uri: null,
      },
      {
        evidenceId: NetworkEvidenceDrawerProof.journalEvidenceId,
        kind: 'journal-entry',
        digest: NetworkEvidenceDrawerProof.journalEvidenceDigest,
        uri: null,
      },
    ],
  };
}
