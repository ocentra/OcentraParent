import { describe, expect, it } from 'vitest';
import {
  HouseholdAuthorityInputSchema,
  HouseholdMembershipState,
  HouseholdRole,
  SessionFreshnessState,
} from '@ocentra-parent/family-domain/household-authority';
import { ParentActorRole, ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  RecoveryIdentityProofState,
  RecoveryKind,
  RecoveryOperationSchema,
  RecoveryState,
  RecoverySupportChannel,
  SetupInvitePurpose,
  SetupInviteSchema,
  SetupInviteState,
} from '@ocentra-parent/family-domain/setup-lifecycle';
import {
  evaluateRegistrationIdentityHandoff,
  RegistrationEntryFailureState,
  RegistrationEntryRejectionReason,
  RegistrationEntryRoute,
  RegistrationEntryRouteContracts,
  RegistrationHandoffFieldLiteral,
  RegistrationIdentityHandoffSchema,
  RegistrationIdentityProviderState,
  RegistrationRecoveryMethod,
  RegistrationSetupState,
  registrationEntryRouteContract,
} from '../../src/registration-entry';
import { SetupPairingIntentIdSchema } from '../../src/pairing-intent';
import { parseUnknown } from '@ocentra-parent/schema-domain/effect';

const ParentAccount = { parentAccountId: 'parent-account-1' } as const;
const Family = { familyId: 'family-main' } as const;
const ChildProfile = { childProfileId: 'child-1', displayName: 'Sam' } as const;
const ChildDevice = {
  deviceId: 'device-child-1',
  childProfileId: 'child-1',
  label: 'Sam Android',
  platform: 'android',
} as const;

const BaseInvite = SetupInviteSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  inviteId: 'invite-child-pairing-1',
  family: Family,
  invitedBy: { actorId: 'actor-owner', role: ParentActorRole.Parent },
  targetAccount: ParentAccount,
  targetChildProfile: ChildProfile,
  targetRole: HouseholdRole.ChildDeviceAgent,
  purpose: SetupInvitePurpose.ChildDevicePairing,
  state: SetupInviteState.Pending,
  expiresAt: '2026-06-14T15:00:00.000Z',
  singleUse: true,
});

const BaseAuthorityInput = HouseholdAuthorityInputSchema.parse({
  actorRole: HouseholdRole.ParentOwner,
  actorAccountState: 'active',
  sameFamily: true,
  membershipState: HouseholdMembershipState.Active,
  childProfileBindingState: 'bound',
  deviceOwnershipScope: 'child-profile-device',
  deviceTrustState: 'trusted',
  sessionFreshnessState: SessionFreshnessState.Fresh,
  capabilityGranted: true,
  action: 'pair-child-device',
});

const BaseRecoveryOperation = RecoveryOperationSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  recoveryOperationId: 'recovery-login-1',
  family: Family,
  requestedBy: { actorId: 'actor-owner', role: ParentActorRole.Parent },
  requesterMembershipState: HouseholdMembershipState.Active,
  relatedAccount: ParentAccount,
  relatedDevice: null,
  kind: RecoveryKind.ForgotLogin,
  state: RecoveryState.OwnerApprovalRequired,
  ownerApprovalRequired: false,
  identityProofState: RecoveryIdentityProofState.Pending,
  supportChannel: RecoverySupportChannel.SelfServe,
  deleteExportHandoffRequired: false,
  openedAt: '2026-06-13T20:01:00.000Z',
  closedAt: null,
});

const BaseHandoff = RegistrationIdentityHandoffSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  routeId: RegistrationEntryRoute.Register,
  recoveryMethod: null,
  providerState: null,
  sessionFreshnessState: null,
  parentAccount: null,
  family: null,
  childProfile: null,
  childDevice: null,
  setupInvite: null,
  recoveryOperation: null,
  pairingIntentId: null,
  householdAuthorityInput: null,
});

describe('setup registration entry contracts', () => {
  it('registration.entry-route-state', () => {
    expect(RegistrationEntryRouteContracts.map((route) => route.routeId)).toEqual([
      RegistrationEntryRoute.Register,
      RegistrationEntryRoute.Login,
      RegistrationEntryRoute.Logout,
      RegistrationEntryRoute.InviteAccept,
      RegistrationEntryRoute.ResumeSetup,
      RegistrationEntryRoute.Recovery,
    ]);
    expect(registrationEntryRouteContract(RegistrationEntryRoute.Recovery).recoveryMethods).toEqual([
      RegistrationRecoveryMethod.Password,
      RegistrationRecoveryMethod.Passkey,
      RegistrationRecoveryMethod.EmailLink,
    ]);

    const unauthenticated = evaluateRegistrationIdentityHandoff(BaseHandoff);
    const authenticatedNoHousehold = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.ResumeSetup,
      sessionFreshnessState: SessionFreshnessState.Fresh,
      parentAccount: ParentAccount,
    });
    const householdNoChild = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.ResumeSetup,
      sessionFreshnessState: SessionFreshnessState.Fresh,
      parentAccount: ParentAccount,
      family: Family,
    });
    const householdChildNoDevice = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.ResumeSetup,
      sessionFreshnessState: SessionFreshnessState.Fresh,
      parentAccount: ParentAccount,
      family: Family,
      childProfile: ChildProfile,
    });
    const paired = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.ResumeSetup,
      sessionFreshnessState: SessionFreshnessState.Fresh,
      parentAccount: ParentAccount,
      family: Family,
      childProfile: ChildProfile,
      childDevice: ChildDevice,
      pairingIntentId: parseUnknown(SetupPairingIntentIdSchema, 'pairing-intent-1'),
    });
    const degraded = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.ResumeSetup,
      sessionFreshnessState: SessionFreshnessState.Expired,
      parentAccount: ParentAccount,
      family: Family,
    });

    expect(unauthenticated.setupState).toBe(RegistrationSetupState.Unauthenticated);
    expect(unauthenticated.allowedRoutes).toEqual([
      RegistrationEntryRoute.Register,
      RegistrationEntryRoute.Login,
      RegistrationEntryRoute.InviteAccept,
      RegistrationEntryRoute.Recovery,
    ]);
    expect(unauthenticated.forbiddenCollections).toContain('child-activity-data');
    expect(unauthenticated.allowedHandoffFields).toContain(RegistrationHandoffFieldLiteral.ProviderState);
    expect(unauthenticated.failureState).toBeNull();

    expect(authenticatedNoHousehold.setupState).toBe(RegistrationSetupState.AuthenticatedNoHousehold);
    expect(authenticatedNoHousehold.allowedHandoffFields).toContain(RegistrationHandoffFieldLiteral.ProviderState);
    expect(authenticatedNoHousehold.failureState).toBeNull();
    expect(householdNoChild.setupState).toBe(RegistrationSetupState.HouseholdNoChild);
    expect(householdNoChild.failureState).toBeNull();
    expect(householdChildNoDevice.setupState).toBe(RegistrationSetupState.HouseholdChildNoDevice);
    expect(householdChildNoDevice.allowedHandoffFields).toContain(RegistrationHandoffFieldLiteral.ChildProfile);
    expect(householdChildNoDevice.failureState).toBeNull();
    expect(paired.setupState).toBe(RegistrationSetupState.Paired);
    expect(paired.allowedHandoffFields).toContain(RegistrationHandoffFieldLiteral.ChildDevice);
    expect(paired.failureState).toBeNull();
    expect(degraded.setupState).toBe(RegistrationSetupState.Degraded);
    expect(degraded.failureState).toBe(RegistrationEntryFailureState.SessionExpired);
  });

  it('registration.expired-invite-rejected', () => {
    const decision = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.InviteAccept,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Expired,
      }),
    });

    expect(decision.setupState).toBe(RegistrationSetupState.Degraded);
    expect(decision.rejectedReason).toBe(RegistrationEntryRejectionReason.ExpiredInvite);
  });

  it('registration.revoked-invite-rejected', () => {
    const decision = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.InviteAccept,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Revoked,
      }),
    });

    expect(decision.setupState).toBe(RegistrationSetupState.Degraded);
    expect(decision.rejectedReason).toBe(RegistrationEntryRejectionReason.RevokedInvite);
  });

  it('registration.cross-family-rejected', () => {
    const decision = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.ResumeSetup,
      sessionFreshnessState: SessionFreshnessState.Fresh,
      parentAccount: ParentAccount,
      family: Family,
      householdAuthorityInput: HouseholdAuthorityInputSchema.parse({
        ...BaseAuthorityInput,
        sameFamily: false,
      }),
      setupInvite: BaseInvite,
    });

    expect(decision.setupState).toBe(RegistrationSetupState.Degraded);
    expect(decision.rejectedReason).toBe(RegistrationEntryRejectionReason.CrossFamily);
    expect(decision.failureState).toBeNull();
  });

  it('registration.wrong-role-rejected', () => {
    const decision = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.InviteAccept,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        inviteId: 'invite-wrong-role',
        targetAccount: null,
        targetRole: HouseholdRole.Observer,
        purpose: SetupInvitePurpose.ChildDevicePairing,
      }),
    });

    expect(decision.setupState).toBe(RegistrationSetupState.Degraded);
    expect(decision.rejectedReason).toBe(RegistrationEntryRejectionReason.WrongRole);
    expect(decision.failureState).toBeNull();
  });

  it('registration.session-expired-state', () => {
    const decision = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.ResumeSetup,
      sessionFreshnessState: SessionFreshnessState.Expired,
      parentAccount: ParentAccount,
      family: Family,
    });

    expect(decision.setupState).toBe(RegistrationSetupState.Degraded);
    expect(decision.rejectedReason).toBeNull();
    expect(decision.failureState).toBe(RegistrationEntryFailureState.SessionExpired);
  });

  it('registration.provider-unavailable-state', () => {
    const decision = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.Login,
      providerState: RegistrationIdentityProviderState.ProviderUnavailable,
    });

    expect(decision.setupState).toBe(RegistrationSetupState.Degraded);
    expect(decision.rejectedReason).toBeNull();
    expect(decision.failureState).toBe(RegistrationEntryFailureState.ProviderUnavailable);
    expect(decision.allowedHandoffFields).toContain(RegistrationHandoffFieldLiteral.ProviderState);
  });

  it('registration.no-child-data-before-household', () => {
    expect(() =>
      evaluateRegistrationIdentityHandoff({
        ...BaseHandoff,
        routeId: RegistrationEntryRoute.ResumeSetup,
        sessionFreshnessState: SessionFreshnessState.Fresh,
        parentAccount: ParentAccount,
        childProfile: ChildProfile,
      })
    ).toThrow('registration.no-child-data-before-household');

    const allowedAfterHousehold = evaluateRegistrationIdentityHandoff({
      ...BaseHandoff,
      routeId: RegistrationEntryRoute.Recovery,
      recoveryMethod: RegistrationRecoveryMethod.EmailLink,
      parentAccount: ParentAccount,
      family: Family,
      childProfile: ChildProfile,
      childDevice: ChildDevice,
      pairingIntentId: parseUnknown(SetupPairingIntentIdSchema, 'pairing-intent-1'),
      recoveryOperation: BaseRecoveryOperation,
    });

    expect(allowedAfterHousehold.setupState).toBe(RegistrationSetupState.Degraded);
    expect(allowedAfterHousehold.rejectedReason).toBeNull();
    expect(allowedAfterHousehold.failureState).toBeNull();
  });
});
