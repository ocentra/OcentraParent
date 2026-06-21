import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const LinuxNftablesCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentNetworkLinuxNftablesLabStateSchema = withParser(
  Schema.Literal('executed-and-rolled-back', 'manual-required', 'unavailable')
);
export const AgentNetworkLinuxNftablesLabCommandKindSchema = withParser(
  Schema.Literal('create-table', 'create-chain', 'add-rule', 'verify-rule-present', 'delete-table', 'verify-table-removed')
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

function linuxNftablesLabCommandEvidenceIsConsistent(status: AgentNetworkLinuxNftablesLabStatusStruct): boolean {
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
    status.tableCreateObserved === byKind.has('create-table') &&
    status.chainCreateObserved === byKind.has('create-chain') &&
    status.ruleAddObserved === byKind.has('add-rule') &&
    status.verifyPresentObserved === byKind.has('verify-rule-present') &&
    status.rollbackObserved === byKind.has('delete-table') &&
    status.verifyRemovedObserved === byKind.has('verify-table-removed') &&
    byKind.get('create-table')?.tablePresentAfterCommand === true &&
    byKind.get('create-table')?.chainPresentAfterCommand === false &&
    byKind.get('create-table')?.rulePresentAfterCommand === false &&
    byKind.get('create-chain')?.tablePresentAfterCommand === true &&
    byKind.get('create-chain')?.chainPresentAfterCommand === true &&
    byKind.get('create-chain')?.rulePresentAfterCommand === false &&
    byKind.get('add-rule')?.tablePresentAfterCommand === true &&
    byKind.get('add-rule')?.chainPresentAfterCommand === true &&
    byKind.get('add-rule')?.rulePresentAfterCommand === true &&
    byKind.get('verify-rule-present')?.tablePresentAfterCommand === true &&
    byKind.get('verify-rule-present')?.chainPresentAfterCommand === true &&
    byKind.get('verify-rule-present')?.rulePresentAfterCommand === true &&
    byKind.get('delete-table')?.tablePresentAfterCommand === false &&
    byKind.get('delete-table')?.chainPresentAfterCommand === false &&
    byKind.get('delete-table')?.rulePresentAfterCommand === false &&
    byKind.get('verify-table-removed')?.tablePresentAfterCommand === false &&
    byKind.get('verify-table-removed')?.chainPresentAfterCommand === false &&
    byKind.get('verify-table-removed')?.rulePresentAfterCommand === false
  );
}
