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

export const AgentNetworkWindowsFirewallLabStatusSchema = withParser(
  Schema.Struct({
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
  })
);

export type AgentNetworkWindowsFirewallLabCommandRow = Infer<typeof AgentNetworkWindowsFirewallLabCommandRowSchema>;
export type AgentNetworkWindowsFirewallLabStatus = Infer<typeof AgentNetworkWindowsFirewallLabStatusSchema>;
