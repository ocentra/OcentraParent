import { describe, expect, it } from 'vitest';
import { AuditRequirementState, HouseholdRole } from '../../src/household-authority';
import {
  authorizeSetupInvite,
  doesSetupInviteMatchTargetRole,
  evaluateRecoveryOperation,
  isSetupInviteActive,
  isSetupInviteSinglePurpose,
  RecoveryChildEvidenceAccessState,
  RecoveryDecisionState,
  RecoveryFailureReason,
  RecoveryKind,
  RecoveryDataCustodyHandoffState,
  recoveryCanAccessChildEvidence,
  recoveryDataCustodyHandoffState,
  RecoveryIdentityProofState,
  RecoveryOperationSchema,
  recoveryRequiresAuditedSupport,
  recoveryRequiresOwnerApproval,
  RecoveryState,
  RecoverySupportChannel,
  SetupInviteDecisionState,
  SetupInviteFailureReason,
  SetupAuditEventSchema,
  SetupInvitePurpose,
  SetupInviteReplayState,
  SetupInviteSchema,
  SetupInviteState,
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

describe('family setup lifecycle contracts', () => {
  it('parses setup invite, recovery operation, and setup audit event contracts exactly', () => {
    expect(
      SetupInviteSchema.parse({
        schemaVersion: 'v0.6',
        inviteId: 'invite-main',
        family: { familyId: 'family-main' },
        invitedBy: { actorId: 'actor-owner', role: 'parent' },
        targetAccount: { parentAccountId: 'parent-account-2' },
        targetChildProfile: null,
        targetRole: 'co-parent-guardian',
        purpose: 'co-parent-invite',
        state: 'pending',
        expiresAt: '2026-06-14T15:00:00.000Z',
        singleUse: true,
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      inviteId: 'invite-main',
      family: { familyId: 'family-main' },
      invitedBy: { actorId: 'actor-owner', role: 'parent' },
      targetAccount: { parentAccountId: 'parent-account-2' },
      targetChildProfile: null,
      targetRole: 'co-parent-guardian',
      purpose: 'co-parent-invite',
      state: 'pending',
      expiresAt: '2026-06-14T15:00:00.000Z',
      singleUse: true,
    });

    expect(
      RecoveryOperationSchema.parse({
        schemaVersion: 'v0.6',
        recoveryOperationId: 'recovery-main',
        family: { familyId: 'family-main' },
        requestedBy: { actorId: 'actor-owner', role: 'parent' },
        requesterMembershipState: 'active',
        relatedAccount: { parentAccountId: 'parent-account-1' },
        relatedDevice: null,
        kind: 'lost-parent-device',
        state: 'owner-approval-required',
        ownerApprovalRequired: true,
        identityProofState: 'verified',
        supportChannel: 'household-owner-assisted',
        deleteExportHandoffRequired: false,
        openedAt: '2026-06-13T15:58:00.000Z',
        closedAt: null,
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-main',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
      requesterMembershipState: 'active',
      relatedAccount: { parentAccountId: 'parent-account-1' },
      relatedDevice: null,
      kind: 'lost-parent-device',
      state: 'owner-approval-required',
      ownerApprovalRequired: true,
      identityProofState: 'verified',
      supportChannel: 'household-owner-assisted',
      deleteExportHandoffRequired: false,
      openedAt: '2026-06-13T15:58:00.000Z',
      closedAt: null,
    });

    expect(
      SetupAuditEventSchema.parse({
        schemaVersion: 'v0.6',
        auditEventId: 'audit-main',
        family: { familyId: 'family-main' },
        actor: { actorId: 'actor-owner', role: 'parent' },
        kind: 'device-paired',
        childProfile: { childProfileId: 'child-1', displayName: 'Sam' },
        device: {
          deviceId: 'device-child-1',
          childProfileId: 'child-1',
          label: 'Sam Android',
          platform: 'android',
        },
        action: null,
        observedAt: '2026-06-13T16:00:00.000Z',
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      auditEventId: 'audit-main',
      family: { familyId: 'family-main' },
      actor: { actorId: 'actor-owner', role: 'parent' },
      kind: 'device-paired',
      childProfile: { childProfileId: 'child-1', displayName: 'Sam' },
      device: {
        deviceId: 'device-child-1',
        childProfileId: 'child-1',
        label: 'Sam Android',
        platform: 'android',
      },
      action: null,
      observedAt: '2026-06-13T16:00:00.000Z',
    });
  });

  it('setup invites are active only while pending and their purpose must match the target role', () => {
    const activeCoParentInvite = SetupInviteSchema.parse({
      schemaVersion: 'v0.6',
      inviteId: 'invite-active',
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

    expect(isSetupInviteActive(activeCoParentInvite)).toBe(true);
    expect(doesSetupInviteMatchTargetRole(activeCoParentInvite)).toBe(true);
    expect(isSetupInviteSinglePurpose(activeCoParentInvite)).toBe(true);

    const revokedObserverInvite = SetupInviteSchema.parse({
      ...activeCoParentInvite,
      inviteId: 'invite-revoked',
      targetRole: HouseholdRole.Observer,
      purpose: SetupInvitePurpose.ObserverInvite,
      state: SetupInviteState.Revoked,
    });

    expect(isSetupInviteActive(revokedObserverInvite)).toBe(false);
    expect(doesSetupInviteMatchTargetRole(revokedObserverInvite)).toBe(true);

    const wrongRoleInvite = SetupInviteSchema.parse({
      ...activeCoParentInvite,
      inviteId: 'invite-wrong-role',
      targetRole: HouseholdRole.Observer,
      purpose: SetupInvitePurpose.ChildDevicePairing,
      targetAccount: null,
    });

    expect(doesSetupInviteMatchTargetRole(wrongRoleInvite)).toBe(false);
    expect(isSetupInviteSinglePurpose(wrongRoleInvite)).toBe(false);

    const reusableInvite = SetupInviteSchema.parse({
      ...activeCoParentInvite,
      inviteId: 'invite-reusable',
      singleUse: false,
    });

    expect(isSetupInviteSinglePurpose(reusableInvite)).toBe(false);
  });

  it('authorizes only fresh single-use matching invites from the right household and inviter role', () => {
    const accepted = authorizeSetupInvite(inviteAuthorizationInput());

    expect(accepted).toEqual({
      decisionState: SetupInviteDecisionState.Acceptable,
      auditRequirementState: AuditRequirementState.Required,
      failureReason: null,
    });

    const replayed = authorizeSetupInvite(
      inviteAuthorizationInput({ replayState: SetupInviteReplayState.ReplayDetected })
    );
    expect(replayed.failureReason).toBe(SetupInviteFailureReason.InviteReplayRejected);

    const expired = authorizeSetupInvite(inviteAuthorizationInput({ inviteState: SetupInviteState.Expired }));
    expect(expired.failureReason).toBe(SetupInviteFailureReason.InviteNotActive);

    const revoked = authorizeSetupInvite(inviteAuthorizationInput({ inviteState: SetupInviteState.Revoked }));
    expect(revoked.failureReason).toBe(SetupInviteFailureReason.InviteNotActive);

    const observerInvite = authorizeSetupInvite(
      inviteAuthorizationInput({
        purpose: SetupInvitePurpose.ObserverInvite,
        targetRole: HouseholdRole.Observer,
      })
    );
    expect(observerInvite.failureReason).toBe(null);

    const childDevicePairing = authorizeSetupInvite(
      inviteAuthorizationInput({
        purpose: SetupInvitePurpose.ChildDevicePairing,
        targetRole: HouseholdRole.ChildDeviceAgent,
      })
    );
    expect(childDevicePairing.failureReason).toBe(null);

    const wrongHousehold = authorizeSetupInvite(inviteAuthorizationInput({ sameFamily: false }));
    expect(wrongHousehold.failureReason).toBe(SetupInviteFailureReason.WrongHousehold);

    const wrongRole = authorizeSetupInvite(
      inviteAuthorizationInput({
        purpose: SetupInvitePurpose.ChildDevicePairing,
        targetRole: HouseholdRole.Observer,
      })
    );
    expect(wrongRole.failureReason).toBe(SetupInviteFailureReason.WrongTargetRole);

    const unauthorizedTransfer = authorizeSetupInvite(
      inviteAuthorizationInput({
        inviterRole: HouseholdRole.CoParentGuardian,
        purpose: SetupInvitePurpose.HouseholdTransfer,
        targetRole: HouseholdRole.ParentOwner,
      })
    );
    expect(unauthorizedTransfer.failureReason).toBe(SetupInviteFailureReason.InviterNotAuthorized);
  });

  it('lost-parent-device and household-transfer recovery remain owner-approved paths', () => {
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

    expect(recoveryRequiresOwnerApproval(lostParentDevice)).toBe(true);

    const childReinstall = RecoveryOperationSchema.parse({
      ...lostParentDevice,
      recoveryOperationId: 'recovery-child-reinstall',
      relatedAccount: null,
      kind: RecoveryKind.ChildReinstall,
      state: 'approved',
      ownerApprovalRequired: false,
    });

    expect(recoveryRequiresOwnerApproval(childReinstall)).toBe(false);
  });

  it('recovery delete export handoff routes through data custody exactly', () => {
    const deleteExportRecovery = RecoveryOperationSchema.parse({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-delete-export',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
      requesterMembershipState: 'active',
      relatedAccount: { parentAccountId: 'parent-account-1' },
      relatedDevice: null,
      kind: RecoveryKind.ForgotLogin,
      state: 'approved',
      ownerApprovalRequired: false,
      identityProofState: RecoveryIdentityProofState.Verified,
      supportChannel: RecoverySupportChannel.SelfServe,
      deleteExportHandoffRequired: true,
      openedAt: '2026-06-13T16:02:00.000Z',
      closedAt: null,
    });

    expect(recoveryDataCustodyHandoffState(deleteExportRecovery)).toBe(
      RecoveryDataCustodyHandoffState.ExportDeleteHandoffRequired
    );

    const transferRecovery = RecoveryOperationSchema.parse({
      ...deleteExportRecovery,
      recoveryOperationId: 'recovery-transfer',
      kind: RecoveryKind.HouseholdTransfer,
      deleteExportHandoffRequired: false,
    });

    expect(recoveryDataCustodyHandoffState(transferRecovery)).toBe(
      RecoveryDataCustodyHandoffState.HouseholdTransferHandoffRequired
    );
  });

  it('evaluates recovery authorization with owner-approval, support, and failure parity', () => {
    const lostParentDevice = evaluateRecoveryOperation(
      recoveryAuthorizationInput({
        kind: RecoveryKind.LostParentDevice,
        state: RecoveryState.OwnerApprovalRequired,
        supportChannel: RecoverySupportChannel.HouseholdOwnerAssisted,
      })
    );

    expect(lostParentDevice).toEqual({
      decisionState: RecoveryDecisionState.Authorized,
      ownerApprovalRequired: true,
      auditRequirementState: AuditRequirementState.Required,
      childEvidenceAccessState: RecoveryChildEvidenceAccessState.Allowed,
      dataCustodyHandoffState: RecoveryDataCustodyHandoffState.None,
      failureReason: null,
    });

    const supportAssisted = evaluateRecoveryOperation(
      recoveryAuthorizationInput({
        requesterRole: HouseholdRole.SupportAdmin,
        sameFamily: false,
        kind: RecoveryKind.CompromisedAccount,
        supportChannel: RecoverySupportChannel.SupportAssisted,
      })
    );
    expect(supportAssisted.decisionState).toBe(RecoveryDecisionState.Authorized);
    expect(supportAssisted.childEvidenceAccessState).toBe(RecoveryChildEvidenceAccessState.Blocked);

    const missingIdentityProof = evaluateRecoveryOperation(
      recoveryAuthorizationInput({
        kind: RecoveryKind.HouseholdTransfer,
        identityProofState: RecoveryIdentityProofState.Pending,
      })
    );
    expect(missingIdentityProof.decisionState).toBe(RecoveryDecisionState.Rejected);
    expect(missingIdentityProof.ownerApprovalRequired).toBe(true);
    expect(missingIdentityProof.dataCustodyHandoffState).toBe(
      RecoveryDataCustodyHandoffState.HouseholdTransferHandoffRequired
    );
    expect(missingIdentityProof.failureReason).toBe(RecoveryFailureReason.IdentityProofRequired);

    const householdTransfer = evaluateRecoveryOperation(
      recoveryAuthorizationInput({
        kind: RecoveryKind.HouseholdTransfer,
      })
    );
    expect(householdTransfer.decisionState).toBe(RecoveryDecisionState.Authorized);
    expect(householdTransfer.ownerApprovalRequired).toBe(true);
    expect(householdTransfer.dataCustodyHandoffState).toBe(
      RecoveryDataCustodyHandoffState.HouseholdTransferHandoffRequired
    );

    const childReinstall = evaluateRecoveryOperation(
      recoveryAuthorizationInput({
        kind: RecoveryKind.ChildReinstall,
      })
    );
    expect(childReinstall.decisionState).toBe(RecoveryDecisionState.Authorized);
    expect(childReinstall.ownerApprovalRequired).toBe(false);

    const wrongHousehold = evaluateRecoveryOperation(recoveryAuthorizationInput({ sameFamily: false }));
    expect(wrongHousehold.failureReason).toBe(RecoveryFailureReason.WrongHousehold);

    const observerRejected = evaluateRecoveryOperation(
      recoveryAuthorizationInput({ requesterRole: HouseholdRole.Observer })
    );
    expect(observerRejected.failureReason).toBe(RecoveryFailureReason.RoleNotAuthorized);

    const revoked = evaluateRecoveryOperation(recoveryAuthorizationInput({ state: RecoveryState.Revoked }));
    expect(revoked.failureReason).toBe(RecoveryFailureReason.RecoveryNotActive);
  });

  it('support-assisted recovery is audited and does not expose child evidence access', () => {
    const supportRecovery = RecoveryOperationSchema.parse({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-support',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
      requesterMembershipState: 'active',
      relatedAccount: { parentAccountId: 'parent-account-1' },
      relatedDevice: null,
      kind: RecoveryKind.CompromisedAccount,
      state: 'owner-approval-required',
      ownerApprovalRequired: true,
      identityProofState: RecoveryIdentityProofState.Verified,
      supportChannel: RecoverySupportChannel.SupportAssisted,
      deleteExportHandoffRequired: false,
      openedAt: '2026-06-13T16:03:00.000Z',
      closedAt: null,
    });

    expect(recoveryRequiresAuditedSupport(supportRecovery)).toBe(true);
    expect(recoveryCanAccessChildEvidence(supportRecovery)).toBe(false);

    const selfServeRecovery = RecoveryOperationSchema.parse({
      ...supportRecovery,
      recoveryOperationId: 'recovery-self-serve',
      supportChannel: RecoverySupportChannel.SelfServe,
      ownerApprovalRequired: false,
      kind: RecoveryKind.ForgotLogin,
      state: 'approved',
    });

    expect(recoveryRequiresAuditedSupport(selfServeRecovery)).toBe(false);
    expect(recoveryCanAccessChildEvidence(selfServeRecovery)).toBe(true);
  });

  it('schema boundary rejects unknown recovery states and malformed invite purposes', () => {
    const badRecovery = RecoveryOperationSchema.safeParse({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-invalid',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
      requesterMembershipState: 'active',
      relatedAccount: null,
      relatedDevice: null,
      kind: 'compromised-account',
      state: 'escalated',
      ownerApprovalRequired: true,
      identityProofState: 'verified',
      supportChannel: 'self-serve',
      deleteExportHandoffRequired: false,
      openedAt: '2026-06-13T16:01:00.000Z',
      closedAt: null,
    });
    expect(badRecovery.success).toBe(false);

    const badInvite = SetupInviteSchema.safeParse({
      schemaVersion: 'v0.6',
      inviteId: 'invite-invalid',
      family: { familyId: 'family-main' },
      invitedBy: { actorId: 'actor-owner', role: 'parent' },
      targetAccount: null,
      targetChildProfile: null,
      targetRole: HouseholdRole.ParentOwner,
      purpose: 'observer-write',
      state: SetupInviteState.Pending,
      expiresAt: '2026-06-14T15:00:00.000Z',
      singleUse: true,
    });
    expect(badInvite.success).toBe(false);
  });
});
