import { RegistrationIdentityHandoffSchema, RegistrationSetupState } from './setup-registration-entry-contracts';
import { registrationEntryFailureState, registrationEntryRejectedReason } from './setup-registration-entry-rules';
import { type RecoveryOperation } from './family-restore-lifecycle';
import { type SetupInvite } from './family-setup-invite';
import { type HouseholdAuthorityInput } from './family-household-authority';
import type { RegistrationIdentityHandoff } from './setup-registration-entry-contracts';

type RegistrationStateRule = {
  state: RegistrationSetupState;
  matches: (input: RegistrationIdentityHandoff & {
    setupInvite: SetupInvite | null;
    recoveryOperation: RecoveryOperation | null;
    householdAuthorityInput: HouseholdAuthorityInput | null;
  }) => boolean;
};

const RegistrationStateRules: readonly RegistrationStateRule[] = [
  {
    state: RegistrationSetupState.Degraded,
    matches: (input) => registrationEntryRejectedReason(input) !== null || registrationEntryFailureState(input) !== null,
  },
  {
    state: RegistrationSetupState.Unauthenticated,
    matches: (input) => input.parentAccount === null,
  },
  {
    state: RegistrationSetupState.AuthenticatedNoHousehold,
    matches: (input) => input.family === null,
  },
  {
    state: RegistrationSetupState.HouseholdNoChild,
    matches: (input) => input.childProfile === null,
  },
  {
    state: RegistrationSetupState.HouseholdChildNoDevice,
    matches: (input) => input.childDevice === null,
  },
];

export function deriveRegistrationSetupState(input: RegistrationIdentityHandoff): RegistrationSetupState {
  const handoff = RegistrationIdentityHandoffSchema.parse(input);
  return RegistrationStateRules.find((rule) => rule.matches(handoff))?.state ?? RegistrationSetupState.Paired;
}
