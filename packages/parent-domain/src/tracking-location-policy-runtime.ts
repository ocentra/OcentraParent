import { TrackingPolicyReasonCodeSchema } from './tracking-location-policy-primitives';
import type {
  TrackingAcknowledgement,
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingChildCheckInResponse,
} from './tracking-location-policy-types';

type TrackingPolicyReasonCode = ReturnType<typeof TrackingPolicyReasonCodeSchema.parse>;
type TrackingAcknowledgementImpactState = 'open' | 'expired' | 'suppressed-by-acknowledgement' | 'critical-still-alert';

type TrackingCheckInResolutionState = 'pending' | 'sent' | 'answered' | 'expired' | 'escalated' | 'cancelled';

export interface TrackingAcknowledgementImpact {
  readonly alertId: TrackingAlertIntent['alertId'];
  readonly acknowledgementId: TrackingAcknowledgement['acknowledgementId'];
  readonly state: TrackingAcknowledgementImpactState;
  readonly suppressesParentAlert: boolean;
  readonly reasonCodes: readonly TrackingPolicyReasonCode[];
}

export interface TrackingChildCheckInResolution {
  readonly checkInId: TrackingChildCheckInRequest['checkInId'];
  readonly state: TrackingCheckInResolutionState;
  readonly escalates: boolean;
  readonly responseKind: TrackingChildCheckInResponse['response'] | null;
  readonly reasonCodes: readonly TrackingPolicyReasonCode[];
}

export function evaluateTrackingAcknowledgementImpact(input: {
  readonly alert: TrackingAlertIntent;
  readonly acknowledgement: TrackingAcknowledgement;
  readonly evaluatedAt: string;
}): TrackingAcknowledgementImpact {
  if (
    input.acknowledgement.expiresAt !== null &&
    Date.parse(input.acknowledgement.expiresAt) <= Date.parse(input.evaluatedAt)
  ) {
    return acknowledgementImpact(input, 'expired', false, ['acknowledgement-expired']);
  }

  if (input.alert.severity === 'critical' && input.acknowledgement.stillAlertForCritical) {
    return acknowledgementImpact(input, 'critical-still-alert', false, ['critical-alert-not-suppressed']);
  }

  if (
    input.acknowledgement.state === 'acknowledged-safe' ||
    input.acknowledgement.state === 'expected' ||
    input.acknowledgement.state === 'holiday-mode' ||
    input.acknowledgement.state === 'trip-exception' ||
    input.acknowledgement.state === 'false-alarm'
  ) {
    return acknowledgementImpact(input, 'suppressed-by-acknowledgement', true, [
      'parent-acknowledgement-suppressed-alert',
    ]);
  }

  return acknowledgementImpact(input, 'open', false, ['acknowledgement-open']);
}

export function resolveTrackingChildCheckIn(input: {
  readonly request: TrackingChildCheckInRequest;
  readonly response: TrackingChildCheckInResponse | null;
  readonly evaluatedAt: string;
}): TrackingChildCheckInResolution {
  if (input.response !== null) {
    return {
      checkInId: input.request.checkInId,
      state: 'answered',
      escalates: input.response.response === 'help' || input.response.response === 'no-response',
      responseKind: input.response.response,
      reasonCodes: [reasonCode('child-check-in-answered')],
    };
  }

  if (Date.parse(input.request.expiresAt) <= Date.parse(input.evaluatedAt)) {
    return {
      checkInId: input.request.checkInId,
      state: 'escalated',
      escalates: true,
      responseKind: null,
      reasonCodes: [reasonCode('child-check-in-expired')],
    };
  }

  return {
    checkInId: input.request.checkInId,
    state: input.request.state === 'sent' ? 'sent' : 'pending',
    escalates: false,
    responseKind: null,
    reasonCodes: [reasonCode('child-check-in-waiting')],
  };
}

function acknowledgementImpact(
  input: {
    readonly alert: TrackingAlertIntent;
    readonly acknowledgement: TrackingAcknowledgement;
  },
  state: TrackingAcknowledgementImpactState,
  suppressesParentAlert: boolean,
  rawReasonCodes: readonly string[]
): TrackingAcknowledgementImpact {
  return {
    alertId: input.alert.alertId,
    acknowledgementId: input.acknowledgement.acknowledgementId,
    state,
    suppressesParentAlert,
    reasonCodes: rawReasonCodes.map(reasonCode),
  };
}

function reasonCode(value: string) {
  return TrackingPolicyReasonCodeSchema.parse(value);
}
