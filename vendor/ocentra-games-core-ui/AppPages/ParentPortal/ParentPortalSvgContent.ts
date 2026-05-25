// @ts-nocheck
import {
  ParentPortalContentDataSchema,
  type ParentPortalContentRow,
  type ParentPortalControlArea,
  type ParentPortalGuideNote,
  type ParentPortalGuidePage,
  type ParentPortalGuideTopic,
  type ParentPortalIconName,
  type ParentPortalModeContent,
  type ParentPortalNavGroup,
  type ParentPortalNavItem,
  type ParentPortalContentData,
  type ParentPortalMode,
  type ParentPortalQuickControl,
  type ParentPortalSeason,
  type ParentPortalTab,
  type ParentPortalTabDetail,
  type ParentPortalTabId,
  type ParentPortalTone,
  type PartialParentPortalContentData,
} from '@ocentra/game-asset-domain/schemas/parent-portal-page-content-schema';

export type {
  ParentPortalContentRow,
  ParentPortalControlArea,
  ParentPortalGuideNote,
  ParentPortalGuidePage,
  ParentPortalGuideTopic,
  ParentPortalIconName,
  ParentPortalModeContent,
  ParentPortalNavGroup,
  ParentPortalNavItem,
  ParentPortalContentData,
  ParentPortalMode,
  ParentPortalQuickControl,
  ParentPortalSeason,
  ParentPortalTab,
  ParentPortalTabDetail,
  ParentPortalTabId,
  ParentPortalTone,
  PartialParentPortalContentData,
};

export const DEFAULT_PARENT_PORTAL_CONTENT: ParentPortalContentData = {
  tabs: [
    { id: 'overall', label: 'OVERVIEW', title: 'PARENT PORTAL OVERVIEW' },
    { id: 'controls', label: 'CONTROLS', title: 'CONTROL DETAIL VIEW' },
    { id: 'aiStatus', label: 'AI OVERALL', title: 'LOCAL AI STATUS' },
    { id: 'routines', label: 'ROUTINES', title: 'INSTALL AND UPDATES' },
    { id: 'support', label: 'SUPPORT', title: 'ACCOUNT AND TRUST' },
  ],
  navGroups: [
    { id: 'today', label: 'TODAY', detail: 'Current state' },
    { id: 'guide', label: 'GUIDE', detail: 'Setup views' },
    { id: 'routines', label: 'EVENTS', detail: 'Operations' },
  ],
  navItems: [
    { label: 'OVERVIEW', detail: 'Daily family snapshot', icon: 'overview', tabId: 'overall', groupId: 'today' },
    {
      label: 'OVERVIEW',
      detail: 'Family overview route',
      icon: 'overview',
      tabId: 'overall',
      groupId: 'guide',
    },
    { label: 'CONTROL AREAS', detail: 'Selected control area', icon: 'portal', tabId: 'controls', groupId: 'guide' },
    { label: 'CONTROL GROUPS', detail: 'Controls by category', icon: 'activity', tabId: 'controls', groupId: 'guide' },
    { label: 'AI OVERALL', detail: 'Local model status', icon: 'ai-setup', tabId: 'aiStatus', groupId: 'guide' },
    { label: 'AI SETUP', detail: 'Provider setup', icon: 'api', tabId: 'aiStatus', groupId: 'guide' },
    {
      label: 'AI MEMORY',
      detail: 'Memory and evidence',
      icon: 'ai-memory',
      tabId: 'aiStatus',
      groupId: 'guide',
    },
    {
      label: 'INSTALL AND UPDATES',
      detail: 'Installer and update state',
      icon: 'updates',
      tabId: 'routines',
      groupId: 'routines',
    },
    { label: 'ACCOUNT', detail: 'Parent account and trust', icon: 'account', tabId: 'support', groupId: 'routines' },
  ],
  tabDetails: {
    overall: {
      eyebrow: 'Parent portal',
      title: 'Overview',
      summary: 'Current portal route, connection state, and family control summary.',
      primary: 'Live status',
      secondary: 'Route state, device status, and support gaps',
      action: 'Open overview',
      tone: 'cyan',
    },
    controls: {
      eyebrow: 'Manage drilldown',
      title: 'Manage controls',
      summary: 'Control tiles and quick access cards route to current parent portal sections.',
      primary: 'Control scope',
      secondary: 'Selected route and typed support context',
      action: 'Open control',
      tone: 'purple',
    },
    aiStatus: {
      eyebrow: 'Local AI guide',
      title: 'AI setup status',
      summary: 'AI-vs-Local AI guide rows track model score, run volume, and control coverage.',
      primary: 'AI boundary',
      secondary: 'Provider and memory status',
      action: 'Open AI setup',
      tone: 'red',
    },
    routines: {
      eyebrow: 'Platform track',
      title: 'Install and updates',
      summary: 'Install status, update readiness, and platform proof gaps.',
      primary: 'Platform scope',
      secondary: 'Install and update routes',
      action: 'Open updates',
      tone: 'gold',
    },
    support: {
      eyebrow: 'Trust state',
      title: 'Account and trust',
      summary: 'Parent identity, trust boundaries, and support routing for this console.',
      primary: 'Account scope',
      secondary: 'Identity and support status',
      action: 'Open account',
      tone: 'cyan',
    },
  },
  controlAreas: [
    {
      id: 'browser-safety',
      order: 1,
      name: 'Web Safety',
      matches: 'Ready',
      growth: 'Now',
      tone: 'gold',
      category: 'Guide',
      subcategory: 'Browser and safety',
      controlCode: 1,
      routePath: '#/browser',
    },
    {
      id: 'policy-rules',
      order: 2,
      name: 'Policy',
      matches: 'Rules',
      growth: 'Next',
      tone: 'cyan',
      category: 'Manage',
      subcategory: 'Rules and decisions',
      controlCode: 3,
      routePath: '#/policy',
    },
    {
      id: 'activity-audit',
      order: 3,
      name: 'Activity',
      matches: 'Events',
      growth: 'Live',
      tone: 'red',
      category: 'Today',
      subcategory: 'Stored activity',
      controlCode: 4,
      routePath: '#/activity',
    },
    {
      id: 'device-control',
      order: 4,
      name: 'Devices',
      matches: 'Inventory',
      growth: 'Scoped',
      tone: 'purple',
      category: 'Manage',
      subcategory: 'Device inventory',
      controlCode: 5,
      routePath: '#/devices',
    },
    {
      id: 'support-diagnostics',
      order: 5,
      name: 'Support',
      matches: 'Logs',
      growth: 'Grounded',
      tone: 'muted',
      category: 'Support',
      subcategory: 'Diagnostics and logs',
      controlCode: 6,
      routePath: '#/diagnostics',
    },
  ],
  quickControls: [
    {
      id: 'browser-safety',
      name: 'WEB SAFETY',
      detail: 'Managed web route',
      icon: 'web',
      tone: 'gold',
      category: 'Guide',
      subcategory: 'Browser and safety',
      controlCode: 1,
      routePath: '#/browser',
    },
    {
      id: 'policy-rules',
      name: 'POLICY',
      detail: 'Policy route',
      icon: 'rules',
      tone: 'purple',
      category: 'Manage',
      subcategory: 'Rules and decisions',
      controlCode: 3,
      routePath: '#/policy',
    },
    {
      id: 'activity-audit',
      name: 'ACTIVITY',
      detail: 'Activity route',
      icon: 'activity',
      tone: 'red',
      category: 'Today',
      subcategory: 'Stored activity',
      controlCode: 4,
      routePath: '#/activity',
    },
    {
      id: 'device-control',
      name: 'DEVICES',
      detail: 'Device route',
      icon: 'devices',
      tone: 'cyan',
      category: 'Manage',
      subcategory: 'Device inventory',
      controlCode: 5,
      routePath: '#/devices',
    },
    {
      id: 'support-diagnostics',
      name: 'SUPPORT',
      detail: 'Support route',
      icon: 'portal',
      tone: 'muted',
      category: 'Support',
      subcategory: 'Diagnostics and logs',
      controlCode: 6,
      routePath: '#/diagnostics',
    },
    {
      id: 'local-ai',
      name: 'LOCAL AI',
      detail: 'Local AI route',
      icon: 'ai-setup',
      tone: 'red',
      category: 'Guide',
      subcategory: 'AI setup and memory',
      controlCode: 7,
      routePath: '#/ai-runtime',
    },
  ],
  guideTopics: [],
  fallbackRows: [
    {
      label: 'Overview route',
      order: 1,
      signalScore: 98,
      readyCount: 24,
      gapCount: 0,
      primaryArea: 'Dashboard',
      trend: 'ready',
      tone: 'gold',
    },
    {
      label: 'Managed web',
      order: 2,
      signalScore: 94,
      readyCount: 18,
      gapCount: 2,
      primaryArea: 'Browser',
      trend: 'ready',
      tone: 'cyan',
    },
    {
      label: 'Policy rules',
      order: 3,
      signalScore: 91,
      readyCount: 16,
      gapCount: 3,
      primaryArea: 'Policy',
      trend: 'typed',
      tone: 'red',
    },
    {
      label: 'Activity audit',
      order: 4,
      signalScore: 88,
      readyCount: 14,
      gapCount: 4,
      primaryArea: 'Activity',
      trend: 'local',
      tone: 'purple',
    },
    {
      label: 'Device inventory',
      order: 5,
      signalScore: 84,
      readyCount: 12,
      gapCount: 5,
      primaryArea: 'Devices',
      trend: 'scoped',
      tone: 'cyan',
    },
    {
      label: 'Support export',
      order: 6,
      signalScore: 78,
      readyCount: 10,
      gapCount: 6,
      primaryArea: 'Diagnostics',
      trend: 'manual',
      tone: 'muted',
    },
  ],
  aiBenchmarkRows: [
    {
      label: 'local-ai-setup',
      order: 1,
      signalScore: 96,
      readyCount: 18,
      gapCount: 1,
      primaryArea: 'Local AI',
      trend: 'private',
      tone: 'red',
    },
    {
      label: 'memory-evidence',
      order: 2,
      signalScore: 92,
      readyCount: 16,
      gapCount: 2,
      primaryArea: 'Memory',
      trend: 'cited',
      tone: 'gold',
    },
    {
      label: 'api-provider-boundary',
      order: 3,
      signalScore: 86,
      readyCount: 14,
      gapCount: 3,
      primaryArea: 'API providers',
      trend: 'planned',
      tone: 'purple',
    },
    {
      label: 'policy-decision',
      order: 4,
      signalScore: 82,
      readyCount: 12,
      gapCount: 4,
      primaryArea: 'Policy decision',
      trend: 'audited',
      tone: 'cyan',
    },
  ],
  distributionLabels: ['DIAMOND 15.2%', 'PLATINUM 26.1%', 'GOLD 28.7%', 'SILVER 17.1%'],
  season: {
    label: 'ALPHA',
    title: 'PARENT PORTAL',
    dateRange: 'LOCAL FIRST',
    actionLabel: 'INSTALL PROOF',
    detailTitle: 'PLATFORM READINESS',
    detailSubtitle: 'Install state, update proof, and support readiness',
    stats: [
      { label: 'SERVICE', value: 'LOCAL' },
      { label: 'EXPORT', value: 'MANUAL' },
      { label: 'LAN', value: 'PLANNED' },
    ],
  },
  metricLabels: {
    controlAreas: 'TOTAL AREAS',
    devices: 'TOTAL GAMES',
    readyPaths: 'READY PATHS',
    events: 'EVENTS',
    season: 'SEASON',
    updated: 'UPDATED',
  },
  uiCopy: {
    hubTitle: 'PARENT PORTAL HUB',
    controlAreasTitle: 'CONTROL AREAS',
    distributionTitle: 'ROUTE COVERAGE',
    distributionCenterLabel: 'TOTAL AREAS',
    feedTitle: 'LIVE SUPPORT FEED',
    liveLabel: 'LIVE',
    viewAllLabel: 'VIEW ALL',
    refreshLabel: 'REFRESH',
    queueLabel: 'QUEUE',
    showLabel: 'SHOW',
    pageLabel: 'PAGE',
    selectedRowLabel: 'SELECTED AREA',
    detailSnapshotTitle: 'CONTROL SNAPSHOT',
    detailSnapshotLines: [
      'Current route state, real-data support, and planned/unavailable gaps are grouped here for review.',
      'Icon, frame, and evidence positions are reserved for parent portal artwork and status proof.',
    ],
    loadingTitle: 'LOADING PARENT PORTAL',
    loadingBody: 'Refreshing parent portal state.',
    errorTitle: 'PARENT PORTAL UNAVAILABLE',
  },
  modes: {
    parentOverview: {
      defaultTab: 'overall',
      selectedControlId: 'browser-safety',
      title: 'Parent Portal',
      routeLabel: '#/overview',
      rowSource: 'api',
    },
    parentManage: {
      defaultTab: 'controls',
      selectedControlId: 'browser-safety',
      title: 'Manage Controls',
      routeLabel: '#/browser',
      rowSource: 'api',
    },
    parentGuide: {
      defaultTab: 'aiStatus',
      selectedControlId: 'ai-benchmarks',
      title: 'AI Setup Guide',
      routeLabel: '#/ai-runtime',
      rowSource: 'aiBenchmarkRows',
    },
  },
};

type JsonRecord = Record<string, unknown>;

function asRecord(value: unknown): JsonRecord {
  return value && typeof value === 'object' && !Array.isArray(value) ? (value as JsonRecord) : {};
}

function cloneContent(): ParentPortalContentData {
  return JSON.parse(JSON.stringify(DEFAULT_PARENT_PORTAL_CONTENT)) as ParentPortalContentData;
}

function mergeKnownValue<T>(fallback: T, source: unknown): T {
  if (Array.isArray(fallback)) {
    return Array.isArray(source) ? (source as T) : fallback;
  }
  if (fallback && typeof fallback === 'object') {
    const fallbackRecord = fallback as JsonRecord;
    const sourceRecord = asRecord(source);
    const merged: JsonRecord = { ...fallbackRecord };
    for (const [key, value] of Object.entries(fallbackRecord)) {
      merged[key] = mergeKnownValue(value, sourceRecord[key]);
    }
    return merged as T;
  }
  return typeof source === typeof fallback ? (source as T) : fallback;
}

export function normalizeParentPortalContent(content?: PartialParentPortalContentData | null): ParentPortalContentData {
  const merged = mergeKnownValue(cloneContent(), content);
  return ParentPortalContentDataSchema.parse(merged);
}

export function parseParentPortalContent(content: unknown): ParentPortalContentData {
  return ParentPortalContentDataSchema.parse(content);
}

function normalizeId(value?: string): string {
  return (value ?? '').trim().toLowerCase();
}

export function resolveParentPortalControlCode(
  content: PartialParentPortalContentData | ParentPortalContentData | null | undefined,
  pageMode: ParentPortalMode,
  controlId?: string
): number | undefined {
  const normalizedContent = normalizeParentPortalContent(content);
  const routeId = normalizeId(controlId);
  const modeControlId = normalizeId(normalizedContent.modes[pageMode].selectedControlId);
  const candidates = [...normalizedContent.controlAreas, ...normalizedContent.quickControls];
  const selected =
    candidates.find((control) => normalizeId(control.id) === routeId) ??
    candidates.find((control) => normalizeId(control.id) === modeControlId) ??
    candidates.find((control) => typeof control.controlCode === 'number');
  return typeof selected?.controlCode === 'number' ? selected.controlCode : undefined;
}
