import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

export const AgentNetworkAndroidVpnServiceGateStateSchema = withParser(
  Schema.Literal('research-only', 'manual-required', 'unavailable', 'physical-device-proof-ready')
);
export const AgentNetworkAndroidVpnServiceCapabilityStateSchema = withParser(
  Schema.Literal('physical-device-ready', 'manual-required', 'unavailable')
);
export const AgentNetworkAndroidVpnServiceRequiredArtifactSchema = withParser(
  Schema.Literal(
    'vpn-service-declaration',
    'user-consent-proof',
    'physical-device-proof',
    'package-identity-proof',
    'virtual-interface-proof',
    'traffic-observation-proof',
    'rollback-plan',
    'audit-event',
    'device-owner-proof'
  )
);
export const AgentNetworkAndroidVpnServiceGateBoundaryReasonSchema = withParser(
  Schema.Literal(
    'research-only-requested',
    'capability-manual-required',
    'capability-unavailable',
    'evidence-grade-below-proof-threshold',
    'policy-not-vpn-service-approved',
    'missing-required-artifact'
  )
);

const AgentNetworkAndroidVpnServiceGateStatusStructSchema = Schema.Struct({
  statusRef: NonEmptyStringSchema,
  androidVpnServiceGateRef: NonEmptyStringSchema,
  policyDecisionRef: NonEmptyStringSchema,
  parentRuleRef: NonEmptyStringSchema,
  evidenceRefs: Schema.Array(NonEmptyStringSchema),
  localAiResultRef: Schema.NullOr(NonEmptyStringSchema),
  packageRef: NonEmptyStringSchema,
  vpnServiceRef: NonEmptyStringSchema,
  capabilityState: AgentNetworkAndroidVpnServiceCapabilityStateSchema,
  gateState: AgentNetworkAndroidVpnServiceGateStateSchema,
  boundaryReasons: Schema.Array(AgentNetworkAndroidVpnServiceGateBoundaryReasonSchema),
  missingRequiredArtifacts: Schema.Array(AgentNetworkAndroidVpnServiceRequiredArtifactSchema),
  vpnServiceDeclarationRef: Schema.NullOr(NonEmptyStringSchema),
  userConsentProofRef: Schema.NullOr(NonEmptyStringSchema),
  physicalDeviceProofRef: Schema.NullOr(NonEmptyStringSchema),
  packageIdentityProofRef: Schema.NullOr(NonEmptyStringSchema),
  virtualInterfaceProofRef: Schema.NullOr(NonEmptyStringSchema),
  trafficObservationProofRef: Schema.NullOr(NonEmptyStringSchema),
  rollbackPlanRef: Schema.NullOr(NonEmptyStringSchema),
  auditEventRef: Schema.NullOr(NonEmptyStringSchema),
  deviceOwnerRequired: Schema.Boolean,
  deviceOwnerProofRef: Schema.NullOr(NonEmptyStringSchema),
  physicalDeviceProofReady: Schema.Boolean,
  deviceOwnerAuthorityProved: Schema.Boolean,
  adapterApplyAuthorized: Schema.Literal(false),
  enforcementCommandPublished: Schema.Literal(false),
  emulatorOnlyProductSupportClaimed: Schema.Literal(false),
  liveVpnTunnelClaimed: Schema.Literal(false),
  packetBlockClaimed: Schema.Literal(false),
  appPackageCorrelationClaimed: Schema.Literal(false),
  exactUrlAvailable: Schema.Literal(false),
  decryptedPayloadAvailable: Schema.Literal(false),
  pageContentAvailable: Schema.Literal(false),
});

type AgentNetworkAndroidVpnServiceGateStatusStruct = Infer<typeof AgentNetworkAndroidVpnServiceGateStatusStructSchema>;

export const AgentNetworkAndroidVpnServiceGateStatusSchema = withParser(
  AgentNetworkAndroidVpnServiceGateStatusStructSchema.pipe(
    Schema.filter(
      (status) =>
        androidVpnServiceGateStatusIsConsistent(status) ||
        'Expected Android VpnService proof-ready rows to preserve proof-ready state and manual-required rows to retain blockers'
    )
  )
);

export type AgentNetworkAndroidVpnServiceGateStatus = Infer<typeof AgentNetworkAndroidVpnServiceGateStatusSchema>;

function androidVpnServiceGateStatusIsConsistent(status: AgentNetworkAndroidVpnServiceGateStatusStruct): boolean {
  if (status.gateState === 'physical-device-proof-ready') {
    return (
      status.capabilityState === 'physical-device-ready' &&
      status.physicalDeviceProofReady &&
      status.boundaryReasons.length === 0 &&
      status.missingRequiredArtifacts.length === 0
    );
  }

  if (status.gateState === 'manual-required') {
    return (
      status.capabilityState === 'manual-required' ||
      status.boundaryReasons.length > 0 ||
      status.missingRequiredArtifacts.length > 0 ||
      !status.physicalDeviceProofReady
    );
  }

  return true;
}
