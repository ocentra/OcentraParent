import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PARENT_PORTAL_ROUTE, type ParentPortalRow, type ParentPortalTone } from './parent-portal-data';
import { PARENT_PORTAL_SERVICE_STATE } from './parent-portal-service-state-constants';
import type { ParentPortalServiceConnectionState, ParentPortalServiceStateInput } from './parent-portal-service-state';

export function parentPortalServiceRows(input: ParentPortalServiceStateInput): ParentPortalRow[] {
  return [
    localAgentRow(input.connectionState),
    lanDiscoveryRow(input),
    devicePairingRow(input),
    browserActivityRow(input),
    activityReportsRow(input),
    networkTrackingRow(input),
  ];
}

function localAgentRow(connectionState: ParentPortalServiceConnectionState): ParentPortalRow {
  if (connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected) {
    return row(
      PARENT_PORTAL_SERVICE_STATE.Label.LocalAgent,
      1,
      100,
      1,
      0,
      PARENT_PORTAL_ROUTE.StatusText.Local,
      PARENT_PORTAL_SERVICE_STATE.Area.Runtime,
      'cyan'
    );
  }
  if (connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connecting) {
    return row(
      PARENT_PORTAL_SERVICE_STATE.Label.LocalAgent,
      1,
      45,
      0,
      1,
      PARENT_PORTAL_ROUTE.StatusText.Connecting,
      PARENT_PORTAL_SERVICE_STATE.Area.Runtime,
      'gold'
    );
  }
  if (connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Error) {
    return row(
      PARENT_PORTAL_SERVICE_STATE.Label.LocalAgent,
      1,
      0,
      0,
      1,
      PARENT_PORTAL_ROUTE.StatusText.CheckService,
      PARENT_PORTAL_SERVICE_STATE.Area.Runtime,
      'red'
    );
  }
  return row(
    PARENT_PORTAL_SERVICE_STATE.Label.LocalAgent,
    1,
    0,
    0,
    1,
    PARENT_PORTAL_ROUTE.StatusText.Offline,
    PARENT_PORTAL_SERVICE_STATE.Area.Runtime,
    'muted'
  );
}

function lanDiscoveryRow(input: ParentPortalServiceStateInput): ParentPortalRow {
  const event = latestEvent(input.events, AgentEvent.LanPairingStatusReported);
  const payload = event?.payload ?? null;
  const trend =
    textValue(payload, AgentProtocolDefaults.Field.LanAddDeviceState) ??
    textValue(payload, AgentProtocolDefaults.Field.LanLocalServiceDiscoveryState) ??
    textValue(payload, AgentProtocolDefaults.Field.LanDiscoveryState) ??
    manualRequiredTrend(input.connectionState);
  const readyCount = lanVisibleDeviceCount(payload);
  const gapCount =
    event === null || readyCount === 0 || trend === PARENT_PORTAL_SERVICE_STATE.Trend.ManualRequired ? 1 : 0;
  return row(
    PARENT_PORTAL_SERVICE_STATE.Label.LanDiscovery,
    2,
    eventScore(event),
    readyCount,
    gapCount,
    trend,
    PARENT_PORTAL_SERVICE_STATE.Area.Lan,
    'purple'
  );
}

function devicePairingRow(input: ParentPortalServiceStateInput): ParentPortalRow {
  const event = latestEvent(input.events, AgentEvent.LanPairingStatusReported);
  const payload = event?.payload ?? null;
  const trustedCount = numberValue(payload, AgentProtocolDefaults.Field.LanTrustedDeviceCount) ?? 0;
  const selected = presentText(payload, AgentProtocolDefaults.Field.LanSelectedChildDeviceId);
  const selectedReady = payload?.[AgentProtocolDefaults.Field.LanSelectedDeviceReady] === true;
  const readyCount = trustedCount + (selected === null ? 0 : 1);
  const trend =
    (selectedReady ? 'ready' : null) ??
    textValue(payload, AgentProtocolDefaults.Field.LanSelectedDeviceReachability) ??
    textValue(payload, AgentProtocolDefaults.Field.LanPairingState) ??
    textValue(payload, AgentProtocolDefaults.Field.LanAddDeviceState) ??
    manualRequiredTrend(input.connectionState);
  return row(
    PARENT_PORTAL_SERVICE_STATE.Label.DevicePairing,
    3,
    eventScore(event),
    readyCount,
    readyCount > 0 ? 0 : 1,
    trend,
    PARENT_PORTAL_SERVICE_STATE.Area.CurrentDevice,
    'cyan'
  );
}

function lanVisibleDeviceCount(payload: AgentProtocolLogFields | null): number {
  const trustedCount = numberValue(payload, AgentProtocolDefaults.Field.LanTrustedDeviceCount) ?? 0;
  const pendingCount = numberValue(payload, AgentProtocolDefaults.Field.LanPendingPairingCount) ?? 0;
  const selectedCount = presentText(payload, AgentProtocolDefaults.Field.LanSelectedChildDeviceId) === null ? 0 : 1;
  return Math.max(trustedCount, selectedCount) + pendingCount;
}

function browserActivityRow(input: ParentPortalServiceStateInput): ParentPortalRow {
  const managed = latestEvent(input.events, AgentEvent.BrowserManagedStatusReported);
  const evidence = latestEvent(input.events, AgentEvent.BrowserEvidenceRecentReported);
  const payload = managed?.payload ?? evidence?.payload ?? null;
  const trend =
    textValue(payload, AgentProtocolDefaults.Field.ManagedState) ??
    textValue(payload, AgentProtocolDefaults.Field.CapabilityStatus) ??
    unavailableTrend(input.connectionState);
  const readyCount = eventCount(managed, evidence);
  return row(
    PARENT_PORTAL_SERVICE_STATE.Label.BrowserActivity,
    4,
    eventScore(managed ?? evidence),
    readyCount,
    readyCount > 0 ? 0 : 1,
    trend,
    PARENT_PORTAL_SERVICE_STATE.Area.Browser,
    'gold'
  );
}

function activityReportsRow(input: ParentPortalServiceStateInput): ParentPortalRow {
  const events = activityEvents(input.events);
  const payload = events.at(-1)?.payload ?? null;
  const stored = numberValue(payload, AgentProtocolDefaults.Field.EventsStored) ?? 0;
  const returned = numberValue(payload, AgentProtocolDefaults.Field.Returned) ?? 0;
  const readyCount = events.length + stored + returned;
  const trend = activityTrend(payload, input.connectionState, events.length);
  return row(
    PARENT_PORTAL_SERVICE_STATE.Label.ActivityReports,
    5,
    scoreForCount(readyCount),
    readyCount,
    readyCount > 0 ? 0 : 1,
    trend,
    PARENT_PORTAL_SERVICE_STATE.Area.Activity,
    'red'
  );
}

function networkTrackingRow(input: ParentPortalServiceStateInput): ParentPortalRow {
  const network = latestEvent(input.events, AgentEvent.NetworkFlowReadModelReported);
  const activityNetwork = latestEvent(input.events, AgentEvent.ActivityNetworkReadModelReported);
  const latest = network ?? activityNetwork;
  const returned = numberValue(latest?.payload ?? null, AgentProtocolDefaults.Field.Returned) ?? 0;
  const trend =
    textValue(latest?.payload ?? null, AgentProtocolDefaults.Field.ActivitySurfaceState) ??
    textValue(latest?.payload ?? null, AgentProtocolDefaults.Field.CapabilityStatus) ??
    unavailableTrend(input.connectionState);
  return row(
    PARENT_PORTAL_SERVICE_STATE.Label.NetworkTracking,
    6,
    eventScore(latest),
    returned,
    latest === null ? 1 : 0,
    trend,
    PARENT_PORTAL_SERVICE_STATE.Area.Network,
    'purple'
  );
}

function activityEvents(events: readonly AgentEventEnvelope[]): AgentEventEnvelope[] {
  return [
    latestEvent(events, AgentEvent.ActivityIngestStatusReported),
    latestEvent(events, AgentEvent.ActivityRecentSummaryReported),
    latestEvent(events, AgentEvent.ActivityReportGenerated),
    latestEvent(events, AgentEvent.ActivityReportSaved),
    latestEvent(events, AgentEvent.ActivityReportHistoryReported),
    latestEvent(events, AgentEvent.ActivityScreenReadModelReported),
    latestEvent(events, AgentEvent.ActivityAppUseReadModelReported),
    latestEvent(events, AgentEvent.ActivityBrowserReadModelReported),
    latestEvent(events, AgentEvent.ActivityGamesReadModelReported),
  ].filter((event): event is AgentEventEnvelope => event !== null);
}

function latestEvent(events: readonly AgentEventEnvelope[], eventName: AgentEventName): AgentEventEnvelope | null {
  for (let index = events.length - 1; index >= 0; index -= 1) {
    if (events[index]?.event === eventName) {
      return events[index] ?? null;
    }
  }
  return null;
}

function row(
  label: ParentPortalRow['label'],
  order: ParentPortalRow['order'],
  signalScore: ParentPortalRow['signalScore'],
  readyCount: NonNullable<ParentPortalRow['readyCount']>,
  gapCount: NonNullable<ParentPortalRow['gapCount']>,
  trend: NonNullable<ParentPortalRow['trend']>,
  primaryArea: NonNullable<ParentPortalRow['primaryArea']>,
  tone: ParentPortalTone
): ParentPortalRow {
  return {
    label,
    order,
    signalScore,
    readyCount,
    gapCount,
    primaryArea,
    trend,
    tone,
  };
}

function manualRequiredTrend(
  connectionState: ParentPortalServiceConnectionState
): NonNullable<ParentPortalRow['trend']> {
  if (connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected) {
    return PARENT_PORTAL_SERVICE_STATE.Trend.ManualRequired;
  }
  return PARENT_PORTAL_SERVICE_STATE.Trend.Offline;
}

function unavailableTrend(connectionState: ParentPortalServiceConnectionState): NonNullable<ParentPortalRow['trend']> {
  if (connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected) {
    return PARENT_PORTAL_SERVICE_STATE.Trend.Unavailable;
  }
  return PARENT_PORTAL_SERVICE_STATE.Trend.Offline;
}

function eventScore(event: AgentEventEnvelope | null): ParentPortalRow['signalScore'] {
  return event === null ? 0 : 100;
}

function scoreForCount(count: number): ParentPortalRow['signalScore'] {
  return count > 0 ? 100 : 0;
}

function eventCount(...events: Array<AgentEventEnvelope | null>): number {
  return events.filter((event) => event !== null).length;
}

function activityTrend(
  payload: AgentProtocolLogFields | null,
  connectionState: ParentPortalServiceConnectionState,
  sourceCount: number
): NonNullable<ParentPortalRow['trend']> {
  return (
    textValue(payload, AgentProtocolDefaults.Field.ActivitySurfaceState) ??
    (sourceCount > 0 ? PARENT_PORTAL_SERVICE_STATE.Trend.Reported : unavailableTrend(connectionState))
  );
}

function textValue(payload: AgentProtocolLogFields | null, field: string): string | null {
  const value = payload?.[field];
  return typeof value === 'string' ? value : null;
}

function presentText(payload: AgentProtocolLogFields | null, field: string): string | null {
  const value = textValue(payload, field);
  return value === null || value === PARENT_PORTAL_SERVICE_STATE.Empty ? null : value;
}

function numberValue(payload: AgentProtocolLogFields | null, field: string): number | null {
  const value = payload?.[field];
  return typeof value === 'number' && Number.isFinite(value) ? value : null;
}
