import { AgentEventDeliveryMode, AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/event-primitives';
import { EventingEventTypeSchema } from '@ocentra-parent/schema-domain/eventing';
import {
  AgentTrackingConfigUpdateRequestSchema,
  AgentTrackingConfigUpdateEventType,
  AgentTrackingRuntimeEnabledState,
  AgentTrackingRuntimeEnabledStateSchema,
  TrackingConfigUpdateAppliedEventSchema,
} from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const TrackingRuntimeEventNameLiteral = {
  LocationObserved: 'tracking.location.observed',
  EvidenceRecorded: 'tracking.evidence.recorded',
  AiAnalysisRequested: 'tracking.ai.analysis.requested',
  NearbyPlaceClassified: 'tracking.nearby-place.classified',
  GeofenceTransitionDetected: 'tracking.geofence.transition.detected',
  ExpectedPlaceStateEvaluated: 'tracking.expected-place.state.evaluated',
  PolicyViolationDetected: 'tracking.policy.violation.detected',
  ParentAcknowledgementRecorded: 'tracking.parent-acknowledgement.recorded',
  ChildCheckInRecorded: 'tracking.child-check-in.recorded',
  ParentNotificationRequested: 'tracking.parent.notification.requested',
} as const;

export const TrackingEventNameSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigUpdateEventType.Parent,
    AgentTrackingConfigUpdateEventType.Child,
    AgentTrackingConfigUpdateEventType.Applied,
    TrackingRuntimeEventNameLiteral.LocationObserved,
    TrackingRuntimeEventNameLiteral.EvidenceRecorded,
    TrackingRuntimeEventNameLiteral.AiAnalysisRequested,
    TrackingRuntimeEventNameLiteral.NearbyPlaceClassified,
    TrackingRuntimeEventNameLiteral.GeofenceTransitionDetected,
    TrackingRuntimeEventNameLiteral.ExpectedPlaceStateEvaluated,
    TrackingRuntimeEventNameLiteral.PolicyViolationDetected,
    TrackingRuntimeEventNameLiteral.ParentAcknowledgementRecorded,
    TrackingRuntimeEventNameLiteral.ChildCheckInRecorded,
    TrackingRuntimeEventNameLiteral.ParentNotificationRequested
  ).pipe(
    Schema.filter(
      (eventName) =>
        EventingEventTypeSchema.safeParse(eventName).success ||
        'Expected tracking event name to satisfy the shared eventing taxonomy'
    )
  )
);

export const TrackingRuntimeConfigUpdatedPayloadSchema = AgentTrackingConfigUpdateRequestSchema;

export const TrackingRuntimeConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    payload: TrackingRuntimeConfigUpdatedPayloadSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        (event.envelope.eventName === TrackingEventName.ConfigUpdated &&
          event.envelope.deliveryMode === AgentEventDeliveryMode.RequestResponse) ||
        'Tracking config update events use the tracking-owned payload schema and request-response delivery'
    )
  )
);

export const TrackingRuntimeChildConfigUpdatedEventSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    payload: TrackingRuntimeConfigUpdatedPayloadSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        (event.envelope.eventName === TrackingEventName.ChildConfigUpdated &&
          event.envelope.deliveryMode === AgentEventDeliveryMode.FireAndForget) ||
        'Child tracking config update events use the tracking-owned payload schema and fire-and-forget child delivery'
    )
  )
);

export const TrackingRuntimeChildConfigAppliedEventSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    payload: TrackingConfigUpdateAppliedEventSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        (event.envelope.eventName === TrackingEventName.ChildConfigApplied &&
          event.envelope.deliveryMode === AgentEventDeliveryMode.FireAndForget) ||
        'Child tracking config applied events use the canonical applied payload schema and fire-and-forget delivery'
    )
  )
);

export const TrackingRuntimeEventEnvelopeSchema = withParser(
  Schema.Struct({
    envelope: AgentEventEnvelopeSchema,
    eventName: TrackingEventNameSchema,
  }).pipe(
    Schema.filter(
      (event) =>
        event.envelope.eventName === event.eventName ||
        'Expected tracking runtime envelope eventName to match the typed tracking event name'
    ),
    Schema.filter(
      (event) =>
        event.eventName !== TrackingEventName.ConfigUpdated ||
        event.envelope.deliveryMode === AgentEventDeliveryMode.RequestResponse ||
        'Tracking config updates require request-response delivery'
    ),
    Schema.filter(
      (event) =>
        event.eventName === TrackingEventName.ConfigUpdated ||
        event.envelope.deliveryMode === AgentEventDeliveryMode.FireAndForget ||
        'Tracking runtime evidence-flow events are fire-and-forget events'
    )
  )
);

export type TrackingEventName = Infer<typeof TrackingEventNameSchema>;
export type TrackingRuntimeEnabledState = Infer<typeof AgentTrackingRuntimeEnabledStateSchema>;
export type TrackingRuntimeConfigUpdatedPayload = Infer<typeof TrackingRuntimeConfigUpdatedPayloadSchema>;
export type TrackingRuntimeConfigUpdatedEvent = Infer<typeof TrackingRuntimeConfigUpdatedEventSchema>;
export type TrackingRuntimeChildConfigUpdatedEvent = Infer<
  typeof TrackingRuntimeChildConfigUpdatedEventSchema
>;
export type TrackingRuntimeChildConfigAppliedEvent = Infer<
  typeof TrackingRuntimeChildConfigAppliedEventSchema
>;
export type TrackingRuntimeEventEnvelope = Infer<typeof TrackingRuntimeEventEnvelopeSchema>;

export const TrackingEventName = {
  ConfigUpdated: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Parent),
  ChildConfigUpdated: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Child),
  ChildConfigApplied: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Applied),
  LocationObserved: TrackingEventNameSchema.parse(TrackingRuntimeEventNameLiteral.LocationObserved),
  EvidenceRecorded: TrackingEventNameSchema.parse(TrackingRuntimeEventNameLiteral.EvidenceRecorded),
  AiAnalysisRequested: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.AiAnalysisRequested
  ),
  NearbyPlaceClassified: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.NearbyPlaceClassified
  ),
  GeofenceTransitionDetected: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.GeofenceTransitionDetected
  ),
  ExpectedPlaceStateEvaluated: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ExpectedPlaceStateEvaluated
  ),
  PolicyViolationDetected: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.PolicyViolationDetected
  ),
  ParentAcknowledgementRecorded: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ParentAcknowledgementRecorded
  ),
  ChildCheckInRecorded: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ChildCheckInRecorded
  ),
  ParentNotificationRequested: TrackingEventNameSchema.parse(
    TrackingRuntimeEventNameLiteral.ParentNotificationRequested
  ),
} as const;

export const TrackingRuntimeEnabledState = AgentTrackingRuntimeEnabledState;
