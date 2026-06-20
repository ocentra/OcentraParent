import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

export const AgentNetworkWindowsWfpGateStateSchema = withParser(
  Schema.Literal('lab-proof-ready', 'manual-required', 'research-only', 'unavailable')
);
export const AgentNetworkWindowsWfpCapabilityStateSchema = withParser(
  Schema.Literal('lab-ready', 'manual-required', 'unavailable')
);

export const AgentNetworkWindowsWfpGateStatusSchema = withParser(
  Schema.Struct({
    statusRef: NonEmptyStringSchema,
    wfpGateRef: NonEmptyStringSchema,
    policyDecisionRef: NonEmptyStringSchema,
    parentRuleRef: NonEmptyStringSchema,
    evidenceRefs: Schema.Array(NonEmptyStringSchema),
    localAiResultRef: Schema.NullOr(NonEmptyStringSchema),
    targetRef: NonEmptyStringSchema,
    wfpProviderRef: NonEmptyStringSchema,
    wfpLayerRef: NonEmptyStringSchema,
    capabilityState: AgentNetworkWindowsWfpCapabilityStateSchema,
    gateState: AgentNetworkWindowsWfpGateStateSchema,
    boundaryReasons: Schema.Array(NonEmptyStringSchema),
    missingRequiredArtifacts: Schema.Array(NonEmptyStringSchema),
    administratorPermissionProofRef: Schema.NullOr(NonEmptyStringSchema),
    driverSigningProofRef: Schema.NullOr(NonEmptyStringSchema),
    driverPackageProofRef: Schema.NullOr(NonEmptyStringSchema),
    providerRegistrationPlanRef: Schema.NullOr(NonEmptyStringSchema),
    layerCapabilityMatrixRef: Schema.NullOr(NonEmptyStringSchema),
    rollbackPlanRef: Schema.NullOr(NonEmptyStringSchema),
    labResultArtifactRef: Schema.NullOr(NonEmptyStringSchema),
    auditEventRef: Schema.NullOr(NonEmptyStringSchema),
    wfpLabProofReady: Schema.Boolean,
    adapterApplyAuthorized: Schema.Literal(false),
    enforcementCommandPublished: Schema.Literal(false),
    liveDriverInstallClaimed: Schema.Literal(false),
    calloutRegistrationClaimed: Schema.Literal(false),
    packetBlockClaimed: Schema.Literal(false),
    kernelPayloadInspectionClaimed: Schema.Literal(false),
    commandInvocationClaimed: Schema.Literal(false),
    exactUrlAvailable: Schema.Literal(false),
    decryptedPayloadAvailable: Schema.Literal(false),
    pageContentAvailable: Schema.Literal(false),
  })
);

export type AgentNetworkWindowsWfpGateStatus = Infer<typeof AgentNetworkWindowsWfpGateStatusSchema>;
