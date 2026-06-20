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

export const AgentNetworkLinuxNftablesLabStatusSchema = withParser(
  Schema.Struct({
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
  })
);

export type AgentNetworkLinuxNftablesLabCommandRow = Infer<typeof AgentNetworkLinuxNftablesLabCommandRowSchema>;
export type AgentNetworkLinuxNftablesLabStatus = Infer<typeof AgentNetworkLinuxNftablesLabStatusSchema>;
