import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const AppleNetworkExtensionRefs = AgentProtocolDefaults.NetworkAppleNetworkExtensionGateStatus;

export const AgentNetworkAppleNetworkExtensionPlatformSchema = withParser(
  Schema.Literal('mac-os', 'ios')
);

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

const AgentNetworkAppleNetworkExtensionGateStatusFields = Schema.Struct({
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

export type AgentNetworkAppleNetworkExtensionGateStatus = Infer<
  typeof AgentNetworkAppleNetworkExtensionGateStatusFields
>;

export const AgentNetworkAppleNetworkExtensionGateStatusSchema = withParser(
  AgentNetworkAppleNetworkExtensionGateStatusFields.pipe(
    Schema.filter(
      (status: AgentNetworkAppleNetworkExtensionGateStatus) =>
        (refsMatch(status) && artifactRefsMatch(status) && statusShapeMatches(status)) ||
        'Network Apple Network Extension gate status must preserve row41 refs, proof-ready ' +
          'entitlement artifacts, and no execution/content/enforcement claims'
    )
  )
);

export type AgentNetworkAppleNetworkExtensionGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkAppleNetworkExtensionGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-apple-network-extension-gate-status'
        | 'invalid-apple-network-extension-gate-status-json'
        | 'invalid-apple-network-extension-gate-status';
    };

export function parseAgentNetworkAppleNetworkExtensionGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkAppleNetworkExtensionGateStatusParseResult {
  if (event.event !== AgentEvent.NetworkAppleNetworkExtensionGateStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-apple-network-extension-gate-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-apple-network-extension-gate-status-json' };
  }

  const parsed = AgentNetworkAppleNetworkExtensionGateStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-apple-network-extension-gate-status' };
  }

  return { ok: true, status: parsed.data };
}

function refsMatch(status: AgentNetworkAppleNetworkExtensionGateStatus): boolean {
  return (
    status.statusRef === AppleNetworkExtensionRefs.StatusRef &&
    status.appleNetworkExtensionGateRef === AppleNetworkExtensionRefs.AppleNetworkExtensionGateRef &&
    status.policyDecisionRef === AppleNetworkExtensionRefs.PolicyDecisionRef &&
    status.parentRuleRef === AppleNetworkExtensionRefs.ParentRuleRef &&
    status.evidenceRefs.length === 1 &&
    status.evidenceRefs[0] === AppleNetworkExtensionRefs.EvidenceRef &&
    status.localAiResultRef === AppleNetworkExtensionRefs.LocalAiResultRef &&
    status.platform === 'ios' &&
    status.bundleRef === AppleNetworkExtensionRefs.BundleRef &&
    status.networkExtensionRef === AppleNetworkExtensionRefs.NetworkExtensionRef
  );
}

function artifactRefsMatch(status: AgentNetworkAppleNetworkExtensionGateStatus): boolean {
  return (
    status.developerTeamProofRef === AppleNetworkExtensionRefs.DeveloperTeamProofRef &&
    status.entitlementApprovalProofRef === AppleNetworkExtensionRefs.EntitlementApprovalProofRef &&
    status.provisioningProfileProofRef === AppleNetworkExtensionRefs.ProvisioningProfileProofRef &&
    status.signingProofRef === AppleNetworkExtensionRefs.SigningProofRef &&
    status.deviceOrTestFlightProofRef === AppleNetworkExtensionRefs.DeviceOrTestFlightProofRef &&
    status.networkExtensionDeclarationRef === AppleNetworkExtensionRefs.NetworkExtensionDeclarationRef &&
    status.extensionConfigurationProofRef === AppleNetworkExtensionRefs.ExtensionConfigurationProofRef &&
    status.rollbackPlanRef === AppleNetworkExtensionRefs.RollbackPlanRef &&
    status.auditEventRef === AppleNetworkExtensionRefs.AuditEventRef &&
    status.supervisionOrMdmProofRef === null
  );
}

function statusShapeMatches(status: AgentNetworkAppleNetworkExtensionGateStatus): boolean {
  return (
    status.capabilityState === 'apple-device-ready' &&
    status.gateState === 'apple-entitlement-proof-ready' &&
    sameStringArray(status.boundaryReasons, []) &&
    sameStringArray(status.missingRequiredArtifacts, []) &&
    !status.supervisionRequired &&
    status.appleEntitlementProofReady &&
    !status.supervisionAuthorityProved
  );
}

function sameStringArray(values: readonly string[], expected: readonly string[]): boolean {
  return values.length === expected.length && values.every((value, index) => value === expected[index]);
}
