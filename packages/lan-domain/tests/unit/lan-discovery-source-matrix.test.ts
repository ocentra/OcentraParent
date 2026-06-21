import { describe, expect, it } from 'vitest';
import { LanDiscoverySourceMatrixSchema } from '@ocentra-parent/schema-domain/lan-source-matrix';
import { LanBrowserAddDeviceReadModelSchema } from '@ocentra-parent/schema-domain/lan-pairing-device';

describe('LAN discovery source matrix', () => {
  registerSourceMatrixSchemaTests();
  registerAddDeviceReadModelTest();
});

function registerSourceMatrixSchemaTests(): void {
  it('accepts an honest matrix covering all 20 LAN workpacks', () => {
    const parsed = LanDiscoverySourceMatrixSchema.parse(sourceMatrix());

    expect(parsed.workpackRows.map((row) => row.workpackId)).toEqual([
      '01',
      '02',
      '03',
      '04',
      '05',
      '06',
      '07',
      '08',
      '09',
      '10',
      '11',
      '12',
      '13',
      '14',
      '15',
      '16',
      '17',
      '18',
      '19',
      '20',
    ]);
    expect(parsed.sourceRows.find((row) => row.source === 'windows-neighbor-table')?.canAssignChildProfile).toBe(false);
    expect(parsed.sourceRows.find((row) => row.source === 'signed-child-agent-hello')?.canConfirmChildAgent).toBe(true);
  });

  it('rejects matrices that omit a LAN workpack row', () => {
    const parsed = LanDiscoverySourceMatrixSchema.safeParse({
      ...sourceMatrix(),
      workpackRows: sourceMatrix().workpackRows.filter((row) => row.workpackId !== '17'),
    });

    expect(parsed.success).toBe(false);
    if (!parsed.success) {
      expect(parsed.error.message).toContain('Expected complete LAN source matrix');
    }
  });

  it('rejects weak discovery sources that try to confirm or assign a child', () => {
    const parsed = LanDiscoverySourceMatrixSchema.safeParse({
      ...sourceMatrix(),
      sourceRows: sourceMatrix().sourceRows.map((row) =>
        row.source === 'mdns-dns-sd-query' ? { ...row, canConfirmChildAgent: true } : row
      ),
    });

    expect(parsed.success).toBe(false);
    if (!parsed.success) {
      expect(parsed.error.message).toContain('Expected complete LAN source matrix');
    }
  });

  it('rejects signed child-agent sources without required manual artifacts', () => {
    const parsed = LanDiscoverySourceMatrixSchema.safeParse({
      ...sourceMatrix(),
      sourceRows: sourceMatrix().sourceRows.map((row) =>
        row.source === 'signed-child-agent-heartbeat' ? { ...row, requiredArtifactSummary: null } : row
      ),
    });

    expect(parsed.success).toBe(false);
    if (!parsed.success) {
      expect(parsed.error.message).toContain('Expected complete LAN source matrix');
    }
  });
}

function registerAddDeviceReadModelTest(): void {
  it('keeps the source matrix on the browser add-device read model', () => {
    const parsed = LanBrowserAddDeviceReadModelSchema.parse({
      schemaVersion: 'v0.9',
      generatedAt: '2026-06-02T12:00:00.000Z',
      discoverySource: 'local-service',
      addDeviceState: 'pending',
      localServiceDiscoveryState: 'pending',
      physicalHouseholdLanState: 'manual-required',
      cloudRelayState: 'unavailable',
      scanSummary: {
        schemaVersion: 'v0.9',
        sourceLabels: ['local-service'],
        scannedDeviceCount: 1,
        agentDeviceCount: 1,
        passiveDeviceCount: 0,
        infrastructureDeviceCount: 0,
        unsupportedDeviceCount: 0,
      },
      discoveredDevices: [],
      canonicalHouseholdDevices: [],
      pairingRequests: [],
      trustedDeviceRegistry: [],
      householdDeviceDecisions: [],
      lanDiscoverySourceMatrix: sourceMatrix(),
      trustedDeviceIds: [],
      revokedDeviceIds: [],
      selectedDeviceReadiness: {
        schemaVersion: 'v0.9',
        selectedChildDeviceId: null,
        routeId: null,
        pairingId: null,
        trustState: 'unpaired',
        reachability: 'offline',
        readyForControl: false,
        staleAt: null,
        offlineAt: null,
      },
      controllerAuthority: 'active-controller',
      observerAuthority: 'observer',
      routeRequirementLabels: ['allowed-origin'],
      auditCheckLabels: ['anonymous', 'wrong-origin', 'wrong-device'],
      honestNonClaims: ['physical-household-lan-manual-required'],
    });

    expect(parsed.lanDiscoverySourceMatrix?.workpackRows).toHaveLength(20);
    expect(parsed.lanDiscoverySourceMatrix?.sourceRows[0]?.source).toBe('windows-neighbor-table');
  });
}

function sourceMatrix() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T15:55:00.000Z',
    workpackRows: workpackRows(),
    sourceRows: sourceRows(),
    claimsProved: [
      'all LAN plan workpacks are represented in a service-backed source matrix read model',
      'weak LAN discovery sources cannot confirm child identity or assign child profiles',
    ],
    claimsNotProved: [
      'packet-mode ARP sweep and passive listeners remain gated until packet driver artifacts exist',
      'physical household LAN completion remains manual-required until real two-host proof is attached',
      'mDNS/SSDP advertisement and responder behavior remains manual-required until fixtures and LAN captures exist',
    ],
  } as const;
}

function workpackRows() {
  return [
    workpack('01', 'Contract boundary and Effect schemas', 'discovered', 'ci-mechanical-proof', 'implemented', null),
    workpack('02', 'Evidence model and device record', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('03', 'Interface detection', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('04', 'Neighbor table ingestion', 'discovered', 'ci-mechanical-proof', 'partial', null),
    workpack('05', 'Targeted ARP checks', 'unavailable', 'not-implemented', 'not-implemented', packetArtifact()),
    workpack('06', 'Bounded ARP sweep', 'unavailable', 'not-implemented', 'not-implemented', packetArtifact()),
    workpack(
      '07',
      'Passive discovery listeners',
      'manual-required',
      'manual-required',
      'manual-required',
      packetArtifact()
    ),
    workpack(
      '08',
      'mDNS and DNS-SD discovery',
      'manual-required',
      'manual-required',
      'manual-required',
      mdnsArtifact()
    ),
    workpack('09', 'SSDP and UPnP discovery', 'manual-required', 'manual-required', 'manual-required', mdnsArtifact()),
    workpack('10', 'NetBIOS, LLMNR, and reverse DNS', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('11', 'Light service probing', 'unavailable', 'not-implemented', 'not-implemented', packetArtifact()),
    workpack('12', 'OUI and vendor lookup', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('13', 'Merge and de-duplication engine', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('14', 'Explainable classification', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('15', 'Household device store', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('16', 'Read models and LAN events', 'discovered', 'ci-mechanical-proof', 'implemented', null),
    workpack(
      '17',
      'Parent and child mDNS advertisements',
      'manual-required',
      'manual-required',
      'manual-required',
      mdnsArtifact()
    ),
    workpack(
      '18',
      'Signed child hello and heartbeat',
      'manual-required',
      'manual-required',
      'manual-required',
      signedArtifact()
    ),
    workpack('19', 'Assignment, revocation, and audit', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('20', 'Proof gates, fixtures, and rollout', 'pending', 'ci-mechanical-proof', 'partial', null),
  ] as const;
}

function sourceRows() {
  return [
    source('windows-neighbor-table', '04', 'implemented', 'weak-identity', false, false, null),
    source('mdns-dns-sd-query', '08', 'manual-required', 'name-only', false, false, mdnsArtifact()),
    source('ssdp-upnp-query', '09', 'manual-required', 'name-only', false, false, mdnsArtifact()),
    source('netbios-name-cache', '10', 'manual-required', 'name-only', false, false, mdnsArtifact()),
    source('service-identity-probe', '11', 'manual-required', 'classification-only', false, false, packetArtifact()),
    source('oui-vendor-lookup', '12', 'manual-required', 'classification-only', false, false, mdnsArtifact()),
    source('signed-child-agent-hello', '18', 'manual-required', 'strong-identity', true, false, signedArtifact()),
    source('signed-child-agent-heartbeat', '18', 'manual-required', 'strong-identity', true, false, signedArtifact()),
  ] as const;
}

function workpack(
  workpackId: string,
  title: string,
  discoveryState: string,
  proofState: string,
  status: string,
  requiredArtifactSummary: string | null
) {
  return {
    schemaVersion: 'v0.9',
    workpackId,
    title,
    discoveryState,
    proofState,
    runtimeOwner:
      proofState === 'manual-required' || proofState === 'not-implemented' ? 'manual-proof' : 'rust-service-read-model',
    status,
    readModelVisible: true,
    requiredArtifactSummary,
  };
}

function source(
  sourceKind: string,
  workpackId: string,
  status: string,
  authority: string,
  canConfirmChildAgent: boolean,
  canAssignChildProfile: boolean,
  requiredArtifactSummary: string | null
) {
  return {
    schemaVersion: 'v0.9',
    source: sourceKind,
    workpackId,
    status,
    authority,
    runtimePath: status === 'implemented' ? 'rust-service-read-model' : 'manual-artifact',
    uiSurface: status === 'implemented' ? 'devices-lan' : 'proof-report',
    canConfirmChildAgent,
    canAssignChildProfile,
    canControlRoute: canConfirmChildAgent,
    requiresSelectedInterface: true,
    persistsAcrossRestart: canConfirmChildAgent,
    evidenceLabel: sourceKind,
    requiredArtifactSummary,
  };
}

function packetArtifact(): string {
  return 'Attach packet-driver or controlled packet IO proof with selected interface, subnet cap, timing, and malformed packet fixtures.';
}

function mdnsArtifact(): string {
  return 'Attach mDNS/DNS-SD and SSDP/UPnP fixtures or LAN captures with sanitized host/service names.';
}

function signedArtifact(): string {
  return 'Attach signed child-agent hello and heartbeat payloads with nonce, family hash, route id, and replay rejection logs.';
}
