import {
  BrowserEvidenceRecentSummarySchema,
  BrowserEvidenceSchemaVersion,
  type BrowserEvidenceRecentSummary,
} from '@ocentra-parent/activity-domain/browser';
import {
  ActivityNetworkFlowDigestSchema,
  type ActivityNetworkFlowDigest,
} from '@ocentra-parent/activity-domain/network-flow';
import {
  ActivityIngestStatusSchema,
  ActivityQuerySchemaVersion,
  ActivityRecentSummarySchema,
  type ActivityIngestStatus,
  type ActivityRecentSummary,
} from '@ocentra-parent/activity-domain/query';
import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { type PortalDetailValue } from '@ocentra-parent/portal-domain/contracts';

export interface PortalLiveActivityState {
  readonly ingestEvent: AgentEventEnvelope | null;
  readonly ingestStatus: ActivityIngestStatus | null;
  readonly recentSummaryEvent: AgentEventEnvelope | null;
  readonly recentSummary: ActivityRecentSummary | null;
  readonly browserEvidenceEvent: AgentEventEnvelope | null;
  readonly browserEvidenceSummary: BrowserEvidenceRecentSummary | null;
  readonly networkFlowEvent: AgentEventEnvelope | null;
  readonly networkFlowDigest: ActivityNetworkFlowDigest | null;
}

export function resolveLiveActivityState(events: readonly AgentEventEnvelope[]): PortalLiveActivityState {
  const ingestEvent = latestEvent(events, AgentEvent.ActivityIngestStatusReported);
  const recentSummaryEvent = latestEvent(events, AgentEvent.ActivityRecentSummaryReported);
  const browserEvidenceEvent = latestEvent(events, AgentEvent.BrowserEvidenceRecentReported);
  const networkFlowEvent = latestEvent(events, AgentEvent.NetworkFlowReported);

  return {
    ingestEvent,
    ingestStatus: ingestEvent === null ? null : parseIngestStatus(ingestEvent.payload),
    recentSummaryEvent,
    recentSummary: recentSummaryEvent === null ? null : parseRecentSummary(recentSummaryEvent.payload),
    browserEvidenceEvent,
    browserEvidenceSummary:
      browserEvidenceEvent === null ? null : parseBrowserEvidenceSummary(browserEvidenceEvent.payload),
    networkFlowEvent,
    networkFlowDigest: networkFlowEvent === null ? null : parseNetworkFlowDigest(networkFlowEvent.payload),
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

function parseNetworkFlowDigest(payload: AgentProtocolLogFields): ActivityNetworkFlowDigest | null {
  const digestValue = payload[AgentProtocolDefaults.Field.ActivityDigest];
  if (!isStringValue(digestValue)) {
    return null;
  }

  let parsedDigest: unknown;
  try {
    parsedDigest = JSON.parse(digestValue);
  } catch {
    return null;
  }

  const parsed = ActivityNetworkFlowDigestSchema.safeParse(parsedDigest);
  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function isStringValue(value: unknown): value is PortalDetailValue {
  return typeof value === AgentProtocolDefaults.Primitive.String;
}
