import { resolveLiveActivityState as resolvePortalDomainLiveActivityState } from '@ocentra-parent/portal-domain/live-activity-state';
import type {
  PortalBrowserRuntimeEventChainEntry as PortalDomainPortalBrowserRuntimeEventChainEntry,
  PortalBrowserRuntimeEventChainStream as PortalDomainPortalBrowserRuntimeEventChainStream,
  PortalLiveActivityState as PortalDomainPortalLiveActivityState,
  PortalNetworkRuntimeEventChainStream as PortalDomainPortalNetworkRuntimeEventChainStream,
} from '@ocentra-parent/portal-domain/live-activity-state';

import {
  AgentEvent,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import type {
  ParentRouteEventSnapshot,
  ParentRouteLiveActivitySnapshot,
} from './generated/parent-ui-bridge';

export type PortalBrowserRuntimeEventChainEntry = PortalDomainPortalBrowserRuntimeEventChainEntry;
export type PortalBrowserRuntimeEventChainStream = PortalDomainPortalBrowserRuntimeEventChainStream;
export type PortalLiveActivityState = PortalDomainPortalLiveActivityState;
export type PortalNetworkRuntimeEventChainStream = PortalDomainPortalNetworkRuntimeEventChainStream;

const EMPTY_LIVE_ACTIVITY_STATE: PortalLiveActivityState = resolvePortalDomainLiveActivityState([]);

export function resolveLiveActivityState(
  events: readonly AgentEventEnvelope[],
  snapshot?: ParentRouteLiveActivitySnapshot | null
): PortalLiveActivityState {
  const fallback = resolvePortalDomainLiveActivityState(events);
  return overlaySnapshotLiveActivityState(fallback, snapshot);
}

export function resolveSnapshotLiveActivityState(
  snapshot?: ParentRouteLiveActivitySnapshot | null
): PortalLiveActivityState {
  return overlaySnapshotLiveActivityState(EMPTY_LIVE_ACTIVITY_STATE, snapshot);
}

function overlaySnapshotLiveActivityState(
  fallback: PortalLiveActivityState,
  snapshot?: ParentRouteLiveActivitySnapshot | null
): PortalLiveActivityState {
  if (snapshot === null || snapshot === undefined) {
    return fallback;
  }
  return {
    ...fallback,
    recentSummary: snapshotValueOrFallback(snapshot.recentSummary, fallback.recentSummary),
    ingestStatus: snapshotValueOrFallback(snapshot.ingestStatus, fallback.ingestStatus),
    activityScreenReadModel: snapshotValueOrFallback(snapshot.activityScreenReadModel, fallback.activityScreenReadModel),
    browserManagedEvent: snapshotEventOrFallback(snapshot.browserManagedEvent, fallback.browserManagedEvent),
    browserManagedStatus: snapshotValueOrFallback(snapshot.browserManagedStatus, fallback.browserManagedStatus),
    browserRuntimeEventChainStream: snapshotValueOrFallback(
      snapshot.browserRuntimeEventChainStream,
      fallback.browserRuntimeEventChainStream
    ),
    browserSocialProviderReceiptStreamStatusIntent: snapshotValueOrFallback(
      snapshot.browserSocialProviderReceiptStreamStatusIntent,
      fallback.browserSocialProviderReceiptStreamStatusIntent
    ),
    browserSocialProviderReceiptIngestionReadinessStatusIntent: snapshotValueOrFallback(
      snapshot.browserSocialProviderReceiptIngestionReadinessStatusIntent,
      fallback.browserSocialProviderReceiptIngestionReadinessStatusIntent
    ),
    localAiRuntimeStatusEvent: snapshotEventOrFallback(
      snapshot.localAiRuntimeStatusEvent,
      fallback.localAiRuntimeStatusEvent
    ),
    lanAiJobEvent: snapshotEventOrFallback(snapshot.lanAiJobEvent, fallback.lanAiJobEvent),
    parentAssistantBoundaryEvent: snapshotEventOrFallback(
      snapshot.parentAssistantBoundaryEvent,
      fallback.parentAssistantBoundaryEvent
    ),
    activityMemoryGraphReadModel: snapshotValueOrFallback(
      snapshot.activityMemoryGraphReadModel,
      fallback.activityMemoryGraphReadModel
    ),
    networkFlowEvent: snapshotEventOrFallback(snapshot.networkFlowEvent, fallback.networkFlowEvent),
    networkFlowReadModel: snapshotValueOrFallback(snapshot.networkFlowReadModel, fallback.networkFlowReadModel),
    networkRuntimeEventChainStream: snapshotValueOrFallback(
      snapshot.networkRuntimeEventChainStream,
      fallback.networkRuntimeEventChainStream
    ),
    lanPairingBrowserDiscoveryEvent: snapshotEventOrFallback(
      snapshot.lanPairingBrowserDiscoveryEvent,
      fallback.lanPairingBrowserDiscoveryEvent
    ),
    lanAddDeviceReadModel: snapshotValueOrFallback(snapshot.lanAddDeviceReadModel, fallback.lanAddDeviceReadModel),
    policyPreviewEvent: snapshotEventOrFallback(snapshot.policyPreviewEvent, fallback.policyPreviewEvent),
    policyPreviewReadModel: snapshotValueOrFallback(snapshot.policyPreviewReadModel, fallback.policyPreviewReadModel),
    appGameNotificationParentSurfaceIntentReadModel: snapshotValueOrFallback(
      snapshot.appGameNotificationParentSurfaceIntentReadModel,
      fallback.appGameNotificationParentSurfaceIntentReadModel
    ),
    appGamePolicyReadinessReadModel: snapshotValueOrFallback(
      snapshot.appGamePolicyReadinessReadModel,
      fallback.appGamePolicyReadinessReadModel
    ),
    appGamePlatformProofStatusReadModel: snapshotValueOrFallback(
      snapshot.appGamePlatformProofStatusReadModel,
      fallback.appGamePlatformProofStatusReadModel
    ),
    appGameChildRuntimeTransportReceiptReadModel: snapshotValueOrFallback(
      snapshot.appGameChildRuntimeTransportReceiptReadModel,
      fallback.appGameChildRuntimeTransportReceiptReadModel
    ),
    appGameAdapterDispatchPreflightReadModel: snapshotValueOrFallback(
      snapshot.appGameAdapterDispatchPreflightReadModel,
      fallback.appGameAdapterDispatchPreflightReadModel
    ),
    appGameAdapterDispatchResultReadModel: snapshotValueOrFallback(
      snapshot.appGameAdapterDispatchResultReadModel,
      fallback.appGameAdapterDispatchResultReadModel
    ),
    appGameAdapterDispatchExecutedResult: snapshotValueOrFallback(
      snapshot.appGameAdapterDispatchExecutedResult,
      fallback.appGameAdapterDispatchExecutedResult
    ),
    appGameTimerParentSurfaceReadModel: snapshotValueOrFallback(
      snapshot.appGameTimerParentSurfaceReadModel,
      fallback.appGameTimerParentSurfaceReadModel
    ),
    browserInterventionEvent: snapshotEventOrFallback(
      snapshot.browserInterventionEvent,
      fallback.browserInterventionEvent
    ),
    browserInterventionReadModel: snapshotValueOrFallback(
      snapshot.browserInterventionReadModel,
      fallback.browserInterventionReadModel
    ),
    activityTrackingReadModelEvent: snapshotEventOrFallback(
      snapshot.activityTrackingReadModelEvent,
      fallback.activityTrackingReadModelEvent
    ),
    activityTrackingReadModel: snapshotValueOrFallback(
      snapshot.activityTrackingReadModel,
      fallback.activityTrackingReadModel
    ),
    activityTrackingRetentionSettingsWriteResult: snapshotValueOrFallback(
      snapshot.activityTrackingRetentionSettingsWriteResult,
      fallback.activityTrackingRetentionSettingsWriteResult
    ),
  };
}

function snapshotEventOrFallback(
  snapshot: ParentRouteEventSnapshot | null | undefined,
  fallback: AgentEventEnvelope | null
): AgentEventEnvelope | null {
  if (snapshot === undefined) {
    return fallback;
  }
  if (snapshot === null) {
    return null;
  }
  return {
    schemaVersion: 1,
    eventId: snapshot.eventId ?? 'host-bridge-event',
    correlationId: 'host-bridge',
    sentAt: snapshot.sentAt ?? '',
    source: {
      peerId: 'host-bridge',
      role: 'portal',
    },
    target: {
      peerId: 'host-bridge',
      role: 'portal',
    },
    event: snapshotEventNameOrFallback(snapshot.event),
    severity: snapshot.severity ?? 'info',
    payload: snapshot.payload ?? {},
    snapshot: null,
  } as AgentEventEnvelope;
}

function snapshotEventNameOrFallback(eventName: string | null | undefined): AgentEventName {
  if (eventName === undefined || eventName === null) {
    return AgentEvent.LogSnapshotReported;
  }
  return eventName as AgentEventName;
}

function snapshotValueOrFallback<T>(snapshot: unknown, fallback: T): T {
  if (snapshot === undefined) {
    return fallback;
  }
  return snapshot as T;
}
