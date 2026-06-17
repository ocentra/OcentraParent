import { describe, expect, it } from 'vitest';
import { ChildProfileSchema } from '../../src/child-profile';
import {
  ActorAccountState,
  AuditRequirementState,
  authorizeHouseholdAction,
  canParentMemberAuthorizeDeviceAction,
  canHouseholdRoleAuthorizeAction,
  ChildProfileBindingState,
  DeviceAuthorityAction,
  DeviceOwnershipScope,
  DeviceRegistrationSchema,
  DeviceTrustState,
  isTrustedDeviceState,
  ElevatedConfirmationState,
  HouseholdAuthorityDecisionSchema,
  HouseholdAuthorityInputSchema,
  HouseholdAuthorizationFailureReason,
  HouseholdAuthorizationState,
  HouseholdMembershipState,
  HouseholdProfileSchema,
  HouseholdRole,
  isTrustedChildAgentRegistrationForProfile,
  isActiveParentMember,
  ParentControllerLeaseSchema,
  ParentControllerLeaseState,
  ParentMemberSchema,
  SessionFreshnessState,
} from '../../src/household-authority';

describe('household authority contracts', () => {
  const trustedParentAuthorityInput = {
    actorRole: HouseholdRole.ParentOwner,
    actorAccountState: ActorAccountState.Active,
    sameFamily: true,
    membershipState: HouseholdMembershipState.Active,
    childProfileBindingState: ChildProfileBindingState.Bound,
    deviceOwnershipScope: DeviceOwnershipScope.ChildProfileDevice,
    deviceTrustState: DeviceTrustState.Trusted,
    sessionFreshnessState: SessionFreshnessState.Fresh,
    capabilityGranted: true,
    action: DeviceAuthorityAction.ChangePolicy,
  } as const;

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

    expect(
      HouseholdAuthorityInputSchema.parse({
        ...trustedParentAuthorityInput,
        controllerLeaseState: ParentControllerLeaseState.Active,
        action: DeviceAuthorityAction.StartRemoteControl,
      })
    ).toEqual({
      ...trustedParentAuthorityInput,
      controllerLeaseState: ParentControllerLeaseState.Active,
      action: DeviceAuthorityAction.StartRemoteControl,
    });

    expect(
      HouseholdAuthorityDecisionSchema.parse({
        authorizationState: 'authorized',
        auditRequirementState: 'required',
        elevatedConfirmationState: 'required',
        failureReason: null,
      })
    ).toEqual({
      authorizationState: 'authorized',
      auditRequirementState: 'required',
      elevatedConfirmationState: 'required',
      failureReason: null,
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

  it('separates user account state from household membership when authorizing member device actions', () => {
    const ownerMember = ParentMemberSchema.parse({
      schemaVersion: 'v0.6',
      membershipId: 'membership-owner',
      family: { familyId: 'family-main' },
      account: { parentAccountId: 'parent-account-owner' },
      role: 'parent-owner',
      membershipState: 'active',
      displayName: 'Owner',
      invitedBy: null,
    });

    expect(
      canParentMemberAuthorizeDeviceAction(
        ownerMember,
        { familyId: 'family-main' },
        DeviceAuthorityAction.ChangePolicy,
        ActorAccountState.Active
      )
    ).toBe(true);
    expect(
      canParentMemberAuthorizeDeviceAction(
        ownerMember,
        { familyId: 'family-other' },
        DeviceAuthorityAction.ChangePolicy,
        ActorAccountState.Active
      )
    ).toBe(false);

    expect(
      canParentMemberAuthorizeDeviceAction(
        ownerMember,
        { familyId: 'family-main' },
        DeviceAuthorityAction.ChangePolicy,
        ActorAccountState.Disabled
      )
    ).toBe(false);

    expect(
      canParentMemberAuthorizeDeviceAction(
        ParentMemberSchema.parse({
          ...ownerMember,
          membershipId: 'membership-owner-revoked',
          membershipState: 'revoked',
        }),
        { familyId: 'family-main' },
        DeviceAuthorityAction.ChangePolicy,
        ActorAccountState.Active
      )
    ).toBe(false);

    expect(
      canParentMemberAuthorizeDeviceAction(
        ParentMemberSchema.parse({
          ...ownerMember,
          membershipId: 'membership-support-admin',
          role: 'support-admin',
        }),
        { familyId: 'family-main' },
        DeviceAuthorityAction.ViewChildStatus,
        ActorAccountState.Active
      )
    ).toBe(false);
  });

  it('preserves cross-family denial when the target family comes from the child-profile contract', () => {
    const childProfile = ChildProfileSchema.parse({
      schemaVersion: 'v0.6',
      childProfileId: 'child-profile-main',
      family: { familyId: 'family-main' },
      displayName: 'Sam Profile',
      createdBy: { actorId: 'actor-owner', role: 'parent' },
      createdAt: '2026-06-13T15:51:00.000Z',
    });

    const externalOwner = ParentMemberSchema.parse({
      schemaVersion: 'v0.6',
      membershipId: 'membership-external-owner',
      family: { familyId: 'family-other' },
      account: { parentAccountId: 'parent-account-external' },
      role: 'parent-owner',
      membershipState: 'active',
      displayName: 'External Owner',
      invitedBy: null,
    });

    expect(
      canParentMemberAuthorizeDeviceAction(
        externalOwner,
        childProfile.family,
        DeviceAuthorityAction.ViewChildStatus,
        ActorAccountState.Active
      )
    ).toBe(false);

    const crossFamilyDecision = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      sameFamily: false,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(crossFamilyDecision.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(crossFamilyDecision.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(crossFamilyDecision.failureReason).toBe(HouseholdAuthorizationFailureReason.ExternalHousehold);
  });

  it('household action matrix requires trusted device, fresh session, and capability for privileged paths', () => {
    const billingDecision = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      action: DeviceAuthorityAction.ManageBilling,
    });

    expect(billingDecision.authorizationState).toBe(HouseholdAuthorizationState.Authorized);
    expect(billingDecision.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(billingDecision.elevatedConfirmationState).toBe(ElevatedConfirmationState.Required);
    expect(billingDecision.failureReason).toBe(null);

    const remoteViewDecision = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.CoParentGuardian,
      capabilityGranted: false,
      action: DeviceAuthorityAction.StartRemoteView,
    });

    expect(remoteViewDecision.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(remoteViewDecision.failureReason).toBe(HouseholdAuthorizationFailureReason.MissingCapabilityGrant);

    const staleRemoteControl = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.CoParentGuardian,
      sessionFreshnessState: SessionFreshnessState.Stale,
      action: DeviceAuthorityAction.StartRemoteControl,
    });

    expect(staleRemoteControl.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(staleRemoteControl.failureReason).toBe(HouseholdAuthorizationFailureReason.SessionNotFresh);
    expect(staleRemoteControl.elevatedConfirmationState).toBe(ElevatedConfirmationState.Required);
  });

  it('requires an active controller lease for remote-sensitive actions once the base matrix passes', () => {
    const activeRemoteControl = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.CoParentGuardian,
      controllerLeaseState: ParentControllerLeaseState.Active,
      action: DeviceAuthorityAction.StartRemoteControl,
    });

    expect(activeRemoteControl.authorizationState).toBe(HouseholdAuthorizationState.Authorized);
    expect(activeRemoteControl.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(activeRemoteControl.elevatedConfirmationState).toBe(ElevatedConfirmationState.Required);
    expect(activeRemoteControl.failureReason).toBe(null);

    const missingLease = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.CoParentGuardian,
      action: DeviceAuthorityAction.StartRemoteView,
    });

    expect(missingLease.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(missingLease.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(missingLease.elevatedConfirmationState).toBe(ElevatedConfirmationState.NotRequired);
    expect(missingLease.failureReason).toBe(HouseholdAuthorizationFailureReason.ControllerLeaseRequired);

    const expiredLease = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.CoParentGuardian,
      controllerLeaseState: ParentControllerLeaseState.Expired,
      action: DeviceAuthorityAction.StartRemoteView,
    });

    expect(expiredLease.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(expiredLease.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(expiredLease.elevatedConfirmationState).toBe(ElevatedConfirmationState.NotRequired);
    expect(expiredLease.failureReason).toBe(HouseholdAuthorizationFailureReason.ControllerLeaseExpired);

    const revokedLease = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.CoParentGuardian,
      controllerLeaseState: ParentControllerLeaseState.Revoked,
      action: DeviceAuthorityAction.StartRemoteControl,
    });

    expect(revokedLease.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(revokedLease.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(revokedLease.elevatedConfirmationState).toBe(ElevatedConfirmationState.Required);
    expect(revokedLease.failureReason).toBe(HouseholdAuthorizationFailureReason.ControllerLeaseRevoked);
  });

  it('separates parent-controller authority from child-agent and observer authority', () => {
    const observerView = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.Observer,
      capabilityGranted: false,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(observerView.authorizationState).toBe(HouseholdAuthorizationState.Authorized);
    expect(observerView.auditRequirementState).toBe(AuditRequirementState.NotRequired);
    expect(observerView.failureReason).toBe(null);

    const observerPolicy = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.Observer,
      action: DeviceAuthorityAction.ChangePolicy,
    });

    expect(observerPolicy.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(observerPolicy.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(observerPolicy.failureReason).toBe(HouseholdAuthorizationFailureReason.RoleNotAuthorized);

    const observerRemoteControl = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.Observer,
      action: DeviceAuthorityAction.StartRemoteControl,
    });

    expect(observerRemoteControl.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(observerRemoteControl.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(observerRemoteControl.failureReason).toBe(HouseholdAuthorizationFailureReason.RoleNotAuthorized);

    const childAgentRemoteView = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.ChildDeviceAgent,
      action: DeviceAuthorityAction.StartRemoteView,
    });

    expect(childAgentRemoteView.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(childAgentRemoteView.failureReason).toBe(HouseholdAuthorizationFailureReason.RoleNotAuthorized);
  });

  it('keeps support-admin access minimized and audits denied child-data reads', () => {
    const supportAdminRead = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorRole: HouseholdRole.SupportAdmin,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(supportAdminRead.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(supportAdminRead.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(supportAdminRead.failureReason).toBe(HouseholdAuthorizationFailureReason.RoleNotAuthorized);
  });

  it('denies revoked, disabled, missing-scope, or wrong-family authority before action execution', () => {
    const wrongFamily = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      sameFamily: false,
      action: DeviceAuthorityAction.PairChildDevice,
    });

    expect(wrongFamily.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(wrongFamily.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(wrongFamily.failureReason).toBe(HouseholdAuthorizationFailureReason.ExternalHousehold);

    const revokedDevice = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      deviceTrustState: DeviceTrustState.Revoked,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(revokedDevice.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(revokedDevice.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(revokedDevice.failureReason).toBe(HouseholdAuthorizationFailureReason.DeviceNotTrusted);

    const pendingDevice = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      deviceTrustState: DeviceTrustState.Pending,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(pendingDevice.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(pendingDevice.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(pendingDevice.failureReason).toBe(HouseholdAuthorizationFailureReason.DeviceNotTrusted);

    const disabledMember = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      membershipState: HouseholdMembershipState.Disabled,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(disabledMember.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(disabledMember.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(disabledMember.failureReason).toBe(HouseholdAuthorizationFailureReason.MembershipNotActive);

    const disabledAccount = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      actorAccountState: ActorAccountState.Disabled,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(disabledAccount.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(disabledAccount.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(disabledAccount.failureReason).toBe(HouseholdAuthorizationFailureReason.AccountNotActive);

    const disabledDevice = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      deviceTrustState: DeviceTrustState.Disabled,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(disabledDevice.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(disabledDevice.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(disabledDevice.failureReason).toBe(HouseholdAuthorizationFailureReason.DeviceNotTrusted);

    const resetRequiredDevice = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      deviceTrustState: DeviceTrustState.ResetRequired,
      action: DeviceAuthorityAction.ViewChildStatus,
    });

    expect(resetRequiredDevice.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(resetRequiredDevice.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(resetRequiredDevice.failureReason).toBe(HouseholdAuthorizationFailureReason.DeviceNotTrusted);

    const wrongScope = authorizeHouseholdAction({
      ...trustedParentAuthorityInput,
      deviceOwnershipScope: DeviceOwnershipScope.OtherDevice,
      action: DeviceAuthorityAction.PairChildDevice,
    });

    expect(wrongScope.authorizationState).toBe(HouseholdAuthorizationState.Rejected);
    expect(wrongScope.auditRequirementState).toBe(AuditRequirementState.Required);
    expect(wrongScope.failureReason).toBe(HouseholdAuthorizationFailureReason.WrongDeviceScope);
  });

  it('treats a child profile and child device as separate authority shapes that only bind through a trusted child-agent registration', () => {
    const childProfile = { childProfileId: 'child-1', displayName: 'Sam' } as const;
    const trustedChildAgent = DeviceRegistrationSchema.parse({
      schemaVersion: 'v0.6',
      registrationId: 'registration-child-agent',
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
      registeredAt: '2026-06-13T15:55:00.000Z',
    });

    expect(isTrustedChildAgentRegistrationForProfile(trustedChildAgent, childProfile)).toBe(true);

    expect(
      isTrustedChildAgentRegistrationForProfile(
        DeviceRegistrationSchema.parse({
          ...trustedChildAgent,
          registrationId: 'registration-parent-observer',
          deviceRole: 'parent-observer',
        }),
        childProfile
      )
    ).toBe(false);

    expect(
      isTrustedChildAgentRegistrationForProfile(
        DeviceRegistrationSchema.parse({
          ...trustedChildAgent,
          registrationId: 'registration-revoked-child-agent',
          trustState: 'revoked',
        }),
        childProfile
      )
    ).toBe(false);

    expect(
      isTrustedChildAgentRegistrationForProfile(
        DeviceRegistrationSchema.parse({
          ...trustedChildAgent,
          registrationId: 'registration-unbound-child-agent',
          device: {
            ...trustedChildAgent.device,
            childProfileId: null,
          },
        }),
        childProfile
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
    expect(DeviceTrustState.ResetRequired).toBe('reset-required');
    expect(ParentControllerLeaseState.Active).toBe('active');
  });

  it('treats only the trusted state as a privileged device authority state', () => {
    expect(isTrustedDeviceState(DeviceTrustState.Trusted)).toBe(true);
    expect(isTrustedDeviceState(DeviceTrustState.Pending)).toBe(false);
    expect(isTrustedDeviceState(DeviceTrustState.ResetRequired)).toBe(false);
    expect(isTrustedDeviceState(DeviceTrustState.Revoked)).toBe(false);
    expect(isTrustedDeviceState(DeviceTrustState.Disabled)).toBe(false);
  });
});
