import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

export const AgentNetworkWindowsWfpGateStateSchema = withParser(
  Schema.Literal('lab-proof-ready', 'manual-required', 'research-only', 'unavailable')
);
export const AgentNetworkWindowsWfpCapabilityStateSchema = withParser(
  Schema.Literal('lab-ready', 'manual-required', 'unavailable')
);

const AgentNetworkWindowsWfpGateStatusStructSchema = Schema.Struct({
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
});

type AgentNetworkWindowsWfpGateStatusStruct = Infer<typeof AgentNetworkWindowsWfpGateStatusStructSchema>;

export const AgentNetworkWindowsWfpGateStatusSchema = withParser(
  AgentNetworkWindowsWfpGateStatusStructSchema.pipe(
    Schema.filter(
      (status) =>
        wfpProofReadinessIsConsistent(status) ||
        'Expected proof-ready Windows WFP gate status to stay reason-free and manual-required states to preserve bounded blockers'
    )
  )
);

export type AgentNetworkWindowsWfpGateStatus = Infer<typeof AgentNetworkWindowsWfpGateStatusSchema>;

function wfpProofReadinessIsConsistent(status: AgentNetworkWindowsWfpGateStatusStruct): boolean {
  if (status.gateState === 'lab-proof-ready') {
    return (
      status.capabilityState === 'lab-ready' &&
      status.wfpLabProofReady &&
      status.boundaryReasons.length === 0 &&
      status.missingRequiredArtifacts.length === 0
    );
  }

  if (status.gateState === 'manual-required') {
    return (
      status.capabilityState === 'manual-required' ||
      status.boundaryReasons.length > 0 ||
      status.missingRequiredArtifacts.length > 0 ||
      !status.wfpLabProofReady
    );
  }

  return true;
}
