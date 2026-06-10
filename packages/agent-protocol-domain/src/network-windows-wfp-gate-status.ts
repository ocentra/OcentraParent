import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const WindowsWfpText = Schema.String.pipe(Schema.minLength(1));
const WindowsWfpRefs = AgentProtocolDefaults.NetworkWindowsWfpGateStatus;

export const AgentNetworkWindowsWfpGateStateSchema = withParser(
  Schema.Literal('lab-proof-ready', 'manual-required', 'research-only', 'unavailable')
);

export const AgentNetworkWindowsWfpCapabilityStateSchema = withParser(
  Schema.Literal('lab-ready', 'manual-required', 'unavailable')
);

const AgentNetworkWindowsWfpGateStatusFields = Schema.Struct({
  statusRef: WindowsWfpText,
  wfpGateRef: WindowsWfpText,
  policyDecisionRef: WindowsWfpText,
  parentRuleRef: WindowsWfpText,
  evidenceRefs: Schema.Array(WindowsWfpText),
  localAiResultRef: Schema.NullOr(WindowsWfpText),
  targetRef: WindowsWfpText,
  wfpProviderRef: WindowsWfpText,
  wfpLayerRef: WindowsWfpText,
  capabilityState: AgentNetworkWindowsWfpCapabilityStateSchema,
  gateState: AgentNetworkWindowsWfpGateStateSchema,
  boundaryReasons: Schema.Array(WindowsWfpText),
  missingRequiredArtifacts: Schema.Array(WindowsWfpText),
  administratorPermissionProofRef: Schema.NullOr(WindowsWfpText),
  driverSigningProofRef: Schema.NullOr(WindowsWfpText),
  driverPackageProofRef: Schema.NullOr(WindowsWfpText),
  providerRegistrationPlanRef: Schema.NullOr(WindowsWfpText),
  layerCapabilityMatrixRef: Schema.NullOr(WindowsWfpText),
  rollbackPlanRef: Schema.NullOr(WindowsWfpText),
  labResultArtifactRef: Schema.NullOr(WindowsWfpText),
  auditEventRef: Schema.NullOr(WindowsWfpText),
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
        'Network Windows WFP gate status must preserve row39 refs, lab-proof-ready status, required artifacts, and no execution/content/enforcement claims'
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
