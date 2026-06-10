import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const WindowsFirewallText = Schema.String.pipe(Schema.minLength(1));
const WindowsFirewallCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const WindowsFirewallRefs = AgentProtocolDefaults.NetworkWindowsFirewallLabStatus;

export const AgentNetworkWindowsFirewallLabStateSchema = withParser(
  Schema.Literal('executed-and-rolled-back', 'manual-required', 'unavailable')
);

export const AgentNetworkWindowsFirewallLabCommandKindSchema = withParser(
  Schema.Literal('apply-rule', 'verify-rule-present', 'rollback-rule', 'verify-rule-removed')
);

const AgentNetworkWindowsFirewallLabCommandRowSchema = Schema.Struct({
  kind: AgentNetworkWindowsFirewallLabCommandKindSchema,
  commandRef: WindowsFirewallText,
  exitStatus: Schema.Number.pipe(Schema.int()),
  outputSha256: WindowsFirewallText,
  rulePresentAfterCommand: Schema.Boolean,
});

const AgentNetworkWindowsFirewallLabStatusFields = Schema.Struct({
  statusRef: WindowsFirewallText,
  labRef: WindowsFirewallText,
  firewallAdapterPlanRef: WindowsFirewallText,
  policyDecisionRef: WindowsFirewallText,
  parentRuleRef: WindowsFirewallText,
  evidenceRefs: Schema.Array(WindowsFirewallText),
  windowsOsScopeRef: WindowsFirewallText,
  targetRef: WindowsFirewallText,
  firewallRuleRef: WindowsFirewallText,
  ruleName: WindowsFirewallText,
  targetRemoteAddress: WindowsFirewallText,
  state: AgentNetworkWindowsFirewallLabStateSchema,
  windowsHostObserved: Schema.Boolean,
  administratorPermissionObserved: Schema.Boolean,
  commandCount: WindowsFirewallCount,
  requiredCommandCount: WindowsFirewallCount,
  applyCommandObserved: Schema.Boolean,
  verifyPresentObserved: Schema.Boolean,
  rollbackCommandObserved: Schema.Boolean,
  verifyRemovedObserved: Schema.Boolean,
  labFirewallMutationExecuted: Schema.Boolean,
  rollbackVerified: Schema.Boolean,
  adapterApplyAuthorized: Schema.Boolean,
  productionEnforcementClaimed: Schema.Literal(false),
  persistentRuleClaimed: Schema.Literal(false),
  exactUrlAvailable: Schema.Literal(false),
  decryptedPayloadAvailable: Schema.Literal(false),
  pageContentAvailable: Schema.Literal(false),
  hostFirewallMutationClaimed: Schema.Literal(false),
  netshCommandInvoked: Schema.Literal(false),
  powershellCommandInvoked: Schema.Literal(false),
  policyEngineExecutionClaimed: Schema.Literal(false),
  enforcementCommandPublished: Schema.Literal(false),
  commandEvidence: Schema.Array(AgentNetworkWindowsFirewallLabCommandRowSchema),
});

export type AgentNetworkWindowsFirewallLabCommandRow = Infer<typeof AgentNetworkWindowsFirewallLabCommandRowSchema>;
export type AgentNetworkWindowsFirewallLabStatus = Infer<typeof AgentNetworkWindowsFirewallLabStatusFields>;

export const AgentNetworkWindowsFirewallLabStatusSchema = withParser(
  AgentNetworkWindowsFirewallLabStatusFields.pipe(
    Schema.filter(
      (status: AgentNetworkWindowsFirewallLabStatus) =>
        (refsMatch(status) &&
          executionShapeMatches(status) &&
          commandCountsMatch(status) &&
          commandEvidenceMatches(status)) ||
        'Network Windows firewall lab status must preserve row38a refs, four command rows, rollback verification, and no production/content/enforcement claims'
    )
  )
);

export type AgentNetworkWindowsFirewallLabStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkWindowsFirewallLabStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-windows-firewall-lab-status'
        | 'invalid-windows-firewall-lab-status-json'
        | 'invalid-windows-firewall-lab-status';
    };

export function parseAgentNetworkWindowsFirewallLabStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkWindowsFirewallLabStatusParseResult {
  if (event.event !== AgentEvent.NetworkWindowsFirewallLabStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-windows-firewall-lab-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-windows-firewall-lab-status-json' };
  }

  const parsed = AgentNetworkWindowsFirewallLabStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-windows-firewall-lab-status' };
  }

  return { ok: true, status: parsed.data };
}

function refsMatch(status: AgentNetworkWindowsFirewallLabStatus): boolean {
  return (
    status.statusRef === WindowsFirewallRefs.StatusRef &&
    status.labRef === WindowsFirewallRefs.LabRef &&
    status.firewallAdapterPlanRef === WindowsFirewallRefs.FirewallAdapterPlanRef &&
    status.policyDecisionRef === WindowsFirewallRefs.PolicyDecisionRef &&
    status.parentRuleRef === WindowsFirewallRefs.ParentRuleRef &&
    status.evidenceRefs.length === 1 &&
    status.evidenceRefs[0] === WindowsFirewallRefs.EvidenceRef &&
    status.windowsOsScopeRef === WindowsFirewallRefs.WindowsOsScopeRef &&
    status.targetRef === WindowsFirewallRefs.TargetRef &&
    status.firewallRuleRef === WindowsFirewallRefs.FirewallRuleRef &&
    status.ruleName === WindowsFirewallRefs.RuleName &&
    status.targetRemoteAddress === WindowsFirewallRefs.TargetRemoteAddress
  );
}

function executionShapeMatches(status: AgentNetworkWindowsFirewallLabStatus): boolean {
  return (
    status.state === 'executed-and-rolled-back' &&
    status.windowsHostObserved &&
    status.administratorPermissionObserved &&
    status.applyCommandObserved &&
    status.verifyPresentObserved &&
    status.rollbackCommandObserved &&
    status.verifyRemovedObserved &&
    status.labFirewallMutationExecuted &&
    status.rollbackVerified &&
    status.adapterApplyAuthorized
  );
}

function commandCountsMatch(status: AgentNetworkWindowsFirewallLabStatus): boolean {
  return (
    status.requiredCommandCount === 4 &&
    status.commandCount === 4 &&
    status.commandEvidence.length === status.commandCount
  );
}

function commandEvidenceMatches(status: AgentNetworkWindowsFirewallLabStatus): boolean {
  return (
    commandMatches(
      status.commandEvidence[0],
      'apply-rule',
      WindowsFirewallRefs.ApplyRuleCommandRef,
      WindowsFirewallRefs.ApplyRuleOutputSha256,
      true
    ) &&
    commandMatches(
      status.commandEvidence[1],
      'verify-rule-present',
      WindowsFirewallRefs.VerifyPresentCommandRef,
      WindowsFirewallRefs.VerifyPresentOutputSha256,
      true
    ) &&
    commandMatches(
      status.commandEvidence[2],
      'rollback-rule',
      WindowsFirewallRefs.RollbackRuleCommandRef,
      WindowsFirewallRefs.RollbackRuleOutputSha256,
      false
    ) &&
    commandMatches(
      status.commandEvidence[3],
      'verify-rule-removed',
      WindowsFirewallRefs.VerifyRemovedCommandRef,
      WindowsFirewallRefs.VerifyRemovedOutputSha256,
      false
    )
  );
}

function commandMatches(
  command: AgentNetworkWindowsFirewallLabCommandRow | undefined,
  kind: AgentNetworkWindowsFirewallLabCommandRow['kind'],
  commandRef: string,
  outputSha256: string,
  rulePresentAfterCommand: boolean
): boolean {
  return (
    command?.kind === kind &&
    command.commandRef === commandRef &&
    command.exitStatus === 0 &&
    command.outputSha256 === outputSha256 &&
    command.rulePresentAfterCommand === rulePresentAfterCommand
  );
}
