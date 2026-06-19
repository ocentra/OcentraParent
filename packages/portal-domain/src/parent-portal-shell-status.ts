import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PARENT_PORTAL_CONTENT, parentPortalRouteContext, type ParentPortalRow, type ParentPortalTone } from './parent-portal-data';
import { resolveLiveActivityState } from './live-activity-state';
import { PARENT_PORTAL_SERVICE_STATE } from './parent-portal-service-state';
import {
  resolveParentPortalServiceState,
  type ParentPortalServiceConnectionState,
} from './parent-portal-service-state';
import { PortalRoute, type PortalRoute as PortalRouteValue } from './routes';

export type PortalShellGlobalConnectionState =
  | 'online'
  | 'offline'
  | 'degraded'
  | 'stale'
  | 'manual-required'
  | 'unauthenticated';

export type PortalShellRouteCapabilityState =
  | 'available'
  | 'unavailable'
  | 'not-configured'
  | 'permission-missing'
  | 'platform-unsupported'
  | 'proof-missing';

export type PortalShellDataSourceLabel =
  | 'live local'
  | 'LAN'
  | 'relay'
  | 'parent cache'
  | 'parent-owned cloud'
  | 'Ocentra-hosted metadata'
  | 'unavailable';

export type PortalShellParentAccessState = 'active-controller' | 'observer-only' | 'unauthenticated' | 'proof-missing';
export type PortalShellHouseholdState = 'LAN household' | 'no household configured' | 'proof missing';
export type PortalShellChildDeviceState = 'selected' | 'no selected child device' | 'proof missing';

export interface ParentPortalShellStatusCard {
  readonly id: 'parent-access' | 'connection' | 'household' | 'child-device' | 'route-capability' | 'data-source';
  readonly label: string;
  readonly value: string;
  readonly detail: string;
  readonly tone: ParentPortalTone;
}

export interface ParentPortalShellStatus {
  readonly routeLabel: string;
  readonly parentAccessState: PortalShellParentAccessState;
  readonly globalConnectionState: PortalShellGlobalConnectionState;
  readonly routeCapabilityState: PortalShellRouteCapabilityState;
  readonly dataSourceLabel: PortalShellDataSourceLabel;
  readonly cards: readonly ParentPortalShellStatusCard[];
}

export interface ParentPortalShellStatusInput {
  readonly route: PortalRouteValue;
  readonly connectionState: ParentPortalServiceConnectionState;
  readonly events: readonly AgentEventEnvelope[];
}

export const PARENT_PORTAL_SHELL_STATUS_COPY = {
  Summary: 'Shell status',
  ParentAccess: 'Parent access',
  Connection: 'Connection',
  Household: 'Household',
  ChildDevice: 'Child device',
  RouteCapability: 'Route capability',
  DataSource: 'Data source',
} as const;

export const PARENT_PORTAL_SHELL_STATUS_DOM = {
  Panel: 'shell-status-panel',
  Grid: 'shell-status-grid',
  Card: 'shell-status-card',
  Detail: 'shell-status-detail',
  ToneCyan: 'shell-status-card-tone-cyan',
  ToneGold: 'shell-status-card-tone-gold',
  TonePurple: 'shell-status-card-tone-purple',
  ToneRed: 'shell-status-card-tone-red',
  ToneMuted: 'shell-status-card-tone-muted',
} as const;

const LanRoutes = new Set<PortalRouteValue>([
  PortalRoute.Devices,
  PortalRoute.LanPairing,
  PortalRoute.CapabilityStatus,
  PortalRoute.PlatformsInstall,
  PortalRoute.InstallUpdates,
]);

const LocalProtocolRoutes = new Set<PortalRouteValue>([PortalRoute.Commands, PortalRoute.Events, PortalRoute.Logs]);

const ParentOwnedCloudRoutes = new Set<PortalRouteValue>([
  PortalRoute.DriveConnections,
  PortalRoute.ExportRetention,
  PortalRoute.RemoteAccess,
]);

const HostedMetadataRoutes = new Set<PortalRouteValue>([PortalRoute.Subscription, PortalRoute.Entitlements]);
const ParentCacheRoutes = new Set<PortalRouteValue>([
  PortalRoute.Notifications,
  PortalRoute.NotificationChannels,
  PortalRoute.AuditHistory,
  PortalRoute.Diagnostics,
]);

type SelectableControl = {
  readonly id: string;
  readonly name: string;
};

export function resolveParentPortalShellStatus(input: ParentPortalShellStatusInput): ParentPortalShellStatus {
  const serviceState = resolveParentPortalServiceState({
    connectionState: input.connectionState,
    events: input.events,
  });
  const liveActivity = resolveLiveActivityState(input.events);
  const routeContext = parentPortalRouteContext(input.route);
  const selectedControl = selectableControl(routeContext.selectedControlId);
  const routeLabel = selectedControl?.name ?? String(routeContext.navLabel);
  const routeRow = selectedControl === null ? null : rowByPrimaryArea(serviceState.parentPortalRows, selectedControl.name);
  const latestLanEvent = latestLanPairingEvent(input.events);
  const lanPayload = latestLanEvent?.payload ?? null;
  const hasLanProof = liveActivity.lanAddDeviceReadModel !== null || latestLanEvent !== null;
  const parentAccessState = resolveParentAccessState(liveActivity, lanPayload, hasLanProof);
  const householdState = resolveHouseholdState(liveActivity, lanPayload, hasLanProof);
  const childDevice = resolveChildDeviceState(liveActivity, lanPayload, hasLanProof);
  const dataSourceLabel = resolveDataSourceLabel({
    connectionState: input.connectionState,
    liveActivity,
    route: input.route,
    routeRow,
  });
  const routeCapabilityState = resolveRouteCapabilityState({
    parentAccessState,
    routeRow,
    dataSourceLabel,
  });
  const globalConnectionState = resolveGlobalConnectionState({
    connectionState: input.connectionState,
    parentAccessState,
    routeCapabilityState,
    selectedDeviceReachability: childDevice.reachability,
  });
  const householdSources = liveActivity.lanAddDeviceReadModel?.scanSummary.sourceLabels ?? [];
  const householdDetail = hasLanProof
    ? `visible devices: ${householdVisibleDeviceCount(liveActivity, lanPayload)} | sources: ${joinLabels(householdSources)}`
    : 'service read-model proof is missing';
  const childDeviceDetail =
    childDevice.reachability === null
      ? `route: ${routeLabel}`
      : `reachability: ${childDevice.reachability} | route: ${routeLabel}`;
  const routeCapabilityDetail = `route: ${routeLabel}${routeRow === null ? '' : ` | trend: ${routeRow.trend ?? 'not-reported'}`}`;
  const dataSourceDetail = `route: ${routeLabel}${selectedControl === null ? '' : ` | control: ${selectedControl.id}`}`;

  return {
    routeLabel,
    parentAccessState,
    globalConnectionState,
    routeCapabilityState,
    dataSourceLabel,
    cards: [
      {
        id: 'parent-access',
        label: PARENT_PORTAL_SHELL_STATUS_COPY.ParentAccess,
        value: parentAccessState,
        detail: parentAccessDetail(liveActivity, lanPayload, hasLanProof),
        tone: parentAccessTone(parentAccessState),
      },
      {
        id: 'connection',
        label: PARENT_PORTAL_SHELL_STATUS_COPY.Connection,
        value: globalConnectionState,
        detail: `route: ${routeLabel}`,
        tone: connectionTone(globalConnectionState),
      },
      {
        id: 'household',
        label: PARENT_PORTAL_SHELL_STATUS_COPY.Household,
        value: householdState,
        detail: householdDetail,
        tone: householdTone(householdState),
      },
      {
        id: 'child-device',
        label: PARENT_PORTAL_SHELL_STATUS_COPY.ChildDevice,
        value: childDevice.value,
        detail: childDeviceDetail,
        tone: childDevice.tone,
      },
      {
        id: 'route-capability',
        label: PARENT_PORTAL_SHELL_STATUS_COPY.RouteCapability,
        value: routeCapabilityState,
        detail: routeCapabilityDetail,
        tone: routeCapabilityTone(routeCapabilityState),
      },
      {
        id: 'data-source',
        label: PARENT_PORTAL_SHELL_STATUS_COPY.DataSource,
        value: dataSourceLabel,
        detail: dataSourceDetail,
        tone: dataSourceTone(dataSourceLabel),
      },
    ],
  };
}

function resolveParentAccessState(
  liveActivity: ReturnType<typeof resolveLiveActivityState>,
  lanPayload: AgentProtocolLogFields | null,
  hasLanProof: boolean
): PortalShellParentAccessState {
  const authenticationState = textValue(lanPayload, AgentProtocolDefaults.Field.LanAuthenticationState);
  const parentAuthority = textValue(lanPayload, AgentProtocolDefaults.Field.LanParentAuthority);
  if (authenticationState === 'unauthenticated') {
    return 'unauthenticated';
  }
  if (
    liveActivity.lanAddDeviceReadModel?.controllerAuthority === 'active-controller' ||
    parentAuthority === 'active-controller'
  ) {
    return 'active-controller';
  }
  if (
    liveActivity.lanAddDeviceReadModel?.observerAuthority === 'observer' ||
    liveActivity.lanAddDeviceReadModel?.controllerAuthority === 'observer' ||
    parentAuthority === 'observer'
  ) {
    return 'observer-only';
  }
  return hasLanProof ? 'proof-missing' : 'proof-missing';
}

function parentAccessDetail(
  liveActivity: ReturnType<typeof resolveLiveActivityState>,
  lanPayload: AgentProtocolLogFields | null,
  hasLanProof: boolean
): string {
  if (!hasLanProof) {
    return 'service read-model proof is missing';
  }
  return `authority: ${
    liveActivity.lanAddDeviceReadModel?.controllerAuthority ??
    liveActivity.lanAddDeviceReadModel?.observerAuthority ??
    textValue(lanPayload, AgentProtocolDefaults.Field.LanParentAuthority) ??
    textValue(lanPayload, AgentProtocolDefaults.Field.LanAuthenticationState) ??
    'unknown'
  }`;
}

function resolveHouseholdState(
  liveActivity: ReturnType<typeof resolveLiveActivityState>,
  lanPayload: AgentProtocolLogFields | null,
  hasLanProof: boolean
): PortalShellHouseholdState {
  if (!hasLanProof) {
    return 'proof missing';
  }
  if (householdVisibleDeviceCount(liveActivity, lanPayload) > 0) {
    return 'LAN household';
  }
  return 'no household configured';
}

function resolveChildDeviceState(
  liveActivity: ReturnType<typeof resolveLiveActivityState>,
  lanPayload: AgentProtocolLogFields | null,
  hasLanProof: boolean
): { readonly value: string; readonly reachability: string | null; readonly tone: ParentPortalTone } {
  if (!hasLanProof) {
    return {
      value: 'proof missing',
      reachability: null,
      tone: 'muted',
    };
  }
  const selectedChildDeviceId =
    liveActivity.lanAddDeviceReadModel?.selectedDeviceReadiness.selectedChildDeviceId ??
    textValue(lanPayload, AgentProtocolDefaults.Field.LanSelectedChildDeviceId);
  const reachability =
    liveActivity.lanAddDeviceReadModel?.selectedDeviceReadiness.reachability ??
    textValue(lanPayload, AgentProtocolDefaults.Field.LanSelectedDeviceReachability);
  if (selectedChildDeviceId === null) {
    return {
      value: 'no selected child device',
      reachability,
      tone: 'gold',
    };
  }
  return {
    value: redactIdentifier(selectedChildDeviceId),
    reachability,
    tone: childDeviceTone(reachability),
  };
}

function resolveDataSourceLabel(input: {
  readonly connectionState: ParentPortalServiceConnectionState;
  readonly liveActivity: ReturnType<typeof resolveLiveActivityState>;
  readonly route: PortalRouteValue;
  readonly routeRow: ParentPortalRow | null;
}): PortalShellDataSourceLabel {
  if (LanRoutes.has(input.route) && hasLanSource(input.liveActivity)) {
    return 'LAN';
  }
  if (input.route === PortalRoute.RemoteAccess && input.liveActivity.networkRemoteDeliveryStatusEvent !== null) {
    return 'relay';
  }
  if (LocalProtocolRoutes.has(input.route) && input.connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Connected) {
    return 'live local';
  }
  if (hasRouteLocalServiceEvidence(input.route, input.liveActivity, input.routeRow)) {
    return 'live local';
  }
  if (ParentOwnedCloudRoutes.has(input.route)) {
    return 'parent-owned cloud';
  }
  if (HostedMetadataRoutes.has(input.route)) {
    return 'Ocentra-hosted metadata';
  }
  if (ParentCacheRoutes.has(input.route)) {
    return 'parent cache';
  }
  return 'unavailable';
}

function resolveRouteCapabilityState(input: {
  readonly parentAccessState: PortalShellParentAccessState;
  readonly routeRow: ParentPortalRow | null;
  readonly dataSourceLabel: PortalShellDataSourceLabel;
}): PortalShellRouteCapabilityState {
  if (input.parentAccessState === 'unauthenticated') {
    return 'permission-missing';
  }
  if (input.routeRow === null) {
    return input.dataSourceLabel === 'unavailable' ? 'proof-missing' : 'available';
  }
  const trend = (input.routeRow.trend ?? '').toLowerCase();
  if (trend.includes('unsupported')) {
    return 'platform-unsupported';
  }
  if (trend.includes('permission') || trend.includes('observer') || trend.includes('authority')) {
    return 'permission-missing';
  }
  if (
    trend.includes('not-claimed') ||
    trend.includes('report-only') ||
    trend.includes('target-list-only') ||
    trend.includes('bridge-missing') ||
    trend.includes('proof-missing')
  ) {
    return 'proof-missing';
  }
  if (trend.includes('manual-required') || trend.includes('unpaired') || trend.includes('not-configured')) {
    return 'not-configured';
  }
  if (
    trend.includes('offline') ||
    trend.includes('unavailable') ||
    trend.includes('backend-not-connected') ||
    trend.includes('adapter-error') ||
    trend.includes('stale')
  ) {
    return 'unavailable';
  }
  if (input.routeRow.readyCount === undefined || input.routeRow.readyCount < 1) {
    return input.dataSourceLabel === 'unavailable' ? 'not-configured' : 'proof-missing';
  }
  return 'available';
}

function resolveGlobalConnectionState(input: {
  readonly connectionState: ParentPortalServiceConnectionState;
  readonly parentAccessState: PortalShellParentAccessState;
  readonly routeCapabilityState: PortalShellRouteCapabilityState;
  readonly selectedDeviceReachability: string | null;
}): PortalShellGlobalConnectionState {
  if (input.connectionState !== PARENT_PORTAL_SERVICE_STATE.Connection.Connected) {
    return input.connectionState === PARENT_PORTAL_SERVICE_STATE.Connection.Error ? 'degraded' : 'offline';
  }
  if (input.parentAccessState === 'unauthenticated') {
    return 'unauthenticated';
  }
  if (input.selectedDeviceReachability === 'stale') {
    return 'stale';
  }
  if (input.routeCapabilityState === 'permission-missing' || input.routeCapabilityState === 'proof-missing') {
    return 'manual-required';
  }
  if (input.routeCapabilityState === 'not-configured') {
    return 'manual-required';
  }
  if (input.routeCapabilityState === 'unavailable' || input.routeCapabilityState === 'platform-unsupported') {
    return 'degraded';
  }
  return 'online';
}

function parentAccessTone(state: PortalShellParentAccessState): ParentPortalTone {
  switch (state) {
    case 'active-controller':
      return 'cyan';
    case 'observer-only':
      return 'gold';
    case 'unauthenticated':
      return 'red';
    case 'proof-missing':
      return 'muted';
  }
}

function connectionTone(state: PortalShellGlobalConnectionState): ParentPortalTone {
  switch (state) {
    case 'online':
      return 'cyan';
    case 'stale':
      return 'gold';
    case 'manual-required':
      return 'gold';
    case 'unauthenticated':
      return 'red';
    case 'degraded':
      return 'red';
    case 'offline':
      return 'muted';
  }
}

function householdTone(state: PortalShellHouseholdState): ParentPortalTone {
  switch (state) {
    case 'LAN household':
      return 'purple';
    case 'no household configured':
      return 'gold';
    case 'proof missing':
      return 'muted';
  }
}

function childDeviceTone(reachability: string | null): ParentPortalTone {
  if (reachability === 'online' || reachability === 'ready') {
    return 'cyan';
  }
  if (reachability === 'stale' || reachability === 'manual-required') {
    return 'gold';
  }
  if (reachability === 'offline') {
    return 'muted';
  }
  return 'purple';
}

function routeCapabilityTone(state: PortalShellRouteCapabilityState): ParentPortalTone {
  switch (state) {
    case 'available':
      return 'cyan';
    case 'not-configured':
      return 'gold';
    case 'permission-missing':
      return 'gold';
    case 'proof-missing':
      return 'muted';
    case 'platform-unsupported':
      return 'red';
    case 'unavailable':
      return 'red';
  }
}

function dataSourceTone(label: PortalShellDataSourceLabel): ParentPortalTone {
  switch (label) {
    case 'live local':
      return 'cyan';
    case 'LAN':
      return 'purple';
    case 'relay':
      return 'gold';
    case 'parent cache':
      return 'muted';
    case 'parent-owned cloud':
      return 'gold';
    case 'Ocentra-hosted metadata':
      return 'gold';
    case 'unavailable':
      return 'muted';
  }
}

function hasLanSource(liveActivity: ReturnType<typeof resolveLiveActivityState>): boolean {
  return liveActivity.lanAddDeviceReadModel !== null || liveActivity.lanPairingStatusEvent !== null;
}

function hasRouteLocalServiceEvidence(
  route: PortalRouteValue,
  liveActivity: ReturnType<typeof resolveLiveActivityState>,
  routeRow: ParentPortalRow | null
): boolean {
  if (routeRow !== null && (routeRow.readyCount ?? 0) > 0) {
    return true;
  }
  switch (route) {
    case PortalRoute.Browser:
    case PortalRoute.BrowserSettings:
      return (
        liveActivity.browserEvidenceEvent !== null ||
        liveActivity.browserInventoryEvent !== null ||
        liveActivity.browserManagedEvent !== null ||
        liveActivity.browserInterventionEvent !== null
      );
    case PortalRoute.Activity:
    case PortalRoute.ReportsGuide:
    case PortalRoute.ReportCompiler:
    case PortalRoute.ScreenAnalysis:
    case PortalRoute.NetworkActivity:
    case PortalRoute.AppGameSessions:
      return (
        liveActivity.recentSummaryEvent !== null ||
        liveActivity.activityReportEvent !== null ||
        liveActivity.activityScreenReadModelEvent !== null ||
        liveActivity.activityAppUseReadModelEvent !== null ||
        liveActivity.activityBrowserReadModelEvent !== null ||
        liveActivity.activityGamesReadModelEvent !== null ||
        liveActivity.activityNetworkReadModelEvent !== null ||
        liveActivity.networkFlowEvent !== null
      );
    case PortalRoute.Policy:
    case PortalRoute.PolicyApps:
    case PortalRoute.PolicyGames:
    case PortalRoute.PolicyScreen:
    case PortalRoute.PolicyNetwork:
    case PortalRoute.PolicyTracking:
    case PortalRoute.PolicyRemoteScreen:
    case PortalRoute.RuleManagement:
    case PortalRoute.Schedules:
    case PortalRoute.Approvals:
    case PortalRoute.Enforcement:
    case PortalRoute.SettingsRules:
      return liveActivity.policyPreviewEvent !== null || liveActivity.appGamePolicyReadinessEvent !== null;
    case PortalRoute.Overview:
    case PortalRoute.Start:
      return routeRow !== null;
    case PortalRoute.Assistant:
    case PortalRoute.AiGuide:
    case PortalRoute.AiRuntime:
    case PortalRoute.ApiProviders:
    case PortalRoute.Memory:
    case PortalRoute.MemorySettings:
      return (
        liveActivity.localAiRuntimeStatusEvent !== null ||
        liveActivity.parentAssistantBoundaryEvent !== null ||
        liveActivity.activityMemoryGraphEvent !== null
      );
    default:
      return false;
  }
}

function latestLanPairingEvent(events: readonly AgentEventEnvelope[]): AgentEventEnvelope | null {
  let latest: AgentEventEnvelope | null = null;
  let latestTime = Number.NEGATIVE_INFINITY;
  let latestIndex = -1;
  for (let index = 0; index < events.length; index += 1) {
    const event = events[index];
    if (
      event === undefined ||
      (event.event !== AgentEvent.LanPairingStatusReported &&
        event.event !== AgentEvent.LanPairingBrowserDiscoveryReported &&
        event.event !== AgentEvent.LanPairingAddDeviceReported)
    ) {
      continue;
    }
    const sentAt = Date.parse(event.sentAt);
    const eventTime = Number.isFinite(sentAt) ? sentAt : index;
    if (eventTime > latestTime || (eventTime === latestTime && index > latestIndex)) {
      latest = event;
      latestTime = eventTime;
      latestIndex = index;
    }
  }
  return latest;
}

function householdVisibleDeviceCount(
  liveActivity: ReturnType<typeof resolveLiveActivityState>,
  lanPayload: AgentProtocolLogFields | null
): number {
  const scanSummaryCount = liveActivity.lanAddDeviceReadModel?.scanSummary.scannedDeviceCount;
  if (scanSummaryCount !== undefined) {
    return scanSummaryCount;
  }
  const trustedCount = numberValue(lanPayload, AgentProtocolDefaults.Field.LanTrustedDeviceCount) ?? 0;
  const pendingCount = numberValue(lanPayload, AgentProtocolDefaults.Field.LanPendingPairingCount) ?? 0;
  const selectedCount = textValue(lanPayload, AgentProtocolDefaults.Field.LanSelectedChildDeviceId) === null ? 0 : 1;
  return Math.max(trustedCount, selectedCount) + pendingCount;
}

function selectableControl(controlId: string): SelectableControl | null {
  const control = [...PARENT_PORTAL_CONTENT.controlAreas, ...PARENT_PORTAL_CONTENT.quickControls].find(
    (candidate) => candidate.id === controlId
  );
  return control === undefined ? null : { id: control.id, name: control.name };
}

function rowByPrimaryArea(rows: readonly ParentPortalRow[], primaryArea: string): ParentPortalRow | null {
  const normalized = normalizedLabel(primaryArea);
  return (
    rows.find((row) => normalizedLabel(row.primaryArea ?? '') === normalized || normalizedLabel(row.label) === normalized) ??
    null
  );
}

function normalizedLabel(value: string): string {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, '');
}

function redactIdentifier(value: string): string {
  if (value.length <= 8) {
    return value;
  }
  return `${value.slice(0, 4)}...${value.slice(-4)}`;
}

function joinLabels(values: readonly string[]): string {
  return values.length > 0 ? values.join(', ') : 'unavailable';
}

function textValue(payload: AgentProtocolLogFields | null, field: string): string | null {
  const value = payload?.[field];
  return typeof value === 'string' ? value : null;
}

function numberValue(payload: AgentProtocolLogFields | null, field: string): number | null {
  const value = payload?.[field];
  return typeof value === 'number' && Number.isFinite(value) ? value : null;
}
