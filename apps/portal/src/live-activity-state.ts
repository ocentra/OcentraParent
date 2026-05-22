import {
  BrowserCustodyLabel,
  BrowserEvidenceReadModelSchema,
  BrowserEvidenceSchemaVersion,
  BrowserManagedSessionStatusSchema,
  BrowserQueryVisibilityLabel,
  type BrowserEvidenceReadModel,
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
import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  parseActivityMemoryGraphReadModel,
  type PortalActivityMemoryGraphReadModel,
} from '@ocentra-parent/portal-domain/contracts';
import { parseNetworkFlowReadModel } from './network-flow-read-model';
import { parsePolicyPreviewReadModel, type PortalPolicyPreviewReadModel } from './policy-preview-read-model';

export interface PortalLiveActivityState {
  readonly ingestEvent: AgentEventEnvelope | null;
  readonly ingestStatus: ActivityIngestStatus | null;
  readonly recentSummaryEvent: AgentEventEnvelope | null;
  readonly recentSummary: ActivityRecentSummary | null;
  readonly browserEvidenceEvent: AgentEventEnvelope | null;
  readonly browserEvidenceReadModel: BrowserEvidenceReadModel | null;
  readonly browserManagedEvent: AgentEventEnvelope | null;
  readonly browserManagedStatus: BrowserManagedSessionStatus | null;
  readonly activityMemoryGraphEvent: AgentEventEnvelope | null;
  readonly activityMemoryGraphReadModel: PortalActivityMemoryGraphReadModel | null;
  readonly networkFlowEvent: AgentEventEnvelope | null;
  readonly networkFlowReadModel: ActivityNetworkFlowReadModel | null;
  readonly policyPreviewEvent: AgentEventEnvelope | null;
  readonly policyPreviewReadModel: PortalPolicyPreviewReadModel | null;
}

export function resolveLiveActivityState(events: readonly AgentEventEnvelope[]): PortalLiveActivityState {
  const ingestEvent = latestEvent(events, AgentEvent.ActivityIngestStatusReported);
  const recentSummaryEvent = latestEvent(events, AgentEvent.ActivityRecentSummaryReported);
  const browserEvidenceEvent = latestEvent(events, AgentEvent.BrowserEvidenceRecentReported);
  const browserManagedEvent = latestEvent(events, AgentEvent.BrowserManagedStatusReported);
  const activityMemoryGraphEvent = latestEvent(events, AgentEvent.ActivityMemoryGraphReported);
  const networkFlowEvent = latestEvent(events, AgentEvent.NetworkFlowReadModelReported);
  const policyPreviewEvent = latestEvent(events, AgentEvent.PolicyPreviewReadModelReported);

  return {
    ingestEvent,
    ingestStatus: ingestEvent === null ? null : parseIngestStatus(ingestEvent.payload),
    recentSummaryEvent,
    recentSummary: recentSummaryEvent === null ? null : parseRecentSummary(recentSummaryEvent.payload),
    browserEvidenceEvent,
    browserEvidenceReadModel:
      browserEvidenceEvent === null ? null : parseBrowserEvidenceReadModel(browserEvidenceEvent.payload),
    browserManagedEvent,
    browserManagedStatus: browserManagedEvent === null ? null : parseBrowserManagedStatus(browserManagedEvent.payload),
    activityMemoryGraphEvent,
    activityMemoryGraphReadModel:
      activityMemoryGraphEvent === null ? null : parseActivityMemoryGraphReadModel(activityMemoryGraphEvent.payload),
    networkFlowEvent,
    networkFlowReadModel: networkFlowEvent === null ? null : parseNetworkFlowReadModel(networkFlowEvent.payload),
    policyPreviewEvent,
    policyPreviewReadModel:
      policyPreviewEvent === null ? null : parsePolicyPreviewReadModel(policyPreviewEvent.payload),
  };
}

function latestEvent(events: readonly AgentEventEnvelope[], eventName: AgentEventName): AgentEventEnvelope | null {
  return events.find((event) => event.event === eventName) ?? null;
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
    processId: payload[AgentProtocolDefaults.Field.ProcessId],
    bridgeKind: payload[AgentProtocolDefaults.Field.BridgeKind],
    bridgeEndpointRef: payload[AgentProtocolDefaults.Field.BridgeEndpointRef],
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
