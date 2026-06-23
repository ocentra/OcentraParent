import { describe, expect, it } from 'vitest';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { AgentLanBrowserAddDeviceReadModelSchema } from '@ocentra-parent/schema-domain/agent-lan-add-device';
import { AgentLanDiscoverySourceMatrixSchema } from '@ocentra-parent/schema-domain/lan-source-matrix';
import { LanPairingSchemaVersion } from '@ocentra-parent/schema-domain/lan-pairing-values';

const generatedAt = '2026-06-02T15:55:00.000Z';

describe('agent protocol LAN discovery source matrix', () => {
  it('parses the source matrix contract', () => {
    const parsed = AgentLanDiscoverySourceMatrixSchema.parse(sourceMatrix());

    expect(parsed.workpackRows).toHaveLength(20);
    expect(parsed.sourceRows.find((row) => row.source === 'windows-neighbor-table')?.canAssignChildProfile).toBe(false);
    expect(parsed.claimsNotProved).toContain(
      'physical household LAN completion remains manual-required until real two-host proof is attached'
    );
  });

  it('carries the matrix in the add-device read model', () => {
    const parsed = AgentLanBrowserAddDeviceReadModelSchema.parse({
      ...addDeviceReadModel(),
      lanDiscoverySourceMatrix: sourceMatrix(),
    });

    expect(parsed.lanDiscoverySourceMatrix?.workpackRows.at(17)?.workpackId).toBe('18');
    expect(
      parsed.lanDiscoverySourceMatrix?.sourceRows.find((row) => row.source === 'mdns-dns-sd-query')
        ?.canConfirmChildAgent
    ).toBe(false);
  });
});

function sourceMatrix() {
  return {
    schemaVersion: LanPairingSchemaVersion.V0_9,
    generatedAt,
    workpackRows: Array.from({ length: 20 }, (_, index) => workpack(index + 1)),
    sourceRows: [
      source('windows-neighbor-table', '04', 'implemented', 'weak-identity', false, false, null),
      source('mdns-dns-sd-query', '08', 'manual-required', 'name-only', false, false, mdnsArtifact()),
      source('ssdp-upnp-query', '09', 'manual-required', 'name-only', false, false, mdnsArtifact()),
      source('signed-child-agent-hello', '18', 'manual-required', 'strong-identity', true, false, signedArtifact()),
      source('signed-child-agent-heartbeat', '18', 'manual-required', 'strong-identity', true, false, signedArtifact()),
    ],
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

function workpack(index: number) {
  const id = index.toString().padStart(2, '0');
  const manual = id === '17' || id === '18';
  return {
    schemaVersion: LanPairingSchemaVersion.V0_9,
    workpackId: id,
    title: `LAN workpack ${id}`,
    discoveryState: manual ? 'manual-required' : 'pending',
    proofState: manual ? 'manual-required' : 'ci-mechanical-proof',
    runtimeOwner: manual ? 'manual-proof' : 'rust-service-read-model',
    status: manual ? 'manual-required' : 'partial',
    readModelVisible: true,
    requiredArtifactSummary: manual ? signedArtifact() : null,
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
    schemaVersion: LanPairingSchemaVersion.V0_9,
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

function mdnsArtifact(): string {
  return 'Attach mDNS/DNS-SD and SSDP/UPnP fixtures or LAN captures with sanitized host/service names.';
}

function signedArtifact(): string {
  return 'Attach signed child-agent hello and heartbeat payloads with nonce, family hash, route id, and replay rejection logs.';
}

function addDeviceReadModel() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    generatedAt,
    discoverySource: 'local-service',
    addDeviceState: 'pending',
    localServiceDiscoveryState: 'pending',
    physicalHouseholdLanState: 'manual-required',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      sourceLabels: ['local-service'],
      scannedDeviceCount: 0,
      agentDeviceCount: 0,
      passiveDeviceCount: 0,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [],
    canonicalHouseholdDevices: [],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    productionHouseholdProof: null,
    signedDiscoveryRelaySpine: null,
    trustedDeviceIds: [],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      selectedChildDeviceId: null,
      routeId: null,
      pairingId: null,
      trustState: 'unpaired',
      reachability: 'offline',
      readyForControl: false,
      staleAt: null,
      offlineAt: null,
    },
    controllerAuthority: 'observer',
    observerAuthority: 'observer',
    routeRequirementLabels: [],
    auditCheckLabels: [],
    honestNonClaims: [],
  } as const;
}
