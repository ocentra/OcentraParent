import { describe, expect, it } from 'vitest';
import {
  LanChildAgentResponseSchema,
  LanPairingAuditEventSchema,
  LanPairingChallengeSchema,
  LanPairingDiscoveryDeviceSchema,
  LanPairingEnablementSchema,
  LanPairingIntentKindSchema,
  LanPairingParentIntentEnvelopeSchema,
  LanPairingProofSchema,
  LanPairingRejectionReason,
  LanPairingRuntimeSupportSurfaceSchema,
  LanSelectedRouteTargetSchema,
  LanTrustedDeviceRegistryEntrySchema,
} from '../src/lan-pairing';

const timestamp = '2026-05-23T14:40:00.000Z';
const laterTimestamp = '2026-05-23T14:45:00.000Z';
const parentDevice = {
  deviceId: 'parent-device-1',
  childProfileId: null,
  label: 'Parent laptop',
  platform: 'windows',
};
const childProfile = { childProfileId: 'child-profile-1', displayName: 'Sam' };
const childDevice = {
  deviceId: 'child-device-1',
  childProfileId: 'child-profile-1',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const evidenceReference = {
  evidenceReferenceId: 'evidence-1',
  kind: 'activity-event',
  observedAt: timestamp,
};

describe('LAN pairing contracts', () => {
  registerReadinessContractTests();
  registerPairingTrustTests();
  registerControlContractTests();
  registerRejectionContractTests();
  registerRuntimeSupportSurfaceTests();
});

function registerReadinessContractTests(): void {
  it('LanPairingEnablementSchema and discovery contracts: parse explicit LAN readiness', () => {
    const enablement = LanPairingEnablementSchema.parse({
      schemaVersion: 'v0.9',
      state: 'lan-enabled',
      networkMode: 'local-network',
      allowedOrigins: ['http://127.0.0.1:4478'],
      updatedAt: timestamp,
    });
    const discovery = LanPairingDiscoveryDeviceSchema.parse({
      schemaVersion: 'v0.9',
      discoveredAt: timestamp,
      childProfile,
      childDevice,
      agentPeerId: 'agent-peer-1',
      routeId: 'lan-route-1',
      networkMode: 'local-network',
      reachability: 'online',
      addressRef: 'lan-address-ref-1',
    });

    expect(enablement.state).toBe('lan-enabled');
    expect(discovery.reachability).toBe('online');
  });
}

function registerPairingTrustTests(): void {
  it('LanPairingChallengeSchema and registry contracts: parse pairing trust state', () => {
    const challenge = LanPairingChallengeSchema.parse({
      schemaVersion: 'v0.9',
      challengeId: 'challenge-1',
      childDevice,
      parentDevice,
      routeId: 'lan-route-1',
      origin: 'http://127.0.0.1:4478',
      issuedAt: timestamp,
      expiresAt: laterTimestamp,
    });
    const proof = LanPairingProofSchema.parse({
      schemaVersion: 'v0.9',
      pairingId: 'pairing-1',
      challengeId: challenge.challengeId,
      childDeviceId: childDevice.deviceId,
      parentDeviceId: parentDevice.deviceId,
      routeId: challenge.routeId,
      origin: challenge.origin,
      proofDigest: 'sha256:proof-digest',
      issuedAt: timestamp,
      expiresAt: laterTimestamp,
    });
    const registryEntry = LanTrustedDeviceRegistryEntrySchema.parse({
      schemaVersion: 'v0.9',
      pairingId: proof.pairingId,
      childDevice,
      parentDevice,
      routeId: proof.routeId,
      origin: proof.origin,
      proofDigest: proof.proofDigest,
      trustState: 'paired',
      trustedAt: timestamp,
      expiresAt: laterTimestamp,
      revokedAt: null,
    });

    expect(registryEntry.trustState).toBe('paired');
    expect(registryEntry.routeId).toBe(challenge.routeId);
  });
}

function registerControlContractTests(): void {
  it('LanPairingParentIntentEnvelopeSchema: parses selected target, response, and audit contracts', () => {
    const proof = LanPairingProofSchema.parse({
      schemaVersion: 'v0.9',
      pairingId: 'pairing-1',
      challengeId: 'challenge-1',
      childDeviceId: childDevice.deviceId,
      parentDeviceId: parentDevice.deviceId,
      routeId: 'lan-route-1',
      origin: 'http://127.0.0.1:4478',
      proofDigest: 'sha256:proof-digest',
      issuedAt: timestamp,
      expiresAt: laterTimestamp,
    });
    const selectedTarget = LanSelectedRouteTargetSchema.parse({
      schemaVersion: 'v0.9',
      selectedChildDeviceId: childDevice.deviceId,
      routeId: proof.routeId,
      pairingId: proof.pairingId,
      networkMode: 'local-network',
      reachability: 'online',
      staleAt: null,
    });
    const intent = LanPairingParentIntentEnvelopeSchema.parse({
      schemaVersion: 'v0.9',
      intentId: 'intent-1',
      intentKind: 'rule-query',
      targetChildDeviceId: childDevice.deviceId,
      routeId: proof.routeId,
      pairingId: proof.pairingId,
      proofDigest: proof.proofDigest,
      origin: proof.origin,
      issuedAt: timestamp,
      expiresAt: laterTimestamp,
      evidenceReferences: [evidenceReference],
    });
    const response = LanChildAgentResponseSchema.parse({
      schemaVersion: 'v0.9',
      intentId: intent.intentId,
      targetChildDeviceId: intent.targetChildDeviceId,
      routeId: intent.routeId,
      state: 'accepted',
      rejectionReason: null,
      auditEventId: 'audit-1',
      respondedAt: timestamp,
    });
    const auditEvent = LanPairingAuditEventSchema.parse({
      schemaVersion: 'v0.9',
      auditEventId: response.auditEventId,
      eventType: 'control-accepted',
      pairingId: proof.pairingId,
      intentId: intent.intentId,
      childDeviceId: childDevice.deviceId,
      parentDeviceId: parentDevice.deviceId,
      routeId: proof.routeId,
      origin: proof.origin,
      rejectionReason: null,
      observedAt: timestamp,
    });

    expect(selectedTarget.selectedChildDeviceId).toBe('child-device-1');
    expect(intent.intentKind).toBe('rule-query');
    expect(response.state).toBe('accepted');
    expect(auditEvent.eventType).toBe('control-accepted');
  });
}

function registerRejectionContractTests(): void {
  it('LanPairingParentIntentEnvelopeSchema: rejects anonymous or malformed control attempts', () => {
    const parsed = LanPairingParentIntentEnvelopeSchema.safeParse({
      schemaVersion: 'v0.9',
      intentId: '',
      intentKind: 'rule-query',
      targetChildDeviceId: childDevice.deviceId,
      routeId: 'lan-route-1',
      pairingId: '',
      proofDigest: '',
      origin: 'http://127.0.0.1:4478',
      issuedAt: timestamp,
      expiresAt: laterTimestamp,
      evidenceReferences: [],
    });

    expect(parsed.success).toBe(false);
  });

  it('LanPairingRejectionReason: keeps unsafe LAN failures exact and parent-visible', () => {
    expect(LanPairingRejectionReason.Anonymous).toBe('anonymous');
    expect(LanPairingRejectionReason.WrongOrigin).toBe('wrong-origin');
    expect(LanPairingRejectionReason.WrongDevice).toBe('wrong-device');
    expect(LanPairingRejectionReason.Revoked).toBe('revoked');
    expect(LanPairingIntentKindSchema.safeParse('cloud-relay').success).toBe(false);
  });
}

function registerRuntimeSupportSurfaceTests(): void {
  it('LanPairingRuntimeSupportSurfaceSchema: represents the honest WebSocket-only runtime surface', () => {
    const support = LanPairingRuntimeSupportSurfaceSchema.parse({
      schemaVersion: 'v0.9',
      transport: 'websocket',
      supportedWebSocketCommands: ['agent.lan-pairing.proof.submit', 'agent.lan-pairing.status.get'],
      unsupportedHttpEndpoints: [
        {
          endpointId: 'lan-pairing.discovery',
          path: '/api/lan-pairing/discovery',
          support: 'planned-unsupported',
        },
        {
          endpointId: 'lan-pairing.challenge',
          path: '/api/lan-pairing/challenge',
          support: 'planned-unsupported',
        },
        {
          endpointId: 'lan-pairing.proof',
          path: '/api/lan-pairing/proof',
          support: 'planned-unsupported',
        },
        {
          endpointId: 'lan-pairing.control',
          path: '/api/lan-pairing/control',
          support: 'planned-unsupported',
        },
        {
          endpointId: 'lan-pairing.registry',
          path: '/api/lan-pairing/registry',
          support: 'planned-unsupported',
        },
      ],
      pairingState: 'unpaired',
      trustedDeviceCount: 0,
      persistenceMode: 'in-memory-fail-closed',
      proofMode: 'direct-proof-submit',
      routeRequirements: [
        'paired-device',
        'allowed-origin',
        'target-device-match',
        'route-id-match',
        'unexpired-intent',
        'non-replayed-intent',
        'unrevoked-pairing',
      ],
      manualProofGaps: ['manual-lan-bind-proof', 'manual-firewall-proof', 'manual-physical-device-proof'],
    });

    expect(support.supportedWebSocketCommands).toEqual([
      'agent.lan-pairing.proof.submit',
      'agent.lan-pairing.status.get',
    ]);
    expect(support.unsupportedHttpEndpoints.map((endpoint) => endpoint.support)).toEqual([
      'planned-unsupported',
      'planned-unsupported',
      'planned-unsupported',
      'planned-unsupported',
      'planned-unsupported',
    ]);
    expect(support.persistenceMode).toBe('in-memory-fail-closed');
  });
}
