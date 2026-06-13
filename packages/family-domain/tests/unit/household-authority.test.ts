import { describe, expect, it } from 'vitest';
import {
  canHouseholdRoleAuthorizeAction,
  DeviceAuthorityAction,
  DeviceRegistrationSchema,
  DeviceTrustState,
  HouseholdMembershipState,
  HouseholdProfileSchema,
  HouseholdRole,
  isActiveParentMember,
  ParentControllerLeaseSchema,
  ParentControllerLeaseState,
  ParentMemberSchema,
} from '../../src/household-authority';

describe('household authority contracts', () => {
  it('parses household, member, device registration, and controller lease contracts exactly', () => {
    expect(
      HouseholdProfileSchema.parse({
        schemaVersion: 'v0.6',
        householdProfileId: 'household-profile-main',
        family: { familyId: 'family-main' },
        displayName: 'Main Household',
        ownerAccount: { parentAccountId: 'parent-account-1' },
        createdAt: '2026-06-13T15:52:00.000Z',
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      householdProfileId: 'household-profile-main',
      family: { familyId: 'family-main' },
      displayName: 'Main Household',
      ownerAccount: { parentAccountId: 'parent-account-1' },
      createdAt: '2026-06-13T15:52:00.000Z',
    });

    expect(
      ParentMemberSchema.parse({
        schemaVersion: 'v0.6',
        membershipId: 'membership-main',
        family: { familyId: 'family-main' },
        account: { parentAccountId: 'parent-account-2' },
        role: 'co-parent-guardian',
        membershipState: 'active',
        displayName: 'Alex Guardian',
        invitedBy: { actorId: 'actor-owner', role: 'parent' },
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      membershipId: 'membership-main',
      family: { familyId: 'family-main' },
      account: { parentAccountId: 'parent-account-2' },
      role: 'co-parent-guardian',
      membershipState: 'active',
      displayName: 'Alex Guardian',
      invitedBy: { actorId: 'actor-owner', role: 'parent' },
    });

    expect(
      DeviceRegistrationSchema.parse({
        schemaVersion: 'v0.6',
        registrationId: 'registration-main',
        family: { familyId: 'family-main' },
        device: {
          deviceId: 'device-child-1',
          childProfileId: 'child-1',
          label: 'Sam Android',
          platform: 'android',
        },
        deviceRole: 'child-agent',
        trustState: 'trusted',
        registeredBy: { actorId: 'actor-owner', role: 'parent' },
        registeredAt: '2026-06-13T15:53:00.000Z',
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      registrationId: 'registration-main',
      family: { familyId: 'family-main' },
      device: {
        deviceId: 'device-child-1',
        childProfileId: 'child-1',
        label: 'Sam Android',
        platform: 'android',
      },
      deviceRole: 'child-agent',
      trustState: 'trusted',
      registeredBy: { actorId: 'actor-owner', role: 'parent' },
      registeredAt: '2026-06-13T15:53:00.000Z',
    });

    expect(
      ParentControllerLeaseSchema.parse({
        schemaVersion: 'v0.6',
        leaseId: 'lease-main',
        family: { familyId: 'family-main' },
        controller: { actorId: 'actor-owner', role: 'parent' },
        device: {
          deviceId: 'device-child-1',
          childProfileId: 'child-1',
          label: 'Sam Android',
          platform: 'android',
        },
        action: 'start-remote-control',
        state: 'active',
        issuedAt: '2026-06-13T15:54:00.000Z',
        expiresAt: '2026-06-13T16:54:00.000Z',
      })
    ).toEqual({
      schemaVersion: 'v0.6',
      leaseId: 'lease-main',
      family: { familyId: 'family-main' },
      controller: { actorId: 'actor-owner', role: 'parent' },
      device: {
        deviceId: 'device-child-1',
        childProfileId: 'child-1',
        label: 'Sam Android',
        platform: 'android',
      },
      action: 'start-remote-control',
      state: 'active',
      issuedAt: '2026-06-13T15:54:00.000Z',
      expiresAt: '2026-06-13T16:54:00.000Z',
    });
  });

  it('role authorization keeps write and billing authority out of observer and child roles', () => {
    expect(canHouseholdRoleAuthorizeAction(HouseholdRole.ParentOwner, DeviceAuthorityAction.ChangePolicy)).toBe(true);
    expect(
      canHouseholdRoleAuthorizeAction(HouseholdRole.CoParentGuardian, DeviceAuthorityAction.StartRemoteControl)
    ).toBe(true);
    expect(canHouseholdRoleAuthorizeAction(HouseholdRole.Observer, DeviceAuthorityAction.ViewChildStatus)).toBe(true);
    expect(canHouseholdRoleAuthorizeAction(HouseholdRole.Observer, DeviceAuthorityAction.ChangePolicy)).toBe(false);
    expect(canHouseholdRoleAuthorizeAction(HouseholdRole.ChildDeviceAgent, DeviceAuthorityAction.StartRemoteView)).toBe(
      false
    );
    expect(canHouseholdRoleAuthorizeAction(HouseholdRole.CoParentGuardian, DeviceAuthorityAction.ManageBilling)).toBe(
      false
    );
    expect(canHouseholdRoleAuthorizeAction(HouseholdRole.ParentOwner, DeviceAuthorityAction.ManageBilling)).toBe(true);
  });

  it('active parent member excludes child and device-agent identities even if their membership is active', () => {
    expect(
      isActiveParentMember(
        ParentMemberSchema.parse({
          schemaVersion: 'v0.6',
          membershipId: 'membership-parent',
          family: { familyId: 'family-main' },
          account: { parentAccountId: 'parent-account-2' },
          role: 'observer',
          membershipState: 'active',
          displayName: 'Morgan Observer',
          invitedBy: { actorId: 'actor-owner', role: 'parent' },
        })
      )
    ).toBe(true);

    expect(
      isActiveParentMember(
        ParentMemberSchema.parse({
          schemaVersion: 'v0.6',
          membershipId: 'membership-child-profile',
          family: { familyId: 'family-main' },
          account: { parentAccountId: 'child-profile-shadow' },
          role: 'child-profile',
          membershipState: 'active',
          displayName: 'Sam Profile',
          invitedBy: null,
        })
      )
    ).toBe(false);
  });

  it('schema boundary rejects unsupported roles, trust states, and lease states', () => {
    const member = ParentMemberSchema.safeParse({
      schemaVersion: 'v0.6',
      membershipId: 'membership-invalid',
      family: { familyId: 'family-main' },
      account: { parentAccountId: 'parent-account-2' },
      role: 'grandparent',
      membershipState: 'active',
      displayName: 'Invalid Role',
      invitedBy: null,
    });
    expect(member.success).toBe(false);

    const registration = DeviceRegistrationSchema.safeParse({
      schemaVersion: 'v0.6',
      registrationId: 'registration-invalid',
      family: { familyId: 'family-main' },
      device: {
        deviceId: 'device-child-1',
        childProfileId: 'child-1',
        label: 'Sam Android',
        platform: 'android',
      },
      deviceRole: 'child-agent',
      trustState: 'unknown',
      registeredBy: { actorId: 'actor-owner', role: 'parent' },
      registeredAt: '2026-06-13T15:55:00.000Z',
    });
    expect(registration.success).toBe(false);

    const lease = ParentControllerLeaseSchema.safeParse({
      schemaVersion: 'v0.6',
      leaseId: 'lease-invalid',
      family: { familyId: 'family-main' },
      controller: { actorId: 'actor-owner', role: 'parent' },
      device: {
        deviceId: 'device-child-1',
        childProfileId: 'child-1',
        label: 'Sam Android',
        platform: 'android',
      },
      action: DeviceAuthorityAction.StartRemoteView,
      state: 'paused',
      issuedAt: '2026-06-13T15:54:00.000Z',
      expiresAt: '2026-06-13T16:54:00.000Z',
    });
    expect(lease.success).toBe(false);

    expect(HouseholdMembershipState.Active).toBe('active');
    expect(DeviceTrustState.Trusted).toBe('trusted');
    expect(ParentControllerLeaseState.Active).toBe('active');
  });
});
