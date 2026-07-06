/* generated from crates/schema/src/child_domain_runtime_events_ts.rs */

import { AgentEventDeliveryMode, AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/event-primitives';
import { EventingEventTypeSchema } from '@ocentra-parent/schema-domain/eventing';
import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const ChildDomainChildDeviceIdSchema = brandedNonEmptyStringSchema('ChildDomainChildDeviceId');
export const ChildDomainChildProfileIdSchema = brandedNonEmptyStringSchema('ChildDomainChildProfileId');
export const ChildDomainObservationIdSchema = brandedNonEmptyStringSchema('ChildDomainObservationId');
export const ChildDomainSubjectRefSchema = brandedNonEmptyStringSchema('ChildDomainSubjectRef');
export const ChildDomainObservedStateSchema = brandedNonEmptyStringSchema('ChildDomainObservedState');
export const ChildDomainObservedAtSchema = brandedNonEmptyStringSchema('ChildDomainObservedAt');
export const ChildDomainEvidenceRefSchema = brandedNonEmptyStringSchema('ChildDomainEvidenceRef');
export const ChildDomainAiRequestIdSchema = brandedNonEmptyStringSchema('ChildDomainAiRequestId');
export const ChildDomainAnalysisPurposeSchema = brandedNonEmptyStringSchema('ChildDomainAnalysisPurpose');
export const ChildDomainFactRefSchema = brandedNonEmptyStringSchema('ChildDomainFactRef');
export const ChildDomainPolicyRequestIdSchema = brandedNonEmptyStringSchema('ChildDomainPolicyRequestId');
export const ChildDomainPolicyViolationIdSchema = brandedNonEmptyStringSchema('ChildDomainPolicyViolationId');
export const ChildDomainPolicyRuleRefSchema = brandedNonEmptyStringSchema('ChildDomainPolicyRuleRef');
export const ChildDomainPolicySeveritySchema = brandedNonEmptyStringSchema('ChildDomainPolicySeverity');
export const ChildDomainNotificationIdSchema = brandedNonEmptyStringSchema('ChildDomainNotificationId');
export const ChildDomainNotificationChannelSchema = brandedNonEmptyStringSchema('ChildDomainNotificationChannel');

export const ChildRuntimeDomainLiteral = {
  App: 'app',
  AppGame: 'app-game',
  Browser: 'browser',
  Lan: 'lan',
  Network: 'network',
  Screen: 'screen',
  ScreenLiveView: 'screen-live-view',
} as const;

export const ChildDomainRuntimeEventTypeLiteral = {
  AppObserved: 'app.activity.observed',
  AppEvidenceRecorded: 'app.evidence.recorded',
  AppAiAnalysisRequested: 'app.ai.analysis.requested',
  AppPolicyEvaluationRequested: 'app.policy.evaluation.requested',
  AppGameObserved: 'app-game.activity.observed',
  AppGameEvidenceRecorded: 'app-game.evidence.recorded',
  AppGameAiAnalysisRequested: 'app-game.ai.analysis.requested',
  AppGamePolicyEvaluationRequested: 'app-game.policy.evaluation.requested',
  BrowserObserved: 'browser.navigation.observed',
  BrowserEvidenceRecorded: 'browser.evidence.recorded',
  BrowserAiAnalysisRequested: 'browser.ai.analysis.requested',
  BrowserPolicyEvaluationRequested: 'browser.policy.evaluation.requested',
  LanObserved: 'lan.peer.observed',
  LanEvidenceRecorded: 'lan.evidence.recorded',
  LanAiAnalysisRequested: 'lan.ai.analysis.requested',
  LanPolicyEvaluationRequested: 'lan.policy.evaluation.requested',
  NetworkObserved: 'network.connection.observed',
  NetworkEvidenceRecorded: 'network.evidence.recorded',
  NetworkAiAnalysisRequested: 'network.ai.analysis.requested',
  NetworkPolicyEvaluationRequested: 'network.policy.evaluation.requested',
  ScreenObserved: 'screen.evidence.observed',
  ScreenEvidenceRecorded: 'screen.evidence.recorded',
  ScreenAiAnalysisRequested: 'screen.ai.analysis.requested',
  ScreenPolicyEvaluationRequested: 'screen.policy.evaluation.requested',
  ScreenLiveViewObserved: 'screen-live-view.session.observed',
  ScreenLiveViewEvidenceRecorded: 'screen-live-view.evidence.recorded',
  ScreenLiveViewAiAnalysisRequested: 'screen-live-view.ai.analysis.requested',
  ScreenLiveViewPolicyEvaluationRequested: 'screen-live-view.policy.evaluation.requested',
  AiAnalysisCompleted: 'child-domain.ai.analysis.completed',
  PolicyViolationDetected: 'child-domain.policy.violation.detected',
  NotificationRequested: 'child-domain.notification.requested',
} as const;

export const ChildDomainAiAnalysisRequirementLiteral = {
  Required: 'required',
  NotRequired: 'not-required',
} as const;

export const ChildDomainPolicyEvaluationRequirementLiteral = {
  Required: 'required',
  NotRequired: 'not-required',
} as const;

export const ChildDomainPrivatePayloadStateLiteral = {
  Excluded: 'excluded',
} as const;

export const ChildRuntimeDomainSchema = withParser(
  Schema.Literal(
    ChildRuntimeDomainLiteral.App,
    ChildRuntimeDomainLiteral.AppGame,
    ChildRuntimeDomainLiteral.Browser,
    ChildRuntimeDomainLiteral.Lan,
    ChildRuntimeDomainLiteral.Network,
    ChildRuntimeDomainLiteral.Screen,
    ChildRuntimeDomainLiteral.ScreenLiveView
  )
);

export const ChildDomainRuntimeEventTypeSchema = withParser(
  Schema.Literal(
    ChildDomainRuntimeEventTypeLiteral.AppObserved,
    ChildDomainRuntimeEventTypeLiteral.AppEvidenceRecorded,
    ChildDomainRuntimeEventTypeLiteral.AppAiAnalysisRequested,
    ChildDomainRuntimeEventTypeLiteral.AppPolicyEvaluationRequested,
    ChildDomainRuntimeEventTypeLiteral.AppGameObserved,
    ChildDomainRuntimeEventTypeLiteral.AppGameEvidenceRecorded,
    ChildDomainRuntimeEventTypeLiteral.AppGameAiAnalysisRequested,
    ChildDomainRuntimeEventTypeLiteral.AppGamePolicyEvaluationRequested,
    ChildDomainRuntimeEventTypeLiteral.BrowserObserved,
    ChildDomainRuntimeEventTypeLiteral.BrowserEvidenceRecorded,
    ChildDomainRuntimeEventTypeLiteral.BrowserAiAnalysisRequested,
    ChildDomainRuntimeEventTypeLiteral.BrowserPolicyEvaluationRequested,
    ChildDomainRuntimeEventTypeLiteral.LanObserved,
    ChildDomainRuntimeEventTypeLiteral.LanEvidenceRecorded,
    ChildDomainRuntimeEventTypeLiteral.LanAiAnalysisRequested,
    ChildDomainRuntimeEventTypeLiteral.LanPolicyEvaluationRequested,
    ChildDomainRuntimeEventTypeLiteral.NetworkObserved,
    ChildDomainRuntimeEventTypeLiteral.NetworkEvidenceRecorded,
    ChildDomainRuntimeEventTypeLiteral.NetworkAiAnalysisRequested,
    ChildDomainRuntimeEventTypeLiteral.NetworkPolicyEvaluationRequested,
    ChildDomainRuntimeEventTypeLiteral.ScreenObserved,
    ChildDomainRuntimeEventTypeLiteral.ScreenEvidenceRecorded,
    ChildDomainRuntimeEventTypeLiteral.ScreenAiAnalysisRequested,
    ChildDomainRuntimeEventTypeLiteral.ScreenPolicyEvaluationRequested,
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewObserved,
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewEvidenceRecorded,
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewAiAnalysisRequested,
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewPolicyEvaluationRequested,
    ChildDomainRuntimeEventTypeLiteral.AiAnalysisCompleted,
    ChildDomainRuntimeEventTypeLiteral.PolicyViolationDetected,
    ChildDomainRuntimeEventTypeLiteral.NotificationRequested
  ).pipe(
    Schema.filter(
      (eventType) =>
        EventingEventTypeSchema.safeParse(eventType).success ||
        'Expected child runtime event type to satisfy the shared eventing taxonomy'
    )
  )
);

export const ChildDomainAiAnalysisRequirementSchema = withParser(
  Schema.Literal(ChildDomainAiAnalysisRequirementLiteral.Required, ChildDomainAiAnalysisRequirementLiteral.NotRequired)
);

export const ChildDomainPolicyEvaluationRequirementSchema = withParser(
  Schema.Literal(
    ChildDomainPolicyEvaluationRequirementLiteral.Required,
    ChildDomainPolicyEvaluationRequirementLiteral.NotRequired
  )
);

export const ChildDomainPrivatePayloadStateSchema = withParser(
  Schema.Literal(ChildDomainPrivatePayloadStateLiteral.Excluded)
);

export const ChildDomainRuntimeEventEnvelopeSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    domain: ChildRuntimeDomainSchema,
    eventType: ChildDomainRuntimeEventTypeSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        event.envelope.eventName === event.eventType ||
        'Expected child runtime envelope eventName to match the child runtime event type'
    ),
    Schema.filter(
      (event) =>
        childDomainRuntimeEventTypeMatchesDomain(event.domain, event.eventType) ||
        'Expected child runtime event type to match the owning child runtime domain'
    ),
    Schema.filter(
      (event) =>
        event.envelope.deliveryMode === AgentEventDeliveryMode.FireAndForget ||
        event.envelope.deliveryMode === AgentEventDeliveryMode.RequestResponse ||
        'Expected child runtime event delivery to use the shared event-domain delivery modes'
    )
  )
);

const ChildDomainEvidenceRefsSchema = Schema.Array(ChildDomainEvidenceRefSchema).pipe(Schema.minItems(1));

export const ChildDomainObservedEventSchema = withParser(
  Schema.Struct({
    eventType: ChildDomainRuntimeEventTypeSchema,
    domain: ChildRuntimeDomainSchema,
    childDeviceId: ChildDomainChildDeviceIdSchema,
    childProfileId: ChildDomainChildProfileIdSchema,
    observationId: ChildDomainObservationIdSchema,
    subjectRef: ChildDomainSubjectRefSchema,
    observedState: ChildDomainObservedStateSchema,
    observedAt: ChildDomainObservedAtSchema,
    aiAnalysisRequirement: ChildDomainAiAnalysisRequirementSchema,
    policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirementSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        childDomainRuntimeEventTypeMatchesDomain(event.domain, event.eventType) ||
        'Expected child observed event type to match the owning child runtime domain'
    )
  )
);

export const ChildDomainEvidenceRecordedEventSchema = withParser(
  Schema.Struct({
    eventType: ChildDomainRuntimeEventTypeSchema,
    domain: ChildRuntimeDomainSchema,
    childDeviceId: ChildDomainChildDeviceIdSchema,
    childProfileId: ChildDomainChildProfileIdSchema,
    evidenceRef: ChildDomainEvidenceRefSchema,
    sourceObservationId: ChildDomainObservationIdSchema,
    sourceObservedAt: ChildDomainObservedAtSchema,
    signal: ChildDomainObservedStateSchema,
    aiAnalysisRequirement: ChildDomainAiAnalysisRequirementSchema,
    policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirementSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        childDomainRuntimeEventTypeMatchesDomain(event.domain, event.eventType) ||
        'Expected child evidence-recorded event type to match the owning child runtime domain'
    )
  )
);

export const ChildDomainAiAnalysisRequestedEventSchema = withParser(
  Schema.Struct({
    eventType: ChildDomainRuntimeEventTypeSchema,
    domain: ChildRuntimeDomainSchema,
    childDeviceId: ChildDomainChildDeviceIdSchema,
    childProfileId: ChildDomainChildProfileIdSchema,
    aiRequestId: ChildDomainAiRequestIdSchema,
    evidenceRefs: ChildDomainEvidenceRefsSchema,
    sourceObservedAt: ChildDomainObservedAtSchema,
    allowedAnalysisPurpose: ChildDomainAnalysisPurposeSchema,
    privatePayloadState: ChildDomainPrivatePayloadStateSchema,
    policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirementSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        childDomainRuntimeEventTypeMatchesDomain(event.domain, event.eventType) ||
        'Expected child AI request event type to match the owning child runtime domain'
    )
  )
);

export const ChildDomainAiAnalysisCompletedEventSchema = withParser(
  Schema.Struct({
    eventType: ChildDomainRuntimeEventTypeSchema,
    domain: ChildRuntimeDomainSchema,
    childDeviceId: ChildDomainChildDeviceIdSchema,
    childProfileId: ChildDomainChildProfileIdSchema,
    sourceAiRequestId: ChildDomainAiRequestIdSchema,
    evidenceRefs: ChildDomainEvidenceRefsSchema,
    sourceObservedAt: ChildDomainObservedAtSchema,
    resultFactRef: ChildDomainFactRefSchema,
    privatePayloadState: ChildDomainPrivatePayloadStateSchema,
    policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirementSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        event.eventType === ChildDomainRuntimeEventType.AiAnalysisCompleted ||
        'Expected AI completion to use the child-domain AI completed event type'
    )
  )
);

export const ChildDomainPolicyEvaluationRequestedEventSchema = withParser(
  Schema.Struct({
    eventType: ChildDomainRuntimeEventTypeSchema,
    domain: ChildRuntimeDomainSchema,
    childDeviceId: ChildDomainChildDeviceIdSchema,
    childProfileId: ChildDomainChildProfileIdSchema,
    policyRequestId: ChildDomainPolicyRequestIdSchema,
    evidenceRefs: ChildDomainEvidenceRefsSchema,
    sourceObservedAt: ChildDomainObservedAtSchema,
    sourceFactRef: ChildDomainFactRefSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        childDomainRuntimeEventTypeMatchesDomain(event.domain, event.eventType) ||
        'Expected child policy-requested event type to match the owning child runtime domain'
    )
  )
);

export const ChildDomainPolicyViolationDetectedEventSchema = withParser(
  Schema.Struct({
    eventType: ChildDomainRuntimeEventTypeSchema,
    domain: ChildRuntimeDomainSchema,
    childDeviceId: ChildDomainChildDeviceIdSchema,
    childProfileId: ChildDomainChildProfileIdSchema,
    violationId: ChildDomainPolicyViolationIdSchema,
    policyRuleRef: ChildDomainPolicyRuleRefSchema,
    severity: ChildDomainPolicySeveritySchema,
    detectedAt: ChildDomainObservedAtSchema,
    evidenceRefs: ChildDomainEvidenceRefsSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        event.eventType === ChildDomainRuntimeEventType.PolicyViolationDetected ||
        'Expected policy violation to use the child-domain policy violation event type'
    )
  )
);

export const ChildDomainNotificationRequestedEventSchema = withParser(
  Schema.Struct({
    eventType: ChildDomainRuntimeEventTypeSchema,
    domain: ChildRuntimeDomainSchema,
    childDeviceId: ChildDomainChildDeviceIdSchema,
    childProfileId: ChildDomainChildProfileIdSchema,
    notificationId: ChildDomainNotificationIdSchema,
    sourcePolicyViolationId: ChildDomainPolicyViolationIdSchema,
    channel: ChildDomainNotificationChannelSchema,
    requestedAt: ChildDomainObservedAtSchema,
    evidenceRefs: ChildDomainEvidenceRefsSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        event.eventType === ChildDomainRuntimeEventType.NotificationRequested ||
        'Expected notification request to use the child-domain notification event type'
    )
  )
);

export type ChildRuntimeDomain = Infer<typeof ChildRuntimeDomainSchema>;
export type ChildDomainRuntimeEventType = Infer<typeof ChildDomainRuntimeEventTypeSchema>;
export type ChildDomainRuntimeEventEnvelope = Infer<typeof ChildDomainRuntimeEventEnvelopeSchema>;
export type ChildDomainAiAnalysisRequirement = Infer<typeof ChildDomainAiAnalysisRequirementSchema>;
export type ChildDomainPolicyEvaluationRequirement = Infer<typeof ChildDomainPolicyEvaluationRequirementSchema>;
export type ChildDomainPrivatePayloadState = Infer<typeof ChildDomainPrivatePayloadStateSchema>;
export type ChildDomainObservedEvent = Infer<typeof ChildDomainObservedEventSchema>;
export type ChildDomainEvidenceRecordedEvent = Infer<typeof ChildDomainEvidenceRecordedEventSchema>;
export type ChildDomainAiAnalysisRequestedEvent = Infer<typeof ChildDomainAiAnalysisRequestedEventSchema>;
export type ChildDomainAiAnalysisCompletedEvent = Infer<typeof ChildDomainAiAnalysisCompletedEventSchema>;
export type ChildDomainPolicyEvaluationRequestedEvent = Infer<typeof ChildDomainPolicyEvaluationRequestedEventSchema>;
export type ChildDomainPolicyViolationDetectedEvent = Infer<typeof ChildDomainPolicyViolationDetectedEventSchema>;
export type ChildDomainNotificationRequestedEvent = Infer<typeof ChildDomainNotificationRequestedEventSchema>;

export const ChildDomainRuntimeEventType = {
  AppObserved: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.AppObserved),
  AppEvidenceRecorded: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.AppEvidenceRecorded),
  AppAiAnalysisRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.AppAiAnalysisRequested
  ),
  AppPolicyEvaluationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.AppPolicyEvaluationRequested
  ),
  AppGameObserved: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.AppGameObserved),
  AppGameEvidenceRecorded: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.AppGameEvidenceRecorded
  ),
  AppGameAiAnalysisRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.AppGameAiAnalysisRequested
  ),
  AppGamePolicyEvaluationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.AppGamePolicyEvaluationRequested
  ),
  BrowserObserved: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.BrowserObserved),
  BrowserEvidenceRecorded: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.BrowserEvidenceRecorded
  ),
  BrowserAiAnalysisRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.BrowserAiAnalysisRequested
  ),
  BrowserPolicyEvaluationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.BrowserPolicyEvaluationRequested
  ),
  LanObserved: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.LanObserved),
  LanEvidenceRecorded: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.LanEvidenceRecorded),
  LanAiAnalysisRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.LanAiAnalysisRequested
  ),
  LanPolicyEvaluationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.LanPolicyEvaluationRequested
  ),
  NetworkObserved: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.NetworkObserved),
  NetworkEvidenceRecorded: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.NetworkEvidenceRecorded
  ),
  NetworkAiAnalysisRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.NetworkAiAnalysisRequested
  ),
  NetworkPolicyEvaluationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.NetworkPolicyEvaluationRequested
  ),
  ScreenObserved: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.ScreenObserved),
  ScreenEvidenceRecorded: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.ScreenEvidenceRecorded
  ),
  ScreenAiAnalysisRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.ScreenAiAnalysisRequested
  ),
  ScreenPolicyEvaluationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.ScreenPolicyEvaluationRequested
  ),
  ScreenLiveViewObserved: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewObserved
  ),
  ScreenLiveViewEvidenceRecorded: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewEvidenceRecorded
  ),
  ScreenLiveViewAiAnalysisRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewAiAnalysisRequested
  ),
  ScreenLiveViewPolicyEvaluationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.ScreenLiveViewPolicyEvaluationRequested
  ),
  AiAnalysisCompleted: ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.AiAnalysisCompleted),
  PolicyViolationDetected: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.PolicyViolationDetected
  ),
  NotificationRequested: ChildDomainRuntimeEventTypeSchema.parse(
    ChildDomainRuntimeEventTypeLiteral.NotificationRequested
  ),
} as const;

export const ChildDomainAiAnalysisRequirement = {
  Required: ChildDomainAiAnalysisRequirementSchema.parse(ChildDomainAiAnalysisRequirementLiteral.Required),
  NotRequired: ChildDomainAiAnalysisRequirementSchema.parse(ChildDomainAiAnalysisRequirementLiteral.NotRequired),
} as const;

export const ChildDomainPolicyEvaluationRequirement = {
  Required: ChildDomainPolicyEvaluationRequirementSchema.parse(ChildDomainPolicyEvaluationRequirementLiteral.Required),
  NotRequired: ChildDomainPolicyEvaluationRequirementSchema.parse(
    ChildDomainPolicyEvaluationRequirementLiteral.NotRequired
  ),
} as const;

export const ChildDomainPrivatePayloadState = {
  Excluded: ChildDomainPrivatePayloadStateSchema.parse(ChildDomainPrivatePayloadStateLiteral.Excluded),
} as const;

function childDomainRuntimeEventTypeMatchesDomain(
  domain: ChildRuntimeDomain,
  eventType: ChildDomainRuntimeEventType
): boolean {
  if (childDomainRuntimeEventTypeIsCrossDomain(eventType)) {
    return true;
  }
  return eventType.startsWith(`${domain}.`);
}

function childDomainRuntimeEventTypeIsCrossDomain(eventType: ChildDomainRuntimeEventType): boolean {
  return (
    eventType === ChildDomainRuntimeEventType.AiAnalysisCompleted ||
    eventType === ChildDomainRuntimeEventType.PolicyViolationDetected ||
    eventType === ChildDomainRuntimeEventType.NotificationRequested
  );
}
