import {
  BrowserActiveProofSource,
  BrowserCustodyLabel,
  BrowserEvidenceReadModelSchema,
  BrowserEvidenceSchemaVersion,
  BrowserInventoryReadModelSchema,
  BrowserManagedSessionStatusSchema,
  BrowserQueryVisibilityLabel,
  type BrowserEvidenceReadModel,
  type BrowserInterventionReadModel,
  type BrowserInventoryReadModel,
  type BrowserManagedSessionStatus,
} from '@ocentra-parent/activity-domain/browser';
import {
  ActivityIngestStatusSchema,
  ActivityQuerySchemaVersion,
  ActivityRecentSummarySchema,
  type ActivityIngestStatus,
  type ActivityRecentSummary,
} from '@ocentra-parent/activity-domain/query';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/activity-domain/network-flow';
import type {
  ActivityAppUseReadModel,
  ActivityBrowserReadModel,
  ActivityGamesReadModel,
  ActivityHistoricalReportList,
  ActivityNetworkReadModel,
  ActivityReportDocument,
  ActivityScreenReadModel,
} from '@ocentra-parent/activity-domain/activity-surface';
import {
  ActivitySurfaceReadModelKindName,
  parseActivityServiceUiSpineEvents,
  parseActivityReadModelEvent,
  parseActivityReportDocumentEvent,
  parseActivityReportHistoryEvent,
  type ActivitySurfaceAdapterResult,
  type ActivityServiceUiSpine,
  type ActivitySurfaceReadModelKind,
} from '@ocentra-parent/agent-protocol-domain/activity-surface-adapter';
import {
  AgentEvent,
  AgentLanBrowserAddDeviceReadModelSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentLanBrowserAddDeviceReadModel,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  parseAgentNetworkLiveCaptureStatusEvent,
  type AgentNetworkLiveCaptureStatusParseResult,
} from '@ocentra-parent/agent-protocol-domain/network-live-capture-status';
import {
  parseAgentNetworkLinuxNftablesLabStatusEvent,
  type AgentNetworkLinuxNftablesLabStatusParseResult,
} from '@ocentra-parent/agent-protocol-domain/network-linux-nftables-lab-status';
import {
  parseAgentNetworkWindowsFirewallLabStatusEvent,
  type AgentNetworkWindowsFirewallLabStatusParseResult,
} from '@ocentra-parent/agent-protocol-domain/network-windows-firewall-lab-status';
import {
  parseAgentNetworkWindowsWfpGateStatusEvent,
  type AgentNetworkWindowsWfpGateStatusParseResult,
} from '@ocentra-parent/agent-protocol-domain/network-windows-wfp-gate-status';
import {
  parseAgentNetworkRemoteDeliveryStatusEvent,
  type AgentNetworkRemoteDeliveryStatusParseResult,
} from '@ocentra-parent/agent-protocol-domain/network-remote-delivery-status';
import {
  parseAgentNetworkRuntimeEvent,
  type AgentNetworkRuntimeEventResult,
} from '@ocentra-parent/agent-protocol-domain/network-runtime-events';
import { parseAgentAppGameNotificationReadinessEvent } from '@ocentra-parent/agent-protocol-domain/app-game-notification-readiness';
import {
  parseAgentActivityTrackingReadModelEvent,
  type AgentActivityTrackingReadModelResult,
} from '@ocentra-parent/agent-protocol-domain/tracking-read-model';
import {
  parseAgentAppGamePolicyReadinessEvent,
  type AgentAppGamePolicyReadinessResult,
} from '@ocentra-parent/agent-protocol-domain/app-game-policy-readiness';
import {
  createAppGameNotificationParentSurfaceReadModelFromReadiness,
  parseActivityMemoryGraphReadModel,
  PortalBrowserInventoryFields,
  type PortalActivityMemoryGraphReadModel,
} from '@ocentra-parent/portal-domain/contracts';
import { parseBrowserInterventionReadModel } from './browser-intervention-read-model';
import { parseNetworkFlowReadModel } from './network-flow-read-model';
import { parsePolicyPreviewReadModel, type PortalPolicyPreviewReadModel } from './policy-preview-read-model';

type ActivitySurfaceReadModel =
  | ActivityScreenReadModel
  | ActivityAppUseReadModel
  | ActivityBrowserReadModel
  | ActivityGamesReadModel
  | ActivityNetworkReadModel;

type NetworkFlowReadModelState = {
  readonly event: AgentEventEnvelope | null;
  readonly readModel: ActivityNetworkFlowReadModel | null;
};

export interface PortalNetworkRuntimeEventChainStream {
  readonly streamedEventCount: number | null;
  readonly events: readonly AgentNetworkRuntimeEventResult[];
  readonly invalidEventCount: number;
}

export interface PortalLiveActivityState {
  readonly activityServiceUiSpine: ActivityServiceUiSpine;
  readonly ingestEvent: AgentEventEnvelope | null;
  readonly ingestStatus: ActivityIngestStatus | null;
  readonly recentSummaryEvent: AgentEventEnvelope | null;
  readonly recentSummary: ActivityRecentSummary | null;
  readonly browserEvidenceEvent: AgentEventEnvelope | null;
  readonly browserEvidenceReadModel: BrowserEvidenceReadModel | null;
  readonly browserInventoryEvent: AgentEventEnvelope | null;
  readonly browserInventoryReadModel: BrowserInventoryReadModel | null;
  readonly browserManagedEvent: AgentEventEnvelope | null;
  readonly browserManagedStatus: BrowserManagedSessionStatus | null;
  readonly localAiRuntimeStatusEvent: AgentEventEnvelope | null;
  readonly lanAiJobEvent: AgentEventEnvelope | null;
  readonly parentAssistantBoundaryEvent: AgentEventEnvelope | null;
  readonly activityMemoryGraphEvent: AgentEventEnvelope | null;
  readonly activityMemoryGraphReadModel: PortalActivityMemoryGraphReadModel | null;
  readonly activityReportEvent: AgentEventEnvelope | null;
  readonly activityReport: ActivitySurfaceAdapterResult<ActivityReportDocument> | null;
  readonly activityReportHistoryEvent: AgentEventEnvelope | null;
  readonly activityReportHistory: ActivitySurfaceAdapterResult<ActivityHistoricalReportList> | null;
  readonly activityScreenReadModelEvent: AgentEventEnvelope | null;
  readonly activityScreenReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityAppUseReadModelEvent: AgentEventEnvelope | null;
  readonly activityAppUseReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityBrowserReadModelEvent: AgentEventEnvelope | null;
  readonly activityBrowserReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityGamesReadModelEvent: AgentEventEnvelope | null;
  readonly activityGamesReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly appGameNotificationReadinessEvent: AgentEventEnvelope | null;
  readonly appGameNotificationParentSurfaceIntentReadModel: unknown | null;
  readonly activityNetworkReadModelEvent: AgentEventEnvelope | null;
  readonly activityNetworkReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly browserInterventionEvent: AgentEventEnvelope | null;
  readonly browserInterventionReadModel: BrowserInterventionReadModel | null;
  readonly networkFlowEvent: AgentEventEnvelope | null;
  readonly networkFlowReadModel: ActivityNetworkFlowReadModel | null;
  readonly networkRuntimeEventChainEvent: AgentEventEnvelope | null;
  readonly networkRuntimeEventChainStream: PortalNetworkRuntimeEventChainStream | null;
  readonly networkRemoteDeliveryStatusEvent: AgentEventEnvelope | null;
  readonly networkRemoteDeliveryStatusResult: AgentNetworkRemoteDeliveryStatusParseResult | null;
  readonly networkLiveCaptureStatusEvent: AgentEventEnvelope | null;
  readonly networkLiveCaptureStatusResult: AgentNetworkLiveCaptureStatusParseResult | null;
  readonly networkLinuxNftablesLabStatusEvent: AgentEventEnvelope | null;
  readonly networkLinuxNftablesLabStatusResult: AgentNetworkLinuxNftablesLabStatusParseResult | null;
  readonly networkWindowsFirewallLabStatusEvent: AgentEventEnvelope | null;
  readonly networkWindowsFirewallLabStatusResult: AgentNetworkWindowsFirewallLabStatusParseResult | null;
  readonly networkWindowsWfpGateStatusEvent: AgentEventEnvelope | null;
  readonly networkWindowsWfpGateStatusResult: AgentNetworkWindowsWfpGateStatusParseResult | null;
  readonly activityTrackingReadModelEvent: AgentEventEnvelope | null;
  readonly activityTrackingReadModel: AgentActivityTrackingReadModelResult | null;
  readonly lanPairingStatusEvent: AgentEventEnvelope | null;
  readonly lanPairingBrowserDiscoveryEvent: AgentEventEnvelope | null;
  readonly lanAddDeviceReadModel: AgentLanBrowserAddDeviceReadModel | null;
  readonly policyPreviewEvent: AgentEventEnvelope | null;
  readonly policyPreviewReadModel: PortalPolicyPreviewReadModel | null;
  readonly appGamePolicyReadinessEvent: AgentEventEnvelope | null;
  readonly appGamePolicyReadinessReadModel: AgentAppGamePolicyReadinessResult | null;
}

export function resolveLiveActivityState(events: readonly AgentEventEnvelope[]): PortalLiveActivityState {
  return {
    activityServiceUiSpine: parseActivityServiceUiSpineEvents(events),
    ...resolveActivityQueryState(events),
    ...resolveBrowserState(events),
    ...resolveActivityReportState(events),
    ...resolveActivityReadModelState(events),
    ...resolveAppGameNotificationState(events),
    ...resolveNetworkState(events),
    ...resolveActivityTrackingReadModel(events),
    ...resolveLanPairingState(events),
    ...resolvePolicyState(events),
  };
}

function resolveActivityQueryState(events: readonly AgentEventEnvelope[]) {
  const ingestEvent = latestEvent(events, AgentEvent.ActivityIngestStatusReported);
  const recentSummaryEvent = latestEvent(events, AgentEvent.ActivityRecentSummaryReported);

  return {
    ingestEvent,
    ingestStatus: ingestEvent === null ? null : parseIngestStatus(ingestEvent.payload),
    recentSummaryEvent,
    recentSummary: recentSummaryEvent === null ? null : parseRecentSummary(recentSummaryEvent.payload),
  };
}

function resolveBrowserState(events: readonly AgentEventEnvelope[]) {
  const browserEvidenceEvent = latestEvent(events, AgentEvent.BrowserEvidenceRecentReported);
  const browserInventoryEvent = latestEvent(events, AgentEvent.BrowserInventoryReadModelReported);
  const browserManagedEvent = latestEvent(events, AgentEvent.BrowserManagedStatusReported);

  return {
    browserEvidenceEvent,
    browserEvidenceReadModel:
      browserEvidenceEvent === null ? null : parseBrowserEvidenceReadModel(browserEvidenceEvent.payload),
    browserInventoryEvent,
    browserInventoryReadModel:
      browserInventoryEvent === null ? null : parseBrowserInventoryReadModel(browserInventoryEvent.payload),
    browserManagedEvent,
    browserManagedStatus: browserManagedEvent === null ? null : parseBrowserManagedStatus(browserManagedEvent.payload),
    ...resolveLocalAiActivityEvents(events),
  };
}

function resolveActivityReportState(events: readonly AgentEventEnvelope[]) {
  const activityMemoryGraphEvent = latestEvent(events, AgentEvent.ActivityMemoryGraphReported);
  const activityReportEvent = latestActivityReportEvent(events);
  const activityReportHistoryEvent = latestEvent(events, AgentEvent.ActivityReportHistoryReported);

  return {
    activityMemoryGraphEvent,
    activityMemoryGraphReadModel:
      activityMemoryGraphEvent === null ? null : parseActivityMemoryGraphReadModel(activityMemoryGraphEvent.payload),
    activityReportEvent,
    activityReport: parseNullableActivityReportEvent(activityReportEvent),
    activityReportHistoryEvent,
    activityReportHistory: parseNullableActivityReportHistoryEvent(activityReportHistoryEvent),
  };
}

function resolveActivityReadModelState(events: readonly AgentEventEnvelope[]) {
  const activityScreenReadModelEvent = latestEvent(events, AgentEvent.ActivityScreenReadModelReported);
  const activityAppUseReadModelEvent = latestEvent(events, AgentEvent.ActivityAppUseReadModelReported);
  const activityBrowserReadModelEvent = latestEvent(events, AgentEvent.ActivityBrowserReadModelReported);
  const activityGamesReadModelEvent = latestEvent(events, AgentEvent.ActivityGamesReadModelReported);
  const activityNetworkReadModelEvent = latestEvent(events, AgentEvent.ActivityNetworkReadModelReported);

  return parseActivityReadModelEvents(
    activityScreenReadModelEvent,
    activityAppUseReadModelEvent,
    activityBrowserReadModelEvent,
    activityGamesReadModelEvent,
    activityNetworkReadModelEvent
  );
}

function resolveAppGameNotificationState(events: readonly AgentEventEnvelope[]) {
  const appGameNotificationReadinessEvent = latestEvent(
    events,
    AgentEvent.ActivityAppGameNotificationReadinessReadModelReported
  );

  return {
    appGameNotificationReadinessEvent,
    appGameNotificationParentSurfaceIntentReadModel: parseNullableAppGameNotificationParentSurfaceReadModel(
      appGameNotificationReadinessEvent
    ),
  };
}

function resolveNetworkState(events: readonly AgentEventEnvelope[]) {
  const browserInterventionEvent = latestEvent(events, AgentEvent.BrowserInterventionReadModelReported);
  const networkFlowState = resolveNetworkFlowReadModelState(events);
  const networkRuntimeEventChainEvent = latestEvent(events, AgentEvent.NetworkRuntimeEventChainStreamReported);
  const networkRemoteDeliveryStatusEvent = latestEvent(events, AgentEvent.NetworkRemoteDeliveryStatusReported);
  const networkLiveCaptureStatusEvent = latestEvent(events, AgentEvent.NetworkLiveCaptureStatusReported);
  const networkLinuxNftablesLabStatusEvent = latestEvent(events, AgentEvent.NetworkLinuxNftablesLabStatusReported);
  const networkWindowsFirewallLabStatusEvent = latestEvent(events, AgentEvent.NetworkWindowsFirewallLabStatusReported);
  const networkWindowsWfpGateStatusEvent = latestEvent(events, AgentEvent.NetworkWindowsWfpGateStatusReported);

  return {
    browserInterventionEvent,
    browserInterventionReadModel:
      browserInterventionEvent === null ? null : parseBrowserInterventionReadModel(browserInterventionEvent.payload),
    networkFlowEvent: networkFlowState.event,
    networkFlowReadModel: networkFlowState.readModel,
    networkRuntimeEventChainEvent,
    networkRuntimeEventChainStream: parseNullableNetworkRuntimeEventChainStream(networkRuntimeEventChainEvent),
    networkRemoteDeliveryStatusEvent,
    networkRemoteDeliveryStatusResult:
      networkRemoteDeliveryStatusEvent === null
        ? null
        : parseAgentNetworkRemoteDeliveryStatusEvent(networkRemoteDeliveryStatusEvent),
    networkLiveCaptureStatusEvent,
    networkLiveCaptureStatusResult:
      networkLiveCaptureStatusEvent === null
        ? null
        : parseAgentNetworkLiveCaptureStatusEvent(networkLiveCaptureStatusEvent),
    networkLinuxNftablesLabStatusEvent,
    networkLinuxNftablesLabStatusResult:
      networkLinuxNftablesLabStatusEvent === null
        ? null
        : parseAgentNetworkLinuxNftablesLabStatusEvent(networkLinuxNftablesLabStatusEvent),
    networkWindowsFirewallLabStatusEvent,
    networkWindowsFirewallLabStatusResult:
      networkWindowsFirewallLabStatusEvent === null
        ? null
        : parseAgentNetworkWindowsFirewallLabStatusEvent(networkWindowsFirewallLabStatusEvent),
    networkWindowsWfpGateStatusEvent,
    networkWindowsWfpGateStatusResult:
      networkWindowsWfpGateStatusEvent === null
        ? null
        : parseAgentNetworkWindowsWfpGateStatusEvent(networkWindowsWfpGateStatusEvent),
  };
}

function resolveNetworkFlowReadModelState(events: readonly AgentEventEnvelope[]): NetworkFlowReadModelState {
  const latestNetworkFlow = latestParsedNetworkFlowEvent(events, false);
  const latestDurableNetworkFlow = latestParsedNetworkFlowEvent(events, true);
  return latestDurableNetworkFlow ?? latestNetworkFlow ?? { event: null, readModel: null };
}

function latestParsedNetworkFlowEvent(
  events: readonly AgentEventEnvelope[],
  requireDurableEvidence: boolean
): NetworkFlowReadModelState | null {
  let latest: NetworkFlowReadModelState | null = null;
  let latestTime = Number.NEGATIVE_INFINITY;
  let latestIndex = Number.POSITIVE_INFINITY;
  for (let index = 0; index < events.length; index += 1) {
    const event = events[index];
    if (event === undefined || event.event !== AgentEvent.NetworkFlowReadModelReported) {
      continue;
    }
    const readModel = parseNetworkFlowReadModel(event.payload);
    if (readModel === null || (requireDurableEvidence && !isDurableNetworkFlowReadModel(readModel))) {
      continue;
    }
    const sentAt = Date.parse(event.sentAt);
    const eventTime = Number.isFinite(sentAt) ? sentAt : events.length - index;
    if (eventTime > latestTime || (eventTime === latestTime && index < latestIndex)) {
      latest = { event, readModel };
      latestTime = eventTime;
      latestIndex = index;
    }
  }
  return latest;
}

function isDurableNetworkFlowReadModel(readModel: ActivityNetworkFlowReadModel): boolean {
  return readModel.rows.length > 0 || readModel.tombstoneRows > 0 || readModel.deletedEvidenceReferenceIds.length > 0;
}

function resolveLanPairingState(events: readonly AgentEventEnvelope[]) {
  const lanPairingStatusEvent = latestEventOf(events, [
    AgentEvent.LanPairingStatusReported,
    AgentEvent.LanPairingBrowserDiscoveryReported,
    AgentEvent.LanPairingAddDeviceReported,
  ]);
  const lanPairingBrowserDiscoveryEvent = latestEvent(events, AgentEvent.LanPairingBrowserDiscoveryReported);

  return {
    lanPairingStatusEvent,
    lanPairingBrowserDiscoveryEvent,
    lanAddDeviceReadModel:
      lanPairingStatusEvent === null ? null : parseLanAddDeviceReadModel(lanPairingStatusEvent.payload),
  };
}

function resolvePolicyState(events: readonly AgentEventEnvelope[]) {
  const policyPreviewEvent = latestEvent(events, AgentEvent.PolicyPreviewReadModelReported);
  const appGamePolicyReadinessEvent = latestEvent(events, AgentEvent.ActivityAppGamePolicyReadinessReadModelReported);

  return {
    policyPreviewEvent,
    policyPreviewReadModel:
      policyPreviewEvent === null ? null : parsePolicyPreviewReadModel(policyPreviewEvent.payload),
    appGamePolicyReadinessEvent,
    appGamePolicyReadinessReadModel:
      appGamePolicyReadinessEvent === null ? null : parseAgentAppGamePolicyReadinessEvent(appGamePolicyReadinessEvent),
  };
}

function resolveLocalAiActivityEvents(events: readonly AgentEventEnvelope[]) {
  return {
    localAiRuntimeStatusEvent: latestEvent(events, AgentEvent.LocalAiRuntimeStatusReported),
    lanAiJobEvent: latestEvent(events, AgentEvent.LanAiJobReported),
    parentAssistantBoundaryEvent: latestEventOf(events, [
      AgentEvent.ParentAssistantAnswerReported,
      AgentEvent.ParentAssistantProviderDegraded,
      AgentEvent.ParentAssistantErrorReported,
    ]),
  };
}

function parseActivityReadModelEvents(
  activityScreenReadModelEvent: AgentEventEnvelope | null,
  activityAppUseReadModelEvent: AgentEventEnvelope | null,
  activityBrowserReadModelEvent: AgentEventEnvelope | null,
  activityGamesReadModelEvent: AgentEventEnvelope | null,
  activityNetworkReadModelEvent: AgentEventEnvelope | null
) {
  return {
    activityScreenReadModelEvent,
    activityScreenReadModel: parseNullableActivityReadModelEvent(
      ActivitySurfaceReadModelKindName.Screen,
      activityScreenReadModelEvent
    ),
    activityAppUseReadModelEvent,
    activityAppUseReadModel: parseNullableActivityReadModelEvent(
      ActivitySurfaceReadModelKindName.AppUse,
      activityAppUseReadModelEvent
    ),
    activityBrowserReadModelEvent,
    activityBrowserReadModel: parseNullableActivityReadModelEvent(
      ActivitySurfaceReadModelKindName.Browser,
      activityBrowserReadModelEvent
    ),
    activityGamesReadModelEvent,
    activityGamesReadModel: parseNullableActivityReadModelEvent(
      ActivitySurfaceReadModelKindName.Games,
      activityGamesReadModelEvent
    ),
    activityNetworkReadModelEvent,
    activityNetworkReadModel: parseNullableActivityReadModelEvent(
      ActivitySurfaceReadModelKindName.Network,
      activityNetworkReadModelEvent
    ),
  };
}

function resolveActivityTrackingReadModel(events: readonly AgentEventEnvelope[]) {
  const event = latestEvent(events, AgentEvent.ActivityTrackingReadModelReported);
  return {
    activityTrackingReadModelEvent: event,
    activityTrackingReadModel: event === null ? null : parseAgentActivityTrackingReadModelEvent(event),
  };
}

function latestEvent(events: readonly AgentEventEnvelope[], eventName: AgentEventName): AgentEventEnvelope | null {
  return latestEventOf(events, [eventName]);
}

function latestEventOf(
  events: readonly AgentEventEnvelope[],
  eventNames: readonly AgentEventName[]
): AgentEventEnvelope | null {
  let latest: AgentEventEnvelope | null = null;
  let latestTime = Number.NEGATIVE_INFINITY;
  let latestIndex = Number.POSITIVE_INFINITY;
  for (let index = 0; index < events.length; index += 1) {
    const event = events[index];
    if (event !== undefined && eventNames.includes(event.event)) {
      const sentAt = Date.parse(event.sentAt);
      const eventTime = Number.isFinite(sentAt) ? sentAt : events.length - index;
      if (eventTime > latestTime || (eventTime === latestTime && index < latestIndex)) {
        latest = event;
        latestTime = eventTime;
        latestIndex = index;
      }
    }
  }
  return latest;
}

function latestActivityReportEvent(events: readonly AgentEventEnvelope[]): AgentEventEnvelope | null {
  return latestEventOf(events, [AgentEvent.ActivityReportSaved, AgentEvent.ActivityReportGenerated]);
}

function parseNullableActivityReportEvent(
  event: AgentEventEnvelope | null
): ActivitySurfaceAdapterResult<ActivityReportDocument> | null {
  if (event === null) {
    return null;
  }
  return parseActivityReportDocumentEvent(event);
}

function parseNullableActivityReportHistoryEvent(
  event: AgentEventEnvelope | null
): ActivitySurfaceAdapterResult<ActivityHistoricalReportList> | null {
  if (event === null) {
    return null;
  }
  return parseActivityReportHistoryEvent(event);
}

function parseNullableActivityReadModelEvent(
  kind: ActivitySurfaceReadModelKind,
  event: AgentEventEnvelope | null
): ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null {
  if (event === null) {
    return null;
  }
  return parseActivityReadModelEvent(kind, event);
}

function parseNullableAppGameNotificationParentSurfaceReadModel(event: AgentEventEnvelope | null): unknown | null {
  if (event === null) {
    return null;
  }

  const parsed = parseAgentAppGameNotificationReadinessEvent(event);
  return parsed.ok ? createAppGameNotificationParentSurfaceReadModelFromReadiness(parsed.value) : null;
}

function parseNullableNetworkRuntimeEventChainStream(
  event: AgentEventEnvelope | null
): PortalNetworkRuntimeEventChainStream | null {
  if (event === null) {
    return null;
  }

  const parsedEvents = networkRuntimeEventInputs(event).map((input) => parseAgentNetworkRuntimeEvent(input));
  return {
    streamedEventCount: numericPayloadValue(event.payload[AgentProtocolDefaults.Field.NetworkRuntimeStreamedEvents]),
    events: parsedEvents,
    invalidEventCount: parsedEvents.filter((result) => !result.ok).length,
  };
}

function networkRuntimeEventInputs(event: AgentEventEnvelope): readonly unknown[] {
  const rawStream = event.payload[AgentProtocolDefaults.Field.NetworkRuntimeEventChainStream];
  if (typeof rawStream !== AgentProtocolDefaults.Primitive.String) {
    return [];
  }

  try {
    const parsed = JSON.parse(String(rawStream)) as unknown;
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function numericPayloadValue(value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined): number | null {
  return typeof value === AgentProtocolDefaults.Primitive.Number ? Number(value) : null;
}

function parseLanAddDeviceReadModel(payload: AgentProtocolLogFields): AgentLanBrowserAddDeviceReadModel | null {
  const rawReadModel = payload[AgentProtocolDefaults.Field.LanAddDeviceReadModel];
  const readModel = parseJsonRecord(rawReadModel);
  const parsed = AgentLanBrowserAddDeviceReadModelSchema.safeParse(readModel);

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function parseJsonRecord(value: unknown): unknown {
  if (typeof value !== AgentProtocolDefaults.Primitive.String) {
    return value;
  }

  try {
    return JSON.parse(String(value));
  } catch {
    return null;
  }
}

function parseIngestStatus(payload: AgentProtocolLogFields): ActivityIngestStatus | null {
  const parsed = ActivityIngestStatusSchema.safeParse({
    schemaVersion: ActivityQuerySchemaVersion,
    databaseReady: payload[AgentProtocolDefaults.Field.DatabaseReady],
    eventsIngested: payload[AgentProtocolDefaults.Field.EventsIngested],
    eventsStored: payload[AgentProtocolDefaults.Field.EventsStored],
    duplicateEvents: payload[AgentProtocolDefaults.Field.DuplicateEvents],
    lastEventId: payload[AgentProtocolDefaults.Field.LastEventId],
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function parseRecentSummary(payload: AgentProtocolLogFields): ActivityRecentSummary | null {
  const parsed = ActivityRecentSummarySchema.safeParse({
    schemaVersion: ActivityQuerySchemaVersion,
    limit: payload[AgentProtocolDefaults.Field.Limit],
    returned: payload[AgentProtocolDefaults.Field.Returned],
    firstObservedAt: payload[AgentProtocolDefaults.Field.FirstObservedAt],
    lastObservedAt: payload[AgentProtocolDefaults.Field.LastObservedAt],
    lastEventId: payload[AgentProtocolDefaults.Field.LastEventId],
    mostRecentKind: payload[AgentProtocolDefaults.Field.MostRecentKind],
    mostRecentObserver: payload[AgentProtocolDefaults.Field.MostRecentObserver],
    mostRecentSubjectKind: payload[AgentProtocolDefaults.Field.MostRecentSubjectKind],
    mostRecentSubjectId: payload[AgentProtocolDefaults.Field.MostRecentSubjectId],
    mostRecentSubjectName: payload[AgentProtocolDefaults.Field.MostRecentSubjectName],
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function parseBrowserEvidenceReadModel(payload: AgentProtocolLogFields): BrowserEvidenceReadModel | null {
  const returned = payload[AgentProtocolDefaults.Field.Returned];
  const rows = returned === 0 ? [] : [browserTabEvidence(payload)];
  const parsed = BrowserEvidenceReadModelSchema.safeParse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    generatedAt: payload[AgentProtocolDefaults.Field.GeneratedAt],
    limit: payload[AgentProtocolDefaults.Field.Limit],
    returned: payload[AgentProtocolDefaults.Field.Returned],
    latestEventId: payload[AgentProtocolDefaults.Field.LatestEventId],
    latestObservedAt: payload[AgentProtocolDefaults.Field.LatestObservedAt],
    capabilityStatus: nullIfMissing(payload[AgentProtocolDefaults.Field.CapabilityStatus]),
    custodyLabel: payload[AgentProtocolDefaults.Field.CustodyLabel] ?? BrowserCustodyLabel.Unavailable,
    queryVisibility: payload[AgentProtocolDefaults.Field.QueryVisibility] ?? BrowserQueryVisibilityLabel.Unavailable,
    rows,
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function parseBrowserInventoryReadModel(payload: AgentProtocolLogFields): BrowserInventoryReadModel | null {
  const returned = payload[AgentProtocolDefaults.Field.Returned];
  const rows = returned === 0 ? [] : [browserInventoryRow(payload)];
  const parsed = BrowserInventoryReadModelSchema.safeParse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    generatedAt: payload[AgentProtocolDefaults.Field.GeneratedAt],
    limit: payload[AgentProtocolDefaults.Field.Limit],
    returned: payload[AgentProtocolDefaults.Field.Returned],
    latestObservedAt: nullIfMissing(payload[AgentProtocolDefaults.Field.LatestObservedAt]),
    capabilityStatus: nullIfMissing(payload[AgentProtocolDefaults.Field.CapabilityStatus]),
    custodyLabel: payload[AgentProtocolDefaults.Field.CustodyLabel] ?? BrowserCustodyLabel.Unavailable,
    queryVisibility: payload[AgentProtocolDefaults.Field.QueryVisibility] ?? BrowserQueryVisibilityLabel.Unavailable,
    rows,
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function browserInventoryRow(payload: AgentProtocolLogFields) {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    inventoryRowId: payload[PortalBrowserInventoryFields.BrowserInventoryRowId],
    scannedAt:
      payload[PortalBrowserInventoryFields.ScannedAt] ??
      payload[AgentProtocolDefaults.Field.LatestObservedAt] ??
      payload[AgentProtocolDefaults.Field.GeneratedAt],
    deviceId:
      payload[AgentProtocolDefaults.Field.DeviceId] ?? AgentProtocolDefaults.Target.LocalhostWindowsAgent.deviceId,
    browserFamily: payload[AgentProtocolDefaults.Field.BrowserFamily],
    browserChannel: payload[AgentProtocolDefaults.Field.BrowserChannel],
    productName: payload[PortalBrowserInventoryFields.ProductName],
    browserVersion: nullIfMissing(payload[AgentProtocolDefaults.Field.BrowserVersion]),
    installState: payload[PortalBrowserInventoryFields.InstallState],
    runningState: payload[PortalBrowserInventoryFields.RunningState],
    managementTier: payload[PortalBrowserInventoryFields.ManagementTier],
    supportTier: payload[PortalBrowserInventoryFields.SupportTier],
    exactUrlCapability: payload[PortalBrowserInventoryFields.ExactUrlCapability],
    activeTabCapability: payload[PortalBrowserInventoryFields.ActiveTabCapability],
    managedProfileState: payload[PortalBrowserInventoryFields.ManagedProfileState],
    unmanagedFallbackCapability: payload[PortalBrowserInventoryFields.UnmanagedFallbackCapability],
    executablePathRef: nullIfMissing(payload[PortalBrowserInventoryFields.ExecutablePathRef]),
    publisherSignatureRef: nullIfMissing(payload[PortalBrowserInventoryFields.PublisherSignatureRef]),
    fileHashRef: nullIfMissing(payload[PortalBrowserInventoryFields.FileHashRef]),
    profileId: nullIfMissing(payload[AgentProtocolDefaults.Field.ProfileId]),
    processId: nullIfMissing(payload[AgentProtocolDefaults.Field.ProcessId]),
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    reasonCode: payload[AgentProtocolDefaults.Field.Reason],
    custodyLabel: payload[AgentProtocolDefaults.Field.CustodyLabel],
    queryVisibility: payload[AgentProtocolDefaults.Field.QueryVisibility],
  };
}

function browserTabEvidence(payload: AgentProtocolLogFields) {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    browserEvidenceId: payload[AgentProtocolDefaults.Field.BrowserEvidenceId],
    observedAt: payload[AgentProtocolDefaults.Field.LatestObservedAt],
    freshUntil: payload[AgentProtocolDefaults.Field.FreshUntil],
    sourceId: payload[AgentProtocolDefaults.Field.SourceId],
    adapterId: payload[AgentProtocolDefaults.Field.AdapterId],
    deviceId: AgentProtocolDefaults.Target.LocalhostWindowsAgent.deviceId,
    managedBrowserSessionId: payload[AgentProtocolDefaults.Field.ManagedBrowserSessionId],
    browserFamily: payload[AgentProtocolDefaults.Field.BrowserFamily],
    browserChannel: payload[AgentProtocolDefaults.Field.BrowserChannel],
    profileId: payload[AgentProtocolDefaults.Field.ProfileId],
    processId: payload[AgentProtocolDefaults.Field.ProcessId],
    windowId: nullIfMissing(payload[AgentProtocolDefaults.Field.WindowId]),
    tabId: nullIfMissing(payload[AgentProtocolDefaults.Field.TabId]),
    targetId: nullIfMissing(payload[AgentProtocolDefaults.Field.TargetId]),
    activeState: payload[AgentProtocolDefaults.Field.ActiveState],
    activeProofSource: BrowserActiveProofSource.TargetListOnly,
    url: payload[AgentProtocolDefaults.Field.Url],
    origin: payload[AgentProtocolDefaults.Field.Origin],
    domain: payload[AgentProtocolDefaults.Field.Domain],
    title: nullIfMissing(payload[AgentProtocolDefaults.Field.Title]),
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    degradedReason: nullIfMissing(payload[AgentProtocolDefaults.Field.Reason]),
    staleAt: payload[AgentProtocolDefaults.Field.StaleAt],
    custodyLabel: payload[AgentProtocolDefaults.Field.CustodyLabel],
    queryVisibility: payload[AgentProtocolDefaults.Field.QueryVisibility],
  };
}

function parseBrowserManagedStatus(payload: AgentProtocolLogFields): BrowserManagedSessionStatus | null {
  const parsed = BrowserManagedSessionStatusSchema.safeParse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    checkedAt: payload[AgentProtocolDefaults.Field.CheckedAt],
    managedBrowserSessionId: payload[AgentProtocolDefaults.Field.ManagedBrowserSessionId],
    browserFamily: payload[AgentProtocolDefaults.Field.BrowserFamily],
    browserChannel: payload[AgentProtocolDefaults.Field.BrowserChannel],
    browserVersion: payload[AgentProtocolDefaults.Field.BrowserVersion],
    profileId: payload[AgentProtocolDefaults.Field.ProfileId],
    profilePathRef: payload[AgentProtocolDefaults.Field.ProfilePathRef],
    profileRootRef: null,
    profileScopeId: null,
    profileLifecycleState: null,
    policyRevision: null,
    processId: payload[AgentProtocolDefaults.Field.ProcessId],
    bridgeKind: payload[AgentProtocolDefaults.Field.BridgeKind],
    bridgeEndpointRef: payload[AgentProtocolDefaults.Field.BridgeEndpointRef],
    unmanagedProcessName: nullIfMissing(payload[AgentProtocolDefaults.Field.UnmanagedProcessName]),
    unmanagedExecutablePathRef: nullIfMissing(payload[AgentProtocolDefaults.Field.UnmanagedExecutablePathRef]),
    unmanagedSignatureRef: nullIfMissing(payload[AgentProtocolDefaults.Field.UnmanagedSignatureRef]),
    unmanagedProcessHashRef: nullIfMissing(payload[AgentProtocolDefaults.Field.UnmanagedProcessHashRef]),
    unmanagedProcessKind: nullIfMissing(payload[AgentProtocolDefaults.Field.UnmanagedProcessKind]),
    unmanagedDetectionConfidence: nullIfMissing(payload[AgentProtocolDefaults.Field.UnmanagedDetectionConfidence]),
    unmanagedDetectionReason: nullIfMissing(payload[AgentProtocolDefaults.Field.UnmanagedDetectionReason]),
    managedState: payload[AgentProtocolDefaults.Field.ManagedState],
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    degradedReason: payload[AgentProtocolDefaults.Field.Reason],
    startedAt: payload[AgentProtocolDefaults.Field.StartedAt],
    custodyLabel: payload[AgentProtocolDefaults.Field.CustodyLabel],
    queryVisibility: payload[AgentProtocolDefaults.Field.QueryVisibility],
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function nullIfMissing(value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined) {
  return value === undefined ? null : value;
}
