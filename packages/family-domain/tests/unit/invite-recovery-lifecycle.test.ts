import { describe, expect, it } from 'vitest';
import {
  AuditRequirementState,
  HouseholdRole,
} from '@ocentra-parent/schema-domain/family-household-authority';
import {
  RecoveryChildEvidenceAccessState,
  RecoveryDataCustodyHandoffState,
  RecoveryDecisionState,
  RecoveryFailureReason,
  RecoveryIdentityProofState,
  RecoveryKind,
  RecoveryOperationSchema,
  RecoveryState,
  RecoverySupportChannel,
} from '@ocentra-parent/schema-domain/family-restore-lifecycle';
import {
  SetupInviteDecisionState,
  SetupInviteFailureReason,
  SetupInvitePurpose,
  SetupInviteReplayState,
  SetupInviteSchema,
  SetupInviteState,
} from '@ocentra-parent/schema-domain/family-setup-invite';
import {
  authorizeSetupInvite,
  doesSetupInviteMatchTargetRole,
  evaluateRecoveryOperation,
  isSetupInviteActive,
  isSetupInviteSinglePurpose,
  recoveryCanAccessChildEvidence,
  recoveryDataCustodyHandoffState,
  recoveryRequiresAuditedSupport,
  recoveryRequiresOwnerApproval,
} from '../../src/setup-lifecycle';

function inviteAuthorizationInput(overrides: Record<string, unknown> = {}) {
  return {
    inviterRole: HouseholdRole.ParentOwner,
    sameFamily: true,
    purpose: SetupInvitePurpose.CoParentInvite,
    targetRole: HouseholdRole.CoParentGuardian,
    inviteState: SetupInviteState.Pending,
    singleUse: true,
    replayState: SetupInviteReplayState.Fresh,
    abuseState: 'within-limit',
    responseTimingState: 'uniform',
    ...overrides,
  };
}

function recoveryAuthorizationInput(overrides: Record<string, unknown> = {}) {
  return {
    requesterRole: HouseholdRole.ParentOwner,
    sameFamily: true,
    kind: RecoveryKind.ForgotLogin,
    state: RecoveryState.Approved,
    ownerApprovalRequired: false,
    identityProofState: RecoveryIdentityProofState.Verified,
    supportChannel: RecoverySupportChannel.SelfServe,
    deleteExportHandoffRequired: false,
    abuseState: 'within-limit',
    responseTimingState: 'uniform',
    ...overrides,
  };
}

describe('invite and recovery lifecycle command target', () => {
  it('keeps invite scopes distinct and rejects replay, reuse, wrong-household, and wrong-role cases', () => {
    const coParentInvite = SetupInviteSchema.parse({
      schemaVersion: 'v0.6',
      inviteId: 'invite-co-parent',
      family: { familyId: 'family-main' },
      invitedBy: { actorId: 'actor-owner', role: 'parent' },
      targetAccount: { parentAccountId: 'parent-account-2' },
      targetChildProfile: null,
      targetRole: HouseholdRole.CoParentGuardian,
      purpose: SetupInvitePurpose.CoParentInvite,
      state: SetupInviteState.Pending,
      expiresAt: '2026-06-14T15:00:00.000Z',
      singleUse: true,
    });

    const observerInvite = SetupInviteSchema.parse({
      ...coParentInvite,
      inviteId: 'invite-observer',
      targetRole: HouseholdRole.Observer,
      purpose: SetupInvitePurpose.ObserverInvite,
    });

    const childDeviceInvite = SetupInviteSchema.parse({
      ...coParentInvite,
      inviteId: 'invite-child-device',
      targetAccount: null,
      targetRole: HouseholdRole.ChildDeviceAgent,
      purpose: SetupInvitePurpose.ChildDevicePairing,
    });

    expect(isSetupInviteActive(coParentInvite)).toBe(true);
    expect(doesSetupInviteMatchTargetRole(coParentInvite)).toBe(true);
    expect(doesSetupInviteMatchTargetRole(observerInvite)).toBe(true);
    expect(doesSetupInviteMatchTargetRole(childDeviceInvite)).toBe(true);
    expect(isSetupInviteSinglePurpose(coParentInvite)).toBe(true);

    expect(authorizeSetupInvite(inviteAuthorizationInput())).toEqual({
      decisionState: SetupInviteDecisionState.Acceptable,
      auditRequirementState: AuditRequirementState.Required,
      failureReason: null,
    });

    expect(
      authorizeSetupInvite(inviteAuthorizationInput({ replayState: SetupInviteReplayState.ReplayDetected }))
        .failureReason
    ).toBe(SetupInviteFailureReason.InviteReplayRejected);
    expect(authorizeSetupInvite(inviteAuthorizationInput({ abuseState: 'throttled' })).failureReason).toBe(
      SetupInviteFailureReason.InviteNotActive
    );
    expect(authorizeSetupInvite(inviteAuthorizationInput({ responseTimingState: 'variable' })).failureReason).toBe(
      SetupInviteFailureReason.InviteNotActive
    );
    expect(authorizeSetupInvite(inviteAuthorizationInput({ inviteState: SetupInviteState.Expired })).failureReason).toBe(
      SetupInviteFailureReason.InviteNotActive
    );
    expect(authorizeSetupInvite(inviteAuthorizationInput({ inviteState: SetupInviteState.Revoked })).failureReason).toBe(
      SetupInviteFailureReason.InviteNotActive
    );
    expect(authorizeSetupInvite(inviteAuthorizationInput({ sameFamily: false })).failureReason).toBe(
      SetupInviteFailureReason.WrongHousehold
    );
    expect(
      authorizeSetupInvite(
        inviteAuthorizationInput({
          purpose: SetupInvitePurpose.ChildDevicePairing,
          targetRole: HouseholdRole.Observer,
        })
      ).failureReason
    ).toBe(SetupInviteFailureReason.WrongTargetRole);
    expect(authorizeSetupInvite(inviteAuthorizationInput({ singleUse: false })).failureReason).toBe(
      SetupInviteFailureReason.InviteNotSingleUse
    );
  });

  it('keeps recovery approval, support audit, child-evidence blocking, and custody handoff explicit', () => {
    const lostParentDevice = RecoveryOperationSchema.parse({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-lost-device',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
      requesterMembershipState: 'active',
      relatedAccount: { parentAccountId: 'parent-account-1' },
      relatedDevice: {
        deviceId: 'device-parent-1',
        childProfileId: null,
        label: 'Parent iPhone',
        platform: 'ios',
      },
      kind: RecoveryKind.LostParentDevice,
      state: 'owner-approval-required',
      ownerApprovalRequired: false,
      identityProofState: RecoveryIdentityProofState.Verified,
      supportChannel: RecoverySupportChannel.HouseholdOwnerAssisted,
      deleteExportHandoffRequired: false,
      openedAt: '2026-06-13T16:01:00.000Z',
      closedAt: null,
    });

    const transferRecovery = RecoveryOperationSchema.parse({
      ...lostParentDevice,
      recoveryOperationId: 'recovery-transfer',
      kind: RecoveryKind.HouseholdTransfer,
      deleteExportHandoffRequired: false,
    });

    const supportRecovery = RecoveryOperationSchema.parse({
      ...lostParentDevice,
      recoveryOperationId: 'recovery-support',
      kind: RecoveryKind.CompromisedAccount,
      state: 'approved',
      ownerApprovalRequired: true,
      supportChannel: RecoverySupportChannel.SupportAssisted,
    });

    const selfServeRecovery = RecoveryOperationSchema.parse({
      ...lostParentDevice,
      recoveryOperationId: 'recovery-self-serve',
      kind: RecoveryKind.ChildReinstall,
      state: 'approved',
      ownerApprovalRequired: false,
      supportChannel: RecoverySupportChannel.SelfServe,
    });

    const deleteExportRecovery = RecoveryOperationSchema.parse({
      ...lostParentDevice,
      recoveryOperationId: 'recovery-delete-export',
      kind: RecoveryKind.ForgotLogin,
      state: 'approved',
      deleteExportHandoffRequired: true,
      supportChannel: RecoverySupportChannel.SelfServe,
    });

    expect(recoveryRequiresOwnerApproval(lostParentDevice)).toBe(true);
    expect(recoveryRequiresOwnerApproval(transferRecovery)).toBe(true);
    expect(recoveryRequiresAuditedSupport(supportRecovery)).toBe(true);
    expect(recoveryCanAccessChildEvidence(supportRecovery)).toBe(false);
    expect(recoveryDataCustodyHandoffState(deleteExportRecovery)).toBe(
      RecoveryDataCustodyHandoffState.ExportDeleteHandoffRequired
    );
    expect(recoveryDataCustodyHandoffState(transferRecovery)).toBe(
      RecoveryDataCustodyHandoffState.HouseholdTransferHandoffRequired
    );
    expect(recoveryCanAccessChildEvidence(selfServeRecovery)).toBe(true);

    expect(
      evaluateRecoveryOperation(
        recoveryAuthorizationInput({
          kind: RecoveryKind.ChildReinstall,
          state: RecoveryState.Approved,
          supportChannel: RecoverySupportChannel.SelfServe,
        })
      )
    ).toEqual({
      decisionState: RecoveryDecisionState.Authorized,
      ownerApprovalRequired: false,
      auditRequirementState: AuditRequirementState.Required,
      childEvidenceAccessState: RecoveryChildEvidenceAccessState.Allowed,
      dataCustodyHandoffState: RecoveryDataCustodyHandoffState.None,
      failureReason: null,
    });

    expect(
      evaluateRecoveryOperation(
        recoveryAuthorizationInput({
          requesterRole: HouseholdRole.SupportAdmin,
          sameFamily: false,
          kind: RecoveryKind.CompromisedAccount,
          state: RecoveryState.Approved,
          ownerApprovalRequired: true,
          supportChannel: RecoverySupportChannel.SupportAssisted,
        })
      )
    ).toEqual({
      decisionState: RecoveryDecisionState.Authorized,
      ownerApprovalRequired: true,
      auditRequirementState: AuditRequirementState.Required,
      childEvidenceAccessState: RecoveryChildEvidenceAccessState.Blocked,
      dataCustodyHandoffState: RecoveryDataCustodyHandoffState.None,
      failureReason: null,
    });
    expect(
      evaluateRecoveryOperation(
        recoveryAuthorizationInput({
          abuseState: 'throttled',
          responseTimingState: 'variable',
          kind: RecoveryKind.ForgotLogin,
          state: RecoveryState.Approved,
        })
      ).failureReason
    ).toBe(RecoveryFailureReason.IdentityProofRequired);
    expect(
      evaluateRecoveryOperation(
        recoveryAuthorizationInput({
          responseTimingState: 'variable',
          kind: RecoveryKind.ForgotLogin,
          state: RecoveryState.Approved,
        })
      ).failureReason
    ).toBe(RecoveryFailureReason.IdentityProofRequired);

    expect(
      evaluateRecoveryOperation(
        recoveryAuthorizationInput({
          sameFamily: false,
          kind: RecoveryKind.ForgotLogin,
          state: RecoveryState.PendingIdentityProof,
        })
      ).failureReason
    ).toBe(RecoveryFailureReason.WrongHousehold);
    expect(
      evaluateRecoveryOperation(
        recoveryAuthorizationInput({
          state: RecoveryState.Revoked,
          kind: RecoveryKind.ForgotLogin,
        })
      ).failureReason
    ).toBe(RecoveryFailureReason.RecoveryNotActive);
  });
});
