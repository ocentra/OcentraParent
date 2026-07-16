import { expect } from 'vitest';
import { projectPortalLanDiagnosticsViewModel } from '@ocentra-parent/portal-domain/live-activity-lan-add-device';
import type { ParentLanDiscoveryEvidenceRecordSnapshot } from '../../generated/parent-ui-bridge';
import { projectLanDiscoverySourceMatrixViewModel } from '../../src/NetworkEvidenceDrawerRoutePanel';
import { lanNeighborHouseholdDecision } from '../fixtures/activity-ui-lan-pairing-fixtures';

type LanSourceMatrixProjection = NonNullable<ReturnType<typeof projectLanDiscoverySourceMatrixViewModel>>;
type LanDiagnosticsProjection = NonNullable<ReturnType<typeof projectPortalLanDiagnosticsViewModel>>;
type LanEvidenceSource = ParentLanDiscoveryEvidenceRecordSnapshot['source'];
type LanEvidenceKind = ParentLanDiscoveryEvidenceRecordSnapshot['evidenceKind'];
type LanEvidenceConfidence = ParentLanDiscoveryEvidenceRecordSnapshot['confidence'];

const LAN_SCHEMA_VERSION = 1;

export function expectLanSourceMatrixProjection(): void {
  const projection = requireProjection(
    projectLanDiscoverySourceMatrixViewModel(LAN_SOURCE_MATRIX_READ_MODEL),
    'LAN source matrix'
  );
  const diagnosticsProjection = requireProjection(
    projectPortalLanDiagnosticsViewModel(LAN_SOURCE_MATRIX_READ_MODEL),
    'LAN diagnostics'
  );

  expectLanSourceMatrixSummaries(projection);
  expectLanDiagnosticsSummaries(diagnosticsProjection);
  expectLanSourceMatrixRows(projection);
  expectLanDiagnosticsRows(diagnosticsProjection);
  expectLanRecentEventRows(projection);
}

export function expectNoLanSourceMatrixProjection(): void {
  expect(
    projectLanDiscoverySourceMatrixViewModel({
      scanSummary: {
        sourceLabels: [],
      },
      discoveryEventHistory: {
        schemaVersion: 1,
        generatedAt: '2026-06-23T00:02:00Z',
        state: 'empty',
        latestEventId: null,
        latestObservedAt: null,
        rows: [],
      },
      lanDiscoverySourceMatrix: null,
    })
  ).toBeNull();
  expect(projectLanDiscoverySourceMatrixViewModel(null)).toBeNull();
}

function requireProjection<T>(projection: T | null, label: string): T {
  if (projection === null) {
    throw new Error(`${label} projection was not available`);
  }
  return projection;
}

function expectLanSourceMatrixSummaries(projection: LanSourceMatrixProjection): void {
  expect(projection.generatedAt).toBe('2026-06-23T00:00:00Z');
  expect(projection.rowSummary).toBe('2 workpacks | 3 sources');
  expect(projection.statusSummary).toBe('2 implemented | 1 partial | 0 manual required | 0 not implemented');
  expect(projection.historyState).toBe('ready');
  expect(projection.historySummary).toBe('3 events | ready');
  expect(projection.latestHistoryEventId).toBe('lan-scan-finished-1');
  expect(projection.latestHistoryObservedAt).toBe('2026-06-23T00:02:00Z');
  expect(projection.currentSourceSummary).toBe('local-service | windows-neighbor-table | previous-scan-snapshot');
  expect(projection.persistedSourceSummary).toBe('previous-scan-snapshot (WP 15)');
  expect(projection.claimsProved).toBe('read-model-source-matrix | weak-sources-fenced');
  expect(projection.claimsNotProved).toBe('packet-mode-not-implemented | physical-proof-manual-required');
}

function expectLanDiagnosticsSummaries(diagnosticsProjection: LanDiagnosticsProjection): void {
  expect(diagnosticsProjection.evidenceWindowSummary).toBe(
    '1 evidence records | first 2026-06-01T15:20:00.000Z | latest 2026-06-01T15:20:00.000Z | next expiry no-expiry'
  );
  expect(diagnosticsProjection.trustedRegistrySummary).toBe(
    '1 trusted routes | latest trust 2026-06-01T15:20:00Z | next expiry 2026-06-01T16:20:00Z'
  );
  expect(diagnosticsProjection.decisionHistorySummary).toBe(
    '2 parent decisions | latest 2026-06-01T15:22:05Z | 1 rename | 1 assign'
  );
  expect(diagnosticsProjection.policyTargetSurfaceSummary).toBe(
    'devices | policy | browser | activity | tracking | network'
  );
  expect(diagnosticsProjection.productionProofSummary).toBe('Not reported');
  expect(diagnosticsProjection.signedProofSummary).toBe('1 signed proof rows | 1 manual-required | 1 degraded');
  expect(diagnosticsProjection.routeSafetySummary).toBe('2 route safety rows | 2 accepted | parent-local-service');
  expect(diagnosticsProjection.relayCacheSummary).toBe(
    '1 relay cache rows | 1 unavailable | no-ocentra-child-data-custody'
  );
}

function expectLanSourceMatrixRows(projection: LanSourceMatrixProjection): void {
  expect(projection.implementedSourceRows).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        label: 'previous-scan-snapshot (WP 15)',
        value: expect.stringContaining('restart-persisted'),
      }),
    ])
  );
  expect(projection.weakSourceRows).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        label: 'previous-scan-snapshot (WP 15)',
        value: expect.stringContaining('weak-identity'),
      }),
      expect.objectContaining({
        label: 'previous-scan-snapshot (WP 15)',
        value: expect.stringContaining('no-route-control'),
      }),
    ])
  );
}

function expectLanDiagnosticsRows(diagnosticsProjection: LanDiagnosticsProjection): void {
  expect(diagnosticsProjection.evidenceRecordRows).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        label: 'local-service | ip-address',
        value: expect.stringContaining('192.168.2.10'),
      }),
    ])
  );
  expect(diagnosticsProjection.trustedRegistryRows).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        label: 'pairing-local-agent-1',
        value: expect.stringContaining('GAMEDEV'),
      }),
    ])
  );
  expect(diagnosticsProjection.decisionRows).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        label: 'rename | Kitchen laptop',
        value: expect.stringContaining('decided 2026-06-01T15:22:05Z'),
      }),
    ])
  );
}

function expectLanRecentEventRows(projection: LanSourceMatrixProjection): void {
  expect(projection.recentEventRows).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        label: 'scan-finished',
        value: expect.stringContaining('lan-scan-finished-1'),
      }),
      expect.objectContaining({
        label: 'device-found',
        value: expect.stringContaining('lan-device-found-1'),
      }),
      expect.objectContaining({
        label: 'device-found',
        value: expect.stringContaining('scan-session-1'),
      }),
      expect.objectContaining({
        label: 'device-found',
        value: expect.stringContaining('child-device-1'),
      }),
      expect.objectContaining({
        label: 'device-found',
        value: expect.stringContaining('lan-evidence-1'),
      }),
    ])
  );
}

const LAN_DIAGNOSTICS_READ_MODEL = {
  canonicalHouseholdDevices: [
    {
      canonicalDeviceId: 'lan-physical-mac-b42e993e72b9',
      displayName: 'GAMEDEV',
      roleBadges: ['child-agent'],
      policyTargetSurfaces: ['devices', 'policy', 'browser', 'activity', 'tracking', 'network'],
      sourceLabels: ['local-service'],
      networkIdentity: {
        ipAddresses: ['192.168.2.10'],
        reachability: 'online',
        evidenceRecords: [
          lanEvidenceRecord('local-service', 'ip-address', '192.168.2.10', 'ip:192.168.2.10', 'confirmed'),
        ],
      },
    },
  ],
  trustedDeviceRegistry: [
    {
      pairingId: 'pairing-local-agent-1',
      childDevice: {
        label: 'GAMEDEV',
      },
      trustState: 'paired',
      routeId: 'lan-route-local-network',
      trustedAt: '2026-06-01T15:20:00Z',
      expiresAt: '2026-06-01T16:20:00Z',
      revokedAt: null,
    },
  ],
  householdDeviceDecisions: [
    lanNeighborHouseholdDecision(),
    {
      actionKind: 'assign',
      canonicalDeviceId: 'lan-physical-mac-b42e993e72b9',
      childProfileId: 'child-profile-1',
      displayName: 'GAMEDEV',
      deviceKind: 'desktop',
      decidedAt: '2026-06-01T15:21:05Z',
      revokedAt: null,
    },
  ],
  productionHouseholdProof: null,
  signedDiscoveryRelaySpine: {
    signedProofRows: [
      {
        check: 'signed-child-agent-hello',
        proofState: 'manual-required',
        responseState: 'degraded',
        evidenceLabel: 'Signed hello manual proof required',
      },
    ],
    routeSafetyRows: [
      {
        check: 'parent-local-service',
        responseState: 'accepted',
        custodyLabel: 'parent-local-service',
      },
      {
        check: 'parent-local-service',
        responseState: 'accepted',
        custodyLabel: 'parent-local-service',
      },
    ],
    relayCacheRows: [
      {
        check: 'relay-cache',
        decisionState: 'unavailable',
        custodyLabel: 'no-ocentra-child-data-custody',
      },
    ],
  },
} as const;

const LAN_SOURCE_MATRIX_READ_MODEL = {
  ...LAN_DIAGNOSTICS_READ_MODEL,
  scanSummary: {
    schemaVersion: 1,
    sourceLabels: ['local-service', 'windows-neighbor-table', 'previous-scan-snapshot'],
    scannedDeviceCount: 3,
    agentDeviceCount: 1,
    passiveDeviceCount: 1,
    infrastructureDeviceCount: 1,
    unsupportedDeviceCount: 2,
  },
  discoveryEventHistory: {
    schemaVersion: 1,
    generatedAt: '2026-06-23T00:02:00Z',
    state: 'ready',
    latestEventId: 'lan-scan-finished-1',
    latestObservedAt: '2026-06-23T00:02:00Z',
    rows: [
      {
        schemaVersion: 1,
        eventId: 'lan-scan-started-1',
        eventKind: 'scan-started',
        occurredAt: '2026-06-23T00:00:00Z',
        previousEventId: null,
        scanSessionId: 'scan-session-1',
        affectedDeviceId: null,
        evidenceId: null,
        summary: 'Passive scan started',
      },
      {
        schemaVersion: 1,
        eventId: 'lan-device-found-1',
        eventKind: 'device-found',
        occurredAt: '2026-06-23T00:01:00Z',
        previousEventId: 'lan-scan-started-1',
        scanSessionId: 'scan-session-1',
        affectedDeviceId: 'child-device-1',
        evidenceId: 'lan-evidence-1',
        summary: 'Observed a child-device candidate',
      },
      {
        schemaVersion: 1,
        eventId: 'lan-scan-finished-1',
        eventKind: 'scan-finished',
        occurredAt: '2026-06-23T00:02:00Z',
        previousEventId: 'lan-device-found-1',
        scanSessionId: 'scan-session-1',
        affectedDeviceId: null,
        evidenceId: null,
        summary: 'Passive scan finished',
      },
    ],
  },
  lanDiscoverySourceMatrix: {
    generatedAt: '2026-06-23T00:00:00Z',
    workpackRows: [
      {
        workpackId: '04',
        title: 'Passive neighbor discovery',
        discoveryState: 'discovered',
        proofState: 'ci-mechanical-proof',
        runtimeOwner: 'rust-service-read-model',
        status: 'implemented',
        readModelVisible: true,
        requiredArtifactSummary: null,
      },
      {
        workpackId: '15',
        title: 'Household device store',
        discoveryState: 'pending',
        proofState: 'ci-mechanical-proof',
        runtimeOwner: 'rust-service-read-model',
        status: 'partial',
        readModelVisible: true,
        requiredArtifactSummary: null,
      },
    ],
    sourceRows: [
      {
        source: 'windows-neighbor-table',
        workpackId: '04',
        status: 'implemented',
        authority: 'weak-identity',
        runtimePath: 'rust-service-read-model',
        uiSurface: 'devices-lan',
        canConfirmChildAgent: false,
        canAssignChildProfile: false,
        canControlRoute: false,
        requiresSelectedInterface: true,
        persistsAcrossRestart: false,
        evidenceLabel: 'Passive neighbor table row',
        requiredArtifactSummary: null,
      },
      {
        source: 'previous-scan-snapshot',
        workpackId: '15',
        status: 'implemented',
        authority: 'weak-identity',
        runtimePath: 'rust-service-read-model',
        uiSurface: 'devices-lan',
        canConfirmChildAgent: false,
        canAssignChildProfile: false,
        canControlRoute: false,
        requiresSelectedInterface: false,
        persistsAcrossRestart: true,
        evidenceLabel: 'Previous scan continuity hint',
        requiredArtifactSummary: null,
      },
      {
        source: 'service-identity-probe',
        workpackId: '11',
        status: 'partial',
        authority: 'presence-only',
        runtimePath: 'rust-service-read-model',
        uiSurface: 'devices-lan',
        canConfirmChildAgent: false,
        canAssignChildProfile: false,
        canControlRoute: false,
        requiresSelectedInterface: false,
        persistsAcrossRestart: false,
        evidenceLabel: 'Service identity probe presence only',
        requiredArtifactSummary: null,
      },
    ],
    claimsProved: ['read-model-source-matrix', 'weak-sources-fenced'],
    claimsNotProved: ['packet-mode-not-implemented', 'physical-proof-manual-required'],
  },
} as const;

function lanEvidenceRecord(
  source: LanEvidenceSource,
  evidenceKind: LanEvidenceKind,
  value: string,
  mergeKey: string,
  confidence: LanEvidenceConfidence
) {
  return {
    schemaVersion: LAN_SCHEMA_VERSION,
    evidenceId: `lan-evidence-${source}-${evidenceKind}-${value.toLowerCase().replace(/[^a-z0-9]/gu, '')}`,
    source,
    evidenceKind,
    deviceId: 'lan-physical-mac-b42e993e72b9',
    value,
    normalizedValue: value.toLowerCase(),
    firstSeenAt: '2026-06-01T15:20:00.000Z',
    lastSeenAt: '2026-06-01T15:20:00.000Z',
    expiresAt: null,
    confidence,
    mergeKey,
    note: null,
  } as const;
}
