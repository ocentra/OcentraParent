export type PolicyControlDeliveryReadModelDomainStateCandidate = {
  readonly deliveryState: string;
  readonly lastAckEventId: string | null;
  readonly lastAppliedEventId: string | null;
};

export type PolicyControlDeliveryReadModelRowCandidate = {
  readonly acknowledgementRequired: boolean;
  readonly ackState: string;
  readonly applyState: string;
  readonly parentVisibleState: string;
  readonly blockedReason: string | null;
  readonly retryScheduleRefs: readonly string[];
  readonly manualProofRequirements: readonly string[];
  readonly transportState: string;
  readonly domainStates: readonly PolicyControlDeliveryReadModelDomainStateCandidate[];
};

export function validateDomainState(state: PolicyControlDeliveryReadModelDomainStateCandidate): true | string {
  if (state.deliveryState === 'acknowledged' && state.lastAckEventId === null) {
    return 'Acknowledged domain rows require an acknowledgement event id';
  }
  if (state.deliveryState === 'applied' && (state.lastAckEventId === null || state.lastAppliedEventId === null)) {
    return 'Applied domain rows require acknowledgement and applied event ids';
  }
  return true;
}

export function validateRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  const validators: readonly ((candidate: PolicyControlDeliveryReadModelRowCandidate) => true | string)[] = [
    validateAcknowledgementRequirement,
    validateAcknowledgedRowState,
    validateAppliedRowState,
    validateDegradedRowState,
    validateManualRequiredRowState,
    validatePartialApplyRowState,
  ];

  return firstValidationFailure(row, validators);
}

function validateAcknowledgementRequirement(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.acknowledgementRequired && row.ackState === 'not-required') {
    return 'Acknowledgement-required rows must not use not-required ack state';
  }
  return true;
}

function validateAcknowledgedRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.parentVisibleState === 'acknowledged' && (row.ackState !== 'acknowledged' || row.applyState !== 'pending')) {
    return 'Acknowledged rows must distinguish acknowledgement from applied policy state';
  }
  return true;
}

function validateAppliedRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (
    row.parentVisibleState === 'applied' &&
    (row.ackState !== 'acknowledged' ||
      row.applyState !== 'applied' ||
      row.blockedReason !== null ||
      row.manualProofRequirements.length > 0 ||
      !row.domainStates.every((state) => state.deliveryState === 'applied'))
  ) {
    return 'Applied rows require acknowledged delivery, applied domain states, and no manual/degraded blockers';
  }
  return true;
}

function validateDegradedRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (
    row.parentVisibleState === 'degraded' &&
    row.blockedReason === null &&
    row.retryScheduleRefs.length === 0 &&
    row.applyState !== 'degraded' &&
    row.transportState !== 'offline' &&
    row.transportState !== 'retry-scheduled' &&
    !row.domainStates.some((state) => state.deliveryState === 'degraded' || state.deliveryState === 'blocked')
  ) {
    return 'Degraded rows require offline, retry, blocked, or degraded domain evidence';
  }
  return true;
}

function validateManualRequiredRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.parentVisibleState === 'manual-required' && row.manualProofRequirements.length === 0) {
    return 'Manual-required rows must surface explicit manual proof requirements';
  }
  return true;
}

function validatePartialApplyRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.applyState === 'partially-applied' && !hasMixedDomainOutcome(row.domainStates)) {
    return 'Partially applied rows require mixed per-domain outcomes';
  }
  return true;
}

function firstValidationFailure<T>(
  candidate: T,
  validators: readonly ((candidate: T) => true | string)[]
): true | string {
  for (const validator of validators) {
    const validation = validator(candidate);
    if (validation !== true) {
      return validation;
    }
  }

  return true;
}

function hasMixedDomainOutcome(states: readonly PolicyControlDeliveryReadModelDomainStateCandidate[]): boolean {
  const applied = states.some((state) => state.deliveryState === 'applied');
  const blocked = states.some(
    (state) =>
      state.deliveryState === 'degraded' ||
      state.deliveryState === 'manual-required' ||
      state.deliveryState === 'blocked'
  );
  return applied && blocked;
}
