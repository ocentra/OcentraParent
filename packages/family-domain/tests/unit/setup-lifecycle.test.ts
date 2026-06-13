import { describe, expect, it } from 'vitest';
import { HouseholdRole } from '../../src/household-authority';
import {
  doesSetupInviteMatchTargetRole,
  isSetupInviteActive,
  RecoveryKind,
  RecoveryOperationSchema,
  recoveryRequiresOwnerApproval,
  SetupAuditEventSchema,
  SetupInvitePurpose,
  SetupInviteSchema,
  SetupInviteState,
} from '../../src/setup-lifecycle';

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
        relatedAccount: { parentAccountId: 'parent-account-1' },
        relatedDevice: null,
        kind: 'lost-parent-device',
        state: 'owner-approval-required',
        ownerApprovalRequired: true,
        openedAt: '2026-06-13T15:58:00.000Z',
        closedAt: null,
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-main',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
      relatedAccount: { parentAccountId: 'parent-account-1' },
      relatedDevice: null,
      kind: 'lost-parent-device',
      state: 'owner-approval-required',
      ownerApprovalRequired: true,
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
  });

  it('lost-parent-device and household-transfer recovery remain owner-approved paths', () => {
    const lostParentDevice = RecoveryOperationSchema.parse({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-lost-device',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
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

  it('schema boundary rejects unknown recovery states and malformed invite purposes', () => {
    const badRecovery = RecoveryOperationSchema.safeParse({
      schemaVersion: 'v0.6',
      recoveryOperationId: 'recovery-invalid',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: 'parent' },
      relatedAccount: null,
      relatedDevice: null,
      kind: 'compromised-account',
      state: 'escalated',
      ownerApprovalRequired: true,
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
