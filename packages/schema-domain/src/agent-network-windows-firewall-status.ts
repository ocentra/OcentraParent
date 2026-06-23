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
type WindowsFirewallCommandEvidenceByKind = ReadonlyMap<
  AgentNetworkWindowsFirewallLabCommandRow['kind'],
  AgentNetworkWindowsFirewallLabCommandRow
>;

function windowsFirewallLabCommandEvidenceIsConsistent(status: AgentNetworkWindowsFirewallLabStatusStruct): boolean {
  if (!windowsFirewallCommandCountsMatch(status)) {
    return false;
  }

  if (status.state !== 'executed-and-rolled-back') {
    return true;
  }

  const byKind = windowsFirewallCommandEvidenceByKind(status.commandEvidence);
  return (
    byKind !== null && windowsFirewallObservedFlagsMatch(status, byKind) && windowsFirewallCommandOutcomesMatch(byKind)
  );
}

function windowsFirewallCommandCountsMatch(status: AgentNetworkWindowsFirewallLabStatusStruct): boolean {
  return (
    status.commandCount === status.commandEvidence.length &&
    status.requiredCommandCount === status.commandEvidence.length
  );
}

function windowsFirewallCommandEvidenceByKind(
  rows: readonly AgentNetworkWindowsFirewallLabCommandRow[]
): WindowsFirewallCommandEvidenceByKind | null {
  const byKind = new Map(rows.map((row) => [row.kind, row] as const));
  return byKind.size === rows.length ? byKind : null;
}

function windowsFirewallObservedFlagsMatch(
  status: AgentNetworkWindowsFirewallLabStatusStruct,
  byKind: WindowsFirewallCommandEvidenceByKind
): boolean {
  const observedFlags = [
    { observed: status.applyCommandObserved, kind: 'apply-rule' },
    { observed: status.verifyPresentObserved, kind: 'verify-rule-present' },
    { observed: status.rollbackCommandObserved, kind: 'rollback-rule' },
    { observed: status.verifyRemovedObserved, kind: 'verify-rule-removed' },
  ] as const;

  return observedFlags.every(({ observed, kind }) => observed === byKind.has(kind));
}

function windowsFirewallCommandOutcomesMatch(byKind: WindowsFirewallCommandEvidenceByKind): boolean {
  const expectedCommands = [
    { kind: 'apply-rule', rulePresentAfterCommand: true },
    { kind: 'verify-rule-present', rulePresentAfterCommand: true },
    { kind: 'rollback-rule', rulePresentAfterCommand: false },
    { kind: 'verify-rule-removed', rulePresentAfterCommand: false },
  ] as const;

  return expectedCommands.every(
    ({ kind, rulePresentAfterCommand }) => byKind.get(kind)?.rulePresentAfterCommand === rulePresentAfterCommand
  );
}
