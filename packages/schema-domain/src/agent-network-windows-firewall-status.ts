import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const WindowsFirewallCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentNetworkWindowsFirewallLabStateSchema = withParser(
  Schema.Literal('executed-and-rolled-back', 'manual-required', 'unavailable')
);
export const AgentNetworkWindowsFirewallLabCommandKindSchema = withParser(
  Schema.Literal('apply-rule', 'verify-rule-present', 'rollback-rule', 'verify-rule-removed')
);

export const AgentNetworkWindowsFirewallLabCommandRowSchema = withParser(
  Schema.Struct({
    kind: AgentNetworkWindowsFirewallLabCommandKindSchema,
    commandRef: NonEmptyStringSchema,
    exitStatus: Schema.Number.pipe(Schema.int()),
    outputSha256: NonEmptyStringSchema,
    rulePresentAfterCommand: Schema.Boolean,
  })
);

const AgentNetworkWindowsFirewallLabStatusStructSchema = Schema.Struct({
  statusRef: NonEmptyStringSchema,
  labRef: NonEmptyStringSchema,
  firewallAdapterPlanRef: NonEmptyStringSchema,
  policyDecisionRef: NonEmptyStringSchema,
  parentRuleRef: NonEmptyStringSchema,
  evidenceRefs: Schema.Array(NonEmptyStringSchema),
  windowsOsScopeRef: NonEmptyStringSchema,
  targetRef: NonEmptyStringSchema,
  firewallRuleRef: NonEmptyStringSchema,
  ruleName: NonEmptyStringSchema,
  targetRemoteAddress: NonEmptyStringSchema,
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

type AgentNetworkWindowsFirewallLabStatusStruct = Infer<typeof AgentNetworkWindowsFirewallLabStatusStructSchema>;

export const AgentNetworkWindowsFirewallLabStatusSchema = withParser(
  AgentNetworkWindowsFirewallLabStatusStructSchema.pipe(
    Schema.filter(
      (status) =>
        windowsFirewallLabCommandEvidenceIsConsistent(status) ||
        'Expected Windows firewall lab command evidence to match the bounded apply/verify/rollback sequence'
    )
  )
);

export type AgentNetworkWindowsFirewallLabCommandRow = Infer<typeof AgentNetworkWindowsFirewallLabCommandRowSchema>;
export type AgentNetworkWindowsFirewallLabStatus = Infer<typeof AgentNetworkWindowsFirewallLabStatusSchema>;

function windowsFirewallLabCommandEvidenceIsConsistent(status: AgentNetworkWindowsFirewallLabStatusStruct): boolean {
  if (status.commandCount !== status.commandEvidence.length || status.requiredCommandCount !== status.commandEvidence.length) {
    return false;
  }

  if (status.state !== 'executed-and-rolled-back') {
    return true;
  }

  const byKind = new Map(status.commandEvidence.map((row) => [row.kind, row] as const));
  if (byKind.size !== status.commandEvidence.length) {
    return false;
  }

  return (
    status.applyCommandObserved === byKind.has('apply-rule') &&
    status.verifyPresentObserved === byKind.has('verify-rule-present') &&
    status.rollbackCommandObserved === byKind.has('rollback-rule') &&
    status.verifyRemovedObserved === byKind.has('verify-rule-removed') &&
    byKind.get('apply-rule')?.rulePresentAfterCommand === true &&
    byKind.get('verify-rule-present')?.rulePresentAfterCommand === true &&
    byKind.get('rollback-rule')?.rulePresentAfterCommand === false &&
    byKind.get('verify-rule-removed')?.rulePresentAfterCommand === false
  );
}
