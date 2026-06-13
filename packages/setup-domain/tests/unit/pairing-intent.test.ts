import { describe, expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';
import {
  SetupPairingIntentSchema,
  SetupPairingState,
  SetupPairingTransport,
  isSetupPairingIntentActive,
} from '../../src/pairing-intent';

const Intent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  family: {
    familyId: 'family-local-1',
  },
  parentAccount: {
    parentAccountId: 'parent-account-1',
  },
  childProfile: {
    childProfileId: 'child-profile-1',
    displayName: 'Ari',
  },
  pairingIntentId: 'setup-pairing-intent-1',
  activeStepId: 'setup-step-pair-child-device',
  pairingCode: 'setup-code-123456',
  transport: SetupPairingTransport.LanQr,
  expiresAt: '2026-06-01T00:15:00Z',
  state: SetupPairingState.Pending,
} as const;

describe('setup pairing intent contracts', () => {
  it('parses active pairing intent contracts', () => {
    expect(isSetupPairingIntentActive(Intent)).toBe(true);
  });

  it('marks expired pairing intents inactive', () => {
    expect(
      isSetupPairingIntentActive({
        ...Intent,
        state: SetupPairingState.Expired,
      })
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
});
