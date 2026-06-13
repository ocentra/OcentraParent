import { describe, expect, it } from 'vitest';
import {
  AgentEventDeliveryMode,
  AgentPeerRoleLiteral,
  AgentRouteLiteral,
} from '@ocentra-parent/event-domain/primitives';
import { AgentTrackingConfigUpdateEventType } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import {
  TrackingEventName,
  TrackingRuntimeEnabledState,
  TrackingRuntimeChildConfigUpdatedEventSchema,
  TrackingRuntimeConfigUpdatedEventSchema,
  TrackingRuntimeConfigUpdatedPayloadSchema,
  TrackingRuntimeEventEnvelopeSchema,
} from '../../src/tracking-event-contracts';

const retentionPolicy = {
  schemaVersion: 1,
  policyId: 'tracking-retention-default',
  mode: '7d',
  custodyLabel: 'child-device-local',
  customRetentionHours: null,
  deleteOnResolution: false,
  exportAllowed: false,
  remoteSyncDefault: 'disabled',
  auditRefs: ['parent-config-update'],
} as const;

describe('tracking event contracts', () => {
  it('parses tracking config update payload as a named tracking schema', () => {
    const payload = TrackingRuntimeConfigUpdatedPayloadSchema.parse({
      schemaVersion: 1,
      requestedBy: { actorId: 'parent-1', role: 'parent' },
      targetDevice: {
        deviceId: 'child-device-1',
        childProfileId: 'child-profile-1',
        label: 'Child Android',
        platform: 'android',
      },
      retentionPolicy,
      trackingEnabledState: TrackingRuntimeEnabledState.Enabled,
    });

    expect(payload.retentionPolicy.policyId).toBe('tracking-retention-default');
  });

  it('composes shared event envelope metadata with tracking-owned payload schema', () => {
    const event = TrackingRuntimeConfigUpdatedEventSchema.parse({
      envelope: {
        eventId: 'event-1',
        eventName: TrackingEventName.ConfigUpdated,
        correlationId: 'correlation-1',
        occurredAt: '2026-06-12T10:00:00.000Z',
        source: { peerId: 'portal-runtime', role: AgentPeerRoleLiteral.Portal },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.RequestResponse,
      },
      payload: {
        schemaVersion: 1,
        requestedBy: { actorId: 'parent-1', role: 'parent' },
        targetDevice: {
          deviceId: 'child-device-1',
          childProfileId: 'child-profile-1',
          label: 'Child Android',
          platform: 'android',
        },
        retentionPolicy,
        trackingEnabledState: TrackingRuntimeEnabledState.Enabled,
      },
    });

    expect(event.envelope.eventName).toBe(AgentTrackingConfigUpdateEventType.Parent);
    expect(event.payload.trackingEnabledState).toBe(TrackingRuntimeEnabledState.Enabled);
  });

  it('rejects tracking config update events that use another tracking event name', () => {
    const result = TrackingRuntimeConfigUpdatedEventSchema.safeParse({
      envelope: {
        eventId: 'event-1',
        eventName: TrackingEventName.LocationObserved,
        correlationId: 'correlation-1',
        occurredAt: '2026-06-12T10:00:00.000Z',
        source: { peerId: 'portal-runtime', role: AgentPeerRoleLiteral.Portal },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.RequestResponse,
      },
      payload: {
        schemaVersion: 1,
        requestedBy: { actorId: 'parent-1', role: 'parent' },
        targetDevice: {
          deviceId: 'child-device-1',
          childProfileId: 'child-profile-1',
          label: 'Child Android',
          platform: 'android',
        },
        retentionPolicy,
        trackingEnabledState: TrackingRuntimeEnabledState.Enabled,
      },
    });

    expect(result.success).toBe(false);
  });

  it('composes child tracking config delivery as a typed child runtime event', () => {
    const event = TrackingRuntimeChildConfigUpdatedEventSchema.parse({
      envelope: {
        eventId: 'event-child-config-1',
        eventName: TrackingEventName.ChildConfigUpdated,
        correlationId: 'correlation-child-config-1',
        occurredAt: '2026-06-12T10:00:01.000Z',
        source: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      payload: {
        schemaVersion: 1,
        requestedBy: { actorId: 'parent-1', role: 'parent' },
        targetDevice: {
          deviceId: 'child-device-1',
          childProfileId: 'child-profile-1',
          label: 'Child Android',
          platform: 'android',
        },
        retentionPolicy,
        trackingEnabledState: TrackingRuntimeEnabledState.Enabled,
      },
    });

    expect(event.envelope.eventName).toBe(AgentTrackingConfigUpdateEventType.Child);
    expect(event.payload.targetDevice.deviceId).toBe('child-device-1');
  });

  it('parses fire-and-forget tracking runtime event envelopes for child evidence flow', () => {
    const event = TrackingRuntimeEventEnvelopeSchema.parse({
      envelope: {
        eventId: 'event-2',
        eventName: TrackingEventName.EvidenceRecorded,
        correlationId: 'correlation-2',
        occurredAt: '2026-06-12T10:01:00.000Z',
        source: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      eventName: TrackingEventName.EvidenceRecorded,
    });

    expect(event.envelope.eventName).toBe(TrackingEventName.EvidenceRecorded);
  });

  it('parses tracking AI policy and acknowledgement event names as one taxonomy', () => {
    expect(TrackingEventName.AiAnalysisRequested).toBe('tracking.ai.analysis.requested');
    expect(TrackingEventName.NearbyPlaceClassified).toBe('tracking.nearby-place.classified');
    expect(TrackingEventName.GeofenceTransitionDetected).toBe(
      'tracking.geofence.transition.detected'
    );
    expect(TrackingEventName.ExpectedPlaceStateEvaluated).toBe(
      'tracking.expected-place.state.evaluated'
    );
    expect(TrackingEventName.PolicyViolationDetected).toBe(
      'tracking.policy.violation.detected'
    );
    expect(TrackingEventName.ParentAcknowledgementRecorded).toBe(
      'tracking.parent-acknowledgement.recorded'
    );
    expect(TrackingEventName.ChildCheckInRecorded).toBe('tracking.child-check-in.recorded');
  });

  it('rejects fire-and-forget delivery for tracking config update events', () => {
    const result = TrackingRuntimeEventEnvelopeSchema.safeParse({
      envelope: {
        eventId: 'event-3',
        eventName: TrackingEventName.ConfigUpdated,
        correlationId: 'correlation-3',
        occurredAt: '2026-06-12T10:02:00.000Z',
        source: { peerId: 'portal-runtime', role: AgentPeerRoleLiteral.Portal },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      eventName: TrackingEventName.ConfigUpdated,
    });

    expect(result.success).toBe(false);
  });

  it('rejects mismatched envelope and typed tracking event names', () => {
    const result = TrackingRuntimeEventEnvelopeSchema.safeParse({
      envelope: {
        eventId: 'event-4',
        eventName: TrackingEventName.PolicyViolationDetected,
        correlationId: 'correlation-4',
        occurredAt: '2026-06-12T10:03:00.000Z',
        source: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      eventName: TrackingEventName.ParentNotificationRequested,
    });

    expect(result.success).toBe(false);
  });
});
