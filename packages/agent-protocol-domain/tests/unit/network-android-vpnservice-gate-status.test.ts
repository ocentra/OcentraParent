import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import { AgentNetworkAndroidVpnServiceGateStatusSchema } from '@ocentra-parent/schema-domain/agent-network-android-vpnservice-status';
import { parseAgentNetworkAndroidVpnServiceGateStatusEvent } from '../../src/network-android-vpnservice-gate-status';

const AndroidRefs = AgentProtocolDefaults.NetworkAndroidVpnServiceGateStatus;

describe('network Android VpnService gate status contract', () => {
  it('accepts proof-ready Android gate status and rejects execution/content/enforcement claims', () => {
    const status = fixtureStatus();

    expect(AgentNetworkAndroidVpnServiceGateStatusSchema.safeParse(status).success).toBe(true);

    expect(
      AgentNetworkAndroidVpnServiceGateStatusSchema.safeParse({
        ...status,
        liveVpnTunnelClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AgentNetworkAndroidVpnServiceGateStatusSchema.safeParse({
        ...status,
        gateState: 'manual-required',
      }).success
    ).toBe(false);
  });
});

describe('network Android VpnService gate status parser', () => {
  it('parses service event payloads with exact missing/json/schema failure reasons', () => {
    expect(parseAgentNetworkAndroidVpnServiceGateStatusEvent(eventWithPayload({}))).toEqual({
      ok: false,
      reason: 'missing-android-vpn-service-gate-status',
    });
    expect(
      parseAgentNetworkAndroidVpnServiceGateStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus]: '{',
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-android-vpn-service-gate-status-json',
    });
    expect(
      parseAgentNetworkAndroidVpnServiceGateStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus]: JSON.stringify({
            ...fixtureStatus(),
            pageContentAvailable: true,
          }),
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-android-vpn-service-gate-status',
    });

    const parsed = parseAgentNetworkAndroidVpnServiceGateStatusEvent(
      eventWithPayload({
        [AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus]: JSON.stringify(fixtureStatus()),
      })
    );

    expect(parsed.ok).toBe(true);
    if (parsed.ok) {
      expect(parsed.status.gateState).toBe('physical-device-proof-ready');
      expect(parsed.status.physicalDeviceProofReady).toBe(true);
      expect(parsed.status.enforcementCommandPublished).toBe(false);
    }
  });

  it('rejects unrelated event names', () => {
    const event = eventWithPayload(
      {
        [AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus]: JSON.stringify(fixtureStatus()),
      },
      AgentEvent.NetworkWindowsWfpGateStatusReported
    );

    expect(parseAgentNetworkAndroidVpnServiceGateStatusEvent(event)).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
  });
});

function fixtureStatus() {
  return {
    statusRef: AndroidRefs.StatusRef,
    androidVpnServiceGateRef: AndroidRefs.AndroidVpnServiceGateRef,
    policyDecisionRef: AndroidRefs.PolicyDecisionRef,
    parentRuleRef: AndroidRefs.ParentRuleRef,
    evidenceRefs: [AndroidRefs.EvidenceRef],
    localAiResultRef: AndroidRefs.LocalAiResultRef,
    packageRef: AndroidRefs.PackageRef,
    vpnServiceRef: AndroidRefs.VpnServiceRef,
    capabilityState: 'physical-device-ready',
    gateState: 'physical-device-proof-ready',
    boundaryReasons: [],
    missingRequiredArtifacts: [],
    vpnServiceDeclarationRef: AndroidRefs.VpnServiceDeclarationRef,
    userConsentProofRef: AndroidRefs.UserConsentProofRef,
    physicalDeviceProofRef: AndroidRefs.PhysicalDeviceProofRef,
    packageIdentityProofRef: AndroidRefs.PackageIdentityProofRef,
    virtualInterfaceProofRef: AndroidRefs.VirtualInterfaceProofRef,
    trafficObservationProofRef: AndroidRefs.TrafficObservationProofRef,
    rollbackPlanRef: AndroidRefs.RollbackPlanRef,
    auditEventRef: AndroidRefs.AuditEventRef,
    deviceOwnerRequired: false,
    deviceOwnerProofRef: null,
    physicalDeviceProofReady: true,
    deviceOwnerAuthorityProved: false,
    adapterApplyAuthorized: false,
    enforcementCommandPublished: false,
    emulatorOnlyProductSupportClaimed: false,
    liveVpnTunnelClaimed: false,
    packetBlockClaimed: false,
    appPackageCorrelationClaimed: false,
    exactUrlAvailable: false,
    decryptedPayloadAvailable: false,
    pageContentAvailable: false,
  } as const;
}

function eventWithPayload(
  payload: AgentEventEnvelope['payload'],
  event: AgentEventEnvelope['event'] = AgentEvent.NetworkAndroidVpnServiceGateStatusReported
): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'network-android-vpnservice-gate-event',
    correlationId: 'network-android-vpnservice-gate-correlation',
    sentAt: '2026-06-14T09:10:00Z',
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
