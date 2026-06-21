import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import { AgentNetworkAppleNetworkExtensionGateStatusSchema } from '@ocentra-parent/schema-domain/agent-network-apple-extension-status';
import { parseAgentNetworkAppleNetworkExtensionGateStatusEvent } from '../../src/network-apple-network-extension-gate-status';

const AppleRefs = AgentProtocolDefaults.NetworkAppleNetworkExtensionGateStatus;

describe('network Apple Network Extension gate status contract', () => {
  it('accepts proof-ready Apple gate status and rejects execution/content/enforcement claims', () => {
    const status = fixtureStatus();

    expect(AgentNetworkAppleNetworkExtensionGateStatusSchema.safeParse(status).success).toBe(true);

    expect(
      AgentNetworkAppleNetworkExtensionGateStatusSchema.safeParse({
        ...status,
        liveNetworkExtensionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AgentNetworkAppleNetworkExtensionGateStatusSchema.safeParse({
        ...status,
        gateState: 'manual-required',
      }).success
    ).toBe(false);
  });
});

describe('network Apple Network Extension gate status parser', () => {
  it('parses service event payloads with exact missing/json/schema failure reasons', () => {
    expect(parseAgentNetworkAppleNetworkExtensionGateStatusEvent(eventWithPayload({}))).toEqual({
      ok: false,
      reason: 'missing-apple-network-extension-gate-status',
    });
    expect(
      parseAgentNetworkAppleNetworkExtensionGateStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus]: '{',
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-apple-network-extension-gate-status-json',
    });
    expect(
      parseAgentNetworkAppleNetworkExtensionGateStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus]: JSON.stringify({
            ...fixtureStatus(),
            exactUrlAvailable: true,
          }),
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-apple-network-extension-gate-status',
    });

    const parsed = parseAgentNetworkAppleNetworkExtensionGateStatusEvent(
      eventWithPayload({
        [AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus]: JSON.stringify(fixtureStatus()),
      })
    );

    expect(parsed.ok).toBe(true);
    if (parsed.ok) {
      expect(parsed.status.platform).toBe('ios');
      expect(parsed.status.gateState).toBe('apple-entitlement-proof-ready');
      expect(parsed.status.appleEntitlementProofReady).toBe(true);
      expect(parsed.status.enforcementCommandPublished).toBe(false);
    }
  });

  it('rejects unrelated event names', () => {
    const event = eventWithPayload(
      {
        [AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus]: JSON.stringify(fixtureStatus()),
      },
      AgentEvent.NetworkAndroidVpnServiceGateStatusReported
    );

    expect(parseAgentNetworkAppleNetworkExtensionGateStatusEvent(event)).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
  });
});

function fixtureStatus() {
  return {
    statusRef: AppleRefs.StatusRef,
    appleNetworkExtensionGateRef: AppleRefs.AppleNetworkExtensionGateRef,
    policyDecisionRef: AppleRefs.PolicyDecisionRef,
    parentRuleRef: AppleRefs.ParentRuleRef,
    evidenceRefs: [AppleRefs.EvidenceRef],
    localAiResultRef: AppleRefs.LocalAiResultRef,
    platform: 'ios',
    bundleRef: AppleRefs.BundleRef,
    networkExtensionRef: AppleRefs.NetworkExtensionRef,
    capabilityState: 'apple-device-ready',
    gateState: 'apple-entitlement-proof-ready',
    boundaryReasons: [],
    missingRequiredArtifacts: [],
    developerTeamProofRef: AppleRefs.DeveloperTeamProofRef,
    entitlementApprovalProofRef: AppleRefs.EntitlementApprovalProofRef,
    provisioningProfileProofRef: AppleRefs.ProvisioningProfileProofRef,
    signingProofRef: AppleRefs.SigningProofRef,
    deviceOrTestFlightProofRef: AppleRefs.DeviceOrTestFlightProofRef,
    networkExtensionDeclarationRef: AppleRefs.NetworkExtensionDeclarationRef,
    extensionConfigurationProofRef: AppleRefs.ExtensionConfigurationProofRef,
    rollbackPlanRef: AppleRefs.RollbackPlanRef,
    auditEventRef: AppleRefs.AuditEventRef,
    supervisionRequired: false,
    supervisionOrMdmProofRef: null,
    appleEntitlementProofReady: true,
    supervisionAuthorityProved: false,
    adapterApplyAuthorized: false,
    enforcementCommandPublished: false,
    simulatorOnlyProductSupportClaimed: false,
    liveNetworkExtensionClaimed: false,
    packetBlockClaimed: false,
    appLevelControlClaimed: false,
    exactUrlAvailable: false,
    decryptedPayloadAvailable: false,
    pageContentAvailable: false,
  } as const;
}

function eventWithPayload(
  payload: AgentEventEnvelope['payload'],
  event: AgentEventEnvelope['event'] = AgentEvent.NetworkAppleNetworkExtensionGateStatusReported
): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'network-apple-network-extension-gate-event',
    correlationId: 'network-apple-network-extension-gate-correlation',
    sentAt: '2026-06-14T09:20:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  };
}
