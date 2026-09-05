import { PortalRoute, type PortalRoute as PortalRouteValue } from './portal-contract-adapter';
import {
  GeneratedPortalAgentEvent as AgentEvent,
  GeneratedPortalAgentProtocolField as ProtocolField,
} from './generated-portal-contracts';
import {
  generatedParentPortalManageLaneForRoute,
  generatedParentPortalRouteState,
  type GeneratedParentPortalManageLane,
  type GeneratedParentPortalPageMode,
} from './portal-route-state.generated';
import { portalRouteHashPath } from './routes';
import { PortalRoutes } from './routes';
import {
  PARENT_PORTAL_NAV_LABELS,
  PARENT_PORTAL_NAV_GROUPS,
  PARENT_PORTAL_NAV_ITEMS,
  type ParentPortalHashRoutePath,
  type ParentPortalNavGroup,
  type ParentPortalNavItem,
  type ParentPortalNavLabel,
} from './parent-portal-nav';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_GUIDE_TOPICS } from './parent-portal-guides';
import { PARENT_PORTAL_MANAGE_QUICK_CONTROLS } from './parent-portal-manage-data';

export type ParentPortalTone = 'cyan' | 'gold' | 'purple' | 'red' | 'muted';
export type ParentPortalTabId = 'overall' | 'controls' | 'aiStatus' | 'routines' | 'support';
export type ParentPortalIconName =
  | 'quick-glance'
  | 'overview'
  | 'start'
  | 'guide'
  | 'manage'
  | 'policy'
  | 'browser'
  | 'web'
  | 'schedule'
  | 'alerts'
  | 'report'
  | 'rules'
  | 'updates'
  | 'activity'
  | 'app'
  | 'games'
  | 'portal'
  | 'privacy'
  | 'lan'
  | 'devices'
  | 'screen'
  | 'remote'
  | 'ai-setup'
  | 'ai-guide'
  | 'ai-memory-set'
  | 'api'
  | 'export'
  | 'drives'
  | 'audit'
  | 'ai-memory'
  | 'account'
  | 'enforcement';
export type ParentPortalRowSource = 'api' | 'aiBenchmarkRows';
export type ParentPortalPageMode = GeneratedParentPortalPageMode;
export type ParentPortalManageLane = GeneratedParentPortalManageLane;
export type ParentPortalAssistantResponseKind = 'answer' | 'error' | 'unavailable';

export type ParentPortalAssistantEventSnapshot = Readonly<{
  event?: string | null;
  eventId?: string | null;
  payload?: Readonly<Record<string, unknown>> | null;
}>;

export type ParentPortalAssistantResponse = Readonly<{
  eventId: string;
  kind: ParentPortalAssistantResponseKind;
  state: string;
  text: string;
}>;

export function latestParentAssistantResponse(
  events: readonly ParentPortalAssistantEventSnapshot[]
): ParentPortalAssistantResponse | null {
  for (const event of events) {
    const response = parentAssistantResponseFromEvent(event);
    if (response !== null) {
      return response;
    }
  }
  return null;
}

function parentAssistantResponseFromEvent(
  event: ParentPortalAssistantEventSnapshot
): ParentPortalAssistantResponse | null {
  const eventId = nonEmptyAssistantValue(event.eventId);
  if (eventId === null) {
    return null;
  }

  if (event.event === AgentEvent.ParentAssistantAnswerReported) {
    return answerResponse(event, eventId);
  }

  if (event.event === AgentEvent.ParentAssistantProviderDegraded) {
    return {
      eventId,
      kind: 'unavailable',
      state: 'provider-degraded',
      text:
        assistantPayloadValue(event, ProtocolField.LocalAiUnavailableReason) ??
        assistantPayloadValue(event, ProtocolField.Reason) ??
        'The MIA provider is unavailable.',
    };
  }

  if (event.event === AgentEvent.ParentAssistantErrorReported) {
    return {
      eventId,
      kind: 'error',
      state: 'error',
      text: assistantPayloadValue(event, ProtocolField.Reason) ?? 'MIA could not complete the request.',
    };
  }

  return null;
}

function answerResponse(event: ParentPortalAssistantEventSnapshot, eventId: string): ParentPortalAssistantResponse {
  const answerText = assistantPayloadValue(event, ProtocolField.ParentAssistantAnswerText);
  const answerState = assistantPayloadValue(event, ProtocolField.ParentAssistantAnswerState) ?? 'unavailable';
  if (answerText !== null) {
    return { eventId, kind: 'answer', state: answerState, text: answerText };
  }
  return {
    eventId,
    kind: 'unavailable',
    state: answerState,
    text:
      assistantPayloadValue(event, ProtocolField.LocalAiUnavailableReason) ??
      assistantPayloadValue(event, ProtocolField.Reason) ??
      'MIA returned no displayable answer.',
  };
}

function assistantPayloadValue(event: ParentPortalAssistantEventSnapshot, field: string): string | null {
  return nonEmptyAssistantValue(event.payload?.[field]);
}

function nonEmptyAssistantValue(value: unknown): string | null {
  if (typeof value !== 'string') {
    return null;
  }
  const normalized = value.trim();
  return normalized.length > 0 ? normalized : null;
}

export type ParentPortalRow = {
  label: string;
  order: number;
  signalScore: number;
  readyCount?: number;
  gapCount?: number;
  primaryArea?: string;
  trend?: string;
  tone?: ParentPortalTone;
};

export type ParentPortalContent = {
  tabs: Array<{
    id: ParentPortalTabId;
    label: string;
    title: string;
  }>;
  navGroups: readonly ParentPortalNavGroup[];
  navItems: readonly ParentPortalNavItem[];
  tabDetails: Record<
    ParentPortalTabId,
    {
      eyebrow: string;
      title: string;
      summary: string;
      primary: string;
      secondary: string;
      action: string;
      tone: ParentPortalTone;
    }
  >;
  controlAreas: Array<{
    id: string;
    order: number;
    name: string;
    matches: string;
    growth: string;
    tone: ParentPortalTone;
    category: string;
    subcategory: string;
    controlCode: number;
    routePath: ParentPortalHashRoutePath;
  }>;
  quickControls: Array<{
    id: string;
    name: string;
    detail: string;
    icon: ParentPortalIconName;
    tone: ParentPortalTone;
    category: string;
    subcategory: string;
    controlCode: number;
    routePath: ParentPortalHashRoutePath;
  }>;
  guideTopics: readonly ParentPortalGuideTopic[];
  aiBenchmarkRows: ParentPortalRow[];
  distributionLabels: string[];
  season: {
    label: string;
    title: string;
    dateRange: string;
    actionLabel: string;
    detailTitle: string;
    detailSubtitle: string;
    stats: Array<{ label: string; value: string }>;
  };
  metricLabels: {
    controlAreas: string;
    devices: string;
    readyPaths: string;
    events: string;
    season: string;
    updated: string;
  };
  uiCopy: {
    hubTitle: string;
    controlAreasTitle: string;
    distributionTitle: string;
    distributionCenterLabel: string;
    feedTitle: string;
    liveLabel: string;
    viewAllLabel: string;
    refreshLabel: string;
    queueLabel: string;
    showLabel: string;
    pageLabel: string;
    selectedRowLabel: string;
    detailSnapshotTitle: string;
    detailSnapshotLines: string[];
    loadingTitle: string;
    loadingBody: string;
    errorTitle: string;
  };
  modes: {
    parentOverview: {
      defaultTab: ParentPortalTabId;
      selectedControlId: string;
      title: string;
      routeLabel: string;
      rowSource: ParentPortalRowSource;
    };
    parentManage: {
      defaultTab: ParentPortalTabId;
      selectedControlId: string;
      title: string;
      routeLabel: string;
      rowSource: ParentPortalRowSource;
    };
    parentGuide: {
      defaultTab: ParentPortalTabId;
      selectedControlId: string;
      title: string;
      routeLabel: string;
      rowSource: ParentPortalRowSource;
    };
  };
};

export type ParentPortalRouteContext = {
  readonly pageMode: ParentPortalPageMode;
  readonly navLabel: ParentPortalNavLabel;
  readonly selectedControlId: string;
  readonly manageLane: ParentPortalManageLane | null;
};

export const PARENT_PORTAL_ROUTE = {
  ClassName: 'parent-portal-route',
  PageMode: 'parentOverview',
  EmptyTimestamp: '',
  HashRoutes: {
    Assistant: portalRouteHashPath(PortalRoute.Assistant),
    Overview: portalRouteHashPath(PortalRoute.Overview),
  },
  StatusText: {
    Local: 'LOCAL',
    Connecting: 'CONNECTING',
    CheckService: 'CHECK SERVICE',
    Offline: 'OFFLINE',
  },
} as const;

const PARENT_PORTAL_ROUTE_NAV_LABELS: Readonly<Record<PortalRouteValue, ParentPortalNavLabel>> = {
  [PortalRoute.Overview]: PARENT_PORTAL_NAV_LABELS.Overview,
  [PortalRoute.Assistant]: PARENT_PORTAL_NAV_LABELS.Ai,
  [PortalRoute.Start]: PARENT_PORTAL_NAV_LABELS.StartHere,
  [PortalRoute.Activity]: PARENT_PORTAL_NAV_LABELS.Activity,
  [PortalRoute.Browser]: PARENT_PORTAL_NAV_LABELS.Web,
  [PortalRoute.BrowserSettings]: PARENT_PORTAL_NAV_LABELS.Browser,
  [PortalRoute.Policy]: PARENT_PORTAL_NAV_LABELS.RulesGuide,
  [PortalRoute.PolicyApps]: PARENT_PORTAL_NAV_LABELS.Apps,
  [PortalRoute.PolicyGames]: PARENT_PORTAL_NAV_LABELS.Games,
  [PortalRoute.PolicyScreen]: PARENT_PORTAL_NAV_LABELS.Screen,
  [PortalRoute.PolicyNetwork]: PARENT_PORTAL_NAV_LABELS.Network,
  [PortalRoute.PolicyTracking]: PARENT_PORTAL_NAV_LABELS.Tracking,
  [PortalRoute.PolicyRemoteScreen]: PARENT_PORTAL_NAV_LABELS.RemoteScreen,
  [PortalRoute.RuleManagement]: PARENT_PORTAL_NAV_LABELS.RuleSet,
  [PortalRoute.Schedules]: PARENT_PORTAL_NAV_LABELS.Schedules,
  [PortalRoute.Approvals]: PARENT_PORTAL_NAV_LABELS.Approvals,
  [PortalRoute.Enforcement]: PARENT_PORTAL_NAV_LABELS.Enforce,
  [PortalRoute.PrivacyDesign]: PARENT_PORTAL_NAV_LABELS.Private,
  [PortalRoute.Memory]: PARENT_PORTAL_NAV_LABELS.MemoryGuide,
  [PortalRoute.MemorySettings]: PARENT_PORTAL_NAV_LABELS.AiMemory,
  [PortalRoute.AiGuide]: PARENT_PORTAL_NAV_LABELS.Ai,
  [PortalRoute.AiRuntime]: PARENT_PORTAL_NAV_LABELS.AiMemory,
  [PortalRoute.ApiProviders]: PARENT_PORTAL_NAV_LABELS.AiMemory,
  [PortalRoute.ReportsGuide]: PARENT_PORTAL_NAV_LABELS.ReportsGuide,
  [PortalRoute.ScreenAnalysis]: PARENT_PORTAL_NAV_LABELS.Activity,
  [PortalRoute.AppGameSessions]: PARENT_PORTAL_NAV_LABELS.AppsGames,
  [PortalRoute.NetworkActivity]: PARENT_PORTAL_NAV_LABELS.Activity,
  [PortalRoute.Devices]: PARENT_PORTAL_NAV_LABELS.Devices,
  [PortalRoute.LanPairing]: PARENT_PORTAL_NAV_LABELS.Devices,
  [PortalRoute.CapabilityStatus]: PARENT_PORTAL_NAV_LABELS.Capability,
  [PortalRoute.Notifications]: PARENT_PORTAL_NAV_LABELS.Portal,
  [PortalRoute.NotificationChannels]: PARENT_PORTAL_NAV_LABELS.Portal,
  [PortalRoute.DriveConnections]: PARENT_PORTAL_NAV_LABELS.DataPrivacy,
  [PortalRoute.ExportRetention]: PARENT_PORTAL_NAV_LABELS.DataPrivacy,
  [PortalRoute.RemoteAccess]: PARENT_PORTAL_NAV_LABELS.Remote,
  [PortalRoute.ReportCompiler]: PARENT_PORTAL_NAV_LABELS.Activity,
  [PortalRoute.AuditHistory]: PARENT_PORTAL_NAV_LABELS.DataPrivacy,
  [PortalRoute.Subscription]: PARENT_PORTAL_NAV_LABELS.Account,
  [PortalRoute.Entitlements]: PARENT_PORTAL_NAV_LABELS.Account,
  [PortalRoute.PlatformsInstall]: PARENT_PORTAL_NAV_LABELS.Platforms,
  [PortalRoute.InstallUpdates]: PARENT_PORTAL_NAV_LABELS.Updates,
  [PortalRoute.Diagnostics]: PARENT_PORTAL_NAV_LABELS.Diagnostics,
  [PortalRoute.ProofPanels]: PARENT_PORTAL_NAV_LABELS.ProofPanels,
  [PortalRoute.SettingsRules]: PARENT_PORTAL_NAV_LABELS.Portal,
  [PortalRoute.AppLayout]: PARENT_PORTAL_NAV_LABELS.AppLayout,
  [PortalRoute.Commands]: PARENT_PORTAL_NAV_LABELS.Commands,
  [PortalRoute.Events]: PARENT_PORTAL_NAV_LABELS.Events,
  [PortalRoute.Logs]: PARENT_PORTAL_NAV_LABELS.Logs,
  [PortalRoute.FrameTuner]: PARENT_PORTAL_NAV_LABELS.AppLayout,
} as const;

export const PARENT_PORTAL_ROUTE_CONTEXT: Readonly<Partial<Record<PortalRouteValue, ParentPortalRouteContext>>> =
  Object.fromEntries(
    PortalRoutes.flatMap((route) => {
      const routeState = generatedParentPortalRouteState(route);
      const navLabel = PARENT_PORTAL_ROUTE_NAV_LABELS[route];
      return routeState === null || navLabel === undefined
        ? []
        : [
            [
              route,
              {
                pageMode: routeState.pageMode,
                navLabel,
                selectedControlId: routeState.selectedControlId,
                manageLane: routeState.manageLane,
              },
            ],
          ];
    })
  ) as Readonly<Partial<Record<PortalRouteValue, ParentPortalRouteContext>>>;

export function parentPortalRouteContext(route: PortalRouteValue): ParentPortalRouteContext {
  return (
    PARENT_PORTAL_ROUTE_CONTEXT[route] ??
    routeContext('parentOverview', PARENT_PORTAL_NAV_LABELS.Overview, 'managed-web')
  );
}

export function parentPortalManageLaneForRoute(route: PortalRouteValue): ParentPortalManageLane | null {
  return generatedParentPortalManageLaneForRoute(route);
}

export const PARENT_PORTAL_CONTENT: ParentPortalContent = {
  tabs: [
    { id: 'overall', label: 'OVERVIEW', title: 'PARENT COMMAND OVERVIEW' },
    { id: 'controls', label: 'CONTROL AREAS', title: 'DEVICE CONTROL AREAS' },
    { id: 'aiStatus', label: 'AI', title: 'AI READINESS' },
    { id: 'routines', label: 'SCHEDULES', title: 'SCHEDULES AND APPROVALS' },
    { id: 'support', label: 'SUPPORT', title: 'SUPPORT AND EXPORTS' },
  ],
  navGroups: PARENT_PORTAL_NAV_GROUPS,
  navItems: PARENT_PORTAL_NAV_ITEMS,
  tabDetails: {
    overall: {
      eyebrow: 'Family command',
      title: 'Your house, your rules',
      summary: 'See local child-device state, recent evidence, and which controls are ready to wire.',
      primary: 'Local device visibility',
      secondary: 'No cloud sharing by default',
      action: 'Review today',
      tone: 'cyan',
    },
    controls: {
      eyebrow: 'Control surface',
      title: 'Browser, app, and policy controls',
      summary: 'Choose which areas the parent can configure per child device.',
      primary: 'Per-device rules',
      secondary: 'Allow, ask, explain, schedule, or block',
      action: 'Open controls',
      tone: 'gold',
    },
    aiStatus: {
      eyebrow: 'AI',
      title: 'Local models, API providers, and memory',
      summary: 'Use local AI first, allow API AI only when a parent chooses provider, data scope, and device policy.',
      primary: 'Evidence cited',
      secondary: 'Per-device model and provider choices',
      action: 'Open AI setup',
      tone: 'red',
    },
    routines: {
      eyebrow: 'Family routine',
      title: 'Schedules, approvals, and devices',
      summary: 'Pair each child device and apply family defaults or per-device overrides.',
      primary: 'Child device first',
      secondary: 'Rules stay auditable',
      action: 'Open settings',
      tone: 'purple',
    },
    support: {
      eyebrow: 'Parent owned',
      title: 'Support contact and parent help',
      summary: 'Support messages are parent-authored and sent only when the parent chooses.',
      primary: 'Private by design',
      secondary: 'Message only when parent chooses',
      action: 'Open support',
      tone: 'cyan',
    },
  },
  controlAreas: [
    {
      id: 'managed-web',
      order: 1,
      name: 'Managed Web',
      matches: 'Ready path',
      growth: 'Advisory',
      tone: 'gold',
      category: 'Browser',
      subcategory: 'Supported browsers',
      controlCode: 1,
      routePath: portalRouteHashPath(PortalRoute.Browser),
    },
    {
      id: 'browser-gap',
      order: 2,
      name: 'Browser Gap',
      matches: 'Visible',
      growth: 'Not configured',
      tone: 'cyan',
      category: 'Browser',
      subcategory: 'Unsupported browsers',
      controlCode: 2,
      routePath: portalRouteHashPath(PortalRoute.Browser),
    },
    {
      id: 'policy-action',
      order: 3,
      name: 'Policy Action',
      matches: 'Allow Ask Block',
      growth: 'Ready',
      tone: 'red',
      category: 'Policy',
      subcategory: 'Block or allow',
      controlCode: 3,
      routePath: portalRouteHashPath(PortalRoute.Policy),
    },
    {
      id: 'activity-store',
      order: 4,
      name: 'Activity Store',
      matches: 'Local only',
      growth: 'Evidence cited',
      tone: 'purple',
      category: 'Activity',
      subcategory: 'Recent events',
      controlCode: 4,
      routePath: portalRouteHashPath(PortalRoute.Activity),
    },
    {
      id: 'drive-exports',
      order: 5,
      name: 'Drive Exports',
      matches: 'Parent owned',
      growth: 'Opt in',
      tone: 'cyan',
      category: 'Support',
      subcategory: 'Connect your drives',
      controlCode: 5,
      routePath: portalRouteHashPath(PortalRoute.DriveConnections),
    },
    {
      id: 'privacy-design',
      order: 6,
      name: 'Private by Design',
      matches: 'Local first',
      growth: 'No cloud share',
      tone: 'gold',
      category: 'Privacy',
      subcategory: 'Data custody',
      controlCode: 6,
      routePath: portalRouteHashPath(PortalRoute.PrivacyDesign),
    },
    {
      id: 'memory-citations',
      order: 7,
      name: 'Memory Citations',
      matches: 'Cited links',
      growth: 'Freshness gated',
      tone: 'purple',
      category: 'Memory',
      subcategory: 'Local knowledge',
      controlCode: 7,
      routePath: portalRouteHashPath(PortalRoute.Memory),
    },
    {
      id: 'notifications',
      order: 8,
      name: 'Notifications',
      matches: 'Parent only',
      growth: 'Opt in',
      tone: 'red',
      category: 'Devices',
      subcategory: 'Parent alerts',
      controlCode: 8,
      routePath: portalRouteHashPath(PortalRoute.Notifications),
    },
    {
      id: 'family-settings',
      order: 9,
      name: 'Family Settings',
      matches: 'Defaults',
      growth: 'Per device',
      tone: 'cyan',
      category: 'Devices',
      subcategory: 'Family defaults',
      controlCode: 9,
      routePath: portalRouteHashPath(PortalRoute.SettingsRules),
    },
    {
      id: 'api-providers',
      order: 10,
      name: 'API Providers',
      matches: 'Optional',
      growth: 'Parent scoped',
      tone: 'purple',
      category: 'AI',
      subcategory: 'External AI setup',
      controlCode: 10,
      routePath: portalRouteHashPath(PortalRoute.ApiProviders),
    },
  ],
  quickControls: [
    {
      id: 'managed-web',
      name: 'MANAGED WEB',
      detail: 'Supported browser path',
      icon: 'web',
      tone: 'gold',
      category: 'Browser',
      subcategory: 'Supported browsers',
      controlCode: 1,
      routePath: portalRouteHashPath(PortalRoute.Browser),
    },
    ...PARENT_PORTAL_MANAGE_QUICK_CONTROLS,
    {
      id: 'policy-action',
      name: 'POLICY ACTION',
      detail: 'Allow, ask, explain, block',
      icon: 'rules',
      tone: 'red',
      category: 'Policy',
      subcategory: 'Rules and approvals',
      controlCode: 3,
      routePath: portalRouteHashPath(PortalRoute.Policy),
    },
    {
      id: 'local-ai',
      name: 'LOCAL AI',
      detail: 'On-device evaluator',
      icon: 'ai-setup',
      tone: 'purple',
      category: 'AI',
      subcategory: 'Evidence summaries',
      controlCode: 6,
      routePath: portalRouteHashPath(PortalRoute.AiRuntime),
    },
    {
      id: 'local-ai-hub',
      name: 'LOCAL AI HUB',
      detail: 'Shared home model queue',
      icon: 'ai-setup',
      tone: 'cyan',
      category: 'AI',
      subcategory: 'Local hub',
      controlCode: 6,
      routePath: portalRouteHashPath(PortalRoute.AiRuntime),
    },
    {
      id: 'privacy-design',
      name: 'PRIVATE BY DESIGN',
      detail: 'Local-first custody',
      icon: 'privacy',
      tone: 'gold',
      category: 'Privacy',
      subcategory: 'Data custody',
      controlCode: 6,
      routePath: portalRouteHashPath(PortalRoute.PrivacyDesign),
    },
    {
      id: 'memory-citations',
      name: 'MEMORY CITATIONS',
      detail: 'Cited local knowledge',
      icon: 'ai-memory',
      tone: 'purple',
      category: 'Memory',
      subcategory: 'Freshness gated',
      controlCode: 7,
      routePath: portalRouteHashPath(PortalRoute.Memory),
    },
    {
      id: 'support-exports',
      name: 'SUPPORT EXPORTS',
      detail: 'Parent-owned bundles',
      icon: 'portal',
      tone: 'gold',
      category: 'Support',
      subcategory: 'Diagnostics and drives',
      controlCode: 8,
      routePath: portalRouteHashPath(PortalRoute.Diagnostics),
    },
  ],
  guideTopics: PARENT_PORTAL_GUIDE_TOPICS,
  aiBenchmarkRows: [
    {
      label: 'Local Models',
      order: 1,
      signalScore: 9812,
      readyCount: 18,
      gapCount: 1,
      primaryArea: 'Local AI',
      trend: 'Ready',
      tone: 'red',
    },
    {
      label: 'API Providers',
      order: 2,
      signalScore: 9381,
      readyCount: 16,
      gapCount: 2,
      primaryArea: 'External AI',
      trend: 'Optional',
      tone: 'purple',
    },
    {
      label: 'Local AI Hub',
      order: 3,
      signalScore: 9034,
      readyCount: 14,
      gapCount: 3,
      primaryArea: 'Model queue',
      trend: 'Planned',
      tone: 'gold',
    },
    {
      label: 'Cited Memory',
      order: 4,
      signalScore: 8810,
      readyCount: 12,
      gapCount: 4,
      primaryArea: 'Memory links',
      trend: 'Local',
      tone: 'purple',
    },
  ],
  distributionLabels: ['LOCAL 100%', 'CLOUD 0%', 'EXPORT OPT-IN', 'RAW PRIVATE'],
  season: {
    label: 'LOCAL',
    title: 'YOUR HOUSE YOUR RULE',
    dateRange: 'Child device service',
    actionLabel: 'SETTINGS',
    detailTitle: 'FAMILY DEFAULTS',
    detailSubtitle: 'Device rules, schedules, approvals, and drive exports',
    stats: [
      { label: 'DATA', value: 'LOCAL' },
      { label: 'EXPORT', value: 'OPT-IN' },
      { label: 'MODE', value: 'ADVISORY' },
    ],
  },
  metricLabels: {
    controlAreas: 'CONTROL AREAS',
    devices: 'DEVICES',
    readyPaths: 'READY PATHS',
    events: 'EVENTS',
    season: 'MODE',
    updated: 'UPDATED',
  },
  uiCopy: {
    hubTitle: 'PARENT HUB',
    controlAreasTitle: 'CONTROL AREAS',
    distributionTitle: 'DATA CUSTODY',
    distributionCenterLabel: 'LOCAL',
    feedTitle: 'LIVE DEVICE FEED',
    liveLabel: 'LIVE',
    viewAllLabel: 'VIEW ALL',
    refreshLabel: 'REFRESH',
    queueLabel: 'RECONNECT',
    showLabel: 'SHOW',
    pageLabel: 'PAGE',
    selectedRowLabel: 'SELECTED CONTROL',
    detailSnapshotTitle: 'CONTROL SNAPSHOT',
    detailSnapshotLines: [
      'Each parent control should map to a typed child-device event, policy decision, or setup gap.',
      'Evidence references stay local unless a parent chooses a diagnostic export or drive connection.',
    ],
    loadingTitle: 'LOADING PARENT SURFACE',
    loadingBody: 'Refreshing local child-device visibility.',
    errorTitle: 'PARENT SURFACE UNAVAILABLE',
  },
  modes: {
    parentOverview: {
      defaultTab: 'overall',
      selectedControlId: 'managed-web',
      title: 'Parent Command Deck',
      routeLabel: portalRouteHashPath(PortalRoute.Overview),
      rowSource: 'api',
    },
    parentManage: {
      defaultTab: 'controls',
      selectedControlId: 'managed-web',
      title: 'Control Detail',
      routeLabel: portalRouteHashPath(PortalRoute.Browser),
      rowSource: 'api',
    },
    parentGuide: {
      defaultTab: 'aiStatus',
      selectedControlId: 'api-providers',
      title: 'AI',
      routeLabel: portalRouteHashPath(PortalRoute.AiRuntime),
      rowSource: 'aiBenchmarkRows',
    },
  },
};

function routeContext(
  pageMode: ParentPortalPageMode,
  navLabel: ParentPortalNavLabel,
  selectedControlId: string
): ParentPortalRouteContext {
  return { pageMode, navLabel, selectedControlId, manageLane: null };
}
