import { AgentEventDeliveryMode, AgentEventEnvelopeSchema } from '@ocentra-parent/event-domain/primitives';
import { EventingEventTypeSchema } from '@ocentra-parent/event-domain/eventing';
import { AgentTrackingConfigUpdateEventType } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import {
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { TrackingRetentionPolicySchema } from './tracking-evidence';
import { TrackingEvidenceSchemaVersion } from './tracking-primitives';

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

export const TrackingRuntimeEnabledStateLiteral = {
  Enabled: 'enabled',
  Disabled: 'disabled',
} as const;

export const TrackingRuntimeEnabledStateSchema = withParser(
  Schema.Literal(
    TrackingRuntimeEnabledStateLiteral.Enabled,
    TrackingRuntimeEnabledStateLiteral.Disabled
  )
);

export const TrackingEventNameSchema = withParser(
  Schema.Literal(
    AgentTrackingConfigUpdateEventType.Parent,
    AgentTrackingConfigUpdateEventType.Child,
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

export const TrackingRuntimeConfigUpdatedPayloadSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    requestedBy: ParentActorReferenceSchema,
    targetDevice: ParentDeviceReferenceSchema,
    retentionPolicy: TrackingRetentionPolicySchema,
    trackingEnabledState: TrackingRuntimeEnabledStateSchema,
  })
);

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
export type TrackingRuntimeEnabledState = Infer<typeof TrackingRuntimeEnabledStateSchema>;
export type TrackingRuntimeConfigUpdatedPayload = Infer<typeof TrackingRuntimeConfigUpdatedPayloadSchema>;
export type TrackingRuntimeConfigUpdatedEvent = Infer<typeof TrackingRuntimeConfigUpdatedEventSchema>;
export type TrackingRuntimeChildConfigUpdatedEvent = Infer<
  typeof TrackingRuntimeChildConfigUpdatedEventSchema
>;
export type TrackingRuntimeEventEnvelope = Infer<typeof TrackingRuntimeEventEnvelopeSchema>;

export const TrackingEventName = {
  ConfigUpdated: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Parent),
  ChildConfigUpdated: TrackingEventNameSchema.parse(AgentTrackingConfigUpdateEventType.Child),
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

export const TrackingRuntimeEnabledState = {
  Enabled: TrackingRuntimeEnabledStateSchema.parse(TrackingRuntimeEnabledStateLiteral.Enabled),
  Disabled: TrackingRuntimeEnabledStateSchema.parse(TrackingRuntimeEnabledStateLiteral.Disabled),
} as const;
