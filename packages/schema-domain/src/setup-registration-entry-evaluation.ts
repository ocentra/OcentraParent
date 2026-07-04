import { RegistrationEntryDecisionSchema } from './setup-registration-entry-contracts';
import type { RegistrationIdentityHandoff } from './setup-registration-entry-contracts';
import {
  assertRegistrationEntryBoundary,
  assertRegistrationRouteRequirements,
  registrationEntryFailureState,
  registrationEntryRejectedReason,
  registrationStateMatrixRow,
} from './setup-registration-entry-rules';
import { deriveRegistrationSetupState } from './setup-registration-entry-state';

export function evaluateRegistrationIdentityHandoff(input: RegistrationIdentityHandoff) {
  assertRegistrationRouteRequirements(input);
  assertRegistrationEntryBoundary(input);

  const setupState = deriveRegistrationSetupState(input);
  const matrixRow = registrationStateMatrixRow(setupState);
  const rejectedReason = registrationEntryRejectedReason(input);
  const failureState = registrationEntryFailureState(input);

  return RegistrationEntryDecisionSchema.parse({
    ...matrixRow,
    rejectedReason,
    failureState,
  });
}
