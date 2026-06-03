import { PortalRoute, type PortalRoute as PortalRouteValue } from './routes';
import {
  PARENT_PORTAL_NAV_LABELS,
  PARENT_PORTAL_NAV_GROUPS,
  PARENT_PORTAL_NAV_ITEMS,
  type ParentPortalHashRoutePath,
  type ParentPortalNavGroup,
  type ParentPortalNavItem,
  type ParentPortalNavLabel,
} from './parent-portal-nav';
import { PARENT_PORTAL_GUIDE_TOPICS, type ParentPortalGuideTopic } from './parent-portal-guides';
import { PARENT_PORTAL_MANAGE_QUICK_CONTROLS, PARENT_PORTAL_MANAGE_ROWS } from './parent-portal-manage-data';

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
export type ParentPortalRowSource = 'api' | 'fallbackRows' | 'aiBenchmarkRows';
export type ParentPortalPageMode = 'parentOverview' | 'parentManage' | 'parentGuide';

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
  fallbackRows: ParentPortalRow[];
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
};

export const PARENT_PORTAL_ROUTE = {
  ClassName: 'parent-portal-route',
  PageMode: 'parentOverview',
  EmptyTimestamp: '',
  HashRoutes: {
    Assistant: '#/assistant',
    Overview: '#/overview',
  },
  StatusText: {
    Local: 'LOCAL',
    Connecting: 'CONNECTING',
    CheckService: 'CHECK SERVICE',
    Offline: 'OFFLINE',
  },
} as const;

export const PARENT_PORTAL_ROUTE_CONTEXT: Readonly<Partial<Record<PortalRouteValue, ParentPortalRouteContext>>> = {
  [PortalRoute.Overview]: routeContext('parentOverview', PARENT_PORTAL_NAV_LABELS.Overview, 'activity-store'),
  [PortalRoute.Assistant]: routeContext('parentGuide', PARENT_PORTAL_NAV_LABELS.AiSetup, 'ai-runtime'),
  [PortalRoute.Start]: routeContext('parentOverview', PARENT_PORTAL_NAV_LABELS.StartHere, 'setup-overall'),
  [PortalRoute.Activity]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Activity, 'reports-settings'),
  [PortalRoute.Browser]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Web, 'managed-web'),
  [PortalRoute.BrowserSettings]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Browser, 'browser-settings'),
  [PortalRoute.Policy]: routeContext('parentGuide', PARENT_PORTAL_NAV_LABELS.RulesGuide, 'rules-policy'),
  [PortalRoute.PolicyApps]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Apps, 'policy-apps'),
  [PortalRoute.PolicyGames]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Games, 'policy-games'),
  [PortalRoute.PolicyScreen]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Screen, 'screen-analysis'),
  [PortalRoute.PolicyNetwork]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Network, 'network-activity'),
  [PortalRoute.PolicyTracking]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Tracking, 'policy-tracking'),
  [PortalRoute.PolicyRemoteScreen]: routeContext(
    'parentManage',
    PARENT_PORTAL_NAV_LABELS.RemoteScreen,
    'policy-remote-screen'
  ),
  [PortalRoute.RuleManagement]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.RuleSet, 'rules-management'),
  [PortalRoute.Schedules]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Schedules, 'schedules-budgets'),
  [PortalRoute.Approvals]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Approvals, 'approvals'),
  [PortalRoute.Enforcement]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Enforce, 'enforcement-readiness'),
  [PortalRoute.PrivacyDesign]: routeContext('parentGuide', PARENT_PORTAL_NAV_LABELS.Private, 'privacy-design'),
  [PortalRoute.Memory]: routeContext('parentGuide', PARENT_PORTAL_NAV_LABELS.MemoryGuide, 'memory-citations'),
  [PortalRoute.MemorySettings]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.AiMemory, 'memory-settings'),
  [PortalRoute.AiGuide]: routeContext('parentGuide', PARENT_PORTAL_NAV_LABELS.Ai, 'local-ai-evidence'),
  [PortalRoute.AiRuntime]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.AiMemory, 'ai-runtime'),
  [PortalRoute.ApiProviders]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.AiMemory, 'api-providers'),
  [PortalRoute.ReportsGuide]: routeContext('parentGuide', PARENT_PORTAL_NAV_LABELS.ReportsGuide, 'reports-summaries'),
  [PortalRoute.ScreenAnalysis]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Activity, 'reports-settings'),
  [PortalRoute.AppGameSessions]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Activity, 'app-game-sessions'),
  [PortalRoute.NetworkActivity]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Activity, 'reports-settings'),
  [PortalRoute.Devices]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Devices, 'lan-pairing'),
  [PortalRoute.LanPairing]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Devices, 'lan-pairing'),
  [PortalRoute.CapabilityStatus]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Devices, 'lan-pairing'),
  [PortalRoute.Notifications]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Portal, 'notifications'),
  [PortalRoute.NotificationChannels]: routeContext(
    'parentManage',
    PARENT_PORTAL_NAV_LABELS.Portal,
    'notification-channels'
  ),
  [PortalRoute.DriveConnections]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.DataPrivacy, 'drive-exports'),
  [PortalRoute.ExportRetention]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.DataPrivacy, 'export-retention'),
  [PortalRoute.RemoteAccess]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.DataPrivacy, 'remote-access'),
  [PortalRoute.ReportCompiler]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Activity, 'reports-settings'),
  [PortalRoute.AuditHistory]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.DataPrivacy, 'audit-history'),
  [PortalRoute.Subscription]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Account, 'subscription-plans'),
  [PortalRoute.Entitlements]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Account, 'entitlements'),
  [PortalRoute.PlatformsInstall]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Devices, 'lan-pairing'),
  [PortalRoute.InstallUpdates]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Devices, 'lan-pairing'),
  [PortalRoute.Diagnostics]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Account, 'support-api-status'),
  [PortalRoute.SettingsRules]: routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.Portal, 'family-settings'),
} as const;

export function parentPortalRouteContext(route: PortalRouteValue): ParentPortalRouteContext {
  return (
    PARENT_PORTAL_ROUTE_CONTEXT[route] ??
    routeContext('parentOverview', PARENT_PORTAL_NAV_LABELS.Overview, 'managed-web')
  );
}

export const PARENT_PORTAL_ROWS: ParentPortalRow[] = [
  {
    label: 'Supported Browsers',
    order: 1,
    signalScore: 4928,
    readyCount: 24,
    gapCount: 0,
    primaryArea: 'Managed Web',
    trend: 'Local',
    tone: 'gold',
  },
  {
    label: 'Unsupported Browsers',
    order: 2,
    signalScore: 3640,
    readyCount: 18,
    gapCount: 2,
    primaryArea: 'Browser Gap',
    trend: 'Review',
    tone: 'cyan',
  },
  {
    label: 'Block or Allow',
    order: 3,
    signalScore: 3215,
    readyCount: 16,
    gapCount: 3,
    primaryArea: 'Policy Action',
    trend: 'Ready',
    tone: 'red',
  },
  {
    label: 'App Sessions',
    order: 4,
    signalScore: 2980,
    readyCount: 14,
    gapCount: 4,
    primaryArea: 'APP AND GAME SESSIONS',
    trend: '+2',
    tone: 'purple',
  },
  {
    label: 'Screen Analysis',
    order: 5,
    signalScore: 2865,
    readyCount: 12,
    gapCount: 5,
    primaryArea: 'SCREEN ANALYSIS',
    trend: '+1',
    tone: 'cyan',
  },
  {
    label: 'Rule Builder',
    order: 6,
    signalScore: 2754,
    readyCount: 10,
    gapCount: 6,
    primaryArea: 'Family Rules',
    trend: 'Draft',
    tone: 'gold',
  },
  {
    label: 'Device Pairing',
    order: 7,
    signalScore: 2645,
    readyCount: 8,
    gapCount: 7,
    primaryArea: 'Child Device',
    trend: 'Local',
    tone: 'purple',
  },
  {
    label: 'Drive Exports',
    order: 8,
    signalScore: 2523,
    readyCount: 6,
    gapCount: 8,
    primaryArea: 'Parent Owned',
    trend: 'Opt in',
    tone: 'cyan',
  },
  {
    label: 'Notifications',
    order: 9,
    signalScore: 2400,
    readyCount: 4,
    gapCount: 1,
    primaryArea: 'Parent Alerts',
    trend: 'Opt in',
    tone: 'red',
  },
  {
    label: 'Private by Design',
    order: 10,
    signalScore: 2320,
    readyCount: 4,
    gapCount: 0,
    primaryArea: 'Data Custody',
    trend: 'Local',
    tone: 'gold',
  },
];

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
      routePath: '#/browser',
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
      routePath: '#/browser',
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
      routePath: '#/policy',
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
      routePath: '#/activity',
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
      routePath: '#/drive-connections',
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
      routePath: '#/privacy-design',
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
      routePath: '#/memory',
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
      routePath: '#/notifications',
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
      routePath: '#/settings-rules',
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
      routePath: '#/api-providers',
    },
    {
      id: 'device-pairing',
      order: 11,
      name: 'Device Pairing',
      matches: 'Trusted',
      growth: 'Per child',
      tone: 'cyan',
      category: 'Devices',
      subcategory: 'Pairing and status',
      controlCode: 11,
      routePath: '#/devices',
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
      routePath: '#/browser',
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
      routePath: '#/policy',
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
      routePath: '#/ai-runtime',
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
      routePath: '#/ai-runtime',
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
      routePath: '#/privacy-design',
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
      routePath: '#/memory',
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
      routePath: '#/diagnostics',
    },
  ],
  guideTopics: PARENT_PORTAL_GUIDE_TOPICS,
  fallbackRows: [...PARENT_PORTAL_ROWS, ...PARENT_PORTAL_MANAGE_ROWS],
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
      routeLabel: '#/overview',
      rowSource: 'fallbackRows',
    },
    parentManage: {
      defaultTab: 'controls',
      selectedControlId: 'managed-web',
      title: 'Control Detail',
      routeLabel: '#/browser',
      rowSource: 'fallbackRows',
    },
    parentGuide: {
      defaultTab: 'aiStatus',
      selectedControlId: 'api-providers',
      title: 'AI',
      routeLabel: '#/ai-runtime',
      rowSource: 'aiBenchmarkRows',
    },
  },
};

function routeContext(
  pageMode: ParentPortalPageMode,
  navLabel: ParentPortalNavLabel,
  selectedControlId: string
): ParentPortalRouteContext {
  return { pageMode, navLabel, selectedControlId };
}
