import {
  AuditRequirementState,
  DeviceTrustState,
  HouseholdMembershipState,
  HouseholdRole,
} from '@ocentra-parent/schema-domain/family-household-authority';
import { ParentActorRole } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  type RecoveryAuthorizationInput,
  RecoveryAuthorizationInputSchema,
  RecoveryBundleState,
  RecoveryChildEvidenceAccessState,
  RecoveryDataCustodyHandoffState,
  type RecoveryDecision,
  RecoveryDecisionState,
  RecoveryDecisionSchema,
  RecoveryDeleteExportState,
  RecoveryFailureReason,
  RecoveryIdentityProofState,
  RecoveryKind,
  type RecoveryOperation,
  RecoveryOperationSchema,
  RecoveryState,
  RecoveryStateSchema,
  RecoverySupportChannel,
} from '@ocentra-parent/schema-domain/family-restore-lifecycle';
import {
  type SetupInvite,
  type SetupInviteAuthorizationInput,
  SetupInviteAuthorizationInputSchema,
  type SetupInviteDecision,
  SetupInviteDecisionSchema,
  SetupInviteFailureReason,
  SetupInvitePurpose,
  SetupInviteReplayState,
  SetupInviteSchema,
  SetupInviteState,
  type SetupInviteTargetRole,
  SetupInviteDecisionState,
  SetupRecoveryAbuseState,
  SetupRecoveryResponseTimingState,
} from '@ocentra-parent/schema-domain/family-setup-invite';

export function isSetupInviteActive(input: SetupInvite): boolean {
  const invite = SetupInviteSchema.parse(input);

  return invite.state === SetupInviteState.Pending;
}

export function doesSetupInviteMatchTargetRole(input: SetupInvite): boolean {
  const invite = SetupInviteSchema.parse(input);

  return purposeMatchesTargetRole(invite.purpose, invite.targetRole);
}

export function isSetupInviteSinglePurpose(input: SetupInvite): boolean {
  const invite = SetupInviteSchema.parse(input);

  return invite.singleUse && doesSetupInviteMatchTargetRole(invite);
}

export function authorizeSetupInvite(input: SetupInviteAuthorizationInput): SetupInviteDecision {
  const parsedInput = SetupInviteAuthorizationInputSchema.parse(input);

  if (parsedInput.inviteState !== SetupInviteState.Pending) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotActive);
  }

  if (!parsedInput.singleUse) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotSingleUse);
  }

  if (parsedInput.replayState !== SetupInviteReplayState.Fresh) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteReplayRejected);
  }

  if (parsedInput.abuseState === SetupRecoveryAbuseState.Throttled) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotActive);
  }

  if (parsedInput.responseTimingState !== SetupRecoveryResponseTimingState.Uniform) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotActive);
  }

  if (!parsedInput.sameFamily) {
    return rejectedSetupInvite(SetupInviteFailureReason.WrongHousehold);
  }

  if (!purposeMatchesTargetRole(parsedInput.purpose, parsedInput.targetRole)) {
    return rejectedSetupInvite(SetupInviteFailureReason.WrongTargetRole);
  }

  if (!inviterCanIssue(parsedInput.inviterRole, parsedInput.purpose)) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviterNotAuthorized);
  }

  return SetupInviteDecisionSchema.parse({
    decisionState: SetupInviteDecisionState.Acceptable,
    auditRequirementState: AuditRequirementState.Required,
    failureReason: null,
  });
}

export function recoveryRequiresOwnerApproval(input: RecoveryOperation): boolean {
  const recovery = RecoveryOperationSchema.parse(input);

  return requiresOwnerApproval(recovery.kind, recovery.ownerApprovalRequired);
}

export function recoveryDataCustodyHandoffState(input: RecoveryOperation): RecoveryDataCustodyHandoffState {
  const recovery = RecoveryOperationSchema.parse(input);

  return dataCustodyHandoffState(
    recovery.kind,
    recovery.deleteExportHandoffRequired,
    recovery.deleteExportState
  );
}

export function recoveryRequiresAuditedSupport(input: RecoveryOperation): boolean {
  const recovery = RecoveryOperationSchema.parse(input);

  return recovery.supportChannel === RecoverySupportChannel.SupportAssisted;
}

export function recoveryCanAccessChildEvidence(input: RecoveryOperation): boolean {
  const recovery = RecoveryOperationSchema.parse(input);
  const hasHouseholdAuthority =
    recovery.requesterMembershipState === HouseholdMembershipState.Active &&
    (recovery.requestedBy.role === ParentActorRole.Parent || recovery.requestedBy.role === ParentActorRole.Guardian);

  return (
    hasHouseholdAuthority &&
    recovery.identityProofState === RecoveryIdentityProofState.Verified &&
    recovery.supportChannel !== RecoverySupportChannel.SupportAssisted
  );
}

export function deviceTrustStateForRecoveryState(state: RecoveryState): typeof DeviceTrustState[keyof typeof DeviceTrustState] {
  const parsedState = RecoveryStateSchema.parse(state);

  if (
    parsedState === RecoveryState.PendingIdentityProof ||
    parsedState === RecoveryState.OwnerApprovalRequired ||
    parsedState === RecoveryState.Approved
  ) {
    return DeviceTrustState.ResetRequired;
  }

  if (parsedState === RecoveryState.Completed) {
    return DeviceTrustState.Pending;
  }

  return DeviceTrustState.Revoked;
}

export function deviceTrustStateForRecoveryOperation(
  input: RecoveryOperation
): typeof DeviceTrustState[keyof typeof DeviceTrustState] {
  const recovery = RecoveryOperationSchema.parse(input);

  if (recovery.state === RecoveryState.Revoked) {
    return DeviceTrustState.Revoked;
  }

  if (recovery.bundleFailureReason !== null) {
    return DeviceTrustState.ResetRequired;
  }

  switch (recovery.bundleState) {
    case RecoveryBundleState.PreviewOnly:
    case RecoveryBundleState.ApplyPending:
    case RecoveryBundleState.PartialRestore:
    case RecoveryBundleState.Rejected:
    case RecoveryBundleState.ManualRequired:
      return DeviceTrustState.ResetRequired;
    case RecoveryBundleState.Applied:
      return dataCustodyHandoffState(
        recovery.kind,
        recovery.deleteExportHandoffRequired,
        recovery.deleteExportState
      ) === RecoveryDataCustodyHandoffState.None && recovery.state === RecoveryState.Completed
        ? DeviceTrustState.Pending
        : DeviceTrustState.ResetRequired;
    default:
      return deviceTrustStateForRecoveryState(recovery.state);
  }
}

export function evaluateRecoveryOperation(input: RecoveryAuthorizationInput): RecoveryDecision {
  const parsedInput = RecoveryAuthorizationInputSchema.parse(input);
  const ownerApprovalRequired = requiresOwnerApproval(parsedInput.kind, parsedInput.ownerApprovalRequired);
  const childEvidenceAccessState = recoveryChildEvidenceAccessState(
    parsedInput.requesterRole,
    parsedInput.sameFamily,
    parsedInput.supportChannel
  );
  const custodyHandoffState = dataCustodyHandoffState(
    parsedInput.kind,
    parsedInput.deleteExportHandoffRequired,
    RecoveryDeleteExportState.None
  );

  if (parsedInput.state === RecoveryState.Revoked) {
    return rejectedRecovery(
      RecoveryFailureReason.RecoveryNotActive,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.requesterRole !== HouseholdRole.SupportAdmin && !parsedInput.sameFamily) {
    return rejectedRecovery(
      RecoveryFailureReason.WrongHousehold,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.identityProofState !== RecoveryIdentityProofState.Verified) {
    return rejectedRecovery(
      RecoveryFailureReason.IdentityProofRequired,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.abuseState === SetupRecoveryAbuseState.Throttled) {
    return rejectedRecovery(
      RecoveryFailureReason.IdentityProofRequired,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.responseTimingState !== SetupRecoveryResponseTimingState.Uniform) {
    return rejectedRecovery(
      RecoveryFailureReason.IdentityProofRequired,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (!requesterCanRecover(parsedInput.requesterRole, parsedInput.kind, parsedInput.supportChannel)) {
    return rejectedRecovery(
      RecoveryFailureReason.RoleNotAuthorized,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  return RecoveryDecisionSchema.parse({
    decisionState: RecoveryDecisionState.Authorized,
    ownerApprovalRequired,
    auditRequirementState: AuditRequirementState.Required,
    childEvidenceAccessState,
    dataCustodyHandoffState: custodyHandoffState,
    failureReason: null,
  });
}

function rejectedSetupInvite(failureReason: SetupInviteFailureReason): SetupInviteDecision {
  return SetupInviteDecisionSchema.parse({
    decisionState: SetupInviteDecisionState.Rejected,
    auditRequirementState: AuditRequirementState.Required,
    failureReason,
  });
}

function rejectedRecovery(
  failureReason: RecoveryFailureReason,
  ownerApprovalRequired: boolean,
  childEvidenceAccessState: RecoveryChildEvidenceAccessState,
  dataCustodyHandoffState: RecoveryDataCustodyHandoffState
): RecoveryDecision {
  return RecoveryDecisionSchema.parse({
    decisionState: RecoveryDecisionState.Rejected,
    ownerApprovalRequired,
    auditRequirementState: AuditRequirementState.Required,
    childEvidenceAccessState,
    dataCustodyHandoffState,
    failureReason,
  });
}

function purposeMatchesTargetRole(
  purpose: SetupInvitePurpose,
  targetRole: SetupInviteTargetRole | HouseholdRole
): boolean {
  return (
    (purpose === SetupInvitePurpose.CoParentInvite && targetRole === HouseholdRole.CoParentGuardian) ||
    (purpose === SetupInvitePurpose.ObserverInvite && targetRole === HouseholdRole.Observer) ||
    (purpose === SetupInvitePurpose.ChildDevicePairing && targetRole === HouseholdRole.ChildDeviceAgent) ||
    (purpose === SetupInvitePurpose.HouseholdTransfer && targetRole === HouseholdRole.ParentOwner)
  );
}

function inviterCanIssue(role: HouseholdRole, purpose: SetupInvitePurpose): boolean {
  switch (purpose) {
    case SetupInvitePurpose.CoParentInvite:
    case SetupInvitePurpose.ObserverInvite:
    case SetupInvitePurpose.ChildDevicePairing:
      return role === HouseholdRole.ParentOwner || role === HouseholdRole.CoParentGuardian;
    case SetupInvitePurpose.HouseholdTransfer:
      return role === HouseholdRole.ParentOwner;
    default:
      return false;
  }
}

function requesterCanRecover(
  role: HouseholdRole,
  kind: RecoveryKind,
  supportChannel: typeof RecoverySupportChannel[keyof typeof RecoverySupportChannel]
): boolean {
  if (role === HouseholdRole.SupportAdmin) {
    return supportChannel === RecoverySupportChannel.SupportAssisted;
  }

  if (kind === RecoveryKind.HouseholdTransfer) {
    return role === HouseholdRole.ParentOwner;
  }

  return role === HouseholdRole.ParentOwner || role === HouseholdRole.CoParentGuardian;
}

function requiresOwnerApproval(kind: RecoveryKind, ownerApprovalRequired: boolean): boolean {
  return (
    ownerApprovalRequired ||
    kind === RecoveryKind.LostParentDevice ||
    kind === RecoveryKind.CompromisedAccount ||
    kind === RecoveryKind.HouseholdTransfer
  );
}

function dataCustodyHandoffState(
  kind: RecoveryKind,
  deleteExportHandoffRequired: boolean,
  deleteExportState: RecoveryDeleteExportState
): RecoveryDataCustodyHandoffState {
  if (kind === RecoveryKind.HouseholdTransfer) {
    return RecoveryDataCustodyHandoffState.HouseholdTransferHandoffRequired;
  }

  if (deleteExportHandoffRequired && deleteExportState !== RecoveryDeleteExportState.DeleteConfirmed) {
    return RecoveryDataCustodyHandoffState.ExportDeleteHandoffRequired;
  }

  return RecoveryDataCustodyHandoffState.None;
}

function recoveryChildEvidenceAccessState(
  requesterRole: HouseholdRole,
  sameFamily: boolean,
  supportChannel: typeof RecoverySupportChannel[keyof typeof RecoverySupportChannel]
): RecoveryChildEvidenceAccessState {
  const hasHouseholdAuthority =
    sameFamily &&
    (requesterRole === HouseholdRole.ParentOwner || requesterRole === HouseholdRole.CoParentGuardian);

  if (hasHouseholdAuthority && supportChannel !== RecoverySupportChannel.SupportAssisted) {
    return RecoveryChildEvidenceAccessState.Allowed;
  }

  return RecoveryChildEvidenceAccessState.Blocked;
}
