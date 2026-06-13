import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const TrackingPolicySchemaVersion = 'v0.5-tracking';

export const TrackingPolicyRuleIdSchema = brandedNonEmptyStringSchema('TrackingPolicyRuleId');
export const TrackingPolicyDecisionIdSchema = brandedNonEmptyStringSchema('TrackingPolicyDecisionId');
export const TrackingAlertIdSchema = brandedNonEmptyStringSchema('TrackingAlertId');
export const TrackingAcknowledgementIdSchema = brandedNonEmptyStringSchema('TrackingAcknowledgementId');
export const TrackingCheckInIdSchema = brandedNonEmptyStringSchema('TrackingCheckInId');
export const TrackingAiAnalysisIdSchema = brandedNonEmptyStringSchema('TrackingAiAnalysisId');
export const TrackingProviderRouteIdSchema = brandedNonEmptyStringSchema('TrackingProviderRouteId');
export const TrackingEscalationIdSchema = brandedNonEmptyStringSchema('TrackingEscalationId');
export const TrackingLiveTrackingGrantIdSchema = brandedNonEmptyStringSchema('TrackingLiveTrackingGrantId');
export const TrackingMissingDeviceCaseIdSchema = brandedNonEmptyStringSchema('TrackingMissingDeviceCaseId');
export const TrackingPolicyReasonCodeSchema = withParser(
  brandedNonEmptyStringSchema('TrackingPolicyReasonCode')
);
export const TrackingPolicyAuditRefSchema = brandedNonEmptyStringSchema('TrackingPolicyAuditRef');

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
  Schema.Literal(
    'contract-proved',
    'manual-required',
    'real-device-required',
    'background-permission-required',
    'platform-unsupported',
    'blocked',
    'not-claimed'
  )
);

