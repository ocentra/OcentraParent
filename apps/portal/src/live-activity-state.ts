import {
  BrowserEvidenceRecentSummarySchema,
  BrowserEvidenceSchemaVersion,
  BrowserManagedSessionStatusSchema,
  type BrowserEvidenceRecentSummary,
  type BrowserManagedSessionStatus,
} from '@ocentra-parent/activity-domain/browser';
import {
  ActivityIngestStatusSchema,
  ActivityQuerySchemaVersion,
  ActivityRecentSummarySchema,
  type ActivityIngestStatus,
  type ActivityRecentSummary,
} from '@ocentra-parent/activity-domain/query';
import {
  ActivityNetworkFlowReadModelSchema,
  type ActivityNetworkFlowReadModel,
} from '@ocentra-parent/activity-domain/network-flow';
import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';

export interface PortalLiveActivityState {
  readonly ingestEvent: AgentEventEnvelope | null;
  readonly ingestStatus: ActivityIngestStatus | null;
  readonly recentSummaryEvent: AgentEventEnvelope | null;
  readonly recentSummary: ActivityRecentSummary | null;
  readonly browserEvidenceEvent: AgentEventEnvelope | null;
  readonly browserEvidenceSummary: BrowserEvidenceRecentSummary | null;
  readonly browserManagedEvent: AgentEventEnvelope | null;
  readonly browserManagedStatus: BrowserManagedSessionStatus | null;
  readonly networkFlowEvent: AgentEventEnvelope | null;
  readonly networkFlowReadModel: ActivityNetworkFlowReadModel | null;
}

export function resolveLiveActivityState(events: readonly AgentEventEnvelope[]): PortalLiveActivityState {
  const ingestEvent = latestEvent(events, AgentEvent.ActivityIngestStatusReported);
  const recentSummaryEvent = latestEvent(events, AgentEvent.ActivityRecentSummaryReported);
  const browserEvidenceEvent = latestEvent(events, AgentEvent.BrowserEvidenceRecentReported);
  const browserManagedEvent = latestEvent(events, AgentEvent.BrowserManagedStatusReported);
  const networkFlowEvent = latestEvent(events, AgentEvent.NetworkFlowReadModelReported);

  return {
    ingestEvent,
    ingestStatus: ingestEvent === null ? null : parseIngestStatus(ingestEvent.payload),
    recentSummaryEvent,
    recentSummary: recentSummaryEvent === null ? null : parseRecentSummary(recentSummaryEvent.payload),
    browserEvidenceEvent,
    browserEvidenceSummary:
      browserEvidenceEvent === null ? null : parseBrowserEvidenceSummary(browserEvidenceEvent.payload),
    browserManagedEvent,
    browserManagedStatus: browserManagedEvent === null ? null : parseBrowserManagedStatus(browserManagedEvent.payload),
    networkFlowEvent,
    networkFlowReadModel: networkFlowEvent === null ? null : parseNetworkFlowReadModel(networkFlowEvent.payload),
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

function parseBrowserEvidenceSummary(payload: AgentProtocolLogFields): BrowserEvidenceRecentSummary | null {
  const parsed = BrowserEvidenceRecentSummarySchema.safeParse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    returned: payload[AgentProtocolDefaults.Field.Returned],
    latestEventId: payload[AgentProtocolDefaults.Field.LatestEventId],
    latestObservedAt: payload[AgentProtocolDefaults.Field.LatestObservedAt],
    browserEvidenceId: payload[AgentProtocolDefaults.Field.BrowserEvidenceId],
    sourceId: payload[AgentProtocolDefaults.Field.SourceId],
    adapterId: payload[AgentProtocolDefaults.Field.AdapterId],
    managedBrowserSessionId: payload[AgentProtocolDefaults.Field.ManagedBrowserSessionId],
    browserFamily: payload[AgentProtocolDefaults.Field.BrowserFamily],
    activeState: payload[AgentProtocolDefaults.Field.ActiveState],
    url: payload[AgentProtocolDefaults.Field.Url],
    origin: payload[AgentProtocolDefaults.Field.Origin],
    domain: payload[AgentProtocolDefaults.Field.Domain],
    title: payload[AgentProtocolDefaults.Field.Title],
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    custodyLabel: payload[AgentProtocolDefaults.Field.CustodyLabel],
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
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

function parseNetworkFlowReadModel(payload: AgentProtocolLogFields): ActivityNetworkFlowReadModel | null {
  const returned = payload[AgentProtocolDefaults.Field.Returned];
  const row = returned === 0 ? [] : [networkFlowObservation(payload)];
  const parsed = ActivityNetworkFlowReadModelSchema.safeParse({
    schemaVersion: ActivityQuerySchemaVersion,
    generatedAt: payload[AgentProtocolDefaults.Field.GeneratedAt],
    custody: payload[AgentProtocolDefaults.Field.Custody],
    limit: payload[AgentProtocolDefaults.Field.Limit],
    returned,
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    rows: row,
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function networkFlowObservation(payload: AgentProtocolLogFields) {
  return {
    schemaVersion: ActivityQuerySchemaVersion,
    eventId: payload[AgentProtocolDefaults.Field.LatestEventId],
    observedAt: payload[AgentProtocolDefaults.Field.LatestObservedAt],
    observer: payload[AgentProtocolDefaults.Field.Observer],
    capabilityStatus: payload[AgentProtocolDefaults.Field.CapabilityStatus],
    adapterId: payload[AgentProtocolDefaults.Field.AdapterId],
    protocol: nullIfMissing(payload[AgentProtocolDefaults.Field.NetworkProtocol]),
    tcpState: nullIfMissing(payload[AgentProtocolDefaults.Field.TcpState]),
    localEndpoint: {
      ip: nullIfMissing(payload[AgentProtocolDefaults.Field.LocalIp]),
      port: nullIfMissing(payload[AgentProtocolDefaults.Field.LocalPort]),
    },
    destinationEndpoint: {
      ip: nullIfMissing(payload[AgentProtocolDefaults.Field.DestinationIp]),
      port: nullIfMissing(payload[AgentProtocolDefaults.Field.DestinationPort]),
    },
    destinationDomain: nullIfMissing(payload[AgentProtocolDefaults.Field.DestinationDomain]),
    domainAttributionStatus: payload[AgentProtocolDefaults.Field.DomainAttributionStatus],
    processAttributionStatus: payload[AgentProtocolDefaults.Field.ProcessAttributionStatus],
    processId: nullIfMissing(payload[AgentProtocolDefaults.Field.ProcessId]),
    processName: nullIfMissing(payload[AgentProtocolDefaults.Field.ProcessName]),
    counters: {
      connectionCount: payload[AgentProtocolDefaults.Field.ConnectionCount],
      bytesSent: nullIfMissing(payload[AgentProtocolDefaults.Field.BytesSent]),
      bytesReceived: nullIfMissing(payload[AgentProtocolDefaults.Field.BytesReceived]),
      firstSeenAt: nullIfMissing(payload[AgentProtocolDefaults.Field.FirstSeenAt]),
      lastSeenAt: nullIfMissing(payload[AgentProtocolDefaults.Field.LastSeenAt]),
    },
    evidence: [],
  };
}

function nullIfMissing(value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined) {
  return value === undefined ? null : value;
}
