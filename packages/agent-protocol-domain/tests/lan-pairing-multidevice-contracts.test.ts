import { describe, expect, it } from 'vitest';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentLanChildAgentResponseSchema,
  AgentLanPairingAuditEventSchema,
  AgentLanPairingChallengeSchema,
  AgentLanPairingDiscoveryDeviceSchema,
  AgentLanPairingProofPreviewSchema,
  AgentLanParentIntentEnvelopeSchema,
  AgentLanPairingSupportedWebSocketCommand,
  AgentProtocolDefaults,
} from '../src/contracts';
import { AgentLanPairingChallengeRequestSchema } from '../src/lan-pairing-challenge';
import { AgentLanPairingRejectionReasonSchema, AgentPairingStateSchema } from '../src/security';

describe('LAN pairing multi-device protocol contracts', () => {
  it(
    'AgentLanPairingSupportedWebSocketCommand: includes route selection but no HTTP discovery claim',
    assertSupportedWebSocketCommands
  );
  it('AgentPairingStateSchema: distinguishes unauthenticated, unpaired, and paired LAN states', assertPairingStates);
  it('AgentCommandEnvelopeSchema: accepts a route select command for a paired child device', assertRouteSelectCommand);
  it(
    'LAN discovery, challenge, and proof preview schemas describe direct WebSocket ceremony without raw proof material',
    assertDiscoveryChallengeAndProofPreview
  );
  it(
    'AgentLanParentIntentEnvelopeSchema: accepts typed rule, query, and approval intents only',
    assertTypedParentIntentEnvelope
  );
  it(
    'AgentLanChildAgentResponseSchema: represents accepted and rejected child-agent responses without evidence payloads',
    assertChildAgentResponses
  );
  it(
    'AgentLanPairingAuditEventSchema: cites local evidence references without raw evidence payloads',
    assertLanAuditEvidenceReferences
  );
});

function assertSupportedWebSocketCommands() {
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).toEqual([
    'agent.lan-pairing.proof.submit',
    'agent.lan-pairing.route.select',
    'agent.lan-pairing.route.revoke',
    'agent.lan-pairing.status.get',
    'agent.lan-pairing.controller-lease.renew',
    'agent.lan-pairing.controller-lease.release',
    'agent.lan-pairing.controller-lease.takeover',
    'agent.lan-ai.provider.status.get',
    'agent.lan-ai.job.submit',
  ]);
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).not.toContain('agent.lan-pairing.discovery.http');
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).not.toContain('agent.lan-pairing.challenge.http');
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).not.toContain('agent.lan-pairing.proof-preview.http');
}

function assertPairingStates() {
  expect(AgentPairingStateSchema.parse('unauthenticated')).toBe('unauthenticated');
  expect(AgentProtocolDefaults.PairingState.Unpaired).toBe('unpaired');
  expect(AgentProtocolDefaults.PairingState.Paired).toBe('paired');
  expect(AgentProtocolDefaults.PairingState.Revoked).toBe('revoked');
  expect(AgentProtocolDefaults.Field.LanRestartBehavior).toBe('restartBehavior');
  expect(AgentProtocolDefaults.LanSelectedDeviceReachability.Stale).toBe('stale');
  expect(AgentProtocolDefaults.LanSelectedDeviceReachability.Offline).toBe('offline');
  expect(AgentLanPairingRejectionReasonSchema.parse('local-network-disabled')).toBe('local-network-disabled');
  expect(AgentLanPairingRejectionReasonSchema.parse('controller-lease-missing')).toBe('controller-lease-missing');
  expect(AgentLanPairingRejectionReasonSchema.parse('controller-lease-expired')).toBe('controller-lease-expired');
  expect(AgentLanPairingRejectionReasonSchema.parse('wrong-controller')).toBe('wrong-controller');
  expect(AgentProtocolDefaults.Field.LanControllerLeaseId).toBe('controllerLeaseId');
  expect(AgentProtocolDefaults.Field.LanParentAuthority).toBe('parentAuthority');
  expect(AgentProtocolDefaults.Field.LanAiProviderStatus).toBe('lanAiProviderStatus');
  expect(AgentProtocolDefaults.Field.LanAiJobId).toBe('lanAiJobId');
  expect(AgentProtocolDefaults.Field.LanAiJobState).toBe('lanAiJobState');
}

function assertRouteSelectCommand() {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    messageId: 'cmd-lan-route-select-1',
    sentAt: '2026-05-23T21:05:00Z',
    source: AgentProtocolDefaults.Peer.PortalDev,
    target: {
      deviceId: 'child-device-1',
      platform: 'windows',
      route: 'local-network',
    },
    command: AgentCommand.LanPairingRouteSelect,
    payload: {
      [AgentProtocolDefaults.Field.LanIntentId]: 'intent-route-select-1',
      [AgentProtocolDefaults.Field.LanPairingId]: 'pairing-child-1',
      [AgentProtocolDefaults.Field.LanRouteId]: 'lan-route-child-1',
      [AgentProtocolDefaults.Field.LanProofDigest]: 'sha256:proof-child-1',
      [AgentProtocolDefaults.Field.Origin]: 'http://127.0.0.1:4678',
      [AgentProtocolDefaults.Field.StartedAt]: '2026-05-23T21:05:00Z',
      [AgentProtocolDefaults.Field.StaleAt]: '2026-05-23T21:10:00Z',
    },
  });

  expect(parsed.success).toBe(true);
}

function assertDiscoveryChallengeAndProofPreview() {
  const discovery = AgentLanPairingDiscoveryDeviceSchema.safeParse(lanDiscoveryDevice());
  const challengeRequest = AgentLanPairingChallengeRequestSchema.safeParse(lanChallengeRequest());
  const challenge = AgentLanPairingChallengeSchema.safeParse(lanChallenge());
  const preview = AgentLanPairingProofPreviewSchema.safeParse(lanProofPreview('sha256:proof-preview-digest'));
  const invalidPreview = AgentLanPairingProofPreviewSchema.safeParse(lanProofPreview(''));

  expect(discovery.success).toBe(true);
  expect(challengeRequest.success).toBe(true);
  expect(challenge.success).toBe(true);
  expect(preview.success).toBe(true);
  expect(JSON.stringify(preview)).not.toContain('rawToken');
  expect(invalidPreview.success).toBe(false);
}

function assertTypedParentIntentEnvelope() {
  const ruleQuery = AgentLanParentIntentEnvelopeSchema.safeParse(
    lanParentIntentEnvelope('intent-rule-query-1', AgentProtocolDefaults.LanIntentKind.RuleQuery)
  );
  const ruleUpdate = AgentLanParentIntentEnvelopeSchema.safeParse(
    lanParentIntentEnvelope('intent-rule-update-1', AgentProtocolDefaults.LanIntentKind.RuleUpdate)
  );
  const approvalDecision = AgentLanParentIntentEnvelopeSchema.safeParse(
    lanParentIntentEnvelope('intent-approval-decision-1', AgentProtocolDefaults.LanIntentKind.ApprovalDecision)
  );
  const observerProviderStatus = AgentLanParentIntentEnvelopeSchema.safeParse({
    ...lanParentIntentEnvelope(
      'intent-lan-ai-provider-status-1',
      AgentProtocolDefaults.LanIntentKind.LanAiProviderStatus
    ),
    parentAuthority: 'observer',
  });
  const lanAiJobSubmit = AgentLanParentIntentEnvelopeSchema.safeParse(
    lanParentIntentEnvelope('intent-lan-ai-job-submit-1', AgentProtocolDefaults.LanIntentKind.LanAiJobSubmit)
  );
  const missingKind = AgentLanParentIntentEnvelopeSchema.safeParse({
    ...lanParentIntentEnvelope('intent-missing-kind-1', AgentProtocolDefaults.LanIntentKind.RuleQuery),
    [AgentProtocolDefaults.Field.LanIntentKind]: undefined,
  });
  const emptyProof = AgentLanParentIntentEnvelopeSchema.safeParse({
    ...lanParentIntentEnvelope('intent-empty-proof-1', AgentProtocolDefaults.LanIntentKind.RuleQuery),
    [AgentProtocolDefaults.Field.LanProofDigest]: '',
  });
  const invalidEvidence = AgentLanParentIntentEnvelopeSchema.safeParse({
    ...lanParentIntentEnvelope('intent-invalid-evidence-1', AgentProtocolDefaults.LanIntentKind.RuleQuery),
    evidenceReferences: [{ ...lanEvidenceReference(), evidenceReferenceId: '' }],
  });

  expect(ruleQuery.success).toBe(true);
  expect(ruleUpdate.success).toBe(true);
  expect(approvalDecision.success).toBe(true);
  expect(observerProviderStatus.success).toBe(true);
  expect(lanAiJobSubmit.success).toBe(true);
  expect(missingKind.success).toBe(false);
  expect(emptyProof.success).toBe(false);
  expect(invalidEvidence.success).toBe(false);
}

function assertChildAgentResponses() {
  const accepted = AgentLanChildAgentResponseSchema.safeParse(
    lanChildAgentResponse(AgentProtocolDefaults.LanResponseState.Accepted, null)
  );
  const rejected = AgentLanChildAgentResponseSchema.safeParse(
    lanChildAgentResponse(AgentProtocolDefaults.LanResponseState.Rejected, 'wrong-origin')
  );
  const invalidReason = AgentLanChildAgentResponseSchema.safeParse(
    lanChildAgentResponse(AgentProtocolDefaults.LanResponseState.Rejected, 'raw-evidence')
  );

  expect(accepted.success).toBe(true);
  expect(rejected.success).toBe(true);
  expect(JSON.stringify(accepted)).not.toContain('rawEvidence');
  expect(JSON.stringify(rejected)).not.toContain('rawEvidence');
  expect(invalidReason.success).toBe(false);
}

function assertLanAuditEvidenceReferences() {
  const accepted = AgentLanPairingAuditEventSchema.safeParse(lanAuditEvent('control-accepted', null));
  const rejected = AgentLanPairingAuditEventSchema.safeParse(lanAuditEvent('control-rejected', 'wrong-origin'));
  const invalidEvidence = AgentLanPairingAuditEventSchema.safeParse({
    ...lanAuditEvent('control-rejected', 'wrong-origin'),
    evidenceReferences: [{ ...lanEvidenceReference(), kind: 'raw-evidence' }],
  });

  expect(accepted.success).toBe(true);
  expect(rejected.success).toBe(true);
  expect(JSON.stringify(accepted)).toContain('evidenceReferenceId');
  expect(JSON.stringify(accepted)).not.toContain('rawEvidence');
  expect(JSON.stringify(rejected)).not.toContain('rawToken');
  expect(invalidEvidence.success).toBe(false);
}

function lanDeviceRef() {
  return {
    deviceId: 'child-device-1',
    childProfileId: 'child-profile-1',
    label: 'Child Windows device',
    platform: 'windows',
  };
}

function parentDeviceRef() {
  return {
    deviceId: 'parent-device-1',
    childProfileId: null,
    label: 'Parent Windows device',
    platform: 'windows',
  };
}

function lanDiscoveryDevice() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    discoveredAt: '2026-05-23T22:40:00Z',
    childDevice: lanDeviceRef(),
    agentPeerId: 'child-agent-1',
    routeId: 'lan-route-child-1',
    networkMode: 'local-network',
    reachability: AgentProtocolDefaults.LanSelectedDeviceReachability.Stale,
    addressRef: 'lan-address-ref-unproven',
    discoveryStatus: 'websocket-direct',
  };
}

function lanChallengeRequest() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    childDeviceId: 'child-device-1',
    parentDeviceId: 'parent-device-1',
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    issuedAt: '2026-05-23T22:40:00Z',
    expiresAt: '2026-05-23T22:45:00Z',
  };
}

function lanChallenge() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    challengeId: 'challenge-child-1',
    childDevice: lanDeviceRef(),
    parentDevice: parentDeviceRef(),
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    issuedAt: '2026-05-23T22:40:00Z',
    expiresAt: '2026-05-23T22:45:00Z',
    challengeStatus: 'websocket-direct',
  };
}

function lanProofPreview(proofDigest: unknown) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    challengeId: 'challenge-child-1',
    childDeviceId: 'child-device-1',
    parentDeviceId: 'parent-device-1',
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    proofDigest,
    issuedAt: '2026-05-23T22:40:00Z',
    expiresAt: '2026-05-23T22:45:00Z',
    proofPreviewStatus: 'websocket-direct',
  };
}

function lanParentIntentEnvelope(intentId: unknown, intentKind: unknown) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    intentId,
    intentKind,
    targetChildDeviceId: 'child-device-1',
    routeId: 'lan-route-child-1',
    pairingId: 'pairing-child-1',
    proofDigest: 'sha256:proof-child-1',
    origin: 'http://127.0.0.1:4678',
    issuedAt: '2026-05-23T23:20:00Z',
    expiresAt: '2026-05-23T23:25:00Z',
    controllerLeaseId: 'controller-lease-1',
    controllerDeviceId: 'parent-device-1',
    parentActorId: 'parent-actor-1',
    parentAuthority: 'active-controller',
    controllerLeaseIssuedAt: '2026-05-23T23:20:00Z',
    controllerLeaseExpiresAt: '2026-05-23T23:25:00Z',
    evidenceReferences: [lanEvidenceReference()],
  };
}

function lanChildAgentResponse(state: unknown, rejectionReason: unknown) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    intentId: 'intent-rule-query-1',
    targetChildDeviceId: 'child-device-1',
    routeId: 'lan-route-child-1',
    state,
    rejectionReason,
    auditEventId: 'lan-audit-rule-query-1',
    respondedAt: '2026-05-23T23:20:05Z',
  };
}

function lanAuditEvent(eventType: unknown, rejectionReason: unknown) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    auditEventId: 'lan-audit-rule-query-1',
    eventType,
    pairingId: 'pairing-child-1',
    intentId: 'intent-rule-query-1',
    childDeviceId: 'child-device-1',
    parentDeviceId: 'parent-device-1',
    controllerLeaseId: 'controller-lease-1',
    controllerDeviceId: 'parent-device-1',
    parentActorId: 'parent-actor-1',
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    rejectionReason,
    observedAt: '2026-05-23T23:20:05Z',
    evidenceReferences: [lanEvidenceReference()],
  };
}

function lanEvidenceReference() {
  return {
    evidenceReferenceId: 'activity-event-lan-control-1',
    kind: 'activity-event',
    observedAt: '2026-05-23T23:20:05Z',
  };
}
