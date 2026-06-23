import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const LinuxNftablesCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentNetworkLinuxNftablesLabStateSchema = withParser(
  Schema.Literal('executed-and-rolled-back', 'manual-required', 'unavailable')
);
export const AgentNetworkLinuxNftablesLabCommandKindSchema = withParser(
  Schema.Literal(
    'create-table',
    'create-chain',
    'add-rule',
    'verify-rule-present',
    'delete-table',
    'verify-table-removed'
  )
);

export const AgentNetworkLinuxNftablesLabCommandRowSchema = withParser(
  Schema.Struct({
    kind: AgentNetworkLinuxNftablesLabCommandKindSchema,
    commandRef: NonEmptyStringSchema,
    exitStatus: Schema.Number.pipe(Schema.int()),
    outputSha256: NonEmptyStringSchema,
    tablePresentAfterCommand: Schema.Boolean,
    chainPresentAfterCommand: Schema.Boolean,
    rulePresentAfterCommand: Schema.Boolean,
  })
);

const AgentNetworkLinuxNftablesLabStatusStructSchema = Schema.Struct({
  statusRef: NonEmptyStringSchema,
  labRef: NonEmptyStringSchema,
  linuxAdapterGateRef: NonEmptyStringSchema,
  policyDecisionRef: NonEmptyStringSchema,
  parentRuleRef: NonEmptyStringSchema,
  evidenceRefs: Schema.Array(NonEmptyStringSchema),
  distroRef: NonEmptyStringSchema,
  kernelRef: NonEmptyStringSchema,
  tableName: NonEmptyStringSchema,
  chainName: NonEmptyStringSchema,
  targetRemoteAddress: NonEmptyStringSchema,
  state: AgentNetworkLinuxNftablesLabStateSchema,
  wslHostObserved: Schema.Boolean,
  rootPermissionObserved: Schema.Boolean,
  nftToolObserved: Schema.Boolean,
  commandCount: LinuxNftablesCount,
  requiredCommandCount: LinuxNftablesCount,
  tableCreateObserved: Schema.Boolean,
  chainCreateObserved: Schema.Boolean,
  ruleAddObserved: Schema.Boolean,
  verifyPresentObserved: Schema.Boolean,
  rollbackObserved: Schema.Boolean,
  verifyRemovedObserved: Schema.Boolean,
  labPacketFilterRuleExecuted: Schema.Boolean,
  rollbackVerified: Schema.Boolean,
  productionEnforcementClaimed: Schema.Literal(false),
  persistentRuleClaimed: Schema.Literal(false),
  genericLinuxSupportClaimed: Schema.Literal(false),
  serviceManagerInstallClaimed: Schema.Literal(false),
  exactUrlAvailable: Schema.Literal(false),
  decryptedPayloadAvailable: Schema.Literal(false),
  pageContentAvailable: Schema.Literal(false),
  policyEngineExecutionClaimed: Schema.Literal(false),
  enforcementCommandPublished: Schema.Literal(false),
  commandEvidence: Schema.Array(AgentNetworkLinuxNftablesLabCommandRowSchema),
});

type AgentNetworkLinuxNftablesLabStatusStruct = Infer<typeof AgentNetworkLinuxNftablesLabStatusStructSchema>;

export const AgentNetworkLinuxNftablesLabStatusSchema = withParser(
  AgentNetworkLinuxNftablesLabStatusStructSchema.pipe(
    Schema.filter(
      (status) =>
        linuxNftablesLabCommandEvidenceIsConsistent(status) ||
        'Expected Linux nftables lab command evidence to match the bounded table/chain/rule apply and rollback sequence'
    )
  )
);

export type AgentNetworkLinuxNftablesLabCommandRow = Infer<typeof AgentNetworkLinuxNftablesLabCommandRowSchema>;
export type AgentNetworkLinuxNftablesLabStatus = Infer<typeof AgentNetworkLinuxNftablesLabStatusSchema>;
type LinuxNftablesCommandEvidenceByKind = ReadonlyMap<
  AgentNetworkLinuxNftablesLabCommandRow['kind'],
  AgentNetworkLinuxNftablesLabCommandRow
>;

function linuxNftablesLabCommandEvidenceIsConsistent(status: AgentNetworkLinuxNftablesLabStatusStruct): boolean {
  if (!linuxNftablesCommandCountsMatch(status)) {
    return false;
  }

  if (status.state !== 'executed-and-rolled-back') {
    return true;
  }

  const byKind = linuxNftablesCommandEvidenceByKind(status.commandEvidence);
  return (
    byKind !== null && linuxNftablesObservedFlagsMatch(status, byKind) && linuxNftablesCommandOutcomesMatch(byKind)
  );
}

function linuxNftablesCommandCountsMatch(status: AgentNetworkLinuxNftablesLabStatusStruct): boolean {
  return (
    status.commandCount === status.commandEvidence.length &&
    status.requiredCommandCount === status.commandEvidence.length
  );
}

function linuxNftablesCommandEvidenceByKind(
  rows: readonly AgentNetworkLinuxNftablesLabCommandRow[]
): LinuxNftablesCommandEvidenceByKind | null {
  const byKind = new Map(rows.map((row) => [row.kind, row] as const));
  return byKind.size === rows.length ? byKind : null;
}

function linuxNftablesObservedFlagsMatch(
  status: AgentNetworkLinuxNftablesLabStatusStruct,
  byKind: LinuxNftablesCommandEvidenceByKind
): boolean {
  const observedFlags = [
    { observed: status.tableCreateObserved, kind: 'create-table' },
    { observed: status.chainCreateObserved, kind: 'create-chain' },
    { observed: status.ruleAddObserved, kind: 'add-rule' },
    { observed: status.verifyPresentObserved, kind: 'verify-rule-present' },
    { observed: status.rollbackObserved, kind: 'delete-table' },
    { observed: status.verifyRemovedObserved, kind: 'verify-table-removed' },
  ] as const;

  return observedFlags.every(({ observed, kind }) => observed === byKind.has(kind));
}

function linuxNftablesCommandOutcomesMatch(byKind: LinuxNftablesCommandEvidenceByKind): boolean {
  const expectedCommands = [
    { kind: 'create-table', table: true, chain: false, rule: false },
    { kind: 'create-chain', table: true, chain: true, rule: false },
    { kind: 'add-rule', table: true, chain: true, rule: true },
    { kind: 'verify-rule-present', table: true, chain: true, rule: true },
    { kind: 'delete-table', table: false, chain: false, rule: false },
    { kind: 'verify-table-removed', table: false, chain: false, rule: false },
  ] as const;

  return expectedCommands.every(({ kind, table, chain, rule }) =>
    linuxNftablesCommandMatches(byKind.get(kind), table, chain, rule)
  );
}

function linuxNftablesCommandMatches(
  row: AgentNetworkLinuxNftablesLabCommandRow | undefined,
  tablePresentAfterCommand: boolean,
  chainPresentAfterCommand: boolean,
  rulePresentAfterCommand: boolean
): boolean {
  return (
    row?.tablePresentAfterCommand === tablePresentAfterCommand &&
    row.chainPresentAfterCommand === chainPresentAfterCommand &&
    row.rulePresentAfterCommand === rulePresentAfterCommand
  );
}
