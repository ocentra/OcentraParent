import { describe, expect, it } from 'vitest';
import {
  ParentContractSchemaVersion,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { parseUnknown } from '@ocentra-parent/schema-domain/effect';
import { ParentStepUpMethod } from '@ocentra-parent/schema-domain/family-household-authority';
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
} from '@ocentra-parent/schema-domain/setup-pairing-intent';

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

function createPairingIntent(overrides: Record<string, unknown> = {}) {
  return SetupPairingIntentSchema.parse({
    ...Intent,
    ...overrides,
  });
}

function createApprovalResponse(overrides: Record<string, unknown> = {}) {
  return SetupPairingApprovalResponseSchema.parse({
    ...ApprovalResponse,
    ...overrides,
  });
}

function expectRecoveryWorkForState(state: SetupPairingState, failureReason: SetupPairingFailureReason) {
  expect(requiresSetupPairingRecovery(createPairingIntent({ state, failureReason }))).toBe(true);
}

function expectActivePairingIntentContractsToParse() {
  expect(isSetupPairingIntentActive(Intent)).toBe(true);
}

function expectExpiredAndReplayedPairingIntentsToBeInactive() {
  expect(isSetupPairingIntentActive(createPairingIntent({ state: SetupPairingState.Expired }))).toBe(false);
  expect(
    isSetupPairingIntentActive(
      createPairingIntent({
        state: SetupPairingState.Replayed,
        failureReason: SetupPairingFailureReason.ReplayRejected,
      })
    )
  ).toBe(false);
}

function expectEmptyPairingCodesToBeRejected() {
  expect(
    SetupPairingIntentSchema.safeParse({
      ...Intent,
      pairingCode: '',
    }).success
  ).toBe(false);
}

function expectTrustedAndRecoveredPairingStatesToBeTrustEstablished() {
  expect(
    isSetupPairingTrustEstablished(
      createPairingIntent({
        state: SetupPairingState.Trusted,
        trustedAt: '2026-06-01T00:05:00Z',
      })
    )
  ).toBe(true);
  expect(
    isSetupPairingTrustEstablished(
      createPairingIntent({
        state: SetupPairingState.Recovered,
        recoveredAt: '2026-06-01T00:10:00Z',
      })
    )
  ).toBe(true);
}

function expectExplicitRejectionStatesToParse() {
  const explicitRejections = [
    [SetupPairingState.WrongDevice, SetupPairingFailureReason.WrongDevice],
    [SetupPairingState.WrongTarget, SetupPairingFailureReason.WrongTarget],
    [SetupPairingState.AnonymousDevice, SetupPairingFailureReason.AnonymousDevice],
    [SetupPairingState.ParentRoleRequired, SetupPairingFailureReason.ParentRoleRequired],
    [SetupPairingState.StaleSignedHello, SetupPairingFailureReason.StaleSignedHello],
  ] as const;

  for (const [state, failureReason] of explicitRejections) {
    expect(createPairingIntent({ state, failureReason }).failureReason).toBe(failureReason);
  }
}

function expectTypedParentStepUpAssertionFromQrApproval() {
  const resolution = deriveParentStepUpAssertionFromSetupPairingApproval({
    challenge: ApprovalChallenge,
    response: ApprovalResponse,
    observedAt: parseUnknown(ParentTimestampSchema, '2026-06-01T00:04:30Z'),
  });

  expect(resolution.failureReason).toBeNull();
  expect(resolution.assertion?.actionDevice.deviceId).toBe('desktop-parent-1');
  expect(resolution.assertion?.approverDevice.deviceId).toBe('parent-device-1');
  expect(resolution.assertion?.targetChildProfile?.childProfileId).toBe('child-profile-1');
}

function expectExpiredAndWrongTargetQrApprovalsToBeRejected() {
  const expiredResolution = deriveParentStepUpAssertionFromSetupPairingApproval({
    challenge: ApprovalChallenge,
    response: ApprovalResponse,
    observedAt: parseUnknown(ParentTimestampSchema, '2026-06-01T00:05:30Z'),
  });
  const wrongTargetResolution = deriveParentStepUpAssertionFromSetupPairingApproval({
    challenge: ApprovalChallenge,
    response: createApprovalResponse({
      approvalResponseId: 'pairing-approval-response-2',
      childProfile: {
        childProfileId: 'child-profile-9',
        displayName: 'Alex',
      },
    }),
    observedAt: parseUnknown(ParentTimestampSchema, '2026-06-01T00:04:30Z'),
  });

  expect(expiredResolution.failureReason).toBe(SetupPairingFailureReason.ApprovalExpired);
  expect(wrongTargetResolution.failureReason).toBe(SetupPairingFailureReason.WrongTarget);
}

function expectRevokedAndWrongHouseholdStatesToRequireRecovery() {
  expectRecoveryWorkForState(SetupPairingState.Revoked, SetupPairingFailureReason.RevokedDevice);
  expectRecoveryWorkForState(SetupPairingState.WrongHousehold, SetupPairingFailureReason.WrongHousehold);
}

function expectExplicitFailureStatesToRequireRecovery() {
  const explicitRecoveryStates = [
    [SetupPairingState.WrongDevice, SetupPairingFailureReason.WrongDevice],
    [SetupPairingState.WrongTarget, SetupPairingFailureReason.WrongTarget],
    [SetupPairingState.AnonymousDevice, SetupPairingFailureReason.AnonymousDevice],
    [SetupPairingState.ParentRoleRequired, SetupPairingFailureReason.ParentRoleRequired],
    [SetupPairingState.StaleSignedHello, SetupPairingFailureReason.StaleSignedHello],
  ] as const;

  for (const [state, failureReason] of explicitRecoveryStates) {
    expectRecoveryWorkForState(state, failureReason);
  }
}

describe('setup pairing intent contracts', () => {
  it('parses active pairing intent contracts', expectActivePairingIntentContractsToParse);

  it('marks expired and replayed pairing intents inactive', expectExpiredAndReplayedPairingIntentsToBeInactive);

  it('rejects empty pairing codes', expectEmptyPairingCodesToBeRejected);

  it(
    'treats trusted and recovered pairing states as trust-established',
    expectTrustedAndRecoveredPairingStatesToBeTrustEstablished
  );

  it(
    'parses explicit wrong-device, wrong-target, anonymous-device, parent-role-required, and stale-signed-hello rejections',
    expectExplicitRejectionStatesToParse
  );

  it(
    'derives a typed parent step-up assertion from a bound QR approval response',
    expectTypedParentStepUpAssertionFromQrApproval
  );

  it('rejects expired and wrong-target QR approval responses', expectExpiredAndWrongTargetQrApprovalsToBeRejected);

  it(
    'marks revoked and wrong-household pairing states as recovery work',
    expectRevokedAndWrongHouseholdStatesToRequireRecovery
  );

  it(
    'marks explicit wrong-device, wrong-target, anonymous-device, parent-role-required, and stale-signed-hello states as recovery work',
    expectExplicitFailureStatesToRequireRecovery
  );
});
