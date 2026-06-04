import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  TrackingAlertIntentSchema,
  TrackingChildCheckInRequestSchema,
  TrackingEscalationChainSchema,
  TrackingEvidenceTraceSchema,
  TrackingLocationAiAnalysisResultSchema,
  TrackingPolicyDecisionSchema,
  TrackingPolicyRuleSchema,
  TrackingTemporaryLiveTrackingGrantSchema,
} from './tracking-location-policy';
import {
  TrackingAlertIdSchema,
  TrackingAlertSeveritySchema,
  TrackingCheckInIdSchema,
  TrackingEscalationIdSchema,
  TrackingLiveTrackingGrantIdSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyDecisionIdSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';
import type { TrackingPolicyRule } from './tracking-location-policy-types';

const TrackingPolicyCompilerTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingPolicyCompilerDurationSecondsSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingPolicyCompilerRequestIdSchema = TrackingPolicyCompilerTextSchema.pipe(
  Schema.brand('TrackingPolicyCompilerRequestId')
);

export const TrackingPolicyCompilerModeSchema = withParser(Schema.Literal('dry-run', 'active'));

export const TrackingPolicyCompilerRequestedActionSchema = withParser(
  Schema.Literal(
    'observe',
    'notify-parent',
    'ask-child-check-in',
    'request-parent-acknowledgement',
    'start-temporary-live-tracking',
    'escalate',
    'critical-alert',
    'suppress',
    'manual-required'
  )
);

export const TrackingPolicyCompilerFinalActionSourceSchema = withParser(
  Schema.Literal('parent-policy-rule', 'disabled-rule', 'manual-required')
);

export const TrackingPolicyCompilerRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    requestId: TrackingPolicyCompilerRequestIdSchema,
    rule: TrackingPolicyRuleSchema,
    requestedAt: ParentTimestampSchema,
    decidedAt: ParentTimestampSchema,
    followUpExpiresAt: ParentTimestampSchema,
    decisionId: TrackingPolicyDecisionIdSchema,
    requestedAction: TrackingPolicyCompilerRequestedActionSchema,
    compilerMode: TrackingPolicyCompilerModeSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    aiAnalysis: Schema.Union(TrackingLocationAiAnalysisResultSchema, Schema.Null),
    alertId: Schema.Union(TrackingAlertIdSchema, Schema.Null),
    alertSeverity: Schema.Union(TrackingAlertSeveritySchema, Schema.Null),
    checkInId: Schema.Union(TrackingCheckInIdSchema, Schema.Null),
    escalationId: Schema.Union(TrackingEscalationIdSchema, Schema.Null),
    liveTrackingGrantId: Schema.Union(TrackingLiveTrackingGrantIdSchema, Schema.Null),
    liveTrackingDurationSeconds: Schema.Union(TrackingPolicyCompilerDurationSecondsSchema, Schema.Null),
    parentConfirmationReceived: Schema.Boolean,
    freshEvidenceAvailable: Schema.Boolean,
    platformManualRequired: Schema.Boolean,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
    .pipe(
      Schema.filter(
        (request) =>
          request.evidenceReferences.length > 0 ||
          request.requestedAction === 'manual-required' ||
          request.requestedAction === 'suppress' ||
          'Tracking policy compiler requests need evidence except manual-required or suppress previews'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          request.rule.action !== 'ask-child-check-in' ||
          request.checkInId !== null ||
          'Tracking child check-in policy actions need a check-in id'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          request.rule.action !== 'start-temporary-live-tracking' ||
          (request.liveTrackingGrantId !== null && request.liveTrackingDurationSeconds !== null) ||
          'Tracking live policy actions need a grant id and duration'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          !trackingPolicyCompilerActionNeedsAlert(request.rule.action) ||
          request.alertId !== null ||
          'Tracking alert policy actions need an alert id'
      )
    )
);

export const TrackingPolicyCompilerResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    requestId: TrackingPolicyCompilerRequestIdSchema,
    requestedAction: TrackingPolicyCompilerRequestedActionSchema,
    finalActionSource: TrackingPolicyCompilerFinalActionSourceSchema,
    decision: TrackingPolicyDecisionSchema,
    alertIntent: Schema.Union(TrackingAlertIntentSchema, Schema.Null),
    childCheckInRequest: Schema.Union(TrackingChildCheckInRequestSchema, Schema.Null),
    escalationChain: Schema.Union(TrackingEscalationChainSchema, Schema.Null),
    temporaryLiveGrant: Schema.Union(TrackingTemporaryLiveTrackingGrantSchema, Schema.Null),
    parentPolicyFinalAuthority: Schema.Literal(true),
    aiFinalAuthority: Schema.Literal(false),
    runtimeEnforcementClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
  })
);

export type TrackingPolicyCompilerRequest = Infer<typeof TrackingPolicyCompilerRequestSchema>;
export type TrackingPolicyCompilerResult = Infer<typeof TrackingPolicyCompilerResultSchema>;

export function trackingPolicyCompilerActionNeedsAlert(action: TrackingPolicyRule['action']): boolean {
  return action === 'notify-parent' || action === 'request-parent-acknowledgement' || action === 'escalate';
}
