import { describe, expect, it } from 'vitest';
import {
  LanChildAgentResponseSchema,
  LanPairingAuditEventSchema,
  LanPairingParentIntentEnvelopeSchema,
} from '@ocentra-parent/schema-domain/lan-pairing-control';
import {
  LanPairingChallengeRequestSchema,
  LanPairingChallengeSchema,
  LanPairingDiscoveryDeviceSchema,
  LanPairingEnablementSchema,
  LanPairingProofPreviewSchema,
  LanPairingProofSchema,
  LanSelectedRouteTargetSchema,
  LanTrustedDeviceRegistryEntrySchema,
} from '@ocentra-parent/schema-domain/lan-pairing-device';
import { LanPairingRuntimeSupportSurfaceSchema } from '@ocentra-parent/schema-domain/lan-pairing-support';
import { LanPairingIntentKindSchema, LanPairingRejectionReason } from '@ocentra-parent/schema-domain/lan-pairing-values';

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
const controllerLease = {
  controllerLeaseId: 'controller-lease-1',
  controllerDeviceId: parentDevice.deviceId,
  parentActorId: 'parent-actor-1',
  parentAuthority: 'active-controller',
  controllerLeaseIssuedAt: timestamp,
  controllerLeaseExpiresAt: laterTimestamp,
};
const sensitiveLanPrivacyMarkers = [
  'activityDigest',
  'activity.sqlite',
  'activity.ndjson',
  'decryptedEvidence',
  'journalPath',
  'rawEvidence',
  'rawProofSecret',
  'rawToken',
  'sqlitePath',
  'controlAuthority',
] as const;
const unsupportedLanHttpEndpoints = [
  { endpointId: 'lan-pairing.discovery', path: '/api/lan-pairing/discovery', support: 'planned-unsupported' },
  { endpointId: 'lan-pairing.challenge', path: '/api/lan-pairing/challenge', support: 'planned-unsupported' },
  { endpointId: 'lan-pairing.proof', path: '/api/lan-pairing/proof', support: 'planned-unsupported' },
  { endpointId: 'lan-pairing.control', path: '/api/lan-pairing/control', support: 'planned-unsupported' },
  { endpointId: 'lan-pairing.registry', path: '/api/lan-pairing/registry', support: 'planned-unsupported' },
] as const;
const lanRouteRequirements = [
  'paired-device',
  'allowed-origin',
  'target-device-match',
  'route-id-match',
  'unexpired-intent',
  'non-replayed-intent',
  'unrevoked-pairing',
  'active-controller-lease',
  'selected-device-reachable',
  'parent-write-authority',
  'lan-ai-job-authorized',
  'discovery-state-explicit',
  'route-recovery-persisted',
] as const;

describe('LAN pairing contracts', () => {
  registerReadinessContractTests();
  registerPairingTrustTests();
  registerControlContractTests();
  registerAuthorityAndLanAiContractTests();
  registerRejectionContractTests();
  registerRuntimeSupportSurfaceTests();
  registerPersistentRuntimeSupportSurfaceTests();
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
      discoveryStatus: 'websocket-direct',
      discoveryState: 'discovered',
    });

    expect(enablement.state).toBe('lan-enabled');
    expect(discovery.reachability).toBe('online');
    expect(discovery.discoveryStatus).toBe('websocket-direct');
    expect(discovery.discoveryState).toBe('discovered');
    expect(Object.keys(discovery).sort()).toEqual(
      [
        'addressRef',
        'agentPeerId',
        'childDevice',
        'childProfile',
        'discoveredAt',
        'discoveryStatus',
        'discoveryState',
        'networkMode',
        'reachability',
        'routeId',
        'schemaVersion',
      ].sort()
    );
    expectNoSensitiveLanPrivacyMarkers(discovery);
  });
}

function registerPairingTrustTests(): void {
  it('LanPairingChallengeSchema and registry contracts: parse pairing trust state', () => {
    const challenge = parsedChallenge();
    const challengeRequest = LanPairingChallengeRequestSchema.parse({
      schemaVersion: 'v0.9',
      childDeviceId: childDevice.deviceId,
      parentDeviceId: parentDevice.deviceId,
      routeId: challenge.routeId,
      origin: challenge.origin,
      issuedAt: timestamp,
      expiresAt: laterTimestamp,
    });
    const proof = parsedProofFor(challenge);
    const registryEntry = parsedRegistryEntryFor(proof);

    expect(challengeRequest.parentDeviceId).toBe(parentDevice.deviceId);
    expect(registryEntry.trustState).toBe('paired');
    expect(registryEntry.routeId).toBe(challenge.routeId);
    expect(challenge.challengeStatus).toBe('websocket-direct');
    expect(Object.keys(challenge).sort()).toEqual(
      [
        'challengeId',
        'challengeStatus',
        'childDevice',
        'expiresAt',
        'issuedAt',
        'origin',
        'parentDevice',
        'routeId',
        'schemaVersion',
      ].sort()
    );
    expect(Object.keys(proof).sort()).toEqual(
      [
        'challengeId',
        'childDeviceId',
        'expiresAt',
        'issuedAt',
        'origin',
        'pairingId',
        'parentDeviceId',
        'proofDigest',
        'routeId',
        'schemaVersion',
      ].sort()
    );
    expectNoSensitiveLanPrivacyMarkers(challenge);
    expectNoSensitiveLanPrivacyMarkers(proof);
  });

  it('LanPairingProofPreviewSchema: records direct WebSocket proof preview state', () => {
    const challenge = parsedChallenge();
    const preview = LanPairingProofPreviewSchema.parse({
      schemaVersion: 'v0.9',
      challengeId: challenge.challengeId,
      childDeviceId: childDevice.deviceId,
      parentDeviceId: parentDevice.deviceId,
      routeId: challenge.routeId,
      origin: challenge.origin,
      proofDigest: 'sha256:proof-preview-digest',
      issuedAt: timestamp,
      expiresAt: laterTimestamp,
      proofPreviewStatus: 'websocket-direct',
    });

    expect(preview.proofPreviewStatus).toBe('websocket-direct');
    expectNoSensitiveLanPrivacyMarkers(preview);
  });
}

function parsedChallenge() {
  return LanPairingChallengeSchema.parse({
    schemaVersion: 'v0.9',
    challengeId: 'challenge-1',
    childDevice,
    parentDevice,
    routeId: 'lan-route-1',
    origin: 'http://127.0.0.1:4478',
    issuedAt: timestamp,
    expiresAt: laterTimestamp,
    challengeStatus: 'websocket-direct',
  });
}

function parsedProofFor(challenge: ReturnType<typeof parsedChallenge>) {
  return LanPairingProofSchema.parse({
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
}

function parsedRegistryEntryFor(proof: ReturnType<typeof parsedProofFor>) {
  return LanTrustedDeviceRegistryEntrySchema.parse({
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
}

function registerControlContractTests(): void {
  it('LanPairingParentIntentEnvelopeSchema: parses selected target, response, and audit contracts', () => {
    const { auditEvent, intent, response, routeSelectedAuditEvent, selectedTarget } = acceptedControlContracts();

    expect(selectedTarget.selectedChildDeviceId).toBe('child-device-1');
    expect(selectedTarget.trustState).toBe('paired');
    expect(selectedTarget.offlineAt).toBeNull();
    expect(intent.intentKind).toBe('rule-query');
    expect(response.state).toBe('accepted');
    expect(auditEvent.eventType).toBe('control-accepted');
    expect(routeSelectedAuditEvent.eventType).toBe('route-selected');
    expect(auditEvent.evidenceReferences).toEqual([evidenceReference]);
  });
}

function registerAuthorityAndLanAiContractTests(): void {
  it('LanPairingParentIntentEnvelopeSchema: distinguishes active controller and observer authority', () => {
    const proof = acceptedProof();
    const observerRuleQuery = LanPairingParentIntentEnvelopeSchema.parse({
      ...parentIntentFor(proof),
      intentId: 'intent-observer-rule-query',
      parentAuthority: 'observer',
    });
    const observerRuleUpdate = LanPairingParentIntentEnvelopeSchema.parse({
      ...parentIntentFor(proof),
      intentId: 'intent-observer-rule-update',
      intentKind: 'rule-update',
      parentAuthority: 'observer',
    });
    const observerRejected = LanChildAgentResponseSchema.parse({
      schemaVersion: 'v0.9',
      intentId: observerRuleUpdate.intentId,
      targetChildDeviceId: observerRuleUpdate.targetChildDeviceId,
      routeId: observerRuleUpdate.routeId,
      state: 'rejected',
      rejectionReason: 'observer-read-only',
      auditEventId: 'audit-observer-read-only',
      respondedAt: timestamp,
    });

    expect(observerRuleQuery.parentAuthority).toBe('observer');
    expect(observerRuleUpdate.intentKind).toBe('rule-update');
    expect(observerRejected.rejectionReason).toBe('observer-read-only');
  });

  it('LanPairingParentIntentEnvelopeSchema: covers lease lifecycle and LAN AI job intents', () => {
    const proof = acceptedProof();
    const leaseRenew = LanPairingParentIntentEnvelopeSchema.parse({
      ...parentIntentFor(proof),
      intentId: 'intent-lease-renew',
      intentKind: 'controller-lease-renew',
    });
    const leaseRelease = LanPairingParentIntentEnvelopeSchema.parse({
      ...parentIntentFor(proof),
      intentId: 'intent-lease-release',
      intentKind: 'controller-lease-release',
    });
    const leaseTakeover = LanPairingParentIntentEnvelopeSchema.parse({
      ...parentIntentFor(proof),
      intentId: 'intent-lease-takeover',
      intentKind: 'controller-lease-takeover',
    });
    const providerStatus = LanPairingParentIntentEnvelopeSchema.parse({
      ...parentIntentFor(proof),
      intentId: 'intent-lan-ai-provider-status',
      intentKind: 'lan-ai-provider-status',
      parentAuthority: 'observer',
    });
    const jobSubmit = LanPairingParentIntentEnvelopeSchema.parse({
      ...parentIntentFor(proof),
      intentId: 'intent-lan-ai-job-submit',
      intentKind: 'lan-ai-job-submit',
    });
    const jobDegraded = LanChildAgentResponseSchema.parse({
      schemaVersion: 'v0.9',
      intentId: jobSubmit.intentId,
      targetChildDeviceId: jobSubmit.targetChildDeviceId,
      routeId: jobSubmit.routeId,
      state: 'degraded',
      rejectionReason: 'lan-ai-provider-unavailable',
      auditEventId: 'audit-lan-ai-job-degraded',
      respondedAt: timestamp,
    });

    expect(leaseRenew.intentKind).toBe('controller-lease-renew');
    expect(leaseRelease.intentKind).toBe('controller-lease-release');
    expect(leaseTakeover.intentKind).toBe('controller-lease-takeover');
    expect(providerStatus.intentKind).toBe('lan-ai-provider-status');
    expect(jobDegraded.state).toBe('degraded');
    expect(jobDegraded.rejectionReason).toBe('lan-ai-provider-unavailable');
  });
}

function acceptedControlContracts() {
  const proof = acceptedProof();
  const selectedTarget = selectedRouteTargetFor(proof);
  const intent = parentIntentFor(proof);
  const response = acceptedResponseFor(intent);
  const auditEvent = acceptedAuditEventFor(proof, intent, response);
  const routeSelectedAuditEvent = routeSelectedAuditEventFor(proof, intent);

  return { auditEvent, intent, response, routeSelectedAuditEvent, selectedTarget };
}

function acceptedProof() {
  return LanPairingProofSchema.parse({
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
}

function selectedRouteTargetFor(proof: ReturnType<typeof acceptedProof>) {
  return LanSelectedRouteTargetSchema.parse({
    schemaVersion: 'v0.9',
    selectedChildDeviceId: childDevice.deviceId,
    routeId: proof.routeId,
    pairingId: proof.pairingId,
    trustState: 'paired',
    networkMode: 'local-network',
    reachability: 'online',
    staleAt: null,
    offlineAt: null,
  });
}

function parentIntentFor(proof: ReturnType<typeof acceptedProof>) {
  return LanPairingParentIntentEnvelopeSchema.parse({
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
    ...controllerLease,
    evidenceReferences: [evidenceReference],
  });
}

function acceptedResponseFor(intent: ReturnType<typeof parentIntentFor>) {
  return LanChildAgentResponseSchema.parse({
    schemaVersion: 'v0.9',
    intentId: intent.intentId,
    targetChildDeviceId: intent.targetChildDeviceId,
    routeId: intent.routeId,
    state: 'accepted',
    rejectionReason: null,
    auditEventId: 'audit-1',
    respondedAt: timestamp,
  });
}

function acceptedAuditEventFor(
  proof: ReturnType<typeof acceptedProof>,
  intent: ReturnType<typeof parentIntentFor>,
  response: ReturnType<typeof acceptedResponseFor>
) {
  return LanPairingAuditEventSchema.parse({
    schemaVersion: 'v0.9',
    auditEventId: response.auditEventId,
    eventType: 'control-accepted',
    pairingId: proof.pairingId,
    intentId: intent.intentId,
    childDeviceId: childDevice.deviceId,
    parentDeviceId: parentDevice.deviceId,
    controllerLeaseId: intent.controllerLeaseId,
    controllerDeviceId: intent.controllerDeviceId,
    parentActorId: intent.parentActorId,
    routeId: proof.routeId,
    origin: proof.origin,
    rejectionReason: null,
    observedAt: timestamp,
    evidenceReferences: [evidenceReference],
  });
}

function routeSelectedAuditEventFor(
  proof: ReturnType<typeof acceptedProof>,
  intent: ReturnType<typeof parentIntentFor>
) {
  return LanPairingAuditEventSchema.parse({
    schemaVersion: 'v0.9',
    auditEventId: 'audit-route-selected-1',
    eventType: 'route-selected',
    pairingId: proof.pairingId,
    intentId: intent.intentId,
    childDeviceId: childDevice.deviceId,
    parentDeviceId: parentDevice.deviceId,
    controllerLeaseId: intent.controllerLeaseId,
    controllerDeviceId: intent.controllerDeviceId,
    parentActorId: intent.parentActorId,
    routeId: proof.routeId,
    origin: proof.origin,
    rejectionReason: null,
    observedAt: timestamp,
    evidenceReferences: [evidenceReference],
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
      ...controllerLease,
      evidenceReferences: [],
    });

    expect(parsed.success).toBe(false);
  });

  it('LanChildAgentResponseSchema: parses unselected trusted-device rejection with audit evidence', () => {
    const response = LanChildAgentResponseSchema.parse({
      schemaVersion: 'v0.9',
      intentId: 'intent-unselected-1',
      targetChildDeviceId: childDevice.deviceId,
      routeId: 'lan-route-1',
      state: 'rejected',
      rejectionReason: 'unselected-device',
      auditEventId: 'audit-unselected-1',
      respondedAt: timestamp,
    });
    const auditEvent = LanPairingAuditEventSchema.parse({
      schemaVersion: 'v0.9',
      auditEventId: response.auditEventId,
      eventType: 'control-rejected',
      pairingId: 'pairing-1',
      intentId: response.intentId,
      childDeviceId: childDevice.deviceId,
      parentDeviceId: parentDevice.deviceId,
      controllerLeaseId: 'controller-lease-1',
      controllerDeviceId: parentDevice.deviceId,
      parentActorId: 'parent-actor-1',
      routeId: response.routeId,
      origin: 'http://127.0.0.1:4478',
      rejectionReason: response.rejectionReason,
      observedAt: timestamp,
      evidenceReferences: [evidenceReference],
    });

    expect(response.state).toBe('rejected');
    expect(response.rejectionReason).toBe('unselected-device');
    expect(auditEvent.rejectionReason).toBe('unselected-device');
    expect(auditEvent.evidenceReferences).toEqual([evidenceReference]);
  });

  it('LanPairingRejectionReason: keeps unsafe LAN failures exact and parent-visible', () => {
    expect(LanPairingRejectionReason.Anonymous).toBe('anonymous');
    expect(LanPairingRejectionReason.WrongOrigin).toBe('wrong-origin');
    expect(LanPairingRejectionReason.WrongDevice).toBe('wrong-device');
    expect(LanPairingRejectionReason.Offline).toBe('offline');
    expect(LanPairingRejectionReason.Revoked).toBe('revoked');
    expect(LanPairingRejectionReason.UnselectedDevice).toBe('unselected-device');
    expect(LanPairingRejectionReason.ControllerLeaseMissing).toBe('controller-lease-missing');
    expect(LanPairingRejectionReason.ControllerLeaseExpired).toBe('controller-lease-expired');
    expect(LanPairingRejectionReason.WrongController).toBe('wrong-controller');
    expect(LanPairingIntentKindSchema.safeParse('cloud-relay').success).toBe(false);
  });
}

function registerRuntimeSupportSurfaceTests(): void {
  it('LanPairingRuntimeSupportSurfaceSchema: represents the honest WebSocket-only runtime surface', () => {
    const support = LanPairingRuntimeSupportSurfaceSchema.parse({
      schemaVersion: 'v0.9',
      transport: 'websocket',
      supportedWebSocketCommands: ['agent.lan-pairing.proof.submit', 'agent.lan-pairing.status.get'],
      unsupportedHttpEndpoints: unsupportedLanHttpEndpoints,
      pairingState: 'unpaired',
      trustedDeviceCount: 0,
      discoveryStatus: 'websocket-direct',
      discoveryState: 'discovered',
      challengeStatus: 'websocket-direct',
      proofPreviewStatus: 'websocket-direct',
      lanAiProviderStatus: 'websocket-direct',
      lanAiProviderRoutingState: 'unavailable',
      lanAiProviderCustodyLabel: 'local-network-ai-provider',
      lanAiJobStatus: 'websocket-direct',
      persistenceMode: 'in-memory-fail-closed',
      restartBehavior: 'fail-closed-unpaired',
      proofMode: 'direct-proof-submit',
      routeRequirements: lanRouteRequirements,
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
    expect(support.restartBehavior).toBe('fail-closed-unpaired');
    expect(support.discoveryStatus).toBe('websocket-direct');
    expect(support.discoveryState).toBe('discovered');
    expect(support.challengeStatus).toBe('websocket-direct');
    expect(support.proofPreviewStatus).toBe('websocket-direct');
    expectNoSensitiveLanPrivacyMarkers(support);
  });
}

function registerPersistentRuntimeSupportSurfaceTests(): void {
  it('LanPairingRuntimeSupportSurfaceSchema: represents explicit local registry persistence', () => {
    const support = LanPairingRuntimeSupportSurfaceSchema.parse({
      schemaVersion: 'v0.9',
      transport: 'websocket',
      supportedWebSocketCommands: ['agent.lan-pairing.proof.submit', 'agent.lan-pairing.status.get'],
      unsupportedHttpEndpoints: [],
      pairingState: 'paired',
      trustedDeviceCount: 1,
      discoveryStatus: 'websocket-direct',
      discoveryState: 'paired',
      challengeStatus: 'websocket-direct',
      proofPreviewStatus: 'websocket-direct',
      lanAiProviderStatus: 'websocket-direct',
      lanAiProviderRoutingState: 'authorized-result',
      lanAiProviderCustodyLabel: 'local-network-ai-provider',
      lanAiJobStatus: 'websocket-direct',
      persistenceMode: 'local-json-registry',
      restartBehavior: 'restore-trusted-registry-selected-route',
      proofMode: 'direct-proof-submit',
      routeRequirements: ['paired-device', 'allowed-origin', 'selected-device-reachable', 'route-recovery-persisted'],
      manualProofGaps: ['manual-lan-bind-proof'],
    });

    expect(support.persistenceMode).toBe('local-json-registry');
    expect(support.restartBehavior).toBe('restore-trusted-registry-selected-route');
    expect(support.trustedDeviceCount).toBe(1);
    expectNoSensitiveLanPrivacyMarkers(support);
  });
}

function expectNoSensitiveLanPrivacyMarkers(value: unknown): void {
  const serialized = JSON.stringify(value);
  for (const marker of sensitiveLanPrivacyMarkers) {
    expect(serialized).not.toContain(marker);
  }
}
