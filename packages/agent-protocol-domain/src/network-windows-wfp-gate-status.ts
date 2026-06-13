import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const WindowsWfpRefs = AgentProtocolDefaults.NetworkWindowsWfpGateStatus;

export const AgentNetworkWindowsWfpGateStateSchema = withParser(
  Schema.Literal('lab-proof-ready', 'manual-required', 'research-only', 'unavailable')
);

export const AgentNetworkWindowsWfpCapabilityStateSchema = withParser(
  Schema.Literal('lab-ready', 'manual-required', 'unavailable')
);

const AgentNetworkWindowsWfpGateStatusFields = Schema.Struct({
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

export type AgentNetworkWindowsWfpGateStatus = Infer<typeof AgentNetworkWindowsWfpGateStatusFields>;

export const AgentNetworkWindowsWfpGateStatusSchema = withParser(
  AgentNetworkWindowsWfpGateStatusFields.pipe(
    Schema.filter(
      (status: AgentNetworkWindowsWfpGateStatus) =>
        (refsMatch(status) && artifactRefsMatch(status) && statusShapeMatches(status)) ||
        'Network Windows WFP gate status must preserve row39 refs, lab-proof-ready status, required ' +
          'artifacts, and no execution/content/enforcement claims'
    )
  )
);

export type AgentNetworkWindowsWfpGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkWindowsWfpGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-windows-wfp-gate-status'
        | 'invalid-windows-wfp-gate-status-json'
        | 'invalid-windows-wfp-gate-status';
    };

export function parseAgentNetworkWindowsWfpGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkWindowsWfpGateStatusParseResult {
  if (event.event !== AgentEvent.NetworkWindowsWfpGateStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-windows-wfp-gate-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-windows-wfp-gate-status-json' };
  }

  const parsed = AgentNetworkWindowsWfpGateStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-windows-wfp-gate-status' };
  }

  return { ok: true, status: parsed.data };
}

function refsMatch(status: AgentNetworkWindowsWfpGateStatus): boolean {
  return (
    status.statusRef === WindowsWfpRefs.StatusRef &&
    status.wfpGateRef === WindowsWfpRefs.WfpGateRef &&
    status.policyDecisionRef === WindowsWfpRefs.PolicyDecisionRef &&
    status.parentRuleRef === WindowsWfpRefs.ParentRuleRef &&
    status.evidenceRefs.length === 1 &&
    status.evidenceRefs[0] === WindowsWfpRefs.EvidenceRef &&
    status.localAiResultRef === WindowsWfpRefs.LocalAiResultRef &&
    status.targetRef === WindowsWfpRefs.TargetRef &&
    status.wfpProviderRef === WindowsWfpRefs.WfpProviderRef &&
    status.wfpLayerRef === WindowsWfpRefs.WfpLayerRef
  );
}

function artifactRefsMatch(status: AgentNetworkWindowsWfpGateStatus): boolean {
  return (
    status.administratorPermissionProofRef === WindowsWfpRefs.AdministratorPermissionProofRef &&
    status.driverSigningProofRef === WindowsWfpRefs.DriverSigningProofRef &&
    status.driverPackageProofRef === WindowsWfpRefs.DriverPackageProofRef &&
    status.providerRegistrationPlanRef === WindowsWfpRefs.ProviderRegistrationPlanRef &&
    status.layerCapabilityMatrixRef === WindowsWfpRefs.LayerCapabilityMatrixRef &&
    status.rollbackPlanRef === WindowsWfpRefs.RollbackPlanRef &&
    status.labResultArtifactRef === WindowsWfpRefs.LabResultArtifactRef &&
    status.auditEventRef === WindowsWfpRefs.AuditEventRef
  );
}

function statusShapeMatches(status: AgentNetworkWindowsWfpGateStatus): boolean {
  return (
    status.capabilityState === 'lab-ready' &&
    status.gateState === 'lab-proof-ready' &&
    status.boundaryReasons.length === 0 &&
    status.missingRequiredArtifacts.length === 0 &&
    status.wfpLabProofReady
  );
}
