import { describe, expect, it } from 'vitest';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { AgentNetworkWindowsFirewallLabStatusSchema } from '@ocentra-parent/schema-domain/agent-network-windows-firewall-status';
import { parseAgentNetworkWindowsFirewallLabStatusEvent } from '../../src/network-windows-firewall-lab-status';

const LabRefs = AgentProtocolDefaults.NetworkWindowsFirewallLabStatus;

describe('network Windows firewall lab status contract', () => {
  it('accepts bounded lab execution status and rejects production/content/enforcement claims', () => {
    const status = fixtureStatus();

    expect(AgentNetworkWindowsFirewallLabStatusSchema.safeParse(status).success).toBe(true);

    const productionClaim = { ...status, productionEnforcementClaimed: true };
    const commandDrift = {
      ...status,
      commandEvidence: status.commandEvidence.map((command, index) =>
        index === 3 ? { ...command, rulePresentAfterCommand: true } : command
      ),
    };

    expect(AgentNetworkWindowsFirewallLabStatusSchema.safeParse(productionClaim).success).toBe(false);
    expect(AgentNetworkWindowsFirewallLabStatusSchema.safeParse(commandDrift).success).toBe(false);
  });

  it('parses service event payloads with exact missing/json/schema failure reasons', () => {
    expect(parseAgentNetworkWindowsFirewallLabStatusEvent(eventWithPayload({}))).toEqual({
      ok: false,
      reason: 'missing-windows-firewall-lab-status',
    });
    expect(
      parseAgentNetworkWindowsFirewallLabStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus]: '{',
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-windows-firewall-lab-status-json',
    });
    expect(
      parseAgentNetworkWindowsFirewallLabStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus]: JSON.stringify({
            ...fixtureStatus(),
            exactUrlAvailable: true,
          }),
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-windows-firewall-lab-status',
    });

    const parsed = parseAgentNetworkWindowsFirewallLabStatusEvent(
      eventWithPayload({
        [AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus]: JSON.stringify(fixtureStatus()),
      })
    );

    expect(parsed.ok).toBe(true);
    if (parsed.ok) {
      expect(parsed.status.commandEvidence[3].kind).toBe('verify-rule-removed');
      expect(parsed.status.enforcementCommandPublished).toBe(false);
    }
  });

  it('rejects unrelated event names', () => {
    const event = eventWithPayload(
      {
        [AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus]: JSON.stringify(fixtureStatus()),
      },
      AgentEvent.NetworkLiveCaptureStatusReported
    );

    expect(parseAgentNetworkWindowsFirewallLabStatusEvent(event)).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
  });
});

function fixtureStatus() {
  return {
    statusRef: LabRefs.StatusRef,
    labRef: LabRefs.LabRef,
    firewallAdapterPlanRef: LabRefs.FirewallAdapterPlanRef,
    policyDecisionRef: LabRefs.PolicyDecisionRef,
    parentRuleRef: LabRefs.ParentRuleRef,
    evidenceRefs: [LabRefs.EvidenceRef],
    windowsOsScopeRef: LabRefs.WindowsOsScopeRef,
    targetRef: LabRefs.TargetRef,
    firewallRuleRef: LabRefs.FirewallRuleRef,
    ruleName: LabRefs.RuleName,
    targetRemoteAddress: LabRefs.TargetRemoteAddress,
    state: 'executed-and-rolled-back',
    windowsHostObserved: true,
    administratorPermissionObserved: true,
    commandCount: 4,
    requiredCommandCount: 4,
    applyCommandObserved: true,
    verifyPresentObserved: true,
    rollbackCommandObserved: true,
    verifyRemovedObserved: true,
    labFirewallMutationExecuted: true,
    rollbackVerified: true,
    adapterApplyAuthorized: true,
    productionEnforcementClaimed: false,
    persistentRuleClaimed: false,
    exactUrlAvailable: false,
    decryptedPayloadAvailable: false,
    pageContentAvailable: false,
    hostFirewallMutationClaimed: false,
    netshCommandInvoked: false,
    powershellCommandInvoked: false,
    policyEngineExecutionClaimed: false,
    enforcementCommandPublished: false,
    commandEvidence: [
      command('apply-rule', LabRefs.ApplyRuleCommandRef, LabRefs.ApplyRuleOutputSha256, true),
      command('verify-rule-present', LabRefs.VerifyPresentCommandRef, LabRefs.VerifyPresentOutputSha256, true),
      command('rollback-rule', LabRefs.RollbackRuleCommandRef, LabRefs.RollbackRuleOutputSha256, false),
      command('verify-rule-removed', LabRefs.VerifyRemovedCommandRef, LabRefs.VerifyRemovedOutputSha256, false),
    ],
  } as const;
}

function command(kind: string, commandRef: string, outputSha256: string, rulePresentAfterCommand: boolean) {
  return {
    kind,
    commandRef,
    exitStatus: 0,
    outputSha256,
    rulePresentAfterCommand,
  };
}

function eventWithPayload(
  payload: AgentEventEnvelope['payload'],
  event: AgentEventEnvelope['event'] = AgentEvent.NetworkWindowsFirewallLabStatusReported
): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'network-windows-firewall-lab-event',
    correlationId: 'network-windows-firewall-lab-correlation',
    sentAt: '2026-06-08T23:40:00Z',
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
