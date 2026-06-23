import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

export const AgentNetworkAppleNetworkExtensionPlatformSchema = withParser(Schema.Literal('mac-os', 'ios'));
export const AgentNetworkAppleNetworkExtensionCapabilityStateSchema = withParser(
  Schema.Literal('apple-device-ready', 'manual-required', 'unavailable')
);
export const AgentNetworkAppleNetworkExtensionGateStateSchema = withParser(
  Schema.Literal('research-only', 'manual-required', 'unavailable', 'apple-entitlement-proof-ready')
);
export const AgentNetworkAppleNetworkExtensionRequiredArtifactSchema = withParser(
  Schema.Literal(
    'developer-team-proof',
    'entitlement-approval-proof',
    'provisioning-profile-proof',
    'signing-proof',
    'device-or-testflight-proof',
    'network-extension-declaration',
    'extension-configuration-proof',
    'rollback-plan',
    'audit-event',
    'supervision-or-mdm-proof'
  )
);
export const AgentNetworkAppleNetworkExtensionGateBoundaryReasonSchema = withParser(
  Schema.Literal(
    'research-only-requested',
    'capability-manual-required',
    'capability-unavailable',
    'evidence-grade-below-proof-threshold',
    'policy-not-network-extension-approved',
    'missing-required-artifact'
  )
);

const AgentNetworkAppleNetworkExtensionGateStatusStructSchema = Schema.Struct({
  statusRef: NonEmptyStringSchema,
  appleNetworkExtensionGateRef: NonEmptyStringSchema,
  policyDecisionRef: NonEmptyStringSchema,
  parentRuleRef: NonEmptyStringSchema,
  evidenceRefs: Schema.Array(NonEmptyStringSchema),
  localAiResultRef: Schema.NullOr(NonEmptyStringSchema),
  platform: AgentNetworkAppleNetworkExtensionPlatformSchema,
  bundleRef: NonEmptyStringSchema,
  networkExtensionRef: NonEmptyStringSchema,
  capabilityState: AgentNetworkAppleNetworkExtensionCapabilityStateSchema,
  gateState: AgentNetworkAppleNetworkExtensionGateStateSchema,
  boundaryReasons: Schema.Array(AgentNetworkAppleNetworkExtensionGateBoundaryReasonSchema),
  missingRequiredArtifacts: Schema.Array(AgentNetworkAppleNetworkExtensionRequiredArtifactSchema),
  developerTeamProofRef: Schema.NullOr(NonEmptyStringSchema),
  entitlementApprovalProofRef: Schema.NullOr(NonEmptyStringSchema),
  provisioningProfileProofRef: Schema.NullOr(NonEmptyStringSchema),
  signingProofRef: Schema.NullOr(NonEmptyStringSchema),
  deviceOrTestFlightProofRef: Schema.NullOr(NonEmptyStringSchema),
  networkExtensionDeclarationRef: Schema.NullOr(NonEmptyStringSchema),
  extensionConfigurationProofRef: Schema.NullOr(NonEmptyStringSchema),
  rollbackPlanRef: Schema.NullOr(NonEmptyStringSchema),
  auditEventRef: Schema.NullOr(NonEmptyStringSchema),
  supervisionRequired: Schema.Boolean,
  supervisionOrMdmProofRef: Schema.NullOr(NonEmptyStringSchema),
  appleEntitlementProofReady: Schema.Boolean,
  supervisionAuthorityProved: Schema.Boolean,
  adapterApplyAuthorized: Schema.Literal(false),
  enforcementCommandPublished: Schema.Literal(false),
  simulatorOnlyProductSupportClaimed: Schema.Literal(false),
  liveNetworkExtensionClaimed: Schema.Literal(false),
  packetBlockClaimed: Schema.Literal(false),
  appLevelControlClaimed: Schema.Literal(false),
  exactUrlAvailable: Schema.Literal(false),
  decryptedPayloadAvailable: Schema.Literal(false),
  pageContentAvailable: Schema.Literal(false),
});

type AgentNetworkAppleNetworkExtensionGateStatusStruct = Infer<
  typeof AgentNetworkAppleNetworkExtensionGateStatusStructSchema
>;

export const AgentNetworkAppleNetworkExtensionGateStatusSchema = withParser(
  AgentNetworkAppleNetworkExtensionGateStatusStructSchema.pipe(
    Schema.filter(
      (status) =>
        appleNetworkExtensionGateStatusIsConsistent(status) ||
        'Expected Apple network extension proof-ready rows to preserve proof-ready state and manual-required rows to retain blockers'
    )
  )
);

export type AgentNetworkAppleNetworkExtensionGateStatus = Infer<
  typeof AgentNetworkAppleNetworkExtensionGateStatusSchema
>;

function appleNetworkExtensionGateStatusIsConsistent(
  status: AgentNetworkAppleNetworkExtensionGateStatusStruct
): boolean {
  if (status.gateState === 'apple-entitlement-proof-ready') {
    return (
      status.capabilityState === 'apple-device-ready' &&
      status.appleEntitlementProofReady &&
      status.boundaryReasons.length === 0 &&
      status.missingRequiredArtifacts.length === 0
    );
  }

  if (status.gateState === 'manual-required') {
    return (
      status.capabilityState === 'manual-required' ||
      status.boundaryReasons.length > 0 ||
      status.missingRequiredArtifacts.length > 0 ||
      !status.appleEntitlementProofReady
    );
  }

  return true;
}
