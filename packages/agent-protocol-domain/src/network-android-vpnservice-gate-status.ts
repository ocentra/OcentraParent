import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const AndroidVpnServiceRefs = AgentProtocolDefaults.NetworkAndroidVpnServiceGateStatus;

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

const AgentNetworkAndroidVpnServiceGateStatusFields = Schema.Struct({
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

export type AgentNetworkAndroidVpnServiceGateStatus = Infer<
  typeof AgentNetworkAndroidVpnServiceGateStatusFields
>;

export const AgentNetworkAndroidVpnServiceGateStatusSchema = withParser(
  AgentNetworkAndroidVpnServiceGateStatusFields.pipe(
    Schema.filter(
      (status: AgentNetworkAndroidVpnServiceGateStatus) =>
        (refsMatch(status) && artifactRefsMatch(status) && statusShapeMatches(status)) ||
        'Network Android VpnService gate status must preserve row40 refs, proof-ready ' +
          'artifacts, and no execution/content/enforcement claims'
    )
  )
);

export type AgentNetworkAndroidVpnServiceGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkAndroidVpnServiceGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-android-vpn-service-gate-status'
        | 'invalid-android-vpn-service-gate-status-json'
        | 'invalid-android-vpn-service-gate-status';
    };

export function parseAgentNetworkAndroidVpnServiceGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkAndroidVpnServiceGateStatusParseResult {
  if (event.event !== AgentEvent.NetworkAndroidVpnServiceGateStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-android-vpn-service-gate-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-android-vpn-service-gate-status-json' };
  }

  const parsed = AgentNetworkAndroidVpnServiceGateStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-android-vpn-service-gate-status' };
  }

  return { ok: true, status: parsed.data };
}

function refsMatch(status: AgentNetworkAndroidVpnServiceGateStatus): boolean {
  return (
    status.statusRef === AndroidVpnServiceRefs.StatusRef &&
    status.androidVpnServiceGateRef === AndroidVpnServiceRefs.AndroidVpnServiceGateRef &&
    status.policyDecisionRef === AndroidVpnServiceRefs.PolicyDecisionRef &&
    status.parentRuleRef === AndroidVpnServiceRefs.ParentRuleRef &&
    status.evidenceRefs.length === 1 &&
    status.evidenceRefs[0] === AndroidVpnServiceRefs.EvidenceRef &&
    status.localAiResultRef === AndroidVpnServiceRefs.LocalAiResultRef &&
    status.packageRef === AndroidVpnServiceRefs.PackageRef &&
    status.vpnServiceRef === AndroidVpnServiceRefs.VpnServiceRef
  );
}

function artifactRefsMatch(status: AgentNetworkAndroidVpnServiceGateStatus): boolean {
  return (
    status.vpnServiceDeclarationRef === AndroidVpnServiceRefs.VpnServiceDeclarationRef &&
    status.userConsentProofRef === AndroidVpnServiceRefs.UserConsentProofRef &&
    status.physicalDeviceProofRef === AndroidVpnServiceRefs.PhysicalDeviceProofRef &&
    status.packageIdentityProofRef === AndroidVpnServiceRefs.PackageIdentityProofRef &&
    status.virtualInterfaceProofRef === AndroidVpnServiceRefs.VirtualInterfaceProofRef &&
    status.trafficObservationProofRef === AndroidVpnServiceRefs.TrafficObservationProofRef &&
    status.rollbackPlanRef === AndroidVpnServiceRefs.RollbackPlanRef &&
    status.auditEventRef === AndroidVpnServiceRefs.AuditEventRef &&
    status.deviceOwnerProofRef === null
  );
}

function statusShapeMatches(status: AgentNetworkAndroidVpnServiceGateStatus): boolean {
  return (
    status.capabilityState === 'physical-device-ready' &&
    status.gateState === 'physical-device-proof-ready' &&
    sameStringArray(status.boundaryReasons, []) &&
    sameStringArray(status.missingRequiredArtifacts, []) &&
    !status.deviceOwnerRequired &&
    status.physicalDeviceProofReady &&
    !status.deviceOwnerAuthorityProved
  );
}

function sameStringArray(values: readonly string[], expected: readonly string[]): boolean {
  return values.length === expected.length && values.every((value, index) => value === expected[index]);
}
