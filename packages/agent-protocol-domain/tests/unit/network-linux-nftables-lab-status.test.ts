import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import {
  AgentNetworkLinuxNftablesLabStatusSchema,
  parseAgentNetworkLinuxNftablesLabStatusEvent,
} from '../../src/network-linux-nftables-lab-status';

const LabRefs = AgentProtocolDefaults.NetworkLinuxNftablesLabStatus;

describe('network Linux nftables lab status contract', () => {
  it('accepts bounded lab execution status and rejects production/content/enforcement claims', () => {
    const status = fixtureStatus();

    expect(AgentNetworkLinuxNftablesLabStatusSchema.safeParse(status).success).toBe(true);

    const productionClaim = { ...status, productionEnforcementClaimed: true };
    const commandDrift = {
      ...status,
      commandEvidence: status.commandEvidence.map((command, index) =>
        index === 2 ? { ...command, rulePresentAfterCommand: false } : command
      ),
    };

    expect(AgentNetworkLinuxNftablesLabStatusSchema.safeParse(productionClaim).success).toBe(false);
    expect(AgentNetworkLinuxNftablesLabStatusSchema.safeParse(commandDrift).success).toBe(false);
  });

  it('parses service event payloads with exact missing/json/schema failure reasons', () => {
    expect(parseAgentNetworkLinuxNftablesLabStatusEvent(eventWithPayload({}))).toEqual({
      ok: false,
      reason: 'missing-linux-nftables-lab-status',
    });
    expect(
      parseAgentNetworkLinuxNftablesLabStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus]: '{',
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-linux-nftables-lab-status-json',
    });
    expect(
      parseAgentNetworkLinuxNftablesLabStatusEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus]: JSON.stringify({
            ...fixtureStatus(),
            exactUrlAvailable: true,
          }),
        })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-linux-nftables-lab-status',
    });

    const parsed = parseAgentNetworkLinuxNftablesLabStatusEvent(
      eventWithPayload({
        [AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus]: JSON.stringify(fixtureStatus()),
      })
    );

    expect(parsed.ok).toBe(true);
    if (parsed.ok) {
      expect(parsed.status.commandEvidence[5].kind).toBe('verify-table-removed');
      expect(parsed.status.enforcementCommandPublished).toBe(false);
    }
  });

  it('rejects unrelated event names', () => {
    const event = eventWithPayload(
      {
        [AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus]: JSON.stringify(fixtureStatus()),
      },
      AgentEvent.NetworkLiveCaptureStatusReported
    );

    expect(parseAgentNetworkLinuxNftablesLabStatusEvent(event)).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
  });
});

function fixtureStatus() {
  return {
    statusRef: LabRefs.StatusRef,
    labRef: LabRefs.LabRef,
    linuxAdapterGateRef: LabRefs.LinuxAdapterGateRef,
    policyDecisionRef: LabRefs.PolicyDecisionRef,
    parentRuleRef: LabRefs.ParentRuleRef,
    evidenceRefs: [LabRefs.EvidenceRef],
    distroRef: LabRefs.DistroRef,
    kernelRef: LabRefs.KernelRef,
    tableName: LabRefs.TableName,
    chainName: LabRefs.ChainName,
    targetRemoteAddress: LabRefs.TargetRemoteAddress,
    state: 'executed-and-rolled-back',
    wslHostObserved: true,
    rootPermissionObserved: true,
    nftToolObserved: true,
    commandCount: 6,
    requiredCommandCount: 6,
    tableCreateObserved: true,
    chainCreateObserved: true,
    ruleAddObserved: true,
    verifyPresentObserved: true,
    rollbackObserved: true,
    verifyRemovedObserved: true,
    labPacketFilterRuleExecuted: true,
    rollbackVerified: true,
    productionEnforcementClaimed: false,
    persistentRuleClaimed: false,
    genericLinuxSupportClaimed: false,
    serviceManagerInstallClaimed: false,
    exactUrlAvailable: false,
    decryptedPayloadAvailable: false,
    pageContentAvailable: false,
    policyEngineExecutionClaimed: false,
    enforcementCommandPublished: false,
    commandEvidence: [
      command('create-table', LabRefs.CreateTableCommandRef, LabRefs.CreateTableOutputSha256, true, false, false),
      command('create-chain', LabRefs.CreateChainCommandRef, LabRefs.CreateChainOutputSha256, true, true, false),
      command('add-rule', LabRefs.AddRuleCommandRef, LabRefs.AddRuleOutputSha256, true, true, true),
      command('verify-rule-present', LabRefs.VerifyRuleCommandRef, LabRefs.VerifyRuleOutputSha256, true, true, true),
      command('delete-table', LabRefs.DeleteTableCommandRef, LabRefs.DeleteTableOutputSha256, false, false, false),
      command(
        'verify-table-removed',
        LabRefs.VerifyRemovedCommandRef,
        LabRefs.VerifyRemovedOutputSha256,
        false,
        false,
        false
      ),
    ],
  } as const;
}

function command(
  kind: string,
  commandRef: string,
  outputSha256: string,
  tablePresentAfterCommand: boolean,
  chainPresentAfterCommand: boolean,
  rulePresentAfterCommand: boolean
) {
  return {
    kind,
    commandRef,
    exitStatus: 0,
    outputSha256,
    tablePresentAfterCommand,
    chainPresentAfterCommand,
    rulePresentAfterCommand,
  };
}

function eventWithPayload(
  payload: AgentEventEnvelope['payload'],
  event: AgentEventEnvelope['event'] = AgentEvent.NetworkLinuxNftablesLabStatusReported
): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'network-linux-nftables-lab-event',
    correlationId: 'network-linux-nftables-lab-correlation',
    sentAt: '2026-06-08T23:10:00Z',
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
