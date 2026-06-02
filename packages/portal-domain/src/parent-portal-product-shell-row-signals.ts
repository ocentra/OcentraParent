import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import type { ParentPortalRow } from './parent-portal-data';
import type { ParentPortalServiceConnectionState, ParentPortalServiceStateInput } from './parent-portal-service-state';
import { PARENT_PORTAL_SERVICE_STATE } from './parent-portal-service-state-constants';
import type { ProductShellSignalKind } from './parent-portal-product-shell-row-specs';

export type ProductShellSignal = {
  readonly signalScore: ParentPortalRow['signalScore'];
  readonly readyCount: NonNullable<ParentPortalRow['readyCount']>;
  readonly gapCount: NonNullable<ParentPortalRow['gapCount']>;
  readonly trend: NonNullable<ParentPortalRow['trend']>;
};

export function productShellSignals(
  input: ParentPortalServiceStateInput
): Record<ProductShellSignalKind, ProductShellSignal> {
  return {
    household: householdSignal(input),
    browser: browserSignal(input),
    activity: activitySignal(input),
    network: networkSignal(input),
    policy: policySignal(input, manualRequiredTrend(input.connectionState)),
    remotePolicy: policySignal(input, backendNotConnectedTrend(input.connectionState)),
    assistant: assistantSignal(input),
    manual: manualSignal(input.connectionState),
  };
}

function householdSignal(input: ParentPortalServiceStateInput): ProductShellSignal {
  const event = latestEventOf(input.events, [
    AgentEvent.LanPairingStatusReported,
    AgentEvent.LanPairingBrowserDiscoveryReported,
    AgentEvent.LanPairingAddDeviceReported,
  ]);
  const payload = event?.payload ?? null;
  const readyCount = householdReadyCount(payload);
  const pendingCount = numberValue(payload, AgentProtocolDefaults.Field.LanPendingPairingCount) ?? 0;
  return {
    signalScore: eventScore(event),
    readyCount,
    gapCount: readyCount > 0 && pendingCount === 0 ? 0 : 1,
    trend: householdTrend(payload, input.connectionState),
  };
}

function householdReadyCount(payload: AgentProtocolLogFields | null): number {
  const trustedCount = numberValue(payload, AgentProtocolDefaults.Field.LanTrustedDeviceCount) ?? 0;
  const selectedCount = presentText(payload, AgentProtocolDefaults.Field.LanSelectedChildDeviceId) === null ? 0 : 1;
  const selectedReadyCount = payload?.[AgentProtocolDefaults.Field.LanSelectedDeviceReady] === true ? 1 : 0;
  return trustedCount + selectedCount + selectedReadyCount;
}

function householdTrend(
  payload: AgentProtocolLogFields | null,
  connectionState: ParentPortalServiceConnectionState
): NonNullable<ParentPortalRow['trend']> {
  if (payload?.[AgentProtocolDefaults.Field.LanSelectedDeviceReady] === true) {
    return 'controller';
  }
  return (
    textValue(payload, AgentProtocolDefaults.Field.LanParentAuthority) ??
    textValue(payload, AgentProtocolDefaults.Field.LanPairingState) ??
    textValue(payload, AgentProtocolDefaults.Field.LanAddDeviceState) ??
    manualRequiredTrend(connectionState)
  );
}

function browserSignal(input: ParentPortalServiceStateInput): ProductShellSignal {
  const managed = latestEvent(input.events, AgentEvent.BrowserManagedStatusReported);
  const evidence = latestEvent(input.events, AgentEvent.BrowserEvidenceRecentReported);
  const latest = managed ?? evidence;
  const readyCount = eventCount(managed, evidence);
  const payload = latest?.payload ?? null;
  return {
    signalScore: eventScore(latest),
    readyCount,
    gapCount: readyCount > 0 ? 0 : 1,
    trend:
      textValue(payload, AgentProtocolDefaults.Field.ManagedState) ??
      textValue(payload, AgentProtocolDefaults.Field.CapabilityStatus) ??
      unavailableTrend(input.connectionState),
  };
}

function activitySignal(input: ParentPortalServiceStateInput): ProductShellSignal {
  const events = activityEvents(input.events);
  const payload = events.at(-1)?.payload ?? null;
  const stored = numberValue(payload, AgentProtocolDefaults.Field.EventsStored) ?? 0;
  const returned = numberValue(payload, AgentProtocolDefaults.Field.Returned) ?? 0;
  const readyCount = events.length + stored + returned;
  return {
    signalScore: scoreForCount(readyCount),
    readyCount,
    gapCount: readyCount > 0 ? 0 : 1,
    trend:
      textValue(payload, AgentProtocolDefaults.Field.ActivitySurfaceState) ??
      (readyCount > 0 ? PARENT_PORTAL_SERVICE_STATE.Trend.Reported : unavailableTrend(input.connectionState)),
  };
}

function networkSignal(input: ParentPortalServiceStateInput): ProductShellSignal {
  const latest =
    latestEvent(input.events, AgentEvent.NetworkFlowReadModelReported) ??
    latestEvent(input.events, AgentEvent.ActivityNetworkReadModelReported);
  const payload = latest?.payload ?? null;
  const returned = numberValue(payload, AgentProtocolDefaults.Field.Returned) ?? 0;
  return {
    signalScore: eventScore(latest),
    readyCount: returned,
    gapCount: latest === null ? 1 : 0,
    trend:
      textValue(payload, AgentProtocolDefaults.Field.ActivitySurfaceState) ??
      textValue(payload, AgentProtocolDefaults.Field.CapabilityStatus) ??
      unavailableTrend(input.connectionState),
  };
}

function policySignal(
  input: ParentPortalServiceStateInput,
  missingTrend: NonNullable<ParentPortalRow['trend']>
): ProductShellSignal {
  const events = compactLatestEvents(input.events, [
    AgentEvent.PolicyPreviewReadModelReported,
    AgentEvent.BrowserPolicyReported,
    AgentEvent.BrowserPolicyPreviewed,
  ]);
  const payload = events.at(-1)?.payload ?? null;
  return {
    signalScore: eventScore(events.at(-1) ?? null),
    readyCount: events.length,
    gapCount: events.length > 0 ? 0 : 1,
    trend:
      textValue(payload, AgentProtocolDefaults.Field.PolicyHandoffState) ??
      textValue(payload, AgentProtocolDefaults.Field.PolicyAction) ??
      textValue(payload, AgentProtocolDefaults.Field.CapabilityStatus) ??
      (events.length > 0 ? PARENT_PORTAL_SERVICE_STATE.Trend.Reported : missingTrend),
  };
}

function assistantSignal(input: ParentPortalServiceStateInput): ProductShellSignal {
  const events = compactLatestEvents(input.events, [
    AgentEvent.LocalAiRuntimeStatusReported,
    AgentEvent.ParentAssistantAnswerReported,
    AgentEvent.ParentAssistantThreadUpdated,
    AgentEvent.ParentAssistantMessageAccepted,
    AgentEvent.ParentAssistantRunStarted,
    AgentEvent.ParentAssistantActionPreviewed,
    AgentEvent.ParentAssistantActionConfirmed,
    AgentEvent.ParentAssistantProviderDegraded,
    AgentEvent.ParentAssistantErrorReported,
  ]);
  const payload = events.at(-1)?.payload ?? null;
  return {
    signalScore: eventScore(events.at(-1) ?? null),
    readyCount: events.length,
    gapCount: events.length > 0 ? 0 : 1,
    trend:
      textValue(payload, AgentProtocolDefaults.Field.ParentAssistantBackendState) ??
      textValue(payload, AgentProtocolDefaults.Field.ParentAssistantProviderState) ??
      textValue(payload, AgentProtocolDefaults.Field.ParentAssistantAnswerState) ??
      (events.length > 0
        ? PARENT_PORTAL_SERVICE_STATE.Trend.Reported
        : backendNotConnectedTrend(input.connectionState)),
  };
}

function manualSignal(connectionState: ParentPortalServiceConnectionState): ProductShellSignal {
  return {
    signalScore: connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected ? 25 : 0,
    readyCount: 0,
    gapCount: 1,
    trend: manualRequiredTrend(connectionState),
  };
}

function activityEvents(events: readonly AgentEventEnvelope[]): AgentEventEnvelope[] {
  return compactLatestEvents(events, [
    AgentEvent.ActivityIngestStatusReported,
    AgentEvent.ActivityRecentSummaryReported,
    AgentEvent.ActivityReportGenerated,
    AgentEvent.ActivityReportSaved,
    AgentEvent.ActivityReportHistoryReported,
    AgentEvent.ActivityScreenReadModelReported,
    AgentEvent.ActivityAppUseReadModelReported,
    AgentEvent.ActivityBrowserReadModelReported,
    AgentEvent.ActivityGamesReadModelReported,
  ]);
}

function compactLatestEvents(
  events: readonly AgentEventEnvelope[],
  eventNames: readonly AgentEventName[]
): AgentEventEnvelope[] {
  return eventNames
    .map((eventName) => latestEvent(events, eventName))
    .filter((event): event is AgentEventEnvelope => event !== null);
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
  let latestIndex = -1;
  for (let index = 0; index < events.length; index += 1) {
    const event = events[index];
    if (event !== undefined && eventNames.includes(event.event)) {
      const sentAt = Date.parse(event.sentAt);
      const eventTime = Number.isFinite(sentAt) ? sentAt : index;
      if (eventTime > latestTime || (eventTime === latestTime && index > latestIndex)) {
        latest = event;
        latestTime = eventTime;
        latestIndex = index;
      }
    }
  }
  return latest;
}

function manualRequiredTrend(
  connectionState: ParentPortalServiceConnectionState
): NonNullable<ParentPortalRow['trend']> {
  return connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected
    ? PARENT_PORTAL_SERVICE_STATE.Trend.ManualRequired
    : PARENT_PORTAL_SERVICE_STATE.Trend.Offline;
}

function unavailableTrend(connectionState: ParentPortalServiceConnectionState): NonNullable<ParentPortalRow['trend']> {
  return connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected
    ? PARENT_PORTAL_SERVICE_STATE.Trend.Unavailable
    : PARENT_PORTAL_SERVICE_STATE.Trend.Offline;
}

function backendNotConnectedTrend(
  connectionState: ParentPortalServiceConnectionState
): NonNullable<ParentPortalRow['trend']> {
  return connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected
    ? PARENT_PORTAL_SERVICE_STATE.Trend.BackendNotConnected
    : PARENT_PORTAL_SERVICE_STATE.Trend.Offline;
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
