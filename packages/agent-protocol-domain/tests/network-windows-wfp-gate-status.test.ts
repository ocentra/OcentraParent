import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import {
  AgentNetworkWindowsWfpGateStatusSchema,
  parseAgentNetworkWindowsWfpGateStatusEvent,
} from '../src/network-windows-wfp-gate-status';

const WfpRefs = AgentProtocolDefaults.NetworkWindowsWfpGateStatus;

describe('network Windows WFP gate status contract', () => {
  it('accepts lab-ready WFP gate status and rejects execution/content/enforcement claims', () => {
    const status = fixtureStatus();

    expect(AgentNetworkWindowsWfpGateStatusSchema.safeParse(status).success).toBe(true);

    expect(
      AgentNetworkWindowsWfpGateStatusSchema.safeParse({
        ...status,
        packetBlockClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AgentNetworkWindowsWfpGateStatusSchema.safeParse({
        ...status,
        adapterApplyAuthorized: true,
      }).success
    ).toBe(false);
    expect(
      AgentNetworkWindowsWfpGateStatusSchema.safeParse({
        ...status,
        boundaryReasons: ['missing-required-artifact'],
      }).success
    ).toBe(false);
  });
});

describe('network Windows WFP gate status parser', () => {
  it('parses service event payloads with exact missing/json/schema failure reasons', () => {
    expect(parseAgentNetworkWindowsWfpGateStatusEvent(eventWithPayload({}))).toEqual({
      ok: false,
      reason: 'missing-windows-wfp-gate-status',
    });
    expect(
      parseAgentNetworkWindowsWfpGateStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus]: '{',
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-windows-wfp-gate-status-json',
    });
    expect(
      parseAgentNetworkWindowsWfpGateStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus]: JSON.stringify({
            ...fixtureStatus(),
            exactUrlAvailable: true,
          }),
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-windows-wfp-gate-status',
    });

    const parsed = parseAgentNetworkWindowsWfpGateStatusEvent(
      eventWithPayload({
        [AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus]: JSON.stringify(fixtureStatus()),
      })
    );

    expect(parsed.ok).toBe(true);
    if (parsed.ok) {
      expect(parsed.status.wfpLabProofReady).toBe(true);
      expect(parsed.status.enforcementCommandPublished).toBe(false);
    }
  });

  it('rejects unrelated event names', () => {
    const event = eventWithPayload(
      {
        [AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus]: JSON.stringify(fixtureStatus()),
      },
      AgentEvent.NetworkLiveCaptureStatusReported
    );

    expect(parseAgentNetworkWindowsWfpGateStatusEvent(event)).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
  });
});

function fixtureStatus() {
  return {
    statusRef: WfpRefs.StatusRef,
    wfpGateRef: WfpRefs.WfpGateRef,
    policyDecisionRef: WfpRefs.PolicyDecisionRef,
    parentRuleRef: WfpRefs.ParentRuleRef,
    evidenceRefs: [WfpRefs.EvidenceRef],
    localAiResultRef: WfpRefs.LocalAiResultRef,
    targetRef: WfpRefs.TargetRef,
    wfpProviderRef: WfpRefs.WfpProviderRef,
    wfpLayerRef: WfpRefs.WfpLayerRef,
    capabilityState: 'lab-ready',
    gateState: 'lab-proof-ready',
    boundaryReasons: [],
    missingRequiredArtifacts: [],
    administratorPermissionProofRef: WfpRefs.AdministratorPermissionProofRef,
    driverSigningProofRef: WfpRefs.DriverSigningProofRef,
    driverPackageProofRef: WfpRefs.DriverPackageProofRef,
    providerRegistrationPlanRef: WfpRefs.ProviderRegistrationPlanRef,
    layerCapabilityMatrixRef: WfpRefs.LayerCapabilityMatrixRef,
    rollbackPlanRef: WfpRefs.RollbackPlanRef,
    labResultArtifactRef: WfpRefs.LabResultArtifactRef,
    auditEventRef: WfpRefs.AuditEventRef,
    wfpLabProofReady: true,
    adapterApplyAuthorized: false,
    enforcementCommandPublished: false,
    liveDriverInstallClaimed: false,
    calloutRegistrationClaimed: false,
    packetBlockClaimed: false,
    kernelPayloadInspectionClaimed: false,
    commandInvocationClaimed: false,
    exactUrlAvailable: false,
    decryptedPayloadAvailable: false,
    pageContentAvailable: false,
  } as const;
}

function eventWithPayload(
  payload: AgentEventEnvelope['payload'],
  event: AgentEventEnvelope['event'] = AgentEvent.NetworkWindowsWfpGateStatusReported
): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'network-windows-wfp-gate-event',
    correlationId: 'network-windows-wfp-gate-correlation',
    sentAt: '2026-06-08T23:50:00Z',
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
