import { describe, expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';
import { ParentStepUpMethod } from '@ocentra-parent/family-domain/household-authority';
import {
  deriveParentStepUpAssertionFromSetupPairingApproval,
  isSetupPairingTrustEstablished,
  requiresSetupPairingRecovery,
  SetupPairingApprovalChallengeSchema,
  SetupPairingApprovalResponseSchema,
  SetupPairingFailureReason,
  SetupPairingIntentSchema,
  SetupPairingState,
  SetupPairingTransport,
  isSetupPairingIntentActive,
} from '../../src/pairing-intent';

const Intent = SetupPairingIntentSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  family: {
    familyId: 'family-local-1',
  },
  parentAccount: {
    parentAccountId: 'parent-account-1',
  },
  parentDevice: {
    deviceId: 'parent-device-1',
    childProfileId: null,
    label: 'Mom phone',
    platform: 'android',
  },
  childProfile: {
    childProfileId: 'child-profile-1',
    displayName: 'Ari',
  },
  childDevice: null,
  pairingIntentId: 'setup-pairing-intent-1',
  activeStepId: 'setup-step-pair-child-device',
  pairingCode: 'setup-code-123456',
  replayNonce: 'pairing-replay-1',
  transport: SetupPairingTransport.LanQr,
  createdAt: '2026-06-01T00:00:00Z',
  displayedAt: '2026-06-01T00:01:00Z',
  acceptedAt: null,
  trustedAt: null,
  recoveredAt: null,
  revokedAt: null,
  expiresAt: '2026-06-01T00:15:00Z',
  state: SetupPairingState.Displayed,
  failureReason: null,
});
const ApprovalChallenge = SetupPairingApprovalChallengeSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  approvalChallengeId: 'pairing-approval-challenge-1',
  pairingIntentId: Intent.pairingIntentId,
  family: Intent.family,
  parentAccount: Intent.parentAccount,
  actionDevice: {
    deviceId: 'desktop-parent-1',
    childProfileId: null,
    label: 'Parent Desktop',
    platform: 'windows',
  },
  desktopSessionId: 'desktop-session-1',
  childProfile: Intent.childProfile,
  action: 'pair-child-device',
  challengeNonce: 'approval-nonce-1',
  createdAt: '2026-06-01T00:02:00Z',
  expiresAt: '2026-06-01T00:05:00Z',
});
const ApprovalResponse = SetupPairingApprovalResponseSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  approvalResponseId: 'pairing-approval-response-1',
  approvalChallengeId: ApprovalChallenge.approvalChallengeId,
  pairingIntentId: ApprovalChallenge.pairingIntentId,
  family: ApprovalChallenge.family,
  parentAccount: ApprovalChallenge.parentAccount,
  actionDevice: ApprovalChallenge.actionDevice,
  desktopSessionId: ApprovalChallenge.desktopSessionId,
  approvalDevice: Intent.parentDevice,
  childProfile: ApprovalChallenge.childProfile,
  action: ApprovalChallenge.action,
  challengeNonce: ApprovalChallenge.challengeNonce,
  approvalMethod: ParentStepUpMethod.PhoneQrApproval,
  approvedAt: '2026-06-01T00:04:00Z',
});

describe('setup pairing intent contracts', () => {
  it('parses active pairing intent contracts', () => {
    expect(isSetupPairingIntentActive(Intent)).toBe(true);
  });

  it('marks expired and replayed pairing intents inactive', () => {
    expect(
      isSetupPairingIntentActive(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.Expired,
        })
      )
    ).toBe(false);
    expect(
      isSetupPairingIntentActive(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.Replayed,
          failureReason: SetupPairingFailureReason.ReplayRejected,
        })
      )
    ).toBe(false);
  });

  it('rejects empty pairing codes', () => {
    expect(
      SetupPairingIntentSchema.safeParse({
        ...Intent,
        pairingCode: '',
      }).success
    ).toBe(false);
  });

  it('treats trusted and recovered pairing states as trust-established', () => {
    expect(
      isSetupPairingTrustEstablished(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.Trusted,
          trustedAt: '2026-06-01T00:05:00Z',
        })
      )
    ).toBe(true);
    expect(
      isSetupPairingTrustEstablished(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.Recovered,
          recoveredAt: '2026-06-01T00:10:00Z',
        })
      )
    ).toBe(true);
  });

  it('parses explicit wrong-device, wrong-target, anonymous-device, parent-role-required, and stale-signed-hello rejections', () => {
    const wrongDevice = SetupPairingIntentSchema.parse({
      ...Intent,
      state: SetupPairingState.WrongDevice,
      failureReason: SetupPairingFailureReason.WrongDevice,
    });
    const wrongTarget = SetupPairingIntentSchema.parse({
      ...Intent,
      state: SetupPairingState.WrongTarget,
      failureReason: SetupPairingFailureReason.WrongTarget,
    });
    const anonymousDevice = SetupPairingIntentSchema.parse({
      ...Intent,
      state: SetupPairingState.AnonymousDevice,
      failureReason: SetupPairingFailureReason.AnonymousDevice,
    });
    const parentRoleRequired = SetupPairingIntentSchema.parse({
      ...Intent,
      state: SetupPairingState.ParentRoleRequired,
      failureReason: SetupPairingFailureReason.ParentRoleRequired,
    });
    const staleSignedHello = SetupPairingIntentSchema.parse({
      ...Intent,
      state: SetupPairingState.StaleSignedHello,
      failureReason: SetupPairingFailureReason.StaleSignedHello,
    });

    expect(wrongDevice.failureReason).toBe(SetupPairingFailureReason.WrongDevice);
    expect(wrongTarget.failureReason).toBe(SetupPairingFailureReason.WrongTarget);
    expect(anonymousDevice.failureReason).toBe(SetupPairingFailureReason.AnonymousDevice);
    expect(parentRoleRequired.failureReason).toBe(SetupPairingFailureReason.ParentRoleRequired);
    expect(staleSignedHello.failureReason).toBe(SetupPairingFailureReason.StaleSignedHello);
  });

  it('derives a typed parent step-up assertion from a bound QR approval response', () => {
    const resolution = deriveParentStepUpAssertionFromSetupPairingApproval({
      challenge: ApprovalChallenge,
      response: ApprovalResponse,
      observedAt: '2026-06-01T00:04:30Z',
    });

    expect(resolution.failureReason).toBeNull();
    expect(resolution.assertion?.actionDevice.deviceId).toBe('desktop-parent-1');
    expect(resolution.assertion?.approverDevice.deviceId).toBe('parent-device-1');
    expect(resolution.assertion?.targetChildProfile?.childProfileId).toBe('child-profile-1');
  });

  it('rejects expired and wrong-target QR approval responses', () => {
    const expiredResolution = deriveParentStepUpAssertionFromSetupPairingApproval({
      challenge: ApprovalChallenge,
      response: ApprovalResponse,
      observedAt: '2026-06-01T00:05:30Z',
    });
    const wrongTargetResolution = deriveParentStepUpAssertionFromSetupPairingApproval({
      challenge: ApprovalChallenge,
      response: SetupPairingApprovalResponseSchema.parse({
        ...ApprovalResponse,
        approvalResponseId: 'pairing-approval-response-2',
        childProfile: {
          childProfileId: 'child-profile-9',
          displayName: 'Alex',
        },
      }),
      observedAt: '2026-06-01T00:04:30Z',
    });

    expect(expiredResolution.failureReason).toBe(SetupPairingFailureReason.ApprovalExpired);
    expect(wrongTargetResolution.failureReason).toBe(SetupPairingFailureReason.WrongTarget);
  });

  it('marks revoked and wrong-household pairing states as recovery work', () => {
    expect(
      requiresSetupPairingRecovery(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.Revoked,
          revokedAt: '2026-06-01T00:12:00Z',
          failureReason: SetupPairingFailureReason.RevokedDevice,
        })
      )
    ).toBe(true);
    expect(
      requiresSetupPairingRecovery(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.WrongHousehold,
          failureReason: SetupPairingFailureReason.WrongHousehold,
        })
      )
    ).toBe(true);
  });

  it('marks explicit wrong-device, wrong-target, anonymous-device, parent-role-required, and stale-signed-hello states as recovery work', () => {
    expect(
      requiresSetupPairingRecovery(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.WrongDevice,
          failureReason: SetupPairingFailureReason.WrongDevice,
        })
      )
    ).toBe(true);
    expect(
      requiresSetupPairingRecovery(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.WrongTarget,
          failureReason: SetupPairingFailureReason.WrongTarget,
        })
      )
    ).toBe(true);
    expect(
      requiresSetupPairingRecovery(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.AnonymousDevice,
          failureReason: SetupPairingFailureReason.AnonymousDevice,
        })
      )
    ).toBe(true);
    expect(
      requiresSetupPairingRecovery(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.ParentRoleRequired,
          failureReason: SetupPairingFailureReason.ParentRoleRequired,
        })
      )
    ).toBe(true);
    expect(
      requiresSetupPairingRecovery(
        SetupPairingIntentSchema.parse({
          ...Intent,
          state: SetupPairingState.StaleSignedHello,
          failureReason: SetupPairingFailureReason.StaleSignedHello,
        })
      )
    ).toBe(true);
  });
});
