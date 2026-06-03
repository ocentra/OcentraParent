import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const TrackingPolicyText = Schema.String.pipe(Schema.minLength(1));

export const TrackingPolicySchemaVersion = 'v0.5-tracking';

export const TrackingPolicyRuleIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingPolicyRuleId'));
export const TrackingPolicyDecisionIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingPolicyDecisionId'));
export const TrackingAlertIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingAlertId'));
export const TrackingAcknowledgementIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingAcknowledgementId'));
export const TrackingCheckInIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingCheckInId'));
export const TrackingAiAnalysisIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingAiAnalysisId'));
export const TrackingProviderRouteIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingProviderRouteId'));
export const TrackingEscalationIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingEscalationId'));
export const TrackingLiveTrackingGrantIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingLiveTrackingGrantId'));
export const TrackingMissingDeviceCaseIdSchema = TrackingPolicyText.pipe(Schema.brand('TrackingMissingDeviceCaseId'));
export const TrackingPolicyReasonCodeSchema = withParser(
  TrackingPolicyText.pipe(Schema.brand('TrackingPolicyReasonCode'))
);
export const TrackingPolicyAuditRefSchema = TrackingPolicyText.pipe(Schema.brand('TrackingPolicyAuditRef'));

export const TrackingPolicyTargetKindSchema = withParser(
  Schema.Literal(
    'location-sample',
    'geofence-transition',
    'expected-place',
    'nearby-place',
    'device-status',
    'child-check-in',
    'missing-device'
  )
);

export const TrackingPolicyActionSchema = withParser(
  Schema.Literal(
    'observe',
    'notify-parent',
    'ask-child-check-in',
    'request-parent-acknowledgement',
    'start-temporary-live-tracking',
    'escalate',
    'manual-required',
    'no-action'
  )
);

export const TrackingAlertSeveritySchema = withParser(Schema.Literal('info', 'watch', 'warning', 'urgent', 'critical'));

export const TrackingAcknowledgementStateSchema = withParser(
  Schema.Literal('open', 'acknowledged-safe', 'expected', 'holiday-mode', 'trip-exception', 'false-alarm', 'expired')
);

export const TrackingCheckInRequestStateSchema = withParser(
  Schema.Literal('pending', 'sent', 'answered', 'expired', 'cancelled', 'escalated')
);

export const TrackingCheckInResponseKindSchema = withParser(
  Schema.Literal('safe', 'help', 'share-location-if-permitted', 'call-parent', 'no-response')
);

export const TrackingAiLocationRiskLevelSchema = withParser(
  Schema.Literal('none', 'low', 'moderate', 'high', 'critical', 'unknown')
);

export const TrackingAiProviderModeSchema = withParser(
  Schema.Literal('child-local', 'parent-local', 'family-ai-hub', 'parent-approved-remote', 'metadata-only', 'no-ai')
);

export const TrackingProviderCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabled-by-default', 'unavailable', 'manual-required', 'degraded')
);

export const TrackingEscalationStateSchema = withParser(
  Schema.Literal('not-started', 'waiting-for-parent', 'waiting-for-child', 'escalated', 'resolved', 'manual-required')
);

export const TrackingLiveTrackingGrantStateSchema = withParser(
  Schema.Literal('requested', 'active', 'expired', 'revoked', 'denied', 'unavailable')
);

export const TrackingMissingDeviceStateSchema = withParser(
  Schema.Literal('open', 'last-known-only', 'offline', 'contact-requested', 'resolved', 'manual-required')
);

export const TrackingPlatformProofRouteStateSchema = withParser(
  Schema.Literal('contract-proved', 'manual-required', 'real-device-required', 'blocked', 'not-claimed')
);
