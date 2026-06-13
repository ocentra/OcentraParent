import { describe, expect, it } from 'vitest';
import {
  AgentEventDeliveryMode,
  AgentPeerRoleLiteral,
  AgentRouteLiteral,
} from '@ocentra-parent/event-domain/primitives';
import {
  AgentTrackingAiBoundaryMode,
  AgentTrackingNotificationMode,
  AgentTrackingRetentionSettingsWriteDefaults,
  AgentTrackingRuntimeMode,
  AgentTrackingConfigUpdateEventType,
  AgentTrackingConfigUpdateResponseStateLiteral,
  AgentTrackingConfigUpdateTargetScopeLiteral,
  AgentTrackingDurableSettingsPersistenceStateLiteral,
  AgentTrackingEffectiveStateLiteral,
} from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import {
  TrackingEventName,
  TrackingRuntimeEnabledState,
  TrackingRuntimeChildConfigUpdatedEventSchema,
  TrackingRuntimeChildConfigAppliedEventSchema,
  TrackingRuntimeConfigUpdatedEventSchema,
  TrackingRuntimeConfigUpdatedPayloadSchema,
  TrackingRuntimeEventEnvelopeSchema,
} from '../../src/tracking-event-contracts';

const trackingConfigUpdatePayload = {
  commandId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
  runtimeConfig: {
    trackingEnabledState: TrackingRuntimeEnabledState.Enabled,
    trackingMode: AgentTrackingRuntimeMode.ObserveOnly,
    aiBoundaryMode: AgentTrackingAiBoundaryMode.RequestWhenUncertain,
    notificationMode: AgentTrackingNotificationMode.ParentPortalOnly,
  },
  retentionSettings: {
    schemaVersion: 1,
    commandId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
    settingsKind: AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    requestedRetentionWindowHours: 168,
    requestedDeleteAfterAlertResolutionState: 'retain-after-alert-resolved',
    requestedParentExportState: 'not-prepared',
    requestedRemoteSyncState: 'disabled',
    requestedRemoteAiState: 'disabled',
    sourceWriterIntentRefs: [AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
    sourceReadModelProofRefs: AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs,
  },
} as const;

describe('tracking event contracts', () => {
  it('parses tracking config update payload as a named tracking schema', () => {
    const payload = TrackingRuntimeConfigUpdatedPayloadSchema.parse(trackingConfigUpdatePayload);

    expect(payload.retentionSettings.settingsKind).toBe(
      AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow
    );
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
      payload: trackingConfigUpdatePayload,
    });

    expect(event.envelope.eventName).toBe(AgentTrackingConfigUpdateEventType.Parent);
    expect(event.payload.runtimeConfig.trackingEnabledState).toBe(
      TrackingRuntimeEnabledState.Enabled
    );
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
      payload: trackingConfigUpdatePayload,
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
      payload: trackingConfigUpdatePayload,
    });

    expect(event.envelope.eventName).toBe(AgentTrackingConfigUpdateEventType.Child);
    expect(event.payload.retentionSettings.commandId).toBe(
      AgentTrackingRetentionSettingsWriteDefaults.CommandId
    );
  });

  it('composes child tracking config applied as a durable typed child runtime event', () => {
    const event = TrackingRuntimeChildConfigAppliedEventSchema.parse({
      envelope: {
        eventId: 'event-child-config-applied-1',
        eventName: TrackingEventName.ChildConfigApplied,
        correlationId: 'correlation-child-config-applied-1',
        occurredAt: '2026-06-12T10:00:02.000Z',
        source: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      payload: {
        parentEventType: AgentTrackingConfigUpdateEventType.Parent,
        childEventType: AgentTrackingConfigUpdateEventType.Child,
        sourceCommandId: 'tracking-retention-settings-write-command',
        target: {
          scope: AgentTrackingConfigUpdateTargetScopeLiteral.ChildDevice,
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        responseState: AgentTrackingConfigUpdateResponseStateLiteral.Applied,
        effectiveTrackingState: AgentTrackingEffectiveStateLiteral.Enabled,
        localServiceStateRevision: 1,
        durableSettingsPersistenceState:
          AgentTrackingDurableSettingsPersistenceStateLiteral.Persisted,
      },
    });

    expect(event.envelope.eventName).toBe(AgentTrackingConfigUpdateEventType.Applied);
    expect(event.payload.responseState).toBe(
      AgentTrackingConfigUpdateResponseStateLiteral.Applied
    );
    expect(event.payload.localServiceStateRevision).toBe(1);
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
