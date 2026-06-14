import { describe, expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';
import {
  isSetupPairingTrustEstablished,
  requiresSetupPairingRecovery,
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

  it('parses explicit wrong-device, anonymous-device, parent-role-required, and stale-signed-hello rejections', () => {
    const wrongDevice = SetupPairingIntentSchema.parse({
      ...Intent,
      state: SetupPairingState.WrongDevice,
      failureReason: SetupPairingFailureReason.WrongDevice,
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
    expect(anonymousDevice.failureReason).toBe(SetupPairingFailureReason.AnonymousDevice);
    expect(parentRoleRequired.failureReason).toBe(SetupPairingFailureReason.ParentRoleRequired);
    expect(staleSignedHello.failureReason).toBe(SetupPairingFailureReason.StaleSignedHello);
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

  it('marks explicit wrong-device, anonymous-device, parent-role-required, and stale-signed-hello states as recovery work', () => {
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
